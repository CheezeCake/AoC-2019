use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::prelude::*;

fn adjacent_bugs(x: usize, y: usize, state: &Vec<Vec<char>>) -> usize {
    [(0, -1), (1, 0), (0, 1), (-1, 0)]
        .iter()
        .map(|(dx, dy)| (x as i32 + dx, y as i32 + dy))
        .filter(|&(x, y)| {
            y >= 0
                && (y as usize) < state.len()
                && x >= 0
                && (x as usize) < state[y as usize].len()
                && state[y as usize][x as usize] == '#'
        })
        .count()
}

fn run_until_repeat(initial_state: &Vec<Vec<char>>) -> Vec<Vec<char>> {
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
                            '#' => {
                                if n == 1 {
                                    '#'
                                } else {
                                    '.'
                                }
                            }
                            '.' => {
                                if n == 1 || n == 2 {
                                    '#'
                                } else {
                                    '.'
                                }
                            }
                            _ => panic!("invalid tile: {}", t),
                        }
                    })
                    .collect()
            })
            .collect();
    }

    state
}

fn biodiversity_rating(state: Vec<Vec<char>>) -> usize {
    let mut rating = 0;
    let mut p = 1;

    for &t in state.iter().flatten() {
        if t == '#' {
            rating += p;
        }
        p *= 2;
    }

    rating
}

fn new_empty_state() -> Vec<Vec<char>> {
    let mut state = Vec::new();

    for _ in 0..5 {
        let mut row = Vec::new();
        for _ in 0..5 {
            row.push('.');
        }
        state.push(row);
    }

    state
}

fn print_states(states: &HashMap<i32, Vec<Vec<char>>>, min_state: &i32, max_state: &i32) {
    for id in *min_state..=*max_state {
        println!("{}:", id);
        let state = states.get(&id).unwrap();
        for y in 0..state.len() {
            for x in 0..state[y].len() {
                print!("{}", state[y][x])
            }
            println!();
        }
    }
}

fn main() {
    let initial_state: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();

    println!(
        "part 1: {}",
        biodiversity_rating(run_until_repeat(&initial_state))
    );

    let mut states = HashMap::new();
    states.insert(0, initial_state.clone());

    let mut min_state = 0;
    let mut max_state = 0;

    for _ in 0..200 {
        let mut new_states = HashMap::new();

        if states
            .get(&min_state)
            .unwrap()
            .iter()
            .flatten()
            .filter(|&t| *t == '#')
            .count()
            > 0
        {
            states.insert(min_state - 1, new_empty_state());
            min_state -= 1;
        }
        if states
            .get(&max_state)
            .unwrap()
            .iter()
            .flatten()
            .filter(|&t| *t == '#')
            .count()
            > 0
        {
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
                                if prev_state[2][1] == '#' {
                                    cnt += 1;
                                }
                            }
                        } else if ax as usize == state[y].len() {
                            if let Some(prev_state) = states.get(&(state_id as i32 - 1)) {
                                if prev_state[2][3] == '#' {
                                    cnt += 1;
                                }
                            }
                        } else if ay == -1 {
                            if let Some(prev_state) = states.get(&(state_id as i32 - 1)) {
                                if prev_state[1][2] == '#' {
                                    cnt += 1;
                                }
                            }
                        } else if ay as usize == state.len() {
                            if let Some(prev_state) = states.get(&(state_id as i32 - 1)) {
                                if prev_state[3][2] == '#' {
                                    cnt += 1;
                                }
                            }
                        } else if ax == 2 && ay == 2 {
                            if x == 1 && y == 2 {
                                if let Some(next_state) = states.get(&(state_id as i32 + 1)) {
                                    cnt += (0..next_state.len())
                                        .map(|y| next_state[y][0])
                                        .filter(|&t| t == '#')
                                        .count();
                                }
                            } else if x == 3 && y == 2 {
                                if let Some(next_state) = states.get(&(state_id as i32 + 1)) {
                                    cnt += (0..next_state.len())
                                        .map(|y| next_state[y][next_state[y].len() - 1])
                                        .filter(|&t| t == '#')
                                        .count();
                                }
                            } else if x == 2 && y == 1 {
                                if let Some(next_state) = states.get(&(state_id as i32 + 1)) {
                                    cnt += (0..next_state[0].len())
                                        .map(|x| next_state[0][x])
                                        .filter(|&t| t == '#')
                                        .count();
                                }
                            } else if x == 2 && y == 3 {
                                if let Some(next_state) = states.get(&(state_id as i32 + 1)) {
                                    cnt += (0..next_state[next_state.len() - 1].len())
                                        .map(|x| next_state[next_state.len() - 1][x])
                                        .filter(|&t| t == '#')
                                        .count();
                                }
                            }
                        } else if state[ay as usize][ax as usize] == '#' {
                            cnt += 1;
                        }
                    }

                    new_state[y][x] = match state[y][x] {
                        '#' => {
                            if cnt == 1 {
                                '#'
                            } else {
                                '.'
                            }
                        }
                        '.' => {
                            if cnt == 1 || cnt == 2 {
                                '#'
                            } else {
                                '.'
                            }
                        }
                        _ => panic!("invalid tile: {}", state[y][x]),
                    };
                }
            }

            new_states.insert(state_id, new_state);
        }

        states = new_states;
    }

    let mut bugs = 0;
    for (_, state) in &states {
        bugs += state.iter().flatten().filter(|&t| *t == '#').count();
    }

    println!("part 2: {}", bugs);
}
