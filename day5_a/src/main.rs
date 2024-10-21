use std::fs;

type Stack<'a> = Vec<&'a str>;
type Stacks<'a> = Vec<Stack<'a>>;
type Instruction = (usize, usize, usize);
type Instructions = Vec<Instruction>;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let (mut stacks, instructions) = extract(&file);

    for instruction in instructions.iter() {
        stacks = do_instruction(instruction, stacks);
    }

    let tops = get_top_of_stacks(&stacks);

    println!("{}", tops)
}

/// Executes the provided instruction by moving the string references between vectors
fn do_instruction<'a>((amount, from, to): &Instruction, mut stacks: Stacks<'a>) -> Stacks<'a> {
    let from_len = stacks[*from].len();
    // Drain the specified amount from the top of the vector
    // Crates are moved on at a time so there order gets reversed
    // The drained values need to be collected into another vector as the iterator from .drain
    // keeps a mutable reference of the vector it's drained from
    // we need to ensure that mutable reference is dropped before we can move the drained values to its new location
    let to_move: Vec<&str> = stacks[*from].drain((from_len - amount)..from_len).rev().collect();

    stacks[*to].extend(to_move);

    stacks
}

/// Gets the last (or top) item of all stacks and adds them to a single string
fn get_top_of_stacks(stacks: &Stacks) -> String {
    let mut tops = String::new();

    for stack in stacks.iter() {
        if let Some(top) = stack.last() {
            tops.push_str(top);
        }
    }

    tops
}

/// Extracts the provided file into another format.
/// In this case it produces 2 sections:
/// - The current state of the stacks
/// - The instructions that need to be applied
fn extract(to_extract: &str) -> (Stacks, Instructions) {
    let (unparsed_stacks, unparsed_instructions) = to_extract.split_once("\n\n").unwrap();

    let stacks = extract_stacks(unparsed_stacks);
    let instructions = extract_instructions(unparsed_instructions);

    (stacks, instructions)
}

/// Extracts the current state of the stacks
/// into a vector
fn extract_stacks(to_extract: &str) -> Stacks {
    let mut stacks: Stacks = Vec::new();

    let identifier_line = to_extract.lines().last().unwrap();

    for (at, char) in identifier_line.chars().enumerate() {
        if !char.is_numeric() { continue; }

        stacks.push(extract_stack(to_extract, at))
    }

    stacks
}

/// Extracts a single stack from the provided data
//  At is the position on the lines where to extract the data from
fn extract_stack(to_extract: &str, at: usize) -> Stack {
    let mut stack: Stack = Vec::new();

    for line in to_extract.lines().rev().skip(1) {
        if line.len() < at + 1 { break; }

        let item = &line[at..(at + 1)];
        if item.trim().is_empty() { break; }

        stack.push(item)
    }

    stack
}

/// Extracts the needed information out of the instructions
/// into a tuple.
fn extract_instructions(to_extract: &str) -> Instructions {
    let mut instructions: Instructions = Vec::new();

    for line in to_extract.lines() {
        let mut line_iterator = line.split(" ");

        let amount: usize = line_iterator.nth(1).unwrap().parse().unwrap();
        // -1 as vector starts at index 0
        let from: usize = line_iterator.nth(1).unwrap().parse::<usize>().unwrap() - 1;
        let to: usize = line_iterator.nth(1).unwrap().parse::<usize>().unwrap() - 1;

        instructions.push((amount, from, to))
    }

    instructions
}