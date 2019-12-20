use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

#[derive(Hash, PartialEq, Eq, Debug)]
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

fn build_portal_map(map: &Vec<Vec<Tile>>) -> HashMap<(char, char), Vec<(usize, usize)>> {
    let mut portal_map = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if let Tile::Portal(portal_pair) = map[y][x] {
                portal_map
                    .entry(portal_pair)
                    .or_insert(Vec::new())
                    .push((x, y));
            }
        }
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

fn shortest_path(
    start: (usize, usize),
    target: Tile,
    map: &Vec<Vec<Tile>>,
    portal_map: &HashMap<(char, char), Vec<(usize, usize)>>,
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
                    Tile::Portal(portal) => {
                        if map[pos.1][pos.0] == target {
                            return Some(n);
                        }
                        for &portal_pos in portal_map.get(&portal).unwrap() {
                            if !discovered.contains(&portal_pos) {
                                let passage = find_portal_passage(portal_pos, map)
                                    .expect("portal passage not found");
                                discovered.insert(portal_pos);
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

fn main() {
    let map = read_map();
    let portal_map = build_portal_map(&map);
    let aa_portal = portal_map.get(&('A', 'A')).expect("portal AA not found");
    assert!(aa_portal.len() == 1);
    let start = find_portal_passage(aa_portal[0], &map).expect("start passage not found");

    println!(
        "part 1: {}",
        shortest_path(start, Tile::Portal(('Z', 'Z')), &map, &portal_map)
            .expect("could not found a way out of the maze")
    );
}
