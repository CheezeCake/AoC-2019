use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Clone)]
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

#[derive(Clone)]
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

    fn run(&mut self, input: i64) -> i64 {
        let mut output = 0;

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
                    output = self.load(self.mem[self.pc + 1], mode_op1);
                    self.pc += 2;
                    break;
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

        output
    }
}

fn get_area_map(program: &Vec<i64>) -> HashMap<(i32, i32), (i64, usize)> {
    let mut area_map = HashMap::new();

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let directions = [(0, 0), (0, 1), (0, -1), (-1, 0), (1, 0)];

    q.push_back((CPU::new(&program), (0, 0), 0));
    visited.insert((0, 0));

    while !q.is_empty() {
        let (cpu, (x, y), n) = q.pop_front().unwrap();

        for direction in 1..=4 {
            let new_pos = (
                x + directions[direction as usize].0,
                y + directions[direction as usize].1,
            );
            if visited.contains(&new_pos) {
                continue;
            }
            visited.insert(new_pos);

            let mut new_cpu = cpu.clone();
            let spot = new_cpu.run(direction);
            area_map.insert(new_pos, (spot, n + 1));

            if spot == 1 {
                q.push_back((new_cpu, new_pos, n + 1));
            }
        }
    }

    area_map
}

fn oxygen_filling(area_map: &HashMap<(i32, i32), (i64, usize)>) -> usize {
    let oxygen_pos = area_map
        .iter()
        .find(|(_, (loc, _))| *loc == 2)
        .expect("oxygen system not found")
        .0;

    let mut q = VecDeque::new();
    let mut visited = HashSet::new();
    let directions = [(0, 0), (0, 1), (0, -1), (-1, 0), (1, 0)];

    q.push_back((*oxygen_pos, 0));
    visited.insert(*oxygen_pos);

    let mut max = 0;

    while !q.is_empty() {
        let ((x, y), n) = q.pop_front().unwrap();

        max = cmp::max(max, n);

        for direction in 1..=4 {
            let new_pos = (
                x + directions[direction as usize].0,
                y + directions[direction as usize].1,
            );
            if visited.contains(&new_pos) {
                continue;
            }
            if area_map.get(&new_pos).unwrap_or(&(0, 0)).0 == 1 {
                visited.insert(new_pos);
                q.push_back((new_pos, n + 1));
            }
        }
    }

    max
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();

    let area_map = get_area_map(&program);
    println!(
        "part 1: {}",
        (area_map
            .iter()
            .find(|(_, (loc, _))| *loc == 2)
            .expect("oxygen system not found")
            .1)
            .1
    );

    println!("part 2: {}", oxygen_filling(&area_map));
}
