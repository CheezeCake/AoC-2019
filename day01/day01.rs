use std::io;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    let reader = BufReader::new(io::stdin());
    let masses: Vec<i32> = reader
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
