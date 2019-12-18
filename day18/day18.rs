use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

fn find(map: &Vec<Vec<char>>, target: char) -> Option<(usize, usize)> {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == target {
                return Some((x, y));
            }
        }
    }
    None
}

fn accessible_keys(
    start: (usize, usize),
    map: &Vec<Vec<char>>,
    keys_found: u32,
) -> Vec<((usize, usize), usize)> {
    let mut accessible = Vec::new();

    let mut q = VecDeque::new();
    let mut discovered = HashSet::new();

    q.push_back((start, 0));
    discovered.insert(start);

    while let Some(((x, y), distance)) = q.pop_front() {
        let c = map[y][x];
        if c.is_lowercase() && (keys_found & (1 << (c as u8 - 'a' as u8))) == 0 {
            accessible.push(((x, y), distance));
            continue;
        }

        for (dx, dy) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if ny < 0 || ny as usize >= map.len() || nx < 0 || nx as usize >= map[ny as usize].len()
            {
                continue;
            }

            let pos = (nx as usize, ny as usize);
            if discovered.contains(&pos) {
                continue;
            }

            let c = map[ny as usize][nx as usize];
            if c == '.'
                || c == '@'
                || c.is_lowercase()
                || (c.is_uppercase()
                    && (keys_found & (1 << (c.to_ascii_lowercase() as u8 - 'a' as u8))) != 0)
            {
                discovered.insert(pos);
                q.push_back((pos, distance + 1));
            }
        }
    }

    accessible
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct VaultState {
    keys_found: u32,
    robots: Vec<(usize, usize)>,
}

impl VaultState {
    fn new(n: usize) -> Self {
        let mut robots = Vec::new();
        robots.resize(n, (0, 0));
        Self {
            keys_found: 0,
            robots,
        }
    }
}

fn solve(
    state: &mut VaultState,
    map: &Vec<Vec<char>>,
    key_count: usize,
    cache: &mut HashMap<VaultState, usize>,
) -> usize {
    if state.keys_found == (1 << key_count) - 1 {
        return 0;
    }

    let mut min = std::usize::MAX;

    for i in 0..state.robots.len() {
        let pos = state.robots[i];
        let accessible = accessible_keys(pos, map, state.keys_found);

        for (key_pos, distance) in accessible {
            state.keys_found |= 1 << (map[key_pos.1][key_pos.0] as u8 - 'a' as u8);
            state.robots[i] = key_pos;

            let length = match cache.get(state) {
                Some(&length) => length,
                _ => solve(state, map, key_count, cache),
            };
            min = cmp::min(min, length.checked_add(distance).unwrap_or(std::usize::MAX));

            state.keys_found &= !(1 << (map[key_pos.1][key_pos.0] as u8 - 'a' as u8));
            state.robots[i] = pos;
        }

        cache.insert(state.clone(), min);
    }

    min
}

fn main() {
    let mut map: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let entrance = find(&map, '@').expect("entrance not found");
    let key_count = map.iter().flatten().filter(|c| c.is_lowercase()).count();
    assert!(key_count <= 32);

    let mut state = VaultState::new(1);
    state.robots[0] = entrance;
    let mut cache = HashMap::new();
    println!("part 1: {}", solve(&mut state, &map, key_count, &mut cache));

    let mut state = VaultState::new(4);
    let mut cache = HashMap::new();
    let (x, y) = entrance;
    for (dx, dy) in &[(0, 0), (0, -1), (1, 0), (0, 1), (-1, 0)] {
        let (x, y) = (x as i32 + dx, y as i32 + dy);
        map[y as usize][x as usize] = '#';
    }
    for (i, (dx, dy)) in [(1, -1), (1, 1), (-1, 1), (-1, -1)].iter().enumerate() {
        let (x, y) = (x as i32 + dx, y as i32 + dy);
        map[y as usize][x as usize] = '@';
        state.robots[i] = (x as usize, y as usize);
    }
    println!("part 2: {}", solve(&mut state, &map, key_count, &mut cache));
}
