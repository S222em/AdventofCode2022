use std::fs;

// Do the same as before but the other way around
// Round result is known, our played shape is not
// Simply find the shape that needs to be played to get the wanted result
fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let mut total_score = 0;

    for line in file.lines() {
        let (left, right) = line.split_once(" ").unwrap();

        total_score += find_score_of_round(left, right);
    }

    println!("{}", total_score)
}

// Finds the total score of the round
fn find_score_of_round(left: &str, right: &str) -> usize {
    find_shape_value(left, right) + get_round_outcome(right)
}

// Finds the shape that needs to played for the correct result and returns it's value
fn find_shape_value(left: &str, right: &str) -> usize {
    match right {
        // To lose
        "X" if left == "A" => 3,
        "X" if left == "B" => 1,
        "X" if left == "C" => 2,
        // To draw
        "Y" if left == "A" => 1,
        "Y" if left == "B" => 2,
        "Y" if left == "C" => 3,
        // To win
        "Z" if left == "A" => 2,
        "Z" if left == "B" => 3,
        "Z" if left == "C" => 1,
        _ => 0,
    }
}


// Returns the result of the round
fn get_round_outcome(right: &str) -> usize {
    match right {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0,
    }
}