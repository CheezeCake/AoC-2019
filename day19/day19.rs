use std::collections::HashMap;
use std::collections::HashSet;
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

fn pulled(x: i64, y: i64, program: &Vec<i64>) -> bool {
    let mut cpu = CPU::new(program);
    cpu.run(Some(x));
    match cpu.run(Some(y)) {
        Some(0) => false,
        Some(1) => true,
        output => panic!("invalid ouput {:?}", output),
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

    println!(
        "part 1: {}",
        (0..50)
            .map(|y| (0..50)
                .map(|x| { pulled(x, y, &program) as usize })
                .sum::<usize>())
            .sum::<usize>()
    );

    let mut beam = HashSet::new();

    const SIZE: i64 = 100;

    let mut prev_y = 0;
    for x in 0.. {
        let mut y = prev_y;
        while !pulled(x, y, &program) {
            y += 1;
            if y - prev_y > 1000 {
                break;
            }
        }
        if y - prev_y > 1000 {
            continue;
        }

        prev_y = y;

        // print!("x = {}, y = {} -> ", x, y);
        let mut h = 0;
        while pulled(x, y, &program) {
            beam.insert((x, y));
            y += 1;
            h += 1;
        }
        // println!("{}, height = {}", y - 1, h);

        if h < SIZE || x < SIZE {
            continue;
        }

        let mut ok = true;
        for y_ in y - h..y - h + SIZE {
            for x_ in x - SIZE + 1..=x {
                if !beam.contains(&(x_, y_)) {
                    ok = false;
                    break;
                }
            }
            if !ok {
                break;
            }
        }
        if ok {
            println!("{}, {}", x - SIZE + 1, y - h);
            println!("part 2: {}", (x - SIZE + 1) * 10_000 + (y - h));
            break;
        }
    }
}
