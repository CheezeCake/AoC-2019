use std::collections::HashMap;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

struct Instruction {
    direction: u8,
    length: usize,
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = s.as_bytes()[0];
        let length: usize = s.get(1..).unwrap().parse()?;
        Ok(Self { direction, length })
    }
}

fn walk_path(instructions: Vec<Instruction>) -> HashMap<(i32, i32), usize> {
    let mut x = 0;
    let mut y = 0;
    let mut steps = 0;
    let mut path_taken = HashMap::new();

    for i in instructions {
        for _ in 0..i.length {
            match i.direction as char {
                'U' => y += 1,
                'R' => x += 1,
                'D' => y -= 1,
                'L' => x -= 1,
                _ => panic!("invalid direction: {}", i.direction),
            }

            steps += 1;
            path_taken.entry((x, y)).or_insert(steps);
        }
    }

    path_taken
}

fn read_wire() -> HashMap<(i32, i32), usize> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    walk_path(
        input
            .trim()
            .split(',')
            .map(|p| p.parse().unwrap())
            .collect(),
    )
}

fn main() {
    let wire1 = read_wire();
    let wire2 = read_wire();
    let intersections: HashMap<(i32, i32), usize> = wire1
        .iter()
        .filter_map(|(p, steps)| Some((*p, wire2.get(p)? + steps)))
        .collect();

    println!(
        "part 1: {}",
        intersections
            .keys()
            .map(|(x, y)| x.abs() + y.abs())
            .min()
            .unwrap()
    );

    println!("part 2: {}", intersections.values().min().unwrap());
}
