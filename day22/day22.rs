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
}
