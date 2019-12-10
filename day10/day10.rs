use std::collections::BTreeMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn calc_angle((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> i32 {
    let mut theta = (x1 as f32 - x2 as f32).atan2(y1 as f32 - y2 as f32);
    if theta > 0.0 {
        theta = (2.0 * std::f32::consts::PI) - theta;
    } else {
        theta = -theta;
    }
    theta *= 10_000.0;
    theta as i32
}

fn detectable_asteroids(location: (usize, usize), asteroids: &HashSet<(usize, usize)>) -> usize {
    let mut detected_angles = HashSet::new();

    for &pos in asteroids {
        if pos != location {
            detected_angles.insert(calc_angle(location, pos));
        }
    }

    detected_angles.len()
}

fn distance((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> usize {
    ((x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs()) as usize
}

fn main() {
    let mut asteroids: HashSet<(usize, usize)> = io::stdin()
        .lock()
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.unwrap()
                .chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(|(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let (best_location, count) = asteroids
        .iter()
        .map(|&location| (location, detectable_asteroids(location, &asteroids)))
        .max_by_key(|(_, detected)| *detected)
        .expect("no asteroid found");

    println!("part 1: {}", count);
    // println!("part 1: {:?}", best_location);

    asteroids.remove(&best_location);

    let mut vaporized = 0;

    while !asteroids.is_empty() {
        let mut visible = BTreeMap::new();

        for &ast_pos in &asteroids {
            let theta = calc_angle(best_location, ast_pos);

            if !visible.contains_key(&theta)
                || distance(best_location, ast_pos)
                    < distance(best_location, *visible.get(&theta).unwrap())
            {
                visible.insert(theta, ast_pos);
            }
        }

        let mut angle = -1;
        for (k, v) in visible {
            if k != angle {
                angle = k;
                asteroids.remove(&v);
                vaporized += 1;
                if vaporized == 200 {
                    let (x, y) = v;
                    println!("part 2: {}", x * 100 + y);
                    return;
                }
            }
        }
    }

    panic!("less than 200 asteroids were vaporized ({})", vaporized);
}
