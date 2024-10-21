use std::collections::HashSet;
use std::fs;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let chars_before_message_marker = find_chars_before_marker(&file).unwrap();

    println!("{}", chars_before_message_marker)
}

/// Finds the amount of chars after with the first message-marker has passed
fn find_chars_before_marker(buffer: &str) -> Option<usize> {
    let mut i = 0;

    while i < buffer.len() - 14 {
        if contains_only_unique_characters(&buffer[i..(i + 14)]) {
            return Some(i + 14);
        }

        i += 1
    }

    None
}

// Checks whether the given str consists of only unique characters
fn contains_only_unique_characters(marker: &str) -> bool {
    let chars: HashSet<char> = marker.chars().collect();

    marker.len() == chars.len()
}
