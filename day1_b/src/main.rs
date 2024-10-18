use std::fs;

// Each inventory is separated by an empty line (\n\n)
// Iterate over the inventories and sum its contents
// Map these sums to a vector and sort the vector (low->high)
// Reverse the sorted vector and take first three, which are the 3 elves carrying the most and sum
fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let lists: Vec<&str> = file.split("\n\n").collect();

    let mut totals: Vec<usize> = lists.iter().map(|list| {
        list.lines().map(|line| line.parse::<usize>().unwrap()).sum()
    }).collect();

    totals.sort();

    let total: usize = totals.iter().rev().take(3).sum();

    println!("{}", total)
}
