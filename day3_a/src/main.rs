use std::collections::HashSet;
use std::fs;


// Split each line into 2 sections of equal length
// Convert the chars in each section to an usize and reduce to range 1-52
// Collect the usizes into a hashset and intersect
fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let rucksacks = parse(&file);

    let mut total_priority = 0;

    for (left, right) in rucksacks {
        for item in left.intersection(&right) { total_priority += item; }
    }

    println!("{}", total_priority)
}

// Parses the puzzle input into 2 compartments containing a hashset with usizes (the item's priority)
fn parse(file: &str) -> Vec<(HashSet<usize>, HashSet<usize>)> {
    let mut rucksacks = Vec::new();

    for line in file.lines() {
        let center = line.len() / 2;
        let (left, right) = line.split_at(center);
        rucksacks.push((parse_compartment(left), parse_compartment(right)));
    }

    rucksacks
}

// Parses a single compartement to a hashset with usizes (item priority) in range 1-52
fn parse_compartment(items: &str) -> HashSet<usize> {
    items.chars().map(|char| {
        if char.is_lowercase() { return char as usize - 96; }

        char as usize - 38
    }).collect()
}