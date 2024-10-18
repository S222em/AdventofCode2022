use std::fs;

// Iterate over the lines in the puzzle
// For each line, sum the value of the shape on the right hand with the result of the round
fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let mut total_score = 0;

    for line in file.lines() {
        let (left, right) = line.split_once(" ").unwrap();

        total_score += get_score_of_round(left, right);
    }

    println!("{}", total_score)
}

// Calculates the total score of the round
fn get_score_of_round(left: &str, right: &str) -> usize {
    get_shape_value(right) + get_round_outcome(left, right)
}

// Returns the value of the played shape
fn get_shape_value(shape: &str) -> usize {
    match shape {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    }
}

// Returns the result of the round
fn get_round_outcome(left: &str, right: &str) -> usize {
    match left {
        "A" if right == "Y" => 6,
        "B" if right == "Z" => 6,
        "C" if right == "X" => 6,
        "A" if right == "X" => 3,
        "B" if right == "Y" => 3,
        "C" if right == "Z" => 3,
        _ => 0,
    }
}

