use std::io;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

enum Technique {
    DealIntoNewStack,
    DealWithIncrement(usize),
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

    const DECK_SIZE: usize = 10007;
    let mut deck: Vec<usize> = (0..DECK_SIZE).collect();

    for t in &instructions {
        assert_eq!(deck.len(), DECK_SIZE);
        match t {
            Technique::DealIntoNewStack => deck.reverse(),
            Technique::DealWithIncrement(n) => {
                let mut tmp = deck.clone();
                let mut write_idx = 0;
                for &c in &deck {
                    tmp[write_idx % deck.len()] = c;
                    write_idx += n;
                }
                deck = tmp;
            }
            Technique::Cut(n) => {
                if *n > 0 {
                    let n = *n as usize;
                    let mut tmp = deck[n..].to_vec();
                    tmp.append(&mut deck[0..n].to_vec());
                    deck = tmp;
                } else {
                    let n = n.abs() as usize;
                    let i = deck.len() - n;
                    let mut tmp = deck[i..].to_vec();
                    tmp.append(&mut deck[0..i].to_vec());
                    deck = tmp;
                }
            }
        }
    }

    println!(
        "part 1: {}",
        deck.iter()
            .enumerate()
            .find(|(_, &c)| c == 2019)
            .expect("card 2019 not found")
            .0
    );
}
