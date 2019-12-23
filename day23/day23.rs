use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::ops::Index;
use std::ops::IndexMut;

struct Memory {
    mem: HashMap<usize, i64>,
}

impl Memory {
    fn new() -> Self {
        Self {
            mem: HashMap::new(),
        }
    }
}

impl Index<usize> for Memory {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        self.mem.get(&index).unwrap_or(&0)
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.mem.entry(index).or_insert(0)
    }
}

struct CPU {
    pc: usize,
    relative_offset: i64,
    mem: Memory,
}

enum CPUStatus {
    Output(i64),
    NoInput,
    Exit,
}

impl CPU {
    fn new(program: &Vec<i64>) -> Self {
        let mut mem = Memory::new();
        for (addr, &instr) in program.iter().enumerate() {
            mem[addr] = instr;
        }
        Self {
            pc: 0,
            relative_offset: 0,
            mem,
        }
    }

    fn load(&self, op: i64, mode: i64) -> i64 {
        match mode {
            0 => self.mem[op as usize],
            1 => op,
            2 => self.mem[(self.relative_offset + op) as usize],
            _ => panic!("invalid mode: {}", mode),
        }
    }

    fn store(&mut self, op: i64, mode: i64, value: i64) {
        let store = match mode {
            0 => op,
            1 => panic!("store instruction with immediate mode"),
            2 => self.relative_offset + op,
            _ => panic!("invalid mode: {}", mode),
        } as usize;

        self.mem[store] = value;
    }

    fn run(&mut self, input_queue: &mut VecDeque<i64>) -> CPUStatus {
        loop {
            let instr = self.mem[self.pc];
            let opcode = instr % 100;
            let mode_op1 = (instr / 100) % 10;
            let mode_op2 = (instr / 1000) % 10;
            let mode_op3 = instr / 10000;

            match opcode {
                1 | 2 => {
                    let op1 = self.mem[self.pc + 1];
                    let op2 = self.mem[self.pc + 2];

                    self.store(
                        self.mem[self.pc + 3],
                        mode_op3,
                        if opcode == 1 {
                            self.load(op1, mode_op1) + self.load(op2, mode_op2)
                        } else {
                            self.load(op1, mode_op1) * self.load(op2, mode_op2)
                        },
                    );

                    self.pc += 4;
                }
                3 => {
                    let input = input_queue.pop_front().unwrap_or(-1);
                    self.store(self.mem[self.pc + 1], mode_op1, input);
                    self.pc += 2;
                    return CPUStatus::NoInput;
                }
                4 => {
                    let output = self.load(self.mem[self.pc + 1], mode_op1);
                    self.pc += 2;
                    return CPUStatus::Output(output);
                }
                5 => {
                    self.pc = if self.load(self.mem[self.pc + 1], mode_op1) != 0 {
                        self.load(self.mem[self.pc + 2], mode_op2) as usize
                    } else {
                        self.pc + 3
                    }
                }
                6 => {
                    self.pc = if self.load(self.mem[self.pc + 1], mode_op1) == 0 {
                        self.load(self.mem[self.pc + 2], mode_op2) as usize
                    } else {
                        self.pc + 3
                    }
                }
                7 => {
                    self.store(
                        self.mem[self.pc + 3],
                        mode_op3,
                        (self.load(self.mem[self.pc + 1], mode_op1)
                            < self.load(self.mem[self.pc + 2], mode_op2))
                            as i64,
                    );
                    self.pc += 4
                }
                8 => {
                    self.store(
                        self.mem[self.pc + 3],
                        mode_op3,
                        (self.load(self.mem[self.pc + 1], mode_op1)
                            == self.load(self.mem[self.pc + 2], mode_op2))
                            as i64,
                    );
                    self.pc += 4
                }
                9 => {
                    self.relative_offset += self.load(self.mem[self.pc + 1], mode_op1);
                    self.pc += 2;
                }
                99 => break,
                _ => panic!("invalid opcode: {}", opcode),
            }
        }

        CPUStatus::Exit
    }
}

fn run_network_computers(n: usize, program: &Vec<i64>) -> (i64, i64) {
    let mut cpus = Vec::new();
    let mut queues = Vec::new();
    for addr in 0..n {
        cpus.push(CPU::new(&program));
        let mut q = VecDeque::new();
        q.push_back(addr as i64);
        queues.push(q);
    }

    let mut nat_received = Vec::new();
    let mut nat_sent = HashSet::new();

    loop {
        let mut idle = true;

        for addr in 0..n {
            if !queues[addr].is_empty() {
                idle = false;
            }

            if let CPUStatus::Output(dest) = cpus[addr].run(&mut queues[addr]) {
                let x = cpus[addr].run(&mut queues[addr]);
                let y = cpus[addr].run(&mut queues[addr]);

                idle = false;

                if let CPUStatus::Output(x) = x {
                    if let CPUStatus::Output(y) = y {
                        if dest == 255 {
                            nat_received.push((x, y));
                        } else {
                            queues[dest as usize].push_back(x);
                            queues[dest as usize].push_back(y);
                        }
                    }
                }
            }
        }

        if idle {
            if let Some(nat_packet) = nat_received.last() {
                let nat_packet = nat_packet.clone();
                queues[0].push_back(nat_packet.0);
                queues[0].push_back(nat_packet.1);
                if !nat_sent.insert(nat_packet) {
                    return (nat_received.first().unwrap().1, nat_packet.1);
                }
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();

    let (part1, part2) = run_network_computers(50, &program);
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
}
