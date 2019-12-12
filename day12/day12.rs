use std::cmp::Ordering;
use std::io;
use std::io::prelude::*;
use std::iter::Sum;
use std::num::ParseIntError;
use std::ops::Add;
use std::ops::AddAssign;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    fn abs_coordinates_sum(&self) -> u32 {
        self.x.abs() as u32 + self.y.abs() as u32 + self.z.abs() as u32
    }
}

impl FromStr for Vec3 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<_> = s
            .trim_matches(|p| p == '<' || p == '>')
            .split(", ")
            .map(|part| part[2..].to_string())
            .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;
        let z_fromstr = coords[2].parse::<i32>()?;

        Ok(Vec3 {
            x: x_fromstr,
            y: y_fromstr,
            z: z_fromstr,
        })
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sum for Vec3 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Vec3::new(), |a, b| a + b)
    }
}

fn gravity(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

fn gravitational_pull(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x: gravity(a.x, b.x),
        y: gravity(a.y, b.y),
        z: gravity(a.z, b.z),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Moon {
    pos: Vec3,
    velocity: Vec3,
}

impl Moon {
    fn potential_energy(&self) -> u32 {
        self.pos.abs_coordinates_sum()
    }

    fn kinetic_energy(&self) -> u32 {
        self.velocity.abs_coordinates_sum()
    }

    fn total_energy(&self) -> u32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

fn step_simultation(moons: &mut Vec<Moon>) {
    let gravity: Vec<Vec3> = moons
        .iter()
        .map(|m| {
            moons
                .iter()
                .map(|o| gravitational_pull(&m.pos, &o.pos))
                .sum::<Vec3>()
        })
        .collect();

    moons.iter_mut().zip(gravity).for_each(|(m, g)| {
        m.velocity += g;
        m.pos += m.velocity;
    });
}

fn find_repetition<F>(mut moons: Vec<Moon>, dim: F) -> usize
where
    F: Fn(&Vec3) -> i32,
{
    let initial_pos: Vec<i32> = moons.iter().map(|m| dim(&m.pos)).collect();
    let initial_vel: Vec<i32> = moons.iter().map(|m| dim(&m.velocity)).collect();

    for steps in 1.. {
        step_simultation(&mut moons);
        let pos: Vec<i32> = moons.iter().map(|m| dim(&m.pos)).collect();
        let vel: Vec<i32> = moons.iter().map(|m| dim(&m.velocity)).collect();
        if pos == initial_pos && vel == initial_vel {
            return steps;
        }
    }

    unreachable!()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn main() {
    let input: Vec<Moon> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let pos = line.unwrap().parse::<Vec3>().expect("error parsing input");
            Moon {
                pos: pos,
                velocity: Vec3::new(),
            }
        })
        .collect();

    let mut moons = input.clone();
    for _ in 0..1000 {
        step_simultation(&mut moons);
    }
    println!(
        "part 1: {}",
        moons.iter().map(|m| m.total_energy()).sum::<u32>()
    );

    let x = find_repetition(input.clone(), |v| v.x);
    let y = find_repetition(input.clone(), |v| v.y);
    let z = find_repetition(input.clone(), |v| v.z);
    println!("part 2: {}", lcm(lcm(x, y), z));
}
