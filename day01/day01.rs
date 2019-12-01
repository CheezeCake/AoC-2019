use std::io;
use std::io::prelude::*;
use std::iter::successors;

fn main() {
    let fuel_masses: Vec<u32> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap() / 3 - 2)
        .collect();

    println!("part 1: {}", fuel_masses.iter().sum::<u32>());

    println!(
        "part 2: {}",
        fuel_masses
            .iter()
            .map(|&mass| successors(Some(mass), |&x| (x / 3).checked_sub(2)).sum::<u32>())
            .sum::<u32>()
    );
}
