use std::fs;

type Range = (usize, usize);
type Pair = (Range, Range);

// Parse the file into a two tuples (start, end), or ranges
// Check if one of the ranges contains the other
fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let pairs: Vec<Pair> = parse(&file);

    let mut total: usize = 0;

    for (left, right) in pairs {
        if fully_contains(&left, &right) || fully_contains(&right, &left) { total += 1 }
    }

    println!("{}", total);
}

// Checks whether range b fits completely in a
fn fully_contains(a: &Range, b: &Range) -> bool {
    a.0 <= b.0 && a.1 >= b.1
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