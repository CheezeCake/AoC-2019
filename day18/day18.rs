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

        for dir in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let (nx, ny) = (x as i32 + dir.0, y as i32 + dir.1);
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

// fn shortest(
//     pos: (usize, usize),
//     length: usize,
//     map: &Vec<Vec<char>>,
//     keys_found: &mut HashSet<char>,
//     key_count: usize,
//     shortest_len: &mut usize,
// ) {
//     if length > *shortest_len {
//         return;
//     }
//     if keys_found.len() == key_count {
//         *shortest_len = cmp::min(*shortest_len, length);
//         println!("{}", shortest_len);
//         return;
//     }

//     let accessible = accessible_keys(pos, map, keys_found);

//     for (key_pos, distance) in accessible {
//         keys_found.insert(map[key_pos.1][key_pos.0]);

//         shortest(
//             key_pos,
//             length + distance,
//             map,
//             keys_found,
//             key_count,
//             shortest_len,
//         );

//         keys_found.remove(&map[key_pos.1][key_pos.0]);
//     }
// }

#[derive(Clone, Debug)]
struct KeySet {
    keys: Vec<char>,
}

impl KeySet {
    fn new() -> Self {
        Self { keys: Vec::new() }
    }
}

impl Hash for KeySet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut sorted = self.keys.clone();
        sorted.sort();
        for c in sorted {
            c.hash(state);
        }
        if let Some(last) = self.keys.last() {
            last.hash(state);
        }
    }
}

impl PartialEq for KeySet {
    fn eq(&self, other: &Self) -> bool {
        let s: HashSet<char> = HashSet::from_iter(self.keys.clone());
        let o: HashSet<char> = HashSet::from_iter(other.keys.clone());
        s == o && self.keys.last() == other.keys.last()
    }
}

impl Eq for KeySet {}

fn solve(
    pos: (usize, usize),
    map: &Vec<Vec<char>>,
    keys_found: &mut KeySet,
    key_count: usize,
    cache: &mut HashMap<KeySet, usize>,
) -> usize {
    if keys_found.keys.len() == key_count {
        return 0;
    }

    let kf_set: HashSet<char> = HashSet::from_iter(keys_found.keys.clone());
    let accessible = accessible_keys(pos, map, &kf_set);
    let mut min = std::usize::MAX;

    for (key_pos, distance) in accessible {
        keys_found.keys.push(map[key_pos.1][key_pos.0]);

        if let Some(length) = cache.get(keys_found) {
            min = cmp::min(min, length + distance);
        } else {
            let length = solve(key_pos, map, keys_found, key_count, cache);
            min = cmp::min(min, length + distance);
        }

        keys_found.keys.pop();
    }

    cache.insert(keys_found.clone(), min);

    min
}

fn main() {
    let map: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let entrance = find(&map, '@').expect("entrance not found");
    let key_count = map.iter().flatten().filter(|c| c.is_lowercase()).count();

    let mut keys_found = KeySet::new();
    let mut cache = HashMap::new();
    let len = solve(entrance, &map, &mut keys_found, key_count, &mut cache);
    // println!("cache = {:?}", cache);
    println!("part 1: {}", len);
}
