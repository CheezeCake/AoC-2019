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

#[derive(PartialEq)]
enum Status {
    Exit,
    Output(char),
    WaitingForInput,
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

    fn run(&mut self, mut input: Option<i64>) -> Status {
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
                        return Status::WaitingForInput;
                    }
                }
                4 => {
                    let output = self.load(self.mem[self.pc + 1], mode_op1);
                    self.pc += 2;
                    return Status::Output(output as u8 as char);
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

        Status::Exit
    }
}

fn run_command(cmd: &str, cpu: &mut CPU) {
    for c in cmd.bytes() {
        cpu.run(Some(c as i64));
    }
}

fn get_inventory(cpu: &mut CPU) -> Vec<String> {
    let mut items = Vec::new();

    run_command("inv\n", cpu);

    while let Status::Output(c) = cpu.run(None) {
        if c == '-' {
            cpu.run(None);
            let mut item = String::new();
            while let Status::Output(c) = cpu.run(None) {
                if c == '\n' {
                    break;
                }
                item.push(c);
            }
            items.push(item);
        }
    }

    items
}

fn drop(item: &str, cpu: &mut CPU) {
    run_command(&format!("drop {}\n", item), cpu);
    print_output(cpu);
}

fn take(item: &str, cpu: &mut CPU) {
    run_command(&format!("take {}\n", item), cpu);
    print_output(cpu);
}

fn print_output(cpu: &mut CPU) {
    while let Status::Output(c) = cpu.run(None) {
        print!("{}", c);
    }
}

fn try(cmd: &str, cpu: &mut CPU) -> bool {
    run_command(cmd, cpu);
    print_output(cpu);
    cpu.run(None) == Status::Exit
}

fn solve_helper(cur: usize, n: usize, items: &mut [String], cpu: &mut CPU) -> bool {
    if cur == n {
        println!("TRYING WITH:");
        run_command("inv\n", cpu);
        print_output(cpu);

        return try("east\n", cpu);
    }

    for i in cur..items.len() {
        items.swap(cur, i);
        take(&items[cur], cpu);

        if solve_helper(cur + 1, n, items, cpu) {
            return true;
        }

        drop(&items[cur], cpu);
        items.swap(cur, i);
    }

    false
}

fn solve(cpu: &mut CPU) {
    let mut inv = get_inventory(cpu);
    println!("inv = {:?}", inv);

    inv.iter().for_each(|item| drop(item, cpu));

    for n in 1..=inv.len() {
        if solve_helper(0, n, &mut inv, cpu) {
            break;
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

    let mut cpu = CPU::new(&program);
    loop {
        while let Status::Output(c) = cpu.run(None) {
            print!("{}", c);
        }

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input == String::from("solve\n") {
            solve(&mut cpu);
            break;
        }

        for c in input.bytes() {
            cpu.run(Some(c as i64));
        }
    }
}
