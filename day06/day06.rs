use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::io;
use std::io::prelude::*;

#[derive(Debug, Clone)]
struct OrbitInfo {
    is_orbiting: usize,
    objects_orbiting: Vec<String>,
}

impl OrbitInfo {
    fn new() -> Self {
        Self {
            is_orbiting: 0,
            objects_orbiting: vec![],
        }
    }
}

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

fn main() {
    let mut omap: HashMap<String, Vec<String>> = HashMap::new();

    let mut orbits: HashMap<String, OrbitInfo> = HashMap::new();
    let mut orbiting_count: HashMap<String, usize> = HashMap::new();

    orbiting_count.insert(String::from("COM"), 0);

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.split(')').collect();
        assert_eq!(parts.len(), 2);
        orbits
            .entry(parts[0].to_string())
            .or_insert(OrbitInfo::new())
            .objects_orbiting
            .push(parts[1].to_string());
        orbits
            .entry(parts[1].to_string())
            .or_insert(OrbitInfo::new())
            .is_orbiting += 1;
        *orbiting_count.entry(parts[1].to_string()).or_insert(0) += 1;

        omap.entry(parts[0].to_string())
            .or_insert(vec![])
            .push(parts[1].to_string());
        omap.entry(parts[1].to_string())
            .or_insert(vec![])
            .push(parts[0].to_string());
    }

    let mut orbit_count = HashMap::new();

    while orbiting_count.len() > 0 {
        let no_orbiting = orbiting_count
            .iter()
            .find(|(_, &n)| n == 0)
            .expect("cycle")
            .0
            .clone();
        let no_orbiting_info = orbits
            .get(&no_orbiting)
            .unwrap_or(&OrbitInfo::new())
            .clone();

        let no_orbiting_count = orbit_count
            .get(&no_orbiting)
            .unwrap_or(&OrbitCount::new())
            .clone();

        for object_orbiting in no_orbiting_info.objects_orbiting {
            let entry = orbit_count
                .entry(object_orbiting.clone())
                .or_insert(OrbitCount::new());
            entry.direct += 1;
            entry.indirect = no_orbiting_count.direct + no_orbiting_count.indirect;

            let cnt = orbiting_count.get_mut(&object_orbiting).unwrap();
            *cnt -= 1;
        }

        orbiting_count.remove(&no_orbiting);
    }

    println!(
        "part 1: {}",
        orbit_count
            .iter()
            .map(|(_, cnt)| cnt.direct + cnt.indirect)
            .sum::<usize>()
    );

    //////////////////////////////////////////////////////////:

    let mut visited = HashSet::new();
    let mut q = VecDeque::new();
    q.push_back((String::from("YOU"), 0));
    visited.insert(String::from("YOU"));

    while !q.is_empty() {
        let (obj, n) = q.pop_front().unwrap();
        if orbits
            .get(&obj)
            .unwrap_or(&OrbitInfo::new())
            .objects_orbiting
            .iter()
            .any(|s| *s == String::from("SAN"))
        {
            println!("part 2: {}", n - 1);
            return;
        }

        for o in omap.get(&obj).unwrap_or(&vec![]) {
            if !visited.contains(o) {
                q.push_back((o.to_string(), n + 1));
                visited.insert(o.to_string());
            }
        }
    }

    unreachable!("aya");
}
