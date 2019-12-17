use std::cmp;
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

fn build_map(program: &Vec<i64>) -> Vec<Vec<char>> {
    let mut cpu = CPU::new(&program);

    let mut map = Vec::new();
    let mut row = Vec::new();
    loop {
        if let Some(output) = cpu.run(None) {
            let output = output as u8 as char;
            match output {
                '#' | '.' | '^' | '>' | 'v' | '<' => row.push(output),
                '\n' => {
                    if row.len() > 0 {
                        map.push(row);
                        row = Vec::new();
                    }
                }
                _ => panic!("invalid {}", output),
            }
        } else {
            break;
        }
    }

    map
}

const UP: (i32, i32) = (0, -1);
const RIGHT: (i32, i32) = (1, 0);
const DOWN: (i32, i32) = (0, 1);
const LEFT: (i32, i32) = (-1, 0);

fn within_bounds(x: i32, y: i32, map: &Vec<Vec<char>>) -> bool {
    y >= 0 && (y as usize) < map.len() && x >= 0 && (x as usize) < map[y as usize].len()
}

fn is_scaffold(x: i32, y: i32, map: &Vec<Vec<char>>) -> bool {
    within_bounds(x, y, &map) && map[y as usize][x as usize] != '.'
}

fn is_intersection(x: i32, y: i32, map: &Vec<Vec<char>>) -> bool {
    [UP, RIGHT, DOWN, LEFT]
        .iter()
        .filter(|(dx, dy)| is_scaffold(x + dx, y + dy, &map))
        .count()
        > 2
}

fn intersections_alignment_parameters_sum(map: &Vec<Vec<char>>) -> usize {
    let mut alignment_param_sum = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == '#' && is_intersection(x as i32, y as i32, &map) {
                alignment_param_sum += x * y;
            }
        }
    }

    alignment_param_sum
}

#[derive(Debug, PartialEq)]
enum Movement {
    MoveForward(usize),
    Turn(char),
}

impl ToString for Movement {
    fn to_string(&self) -> String {
        match self {
            Movement::MoveForward(len) => len.to_string(),
            Movement::Turn(c) => c.to_string(),
        }
    }
}

fn starting_position(map: &Vec<Vec<char>>) -> (i32, i32) {
    let p = map
        .iter()
        .flatten()
        .enumerate()
        .find(|(_, &c)| c != '.' && c != '#')
        .unwrap()
        .0;
    ((p % map[0].len()) as i32, (p / map[0].len()) as i32)
}

fn build_path(map: &Vec<Vec<char>>) -> Vec<Movement> {
    let (mut x, mut y) = starting_position(&map);
    let mut dir = match map[y as usize][x as usize] {
        '^' => UP,
        '>' => RIGHT,
        'v' => DOWN,
        '<' => LEFT,
        c => panic!("invalid direction robot char: {}", c),
    };

    let mut path = Vec::new();
    let mut straight_len = 0;

    loop {
        let (nx, ny) = (x + dir.0, y + dir.1);
        if is_scaffold(nx, ny, &map) {
            x = nx;
            y = ny;
            straight_len += 1;
            continue;
        }

        if straight_len > 0 {
            path.push(Movement::MoveForward(straight_len));
            straight_len = 0;
        }

        let dir_left = match dir {
            UP => LEFT,
            RIGHT => UP,
            DOWN => RIGHT,
            LEFT => DOWN,
            _ => panic!("invalid dir"),
        };
        let dir_right = match dir {
            UP => RIGHT,
            RIGHT => DOWN,
            DOWN => LEFT,
            LEFT => UP,
            _ => panic!("invalid dir"),
        };

        let left = (x + dir_left.0, y + dir_left.1);
        let right = (x + dir_right.0, y + dir_right.1);

        if is_scaffold(left.0, left.1, &map) && is_scaffold(right.0, right.1, &map) {
            panic!("T intersection");
        } else if is_scaffold(left.0, left.1, &map) {
            dir = dir_left;
            path.push(Movement::Turn('L'));
        } else if is_scaffold(right.0, right.1, &map) {
            dir = dir_right;
            path.push(Movement::Turn('R'));
        } else {
            break;
        }
    }

    path
}

fn solve<'a>(
    cur_func: usize,
    path: &'a [Movement],
    main: &mut String,
    functions: &mut [&'a [Movement]; 3],
) -> bool {
    if main.len() > 20 {
        return false;
    }

    for i in 0..cur_func {
        let f = functions[i];
        if path.len() >= f.len() && &path[0..f.len()] == f {
            main.push(('A' as u8 + i as u8) as char);
            if solve(cur_func, &path[f.len()..], main, functions) {
                return true;
            }
            main.pop();
            return false;
        }
    }

    if cur_func == 3 {
        return path.len() == 0;
    }

    for len in 1..cmp::min(path.len(), 20) {
        functions[cur_func] = &path[0..len];
        if solve(cur_func + 1, path, main, functions) {
            return true;
        }
    }

    false
}

fn function_str(func: &[Movement]) -> String {
    let mut s = String::new();
    for (i, m) in func.iter().enumerate() {
        if i != 0 {
            s += ",";
        }
        s += &m.to_string();
    }
    s + "\n"
}

fn collect_space_dust(program: &Vec<i64>, main_routine: &str, functions: &[&[Movement]; 3]) -> i64 {
    let mut program = program.clone();
    program[0] = 2;

    let mut cpu = CPU::new(&program);
    while let Some(_) = cpu.run(None) {}

    for (i, &c) in main_routine.as_bytes().iter().enumerate() {
        if i != 0 {
            cpu.run(Some(',' as i64));
        }
        cpu.run(Some(c as i64));
    }
    cpu.run(Some('\n' as i64));
    while let Some(_) = cpu.run(None) {}

    for func in functions {
        for &c in function_str(func).as_bytes() {
            cpu.run(Some(c as i64));
        }
        while let Some(_) = cpu.run(None) {}
    }

    cpu.run(Some('n' as i64));
    cpu.run(Some('\n' as i64));

    let mut dust = 0;
    while let Some(output) = cpu.run(None) {
        dust = output;
    }

    dust
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();

    let map = build_map(&program);

    println!("part 1: {}", intersections_alignment_parameters_sum(&map));

    let path = build_path(&map);
    let mut functions: [&[Movement]; 3] = [&path, &path, &path];
    let mut main = String::new();
    assert!(solve(0, &path, &mut main, &mut functions));

    println!(
        "part 2: {}",
        collect_space_dust(&program, &main, &functions)
    );
}
