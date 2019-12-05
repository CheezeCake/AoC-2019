use std::io;

fn get(prog: &Vec<i32>, op: i32, mode: i32) -> i32 {
    match mode {
        0 => prog[op as usize],
        1 => op,
        _ => panic!("invalid mode: {}", mode),
    }
}

fn run(prog: &mut Vec<i32>, input: i32) -> i32 {
    let mut pc = 0;
    let mut output = 0;

    loop {
        let opcode = prog[pc] % 100;
        let mode_op1 = (prog[pc] / 100) % 10;
        let mode_op2 = (prog[pc] / 1000) % 10;
        let _mode_op3 = prog[pc] / 10000;

        match opcode {
            1 | 2 => {
                let store = prog[pc + 3] as usize;
                let op1 = prog[pc + 1];
                let op2 = prog[pc + 2];

                prog[store] = if opcode == 1 {
                    get(prog, op1, mode_op1) + get(prog, op2, mode_op2)
                } else {
                    get(prog, op1, mode_op1) * get(prog, op2, mode_op2)
                };

                pc += 4;
            }
            3 => {
                let store = prog[pc + 1] as usize;
                prog[store] = input;
                pc += 2;
            }
            4 => {
                output = get(prog, prog[pc + 1], mode_op1);
                pc += 2;
            }
            5 => {
                if get(prog, prog[pc + 1], mode_op1) != 0 {
                    pc = get(prog, prog[pc + 2], mode_op2) as usize;
                } else {
                    pc += 3;
                }
            }
            6 => {
                if get(prog, prog[pc + 1], mode_op1) == 0 {
                    pc = get(prog, prog[pc + 2], mode_op2) as usize;
                } else {
                    pc += 3;
                }
            }
            7 => {
                let store = prog[pc + 3] as usize;
                prog[store] =
                    (get(prog, prog[pc + 1], mode_op1) < get(prog, prog[pc + 2], mode_op2)) as i32;
                pc += 4
            }
            8 => {
                let store = prog[pc + 3] as usize;
                prog[store] =
                    (get(prog, prog[pc + 1], mode_op1) == get(prog, prog[pc + 2], mode_op2)) as i32;
                pc += 4
            }
            99 => break,
            _ => panic!(format!("invalid opcode: {}", opcode)),
        }
    }

    output
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let opcodes: Vec<i32> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();

    let mut prog = opcodes.clone();
    println!("part 1: {}", run(&mut prog, 1));

    prog = opcodes.clone();
    println!("part 2: {}", run(&mut prog, 5));
}
