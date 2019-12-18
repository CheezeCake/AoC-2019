use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::prelude::*;
use std::iter::FromIterator;

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
    keys_found: &HashSet<char>,
) -> Vec<((usize, usize), usize)> {
    let mut accessible = Vec::new();

    let mut q = VecDeque::new();
    let mut discovered = HashSet::new();

    q.push_back((start, 0));
    discovered.insert(start);

    while let Some(((x, y), distance)) = q.pop_front() {
        let c = map[y][x];
        if c.is_lowercase() && !keys_found.contains(&c) {
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
                || (c.is_uppercase() && keys_found.contains(&c.to_ascii_lowercase()))
            {
                discovered.insert(pos);
                q.push_back((pos, distance + 1));
            }
        }
    }

    accessible
}

#[derive(Clone, Debug)]
struct VaultState {
    keys_found: Vec<char>,
    robots: Vec<(usize, usize)>,
}

impl VaultState {
    fn new(n: usize) -> Self {
        let mut robots = Vec::new();
        robots.resize(n, (0, 0));
        Self {
            keys_found: Vec::new(),
            robots,
        }
    }
}

impl Hash for VaultState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut sorted = self.keys_found.clone();
        sorted.sort();
        for c in sorted {
            c.hash(state);
        }
        for p in &self.robots {
            p.hash(state);
        }
    }
}

impl PartialEq for VaultState {
    fn eq(&self, other: &Self) -> bool {
        let s: HashSet<char> = HashSet::from_iter(self.keys_found.clone());
        let o: HashSet<char> = HashSet::from_iter(other.keys_found.clone());
        s == o && self.robots == other.robots
    }
}

impl Eq for VaultState {}

fn solve(
    state: &mut VaultState,
    map: &Vec<Vec<char>>,
    key_count: usize,
    cache: &mut HashMap<VaultState, usize>,
) -> usize {
    if state.keys_found.len() == key_count {
        return 0;
    }

    let mut min = std::usize::MAX;

    for i in 0..state.robots.len() {
        let pos = state.robots[i];
        let kf_set: HashSet<char> = HashSet::from_iter(state.keys_found.clone());
        let accessible = accessible_keys(pos, map, &kf_set);

        for (key_pos, distance) in accessible {
            state.keys_found.push(map[key_pos.1][key_pos.0]);
            state.robots[i] = key_pos;

            let length = match cache.get(state) {
                Some(&length) => length,
                _ => solve(state, map, key_count, cache),
            };
            min = cmp::min(min, length.checked_add(distance).unwrap_or(std::usize::MAX));

            state.keys_found.pop();
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
