use std::collections::HashMap;
use std::io;
use std::ops::Index;
use std::ops::IndexMut;
use std::ops::RangeInclusive;

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

    fn run(&mut self, input: i64) -> Option<i64> {
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
                    self.store(self.mem[self.pc + 1], mode_op1, input);
                    self.pc += 2;
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

fn paint(program: &Vec<i64>, starting_color: i64) -> HashMap<(i32, i32), i64> {
    let mut panels = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut direction = 0;
    let mut cpu = CPU::new(&program);

    let directions = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    *panels.entry((x, y)).or_insert(0) = starting_color;

    while let Some(color) = cpu.run(*panels.get(&(x, y)).unwrap_or(&0)) {
        let entry = panels.entry((x, y)).or_insert(0);
        *entry = color;

        if let Some(d) = cpu.run(0) {
            if d == 0 {
                if direction == 0 {
                    direction = directions.len() - 1;
                } else {
                    direction -= 1;
                }
            } else {
                direction += 1;
            }
            direction %= directions.len();

            x += directions[direction].0;
            y += directions[direction].1;
        } else {
            panic!("no direction given by the program");
        }
    }

    panels
}

fn coord_range<F>(panels: &HashMap<(i32, i32), i64>, coord: F) -> RangeInclusive<i32>
where
    F: Fn(&(i32, i32)) -> i32,
{
    let min = panels.keys().map(&coord).min().unwrap();
    let max = panels.keys().map(&coord).max().unwrap();
    min..=max
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();

    println!("part 1: {}", paint(&program, 0).len());

    let panels = paint(&program, 1);

    println!("part 2:");
    for y in coord_range(&panels, |&(_, y)| y) {
        for x in coord_range(&panels, |&(x, _)| x) {
            print!(
                "{}",
                match panels.get(&(x, y)) {
                    Some(&color) if color == 1 => '#',
                    _ => ' ',
                }
            );
        }
        println!();
    }
}
