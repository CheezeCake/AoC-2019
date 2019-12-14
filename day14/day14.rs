use std::collections::HashMap;
use std::io;
use std::io::prelude::*;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Chemical {
    name: String,
    quantity: usize,
}

impl FromStr for Chemical {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        assert_eq!(parts.len(), 2);
        Ok(Chemical {
            name: parts[1].to_string(),
            quantity: parts[0].parse()?,
        })
    }
}

#[derive(Debug)]
struct Reaction {
    quantity_produced: usize,
    ingredients: Vec<Chemical>,
}

fn get_ore_requirements(
    target: &str,
    amount: usize,
    reactions: &HashMap<String, Reaction>,
    leftovers: &mut HashMap<String, usize>,
) -> usize {
    let mut chemicals: HashMap<String, usize> = reactions
        .get(&target.to_string())
        .expect("target element not found")
        .ingredients
        .iter()
        .map(|chemical| (chemical.name.clone(), chemical.quantity * amount))
        .collect();

    let mut done = false;

    while !done {
        let mut new_chemicals = HashMap::new();

        done = true;

        for (chemical, quantity) in &chemicals {
            if *chemical == String::from("ORE") {
                *new_chemicals.entry(chemical.clone()).or_insert(0) += *quantity;
                continue;
            }

            done = false;

            let quantity_leftovers = leftovers.get(&chemical.clone()).unwrap_or(&0).clone();
            if quantity_leftovers >= *quantity {
                leftovers.insert(chemical.clone(), quantity_leftovers - quantity);
                continue;
            }

            let quantity = quantity - quantity_leftovers;
            leftovers.remove(chemical);

            let produced = reactions.get(chemical).unwrap().quantity_produced;
            let factor = (quantity / produced) + if quantity % produced > 0 { 1 } else { 0 };

            for new_chemical in &reactions.get(chemical).unwrap().ingredients {
                *new_chemicals.entry(new_chemical.name.clone()).or_insert(0) +=
                    new_chemical.quantity * factor;
            }

            if produced * factor > quantity {
                *leftovers.entry(chemical.clone()).or_insert(0) += produced * factor - quantity;
            }
        }

        chemicals = new_chemicals;
    }

    *chemicals.get("ORE").expect("no ORE")
}

fn main() {
    let reactions: HashMap<String, Reaction> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let parts: Vec<&str> = line.split(" => ").collect();
            assert_eq!(parts.len(), 2);
            let ingredients: Vec<Chemical> = parts[0]
                .split(", ")
                .map(|chemical| chemical.parse().unwrap())
                .collect();
            let output: Chemical = parts[1].parse().unwrap();
            (
                output.name,
                Reaction {
                    quantity_produced: output.quantity,
                    ingredients,
                },
            )
        })
        .collect();

    let mut leftovers: HashMap<String, usize> = HashMap::new();
    let ore_per_fuel = get_ore_requirements("FUEL", 1, &reactions, &mut leftovers);
    println!("part 1: {}", ore_per_fuel);

    const TRILLION: usize = 1_000_000_000_000;
    let mut fuel = 0;
    let mut ore_required = 0;
    let mut try = 1_000_000;
    let mut leftovers: HashMap<String, usize> = HashMap::new();
    while try > 0 {
        let mut leftovers_copy = leftovers.clone();
        let ore = get_ore_requirements("FUEL", try, &reactions, &mut leftovers_copy);
        if ore_required + ore <= TRILLION {
            ore_required += ore;
            fuel += try;
            leftovers = leftovers_copy;
        } else {
            try /= 2;
        }
    }

    println!("part 2: {}", fuel);
}
