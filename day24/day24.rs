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
                    .map(|(x, t)| {
                        let n = adjacent_bugs(x, y, &state);
                        match t {
                            Tile::Bug => {
                                if n == 1 {
                                    Tile::Bug
                                } else {
                                    Tile::Empty
                                }
                            }
                            Tile::Empty => {
                                if n == 1 || n == 2 {
                                    Tile::Bug
                                } else {
                                    Tile::Empty
                                }
                            }
                        }
                    })
                    .collect()
            })
            .collect();
    }

    state
}

fn biodiversity_rating(state: State) -> usize {
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

fn new_empty_state() -> State {
    let mut state = Vec::new();

    for _ in 0..5 {
        let mut row = Vec::new();
        for _ in 0..5 {
            row.push(Tile::Empty);
        }
        state.push(row);
    }

    state
}

fn state_is_empty(state: &State) -> bool {
    state.iter().flatten().filter(|&t| *t == Tile::Bug).count() > 0
}

fn run_for(initial_state: &State, minutes: usize) -> HashMap<i32, State> {
    let mut states = HashMap::new();
    states.insert(0, initial_state.clone());

    let mut min_state = 0;
    let mut max_state = 0;

    for _ in 0..minutes {
        let mut new_states = HashMap::new();

        if state_is_empty(states.get(&min_state).unwrap()) {
            states.insert(min_state - 1, new_empty_state());
            min_state -= 1;
        }
        if state_is_empty(states.get(&max_state).unwrap()) {
            states.insert(max_state + 1, new_empty_state());
            max_state += 1;
        }

        for (&state_id, state) in &states {
            let mut new_state = state.clone();

            for y in 0..state.len() {
                for x in 0..state[y].len() {
                    if y == 2 && x == 2 {
                        continue;
                    }

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

                    new_state[y][x] = match state[y][x] {
                        Tile::Bug => {
                            if cnt == 1 {
                                Tile::Bug
                            } else {
                                Tile::Empty
                            }
                        }
                        Tile::Empty => {
                            if cnt == 1 || cnt == 2 {
                                Tile::Bug
                            } else {
                                Tile::Empty
                            }
                        }
                    };
                }
            }

            new_states.insert(state_id, new_state);
        }

        states = new_states;
    }

    states
}

fn count_bugs(states: HashMap<i32, State>) -> usize {
    states
        .iter()
        .map(|(_, state)| state.iter().flatten().filter(|&t| *t == Tile::Bug).count())
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
        biodiversity_rating(run_until_repeat(&initial_state))
    );

    println!("part 2: {}", count_bugs(run_for(&initial_state, 200)));
}
