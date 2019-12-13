use std::cmp::Ordering;
use std::collections::HashMap;
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

#[derive(Debug)]
enum IntOuput {
    Exit,
    Output(i64),
    WaitingForInput,
}

impl IntOuput {
    fn expect(self, msg: &'static str) -> i64 {
        if let IntOuput::Output(output) = self {
            output
        } else {
            panic!(msg);
        }
    }
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

    fn run(&mut self, input: &mut Option<i64>) -> IntOuput {
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
                    if let Some(_input) = input {
                        self.store(self.mem[self.pc + 1], mode_op1, *_input);
                        self.pc += 2;
                        *input = None;
                    } else {
                        return IntOuput::WaitingForInput;
                    }
                }
                4 => {
                    let output = self.load(self.mem[self.pc + 1], mode_op1);
                    self.pc += 2;
                    return IntOuput::Output(output);
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

        IntOuput::Exit
    }
}

const PADDLE: i64 = 3;
const BALL: i64 = 4;

#[derive(Clone)]
struct Arcade {
    cpu: CPU,
    score: i64,
    screen: HashMap<(i64, i64), i64>,
}

impl Arcade {
    fn new(game: &Vec<i64>) -> Self {
        Self {
            cpu: CPU::new(game),
            score: 0,
            screen: HashMap::new(),
        }
    }

    fn run_until_exit(&mut self) {
        loop {
            if let IntOuput::Exit = self.run(Some(0)) {
                break;
            }
        }
    }

    fn run(&mut self, mut input: Option<i64>) -> IntOuput {
        loop {
            let output = self.cpu.run(&mut input);
            if let IntOuput::Exit | IntOuput::WaitingForInput = output {
                return output;
            }

            let x = output.expect("no x");
            let y = self.cpu.run(&mut input).expect("no y");
            let tile_score = self.cpu.run(&mut input).expect("no tile/score");

            if x == -1 && y == 0 {
                self.score = tile_score;
            } else {
                self.screen.insert((x, y), tile_score);
            }
        }
    }

    fn winning_score(&mut self) -> i64 {
        let mut input = 0;

        self.cpu.mem[0] = 2;

        loop {
            if let IntOuput::Exit = self.run(Some(input)) {
                return self.score;
            }

            let (pad_x, pad_y) = find_tile_position(PADDLE, &self.screen).expect("no paddle");

            let mut copy = self.clone();
            for i in 0.. {
                let (ball_dst_x, ball_dst_y) =
                    find_tile_position(BALL, &copy.screen).expect("no ball");
                if ball_dst_y == pad_y - 1 {
                    let mut pad_x = pad_x;
                    for _ in 0..i {
                        input = match pad_x.cmp(&ball_dst_x) {
                            Ordering::Less => 1,
                            Ordering::Equal => 0,
                            Ordering::Greater => -1,
                        };
                        pad_x += input;
                        self.run(Some(input));
                    }
                    break;
                }
                if let IntOuput::Exit = copy.run(Some(0)) {
                    input = 0;
                    break;
                }
            }
        }
    }
}

fn find_tile_position(tile: i64, screen: &HashMap<(i64, i64), i64>) -> Option<(i64, i64)> {
    screen.iter().find(|(_, &t)| t == tile).map(|(&p, _)| p)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let program: Vec<i64> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();

    let mut arcade = Arcade::new(&program);
    arcade.run_until_exit();
    println!(
        "part 1: {}",
        arcade.screen.iter().filter(|(_, &tile)| tile == 2).count()
    );

    arcade = Arcade::new(&program);
    println!("part 2: {}", arcade.winning_score());
}
