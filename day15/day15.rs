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

enum PositionType {
    Wall,
    Empty,
    OxygenSystem,
}

type Position = (i32, i32);

fn adjacent_positions(p: Position) -> impl Iterator<Item = (Position, usize)> {
    [(0, 1), (0, -1), (-1, 0), (1, 0)]
        .iter()
        .enumerate()
        .map(move |(i, d)| ((p.0 + d.0, p.1 + d.1), i + 1))
}

fn get_area_map(program: &Vec<i64>) -> HashMap<Position, PositionType> {
    let mut area_map = HashMap::new();

    let mut q = VecDeque::new();
    let mut discovered = HashSet::new();

    q.push_back((CPU::new(&program), (0, 0)));
    discovered.insert((0, 0));

    while !q.is_empty() {
        let (cpu, pos) = q.pop_front().unwrap();
        for (new_pos, direction) in adjacent_positions(pos) {
            if discovered.contains(&new_pos) {
                continue;
            }
            discovered.insert(new_pos);

            let mut new_cpu = cpu.clone();
            let pt = new_cpu.run(direction as i64);
            area_map.insert(
                new_pos,
                match pt {
                    0 => PositionType::Wall,
                    1 => {
                        q.push_back((new_cpu, new_pos));
                        PositionType::Empty
                    }
                    2 => PositionType::OxygenSystem,
                    _ => panic!("invalid position type: {}", pt),
                },
            );
        }
    }

    area_map
}

fn distance_map(
    from: &Position,
    map: &HashMap<Position, PositionType>,
) -> HashMap<Position, usize> {
    let mut dist_map = HashMap::new();

    let mut q = VecDeque::new();
    let mut discovered = HashSet::new();

    q.push_back((*from, 0));
    discovered.insert(*from);

    while !q.is_empty() {
        let (pos, dist) = q.pop_front().unwrap();
        dist_map.insert(pos, dist);

        for (new_pos, _) in adjacent_positions(pos) {
            if discovered.contains(&new_pos) {
                continue;
            }
            discovered.insert(new_pos.clone());

            if let PositionType::Empty | PositionType::OxygenSystem =
                map.get(&new_pos).unwrap_or(&PositionType::Wall)
            {
                q.push_back((new_pos, dist + 1));
            }
        }
    }

    dist_map
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();

    let map = get_area_map(&program);
    let oxygen_system_position = map
        .iter()
        .find(|(_, pt)| match pt {
            PositionType::OxygenSystem => true,
            _ => false,
        })
        .expect("no oxygen")
        .0;

    println!(
        "part 1: {}",
        distance_map(&(0, 0), &map)
            .get(oxygen_system_position)
            .unwrap()
    );

    println!(
        "part 2: {}",
        distance_map(oxygen_system_position, &map)
            .iter()
            .map(|(_, dist)| dist)
            .max()
            .unwrap()
    );
}
