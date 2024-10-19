use std::collections::HashSet;
use std::fs;

// Each rucksack needs to be converted to a hashset with usizes (item priority in range 1-52)
// Calculate the intersection between 3 consecutive lines in the puzzle
// Only result from that intersection is the badge of the group
fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let rucksacks = parse(&file);

    let mut rucksacks_iter = rucksacks.into_iter();

    let mut total_priority = 0;

    while let (Some(a), Some(b), Some(c)) = (rucksacks_iter.next(), rucksacks_iter.next(), rucksacks_iter.next()) {
        let intersection_bc: HashSet<usize> = b.intersection(&c).copied().collect();
        let mut intersection_abc = a.intersection(&intersection_bc);

        total_priority += intersection_abc.next().unwrap();
    }

    println!("{}", total_priority)
}

// Parses the file into hashsets with item priority
fn parse(file: &str) -> Vec<HashSet<usize>> {
    let mut rucksacks = Vec::new();

    for line in file.lines() {
        rucksacks.push(parse_compartment(line));
    }

    rucksacks
}

// Parses a single compartment into a hashwith with item priority 1-52
fn parse_compartment(items: &str) -> HashSet<usize> {
    items.chars().map(|char| {
        if char.is_lowercase() { return char as usize - 96; }

        char as usize - 38
    }).collect()
}