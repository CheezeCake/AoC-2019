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

fn adjacent_tiles(
    x: usize,
    y: usize,
    map: &Vec<Vec<Tile>>,
) -> impl Iterator<Item = (usize, usize)> + '_ {
    [(0, 1), (0, -1), (-1, 0), (1, 0)]
        .iter()
        .map(move |(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(move |(x, y)| within_bounds(*x, *y, map))
        .map(|(x, y)| (x as usize, y as usize))
}

fn find_portal_passage((x, y): (usize, usize), map: &Vec<Vec<Tile>>) -> Option<(usize, usize)> {
    adjacent_tiles(x, y, map).find(|(ax, ay)| map[*ay][*ax] == Tile::Passage)
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

    while let Some(((x, y), n)) = q.pop_front() {
        for npos in adjacent_tiles(x, y, map) {
            if discovered.contains(&npos) {
                continue;
            }

            discovered.insert(npos);

            let (nx, ny) = npos;
            match map[ny][nx] {
                Tile::Passage => {
                    q.push_back((npos, n + 1));
                    discovered.insert(npos);
                }
                Tile::Portal(_) => {
                    if map[ny][nx] == target {
                        return Some(n);
                    }
                    if let Some(dst_pos) = portal_map.get(&npos) {
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

    q.push_back((start, 0, 0usize));
    discovered.entry(0).or_insert(HashSet::new()).insert(start);

    while let Some(((x, y), n, level)) = q.pop_front() {
        for npos in adjacent_tiles(x, y, map) {
            if discovered.get(&level).unwrap().contains(&npos) {
                continue;
            }

            discovered.get_mut(&level).unwrap().insert(npos);

            let (nx, ny) = npos;
            match map[ny][nx] {
                Tile::Passage => {
                    q.push_back((npos, n + 1, level));
                }
                Tile::Portal(_) => {
                    if level == 0 && map[ny][nx] == target {
                        return Some(n);
                    }
                    if let Some(dst_pos) = portal_map.get(&npos) {
                        let next_level = if outer_portal(npos, map) {
                            level.checked_sub(1)
                        } else {
                            level.checked_add(1)
                        };
                        if let Some(next_level) = next_level {
                            if !discovered
                                .entry(next_level)
                                .or_insert(HashSet::new())
                                .contains(&dst_pos)
                            {
                                let passage = find_portal_passage(*dst_pos, map)
                                    .expect("portal passage not found");
                                discovered.get_mut(&next_level).unwrap().insert(*dst_pos);
                                discovered.get_mut(&next_level).unwrap().insert(passage);
                                q.push_back((passage, n + 1, next_level));
                            }
                        }
                    }
                }
                _ => (),
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
