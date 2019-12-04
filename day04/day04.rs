use std::collections::HashSet;
use std::env;

fn group_lengths(s: &str) -> HashSet<usize> {
    let bytes = s.as_bytes();
    let mut len = 1;
    let mut lengths = HashSet::new();

    for i in 1..bytes.len() {
        if bytes[i - 1] == bytes[i] {
            len += 1;
        } else {
            lengths.insert(len);
            len = 1;
        }
    }

    lengths.insert(len);

    lengths
}

fn increasing(s: &str) -> bool {
    let bytes = s.as_bytes();
    for i in 1..bytes.len() {
        if bytes[i - 1] > bytes[i] {
            return false;
        }
    }
    true
}

fn main() {
    let bounds: Vec<u32> = env::args()
        .nth(1)
        .expect("arguments: lower_bound-upper_bound")
        .split('-')
        .map(|s| s.parse().unwrap())
        .collect();
    assert_eq!(bounds.len(), 2);

    let increasing: Vec<HashSet<usize>> = (bounds[0]..=bounds[1])
        .map(|x| x.to_string())
        .filter(|s| increasing(s))
        .map(|s| group_lengths(&s))
        .collect();

    let count = |pred: &dyn Fn(usize) -> bool| {
        increasing
            .iter()
            .filter(|gl| gl.iter().any(|&len| pred(len)))
            .count()
    };

    println!("part 1: {}", count(&|len| len >= 2));

    println!("part 2: {}", count(&|len| len == 2));
}
