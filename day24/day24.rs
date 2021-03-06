use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Bug,
    Empty,
}

type State = Vec<Vec<Tile>>;

fn adjacent_bugs(x: usize, y: usize, state: &State) -> usize {
    [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .iter()
        .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(|&(x, y)| {
            y >= 0
                && (y as usize) < state.len()
                && x >= 0
                && (x as usize) < state[y as usize].len()
                && state[y as usize][x as usize] == Tile::Bug
        })
        .count()
}

fn next_tile(tile: &Tile, adj_bugs: usize) -> Tile {
    match tile {
        Tile::Bug if adj_bugs == 1 => Tile::Bug,
        Tile::Empty if adj_bugs == 1 || adj_bugs == 2 => Tile::Bug,
        _ => Tile::Empty,
    }
}

fn run_until_repeat(initial_state: &State) -> State {
    let mut state = initial_state.clone();
    let mut seen_states = HashSet::new();

    while seen_states.insert(state.clone()) {
        state = state
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, t)| next_tile(t, adjacent_bugs(x, y, &state)))
                    .collect()
            })
            .collect();
    }

    state
}

fn biodiversity_rating(state: &State) -> usize {
    state
        .iter()
        .flatten()
        .fold((1, 0), |(p, rating), t| {
            (
                p * 2,
                match t {
                    Tile::Bug => rating + p,
                    _ => rating,
                },
            )
        })
        .1
}

fn new_empty_state(height: usize, width: usize) -> State {
    (0..height)
        .map(|_| (0..width).map(|_| Tile::Empty).collect())
        .collect()
}

fn state_count_bugs(state: &State) -> usize {
    state.iter().flatten().filter(|&t| *t == Tile::Bug).count()
}

fn adjacent_bugs_recursive(
    x: usize,
    y: usize,
    state: &State,
    state_id: i32,
    states: &HashMap<i32, State>,
) -> usize {
    let mut cnt = 0;

    for (dx, dy) in &[(0, -1), (1, 0), (0, 1), (-1, 0)] {
        let (ax, ay) = (x as i32 + dx, y as i32 + dy);
        if ax == -1 {
            if let Some(prev_state) = states.get(&(state_id as i32 - 1)) {
                if prev_state[2][1] == Tile::Bug {
                    cnt += 1;
                }
            }
        } else if ax as usize == state[y].len() {
            if let Some(prev_state) = states.get(&(state_id as i32 - 1)) {
                if prev_state[2][3] == Tile::Bug {
                    cnt += 1;
                }
            }
        } else if ay == -1 {
            if let Some(prev_state) = states.get(&(state_id as i32 - 1)) {
                if prev_state[1][2] == Tile::Bug {
                    cnt += 1;
                }
            }
        } else if ay as usize == state.len() {
            if let Some(prev_state) = states.get(&(state_id as i32 - 1)) {
                if prev_state[3][2] == Tile::Bug {
                    cnt += 1;
                }
            }
        } else if ax == 2 && ay == 2 {
            if x == 1 && y == 2 {
                if let Some(next_state) = states.get(&(state_id as i32 + 1)) {
                    cnt += (0..next_state.len())
                        .map(|y| next_state[y][0])
                        .filter(|&t| t == Tile::Bug)
                        .count();
                }
            } else if x == 3 && y == 2 {
                if let Some(next_state) = states.get(&(state_id as i32 + 1)) {
                    cnt += (0..next_state.len())
                        .map(|y| next_state[y][next_state[y].len() - 1])
                        .filter(|&t| t == Tile::Bug)
                        .count();
                }
            } else if x == 2 && y == 1 {
                if let Some(next_state) = states.get(&(state_id as i32 + 1)) {
                    cnt += (0..next_state[0].len())
                        .map(|x| next_state[0][x])
                        .filter(|&t| t == Tile::Bug)
                        .count();
                }
            } else if x == 2 && y == 3 {
                if let Some(next_state) = states.get(&(state_id as i32 + 1)) {
                    cnt += (0..next_state[next_state.len() - 1].len())
                        .map(|x| next_state[next_state.len() - 1][x])
                        .filter(|&t| t == Tile::Bug)
                        .count();
                }
            }
        } else if state[ay as usize][ax as usize] == Tile::Bug {
            cnt += 1;
        }
    }

    cnt
}

fn next_state(state: &State, state_id: i32, states: &HashMap<i32, State>) -> State {
    state
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &t)| {
                    if x == 2 && y == 2 {
                        t
                    } else {
                        next_tile(&t, adjacent_bugs_recursive(x, y, state, state_id, states))
                    }
                })
                .collect()
        })
        .collect()
}

fn run_for(initial_state: &State, minutes: usize) -> HashMap<i32, State> {
    let mut states = HashMap::new();
    states.insert(0, initial_state.clone());

    let mut min_state = 0;
    let mut max_state = 0;

    for _ in 0..minutes {
        let mut new_states = HashMap::new();

        if state_count_bugs(states.get(&min_state).unwrap()) > 0 {
            states.insert(
                min_state - 1,
                new_empty_state(initial_state.len(), initial_state.len()),
            );
            min_state -= 1;
        }
        if state_count_bugs(states.get(&max_state).unwrap()) > 0 {
            states.insert(
                max_state + 1,
                new_empty_state(initial_state.len(), initial_state.len()),
            );
            max_state += 1;
        }

        for (&state_id, state) in &states {
            new_states.insert(state_id, next_state(state, state_id, &states));
        }

        states = new_states;
    }

    states
}

fn count_bugs(states: &HashMap<i32, State>) -> usize {
    states
        .iter()
        .map(|(_, state)| state_count_bugs(state))
        .sum()
}

fn main() {
    let initial_state: State = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| match c {
                    '#' => Tile::Bug,
                    '.' => Tile::Empty,
                    _ => panic!("invalid tile: {}", c),
                })
                .collect()
        })
        .collect();

    println!(
        "part 1: {}",
        biodiversity_rating(&run_until_repeat(&initial_state))
    );

    println!("part 2: {}", count_bugs(&run_for(&initial_state, 200)));
}
