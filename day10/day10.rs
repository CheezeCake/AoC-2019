use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn detect(
    map: &Vec<Vec<char>>,
    row: i32,
    col: i32,
    dx: i32,
    dy: i32,
    detected: &mut HashSet<(i32, i32)>,
) -> bool {
    let mut found = false;
    let mut x = col + dx;
    let mut y = row + dy;

    while x >= 0 && x < map.len() as i32 && y >= 0 && y < map.len() as i32 {
        if map[y as usize][x as usize] == '#' {
            if detected.contains(&(x, y)) {
                return false;
            } else {
                found = true;
                detected.insert((x, y));
            }
        }

        x += dx;
        y += dy;
    }

    found
}

fn detectable(map: &Vec<Vec<char>>, row: usize, col: usize) -> usize {
    let row = row as i32;
    let col = col as i32;

    let mut count = 0;
    let mut detected = HashSet::new();

    for x in (0..=col).rev() {
        let dx = x - col;
        for y in (0..=row).rev() {
            let dy = y - row;
            if dx == 0 && dy == 0 {
                continue;
            }
            if detect(&map, row, col, dx, dy, &mut detected) {
                // println!("found with {}, {}", dx, dy);
                count += 1;
            }
        }
        for y in row..map.len() as i32 {
            let dy = y - row;
            if dx == 0 && dy == 0 {
                continue;
            }
            if detect(&map, row, col, dx, dy, &mut detected) {
                // println!("found with {}, {}", dx, dy);
                count += 1;
            }
        }
    }

    for x in col..map.len() as i32 {
        let dx = x - col;
        for y in (0..=row).rev() {
            let dy = y - row;
            if dx == 0 && dy == 0 {
                continue;
            }
            if detect(&map, row, col, dx, dy, &mut detected) {
                // println!("found with {}, {}", dx, dy);
                count += 1;
            }
        }
        for y in row..map.len() as i32 {
            let dy = y - row;
            if dx == 0 && dy == 0 {
                continue;
            }
            if detect(&map, row, col, dx, dy, &mut detected) {
                // println!("found with {}, {}", dx, dy);
                count += 1;
            }
        }
    }

    // println!("{}, {} -> {}", col, row, count);
    count
}

fn main() {
    let map: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut asteroids = HashSet::new();
    let mut best_x = -1;
    let mut best_y = -1;
    let mut max_count = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '#' {
                asteroids.insert((j as i32, i as i32));
                let count = detectable(&map, i, j);
                if count > max_count {
                    max_count = count;
                    best_x = j as i32;
                    best_y = i as i32;
                }
            }
        }
    }

    println!("part 1: {}", max_count);

    asteroids.remove(&(best_x, best_y));

    let mut shot: Vec<(i32, i32)> = Vec::new();

    while !asteroids.is_empty() {
        let mut aya = BTreeMap::new();

        for a in &asteroids {
            let mut theta = ((best_x - a.0) as f64).atan2((best_y - a.1) as f64);
            if theta > 0.0 {
                theta = (2.0 * std::f64::consts::PI) - theta;
            } else {
                theta = -theta;
            }
            theta *= 10_000.0;
            let theta = theta as i64;

            if aya.contains_key(&theta) {
                let (x, y) = aya.get(&theta).unwrap();
                if (best_x - a.0).abs() + (best_y - a.1).abs()
                    < (best_y - y).abs() + (best_x - x).abs()
                {
                    aya.insert(theta, *a);
                }
            } else {
                aya.insert(theta, *a);
            }
        }

        let mut angle = -1;
        for (k, v) in aya {
            if k != angle {
                angle = k;
                shot.push(v);
                asteroids.remove(&v);
            }
        }
    }

    let (x, y) = shot[200 - 1];
    // println!("200th shot: {}, {}", x, y);
    println!("part 2: {}", x * 100 + y);
}
