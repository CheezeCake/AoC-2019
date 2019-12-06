use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

type OrbitInfo = HashMap<String, HashSet<String>>;

#[derive(Debug, Clone)]
struct OrbitCount {
    direct: usize,
    indirect: usize,
}

impl OrbitCount {
    fn new() -> Self {
        Self {
            direct: 0,
            indirect: 0,
        }
    }
}

fn count_orbits(objects_orbiting: &OrbitInfo, in_orbit_around: &OrbitInfo) -> usize {
    let mut in_orbit_around = in_orbit_around.clone();
    let mut orbit_count = HashMap::new();
    orbit_count.insert(String::from("COM"), OrbitCount::new());

    while !in_orbit_around.is_empty() {
        let not_orbiting_name = in_orbit_around
            .iter()
            .find(|(_, s)| s.is_empty())
            .expect("dependency cycle detected")
            .0
            .clone();

        let not_orbiting_count = orbit_count
            .entry(not_orbiting_name.clone())
            .or_insert(OrbitCount::new())
            .clone();

        for object_orbiting in objects_orbiting
            .get(&not_orbiting_name)
            .unwrap_or(&HashSet::new())
        {
            let entry = orbit_count
                .entry(object_orbiting.to_string())
                .or_insert(OrbitCount::new());
            entry.direct += 1;
            entry.indirect = not_orbiting_count.direct + not_orbiting_count.indirect;
            in_orbit_around
                .get_mut(object_orbiting)
                .unwrap()
                .remove(&not_orbiting_name);
        }

        in_orbit_around.remove(&not_orbiting_name);
    }

    orbit_count
        .iter()
        .map(|(_, cnt)| cnt.direct + cnt.indirect)
        .sum()
}

fn shortest_path(
    objects_orbiting: &OrbitInfo,
    in_orbit_around: &OrbitInfo,
    start: &String,
    target: &String,
) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((start, 0));
    visited.insert(start);

    let empty = HashSet::new();
    while !q.is_empty() {
        let (obj, n) = q.pop_front().unwrap();
        if in_orbit_around
            .get(target)
            .unwrap_or(&HashSet::new())
            .contains(obj)
        {
            return Some(n - 1);
        }

        for o in objects_orbiting.get(obj).unwrap_or(&empty) {
            if !visited.contains(o) {
                q.push_back((o, n + 1));
                visited.insert(o);
            }
        }
        for o in in_orbit_around.get(obj).unwrap_or(&empty) {
            if !visited.contains(o) {
                q.push_back((o, n + 1));
                visited.insert(o);
            }
        }
    }

    None
}

fn main() {
    let mut objects_orbiting: OrbitInfo = OrbitInfo::new();
    let mut in_orbit_around: OrbitInfo = OrbitInfo::new();

    in_orbit_around.insert(String::from("COM"), HashSet::new());

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(')').collect();
        assert_eq!(parts.len(), 2);

        let (obj1, obj2) = (parts[0].to_string(), parts[1].to_string());
        objects_orbiting
            .entry(obj1.clone())
            .or_insert(HashSet::new())
            .insert(obj2.clone());
        in_orbit_around
            .entry(obj2.clone())
            .or_insert(HashSet::new())
            .insert(obj1.clone());
    }

    println!(
        "part 1: {}",
        count_orbits(&objects_orbiting, &in_orbit_around)
    );

    println!(
        "part 2: {}",
        shortest_path(
            &objects_orbiting,
            &in_orbit_around,
            &String::from("YOU"),
            &String::from("SAN")
        )
        .expect("no path found")
    );
}
