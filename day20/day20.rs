use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

#[derive(Hash, PartialEq, Eq, Debug, Copy, Clone)]
enum Tile {
    Portal((char, char)),
    Wall,
    Passage,
    Empty,
}

fn within_bounds<T>(x: i32, y: i32, map: &Vec<Vec<T>>) -> bool {
    y >= 0 && (y as usize) < map.len() && x >= 0 && (x as usize) < map[y as usize].len()
}

fn read_map() -> Vec<Vec<Tile>> {
    let chars: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let mut map = Vec::new();

    for y in 0..chars.len() {
        let mut row = Vec::new();
        for x in 0..chars[y].len() {
            let tile = match chars[y][x] {
                '#' => Tile::Wall,
                '.' => Tile::Passage,
                ' ' => Tile::Empty,
                c if c.is_ascii_alphabetic() => {
                    if within_bounds(x as i32, y as i32 - 1, &chars) && chars[y - 1][x] == '.' {
                        Tile::Portal((c, chars[y + 1][x]))
                    } else if within_bounds((x + 1) as i32, y as i32, &chars)
                        && chars[y][x + 1] == '.'
                    {
                        Tile::Portal((chars[y][x - 1], c))
                    } else if within_bounds(x as i32, (y + 1) as i32, &chars)
                        && chars[y + 1][x] == '.'
                    {
                        Tile::Portal((chars[y - 1][x], c))
                    } else if within_bounds(x as i32 - 1, y as i32, &chars)
                        && chars[y][x - 1] == '.'
                    {
                        Tile::Portal((c, chars[y][x + 1]))
                    } else {
                        Tile::Empty
                    }
                }
                x => panic!("invalid character in input: {}", x),
            };
            row.push(tile);
        }
        map.push(row);
    }

    map
}

fn build_portal_map(map: &Vec<Vec<Tile>>) -> HashMap<(usize, usize), (usize, usize)> {
    let mut portals = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Tile::Portal(portal_pair) = map[y][x] {
                portals
                    .entry(portal_pair)
                    .or_insert(Vec::new())
                    .push((x, y));
            }
        }
    }

    let mut portal_map = HashMap::new();
    for (_, positions) in portals {
        if positions.len() == 1 {
            continue;
        }
        assert_eq!(positions.len(), 2);
        portal_map.insert(positions[0], positions[1]);
        portal_map.insert(positions[1], positions[0]);
    }

    portal_map
}

fn find_portal_passage((x, y): (usize, usize), map: &Vec<Vec<Tile>>) -> Option<(usize, usize)> {
    for (dx, dy) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let (sx, sy) = (x as i32 + dx, y as i32 + dy);
        if within_bounds(sx, sy, map) && map[sy as usize][sx as usize] == Tile::Passage {
            return Some((sx as usize, sy as usize));
        }
    }
    None
}

fn outer_portal((x, y): (usize, usize), map: &Vec<Vec<Tile>>) -> bool {
    y == 1 || x == 1 || y == map.len() - 2 || x == map[y].len() - 2
}

fn shortest_path(
    start: (usize, usize),
    target: Tile,
    map: &Vec<Vec<Tile>>,
    portal_map: &HashMap<(usize, usize), (usize, usize)>,
) -> Option<usize> {
    let mut q = VecDeque::new();
    let mut discovered = HashSet::new();

    q.push_back((start, 0));
    discovered.insert(start);

    while let Some((pos, n)) = q.pop_front() {
        let (x, y) = pos;
        for (dx, dy) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if within_bounds(nx, ny, map) && !discovered.contains(&(nx as usize, ny as usize)) {
                let pos = (nx as usize, ny as usize);

                match map[pos.1][pos.0] {
                    Tile::Passage => {
                        q.push_back((pos, n + 1));
                        discovered.insert(pos);
                    }
                    Tile::Portal(_) => {
                        if map[pos.1][pos.0] == target {
                            return Some(n);
                        }
                        if let Some(dst_pos) = portal_map.get(&pos) {
                            if !discovered.contains(&dst_pos) {
                                let passage = find_portal_passage(*dst_pos, map)
                                    .expect("portal passage not found");
                                discovered.insert(*dst_pos);
                                discovered.insert(passage);
                                q.push_back((passage, n + 1));
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    None
}

fn shortest_path_recursive(
    start: (usize, usize),
    target: Tile,
    map: &Vec<Vec<Tile>>,
    portal_map: &HashMap<(usize, usize), (usize, usize)>,
) -> Option<usize> {
    let mut q = VecDeque::new();
    let mut discovered = HashMap::new();

    q.push_back((start, 0, 0));
    discovered.entry(0).or_insert(HashSet::new()).insert(start);

    while let Some((pos, n, level)) = q.pop_front() {
        let (x, y) = pos;

        for (dx, dy) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);

            if within_bounds(nx, ny, map)
                && !discovered
                    .get(&level)
                    .unwrap()
                    .contains(&(nx as usize, ny as usize))
            {
                let pos = (nx as usize, ny as usize);

                discovered.get_mut(&level).unwrap().insert(pos);

                match map[pos.1][pos.0] {
                    Tile::Passage => {
                        q.push_back((pos, n + 1, level));
                    }
                    Tile::Portal(_) => {
                        if level == 0 && map[pos.1][pos.0] == target {
                            return Some(n);
                        }
                        if let Some(dst_pos) = portal_map.get(&pos) {
                            if outer_portal(pos, map) {
                                if level > 0
                                    && !discovered
                                        .entry(level - 1)
                                        .or_insert(HashSet::new())
                                        .contains(&dst_pos)
                                {
                                    let passage = find_portal_passage(*dst_pos, map)
                                        .expect("portal passage not found");
                                    discovered.get_mut(&(level - 1)).unwrap().insert(*dst_pos);
                                    discovered.get_mut(&(level - 1)).unwrap().insert(passage);
                                    q.push_back((passage, n + 1, level - 1));
                                }
                            } else {
                                if !discovered
                                    .entry(level + 1)
                                    .or_insert(HashSet::new())
                                    .contains(&dst_pos)
                                {
                                    let passage = find_portal_passage(*dst_pos, map)
                                        .expect("portal passage not found");
                                    discovered.get_mut(&(level + 1)).unwrap().insert(*dst_pos);
                                    discovered.get_mut(&(level + 1)).unwrap().insert(passage);
                                    q.push_back((passage, n + 1, level + 1));
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    None
}

fn main() {
    let map = read_map();
    let portal_map = build_portal_map(&map);
    let aa_portal = map
        .iter()
        .flatten()
        .enumerate()
        .find(|(_, &tile)| Tile::Portal(('A', 'A')) == tile)
        .map(|(p, _)| (p % map[0].len(), p / map[0].len()))
        .expect("portal AA not found");
    let start = find_portal_passage(aa_portal, &map).expect("start passage not found");

    println!(
        "part 1: {}",
        shortest_path(start, Tile::Portal(('Z', 'Z')), &map, &portal_map)
            .expect("could not found a way out of the maze")
    );
    println!(
        "part 2: {}",
        shortest_path_recursive(start, Tile::Portal(('Z', 'Z')), &map, &portal_map)
            .expect("could not found a way out of the maze")
    );
}
