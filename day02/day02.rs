use std::io;

fn run(prog: &mut Vec<u32>) {
    let mut pc = 0;

    loop {
        match prog[pc] {
            1 | 2 => {
                let store = prog[pc + 3] as usize;
                let op1 = prog[pc + 1] as usize;
                let op2 = prog[pc + 2] as usize;
                prog[store] = if prog[pc] == 1 {
                    prog[op1] + prog[op2]
                } else {
                    prog[op1] * prog[op2]
                }
            }
            99 => break,
            _ => panic!(format!("invalid opcode: {}", prog[pc])),
        }
        pc += 4;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let opcodes: Vec<u32> = input
        .trim()
        .split(',')
        .map(|opcode| opcode.parse().unwrap())
        .collect();

    let mut prog = opcodes.clone();
    prog[1] = 12;
    prog[2] = 2;
    run(&mut prog);
    println!("part 1: {}", prog[0]);

    for noun in 0..=99 {
        for verb in 0..=99 {
            prog = opcodes.clone();
            prog[1] = noun;
            prog[2] = verb;
            run(&mut prog);
            if prog[0] == 19690720 {
                println!("part 2: {}", 100 * noun + verb);
                return;
            }
        }
    }
}
