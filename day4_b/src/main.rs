use std::fs;

type Range = (usize, usize);
type Pair = (Range, Range);

// Convert the puzzle to a vec with 2 tuples (start, end)
// Check whether the ranges overlap
fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let pairs: Vec<Pair> = parse(&file);

    let mut total: usize = 0;

    for (left, right) in pairs {
        if overlap(&left, &right) { total += 1 }
    }

    println!("{}", total);
}

// Checks whether 2 ranges overlap
fn overlap(a: &Range, b: &Range) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

// Parses the file into a vec with 2 (start, end) tuples
fn parse(file: &str) -> Vec<Pair> {
    let mut pairs = Vec::new();

    for line in file.lines() {
        let (left, right) = line.split_once(",").unwrap();

        pairs.push((parse_range(left), parse_range(right)))
    }

    pairs
}

// Parses a single range x-y into a (start, end) tuple
fn parse_range(range: &str) -> Range {
    let (start, end) = range.split_once("-").unwrap();

    (start.parse().unwrap(), end.parse().unwrap())
}