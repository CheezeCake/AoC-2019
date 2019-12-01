use std::io;
use std::io::prelude::*;

fn main() {
    let masses: Vec<u32> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();

    println!(
        "part 1: {}",
        masses.iter().map(|mass| mass / 3 - 2).sum::<i32>()
    );

    println!(
        "part 2: {}",
        masses
            .iter()
            .map(|mass| {
                let mut mass = mass / 3 - 2;
                let mut sum = 0;
                while mass > 0 {
                    sum += mass;
                    mass = mass / 3 - 2;
                }
                sum
            })
            .sum::<i32>()
    );
}
