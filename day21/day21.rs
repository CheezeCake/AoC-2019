use std::collections::HashMap;
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

    fn run(&mut self, mut input: Option<i64>) -> Option<i64> {
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
                    if let Some(input_) = input {
                        self.store(self.mem[self.pc + 1], mode_op1, input_);
                        self.pc += 2;
                        input = None;
                    } else {
                        break;
                    }
                }
                4 => {
                    let output = self.load(self.mem[self.pc + 1], mode_op1);
                    self.pc += 2;
                    return Some(output);
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
                _ => panic!(format!("invalid opcode: {}", opcode)),
            }
        }

        None
    }
}

fn run_springscript(springscript_program: &[&str], program: &Vec<i64>) -> Option<i64> {
    let mut cpu = CPU::new(program);

    while let Some(_) = cpu.run(None) {}

    for instr in springscript_program {
        for &b in instr.as_bytes() {
            cpu.run(Some(b as i64));
        }
    }

    while let Some(output) = cpu.run(None) {
        if output > 128 {
            return Some(output);
        }
    }

    None
}

const PART1_SC: [&str; 6] = [
    "NOT C J\n",
    "AND D J\n",
    "NOT A T\n",
    "AND D T\n",
    "OR T J\n",
    "WALK\n",
];

const PART2_SC: [&str; 12] = [
    "NOT A J\n",
    "NOT B T\n",
    "OR T J\n",
    "NOT C T\n",
    "OR T J\n",
    "NOT D T\n",
    "NOT T T\n",
    "AND T J\n",
    "AND E T\n",
    "OR H T\n",
    "AND T J\n",
    "RUN\n",
];

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();

    println!(
        "part 1: {}",
        run_springscript(&PART1_SC, &program).expect("fail")
    );
    println!(
        "part 2: {}",
        run_springscript(&PART2_SC, &program).expect("fail")
    );
}
