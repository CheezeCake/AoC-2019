use std::collections::VecDeque;
use std::i32;
use std::io;

struct Permutations {
    permutation: Vec<i32>,
    done: bool,
}

impl Permutations {
    fn new(start: i32, end: i32) -> Self {
        assert!(start <= end);
        Self {
            permutation: (start..=end).collect(),
            done: false,
        }
    }
}

impl Iterator for Permutations {
    type Item = Vec<i32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let ret = self.permutation.clone();
        let mut k = self.permutation.len().checked_sub(2);
        loop {
            match k {
                Some(i) if self.permutation[i] >= self.permutation[i + 1] => k = i.checked_sub(1),
                _ => break,
            }
        }
        if let Some(k) = k {
            let x = self
                .permutation
                .iter()
                .enumerate()
                .rev()
                .find(|(_, &x)| x > self.permutation[k])
                .unwrap()
                .0;
            self.permutation.swap(x, k);
            self.permutation[k + 1..].reverse();
        } else {
            self.done = true;
        }
        Some(ret)
    }
}

struct Amplifier {
    prog: Vec<i32>,
    pc: usize,
}

impl Amplifier {
    fn new(prog: &Vec<i32>) -> Self {
        Amplifier {
            prog: prog.clone(),
            pc: 0,
        }
    }

    fn get(&self, op: i32, mode: i32) -> i32 {
        match mode {
            0 => self.prog[op as usize],
            1 => op,
            _ => panic!("invalid mode: {}", mode),
        }
    }

    fn run(&mut self, io: &mut Vec<VecDeque<i32>>, input: usize, output: usize) -> bool {
        loop {
            let opcode = self.prog[self.pc] % 100;
            let mode_op1 = (self.prog[self.pc] / 100) % 10;
            let mode_op2 = (self.prog[self.pc] / 1000) % 10;
            let _mode_op3 = self.prog[self.pc] / 10000;

            match opcode {
                1 | 2 => {
                    let store = self.prog[self.pc + 3] as usize;
                    let op1 = self.prog[self.pc + 1];
                    let op2 = self.prog[self.pc + 2];

                    self.prog[store] = if opcode == 1 {
                        self.get(op1, mode_op1) + self.get(op2, mode_op2)
                    } else {
                        self.get(op1, mode_op1) * self.get(op2, mode_op2)
                    };

                    self.pc += 4;
                }
                3 => {
                    let store = self.prog[self.pc + 1] as usize;
                    if let Some(input) = io[input].pop_front() {
                        self.prog[store] = input;
                        self.pc += 2;
                    } else {
                        return false;
                    }
                }
                4 => {
                    io[output].push_back(self.get(self.prog[self.pc + 1], mode_op1));
                    self.pc += 2;
                }
                5 => {
                    self.pc = if self.get(self.prog[self.pc + 1], mode_op1) != 0 {
                        self.get(self.prog[self.pc + 2], mode_op2) as usize
                    } else {
                        self.pc + 3
                    }
                }
                6 => {
                    self.pc = if self.get(self.prog[self.pc + 1], mode_op1) == 0 {
                        self.get(self.prog[self.pc + 2], mode_op2) as usize
                    } else {
                        self.pc + 3
                    }
                }
                7 => {
                    let store = self.prog[self.pc + 3] as usize;
                    self.prog[store] = (self.get(self.prog[self.pc + 1], mode_op1)
                        < self.get(self.prog[self.pc + 2], mode_op2))
                        as i32;
                    self.pc += 4
                }
                8 => {
                    let store = self.prog[self.pc + 3] as usize;
                    self.prog[store] = (self.get(self.prog[self.pc + 1], mode_op1)
                        == self.get(self.prog[self.pc + 2], mode_op2))
                        as i32;
                    self.pc += 4
                }
                99 => return true,
                _ => panic!(format!("invalid opcode: {}", opcode)),
            }
        }
    }
}

fn run_amplifiers(prog: &Vec<i32>, phase_settings: &Vec<i32>) -> i32 {
    let mut amplifiers = Vec::new();
    let mut io = Vec::new();
    let amplifiers_count = phase_settings.len();
    for &ps in phase_settings {
        amplifiers.push(Amplifier::new(prog));
        let mut q = VecDeque::new();
        q.push_back(ps);
        io.push(q);
    }

    io[0].push_back(0);

    let mut running = true;
    while running {
        for (i, amp) in amplifiers.iter_mut().enumerate() {
            if amp.run(&mut io, i, (i + 1) % amplifiers_count) {
                running = false;
            }
        }
    }

    io[0].pop_back().expect("no output")
}

fn highest_signal(prog: &Vec<i32>, phase_settings_start: i32, phase_settings_end: i32) -> i32 {
    Permutations::new(phase_settings_start, phase_settings_end)
        .map(|phase_settings| run_amplifiers(prog, &phase_settings))
        .max()
        .unwrap()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let prog: Vec<i32> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();

    println!("part 1: {}", highest_signal(&prog, 0, 4));
    println!("part 2: {}", highest_signal(&prog, 5, 9));
}
