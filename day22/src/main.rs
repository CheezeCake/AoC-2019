use mod_exp::mod_exp;
use std::io;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

enum Technique {
    DealIntoNewStack,
    DealWithIncrement(isize),
    Cut(isize),
}

impl FromStr for Technique {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        Ok(match parts[0] {
            "deal" if parts[1] == "with" => Technique::DealWithIncrement(parts[3].parse()?),
            "deal" => Technique::DealIntoNewStack,
            "cut" => Technique::Cut(parts[1].parse()?),
            _ => panic!("invalid input: '{}'", s),
        })
    }
}

fn part2(instructions: &[Technique]) -> i128 {
    const DECK_SIZE: i128 = 119315717514047;
    const REPEATS: i128 = 101741582076661;

    let (a, b) = instructions.iter().rev().fold((1, 0), |(a, b), t| {
        let (a_next, b_next) = match t {
            Technique::DealIntoNewStack => (-a, DECK_SIZE - b - 1),
            Technique::DealWithIncrement(n) => {
                let n = mod_exp(*n as i128, DECK_SIZE - 2, DECK_SIZE);
                (a * n, b * n)
            }
            Technique::Cut(n) => (a, b + *n as i128 + DECK_SIZE),
        };
        (a_next % DECK_SIZE, b_next % DECK_SIZE)
    });

    let x = 2020 * mod_exp(a, REPEATS, DECK_SIZE) % DECK_SIZE;
    let tmp =
        (mod_exp(a, REPEATS, DECK_SIZE) - 1) * mod_exp(a - 1, DECK_SIZE - 2, DECK_SIZE) % DECK_SIZE;
    let y = b * tmp % DECK_SIZE;
    (x + y) % DECK_SIZE
}

fn main() {
    let instructions: Vec<Technique> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().expect("error parsing input"))
        .collect();

    const DECK_SIZE: isize = 10007;
    println!(
        "part 1: {}",
        instructions.iter().fold(2019, |i, t| match t {
            Technique::DealIntoNewStack => DECK_SIZE - i - 1,
            Technique::DealWithIncrement(n) => (i * n) % DECK_SIZE,
            Technique::Cut(n) => (i - n) % DECK_SIZE,
        })
    );

    println!("part 2: {}", part2(&instructions));
}
