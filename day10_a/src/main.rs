use std::fs;
use std::ops::RangeInclusive;

enum Instruction {
    Noop,
    Addx(isize),
}

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let instructions = extract(&file);

    let signal_strength_sum = execute_program(&instructions);

    println!("{}", signal_strength_sum);
}

/// Runs the program and returns the total sum of interesting signal strengths
fn execute_program(instructions: &[Instruction]) -> isize {
    let mut current_cycle: isize = 0;
    let mut register: isize = 1;
    let mut signal_strength_sum: isize = 0;

    for instruction in instructions.iter() {
        let (cycle, x) = match instruction {
            Instruction::Noop => { (1, 0) }
            Instruction::Addx(x) => (2, *x)
        };

        // +1 as we already checked the current cycle at the previous run
        let cycle_range = (current_cycle + 1)..=(current_cycle + cycle);

        if let Some(interesting_cycle) = get_interesting_signal(cycle_range) {
            // The instruction has not completed yet so the register is still
            // the same as it was before the instruction
            let signal_strength = interesting_cycle * register;
            signal_strength_sum += signal_strength;
        }

        current_cycle += cycle;
        register += x;
    }

    signal_strength_sum
}

/// If the given range contains a cycle that is interesting, return the cycle of interest.
fn get_interesting_signal(mut range: RangeInclusive<isize>) -> Option<isize> {
    range.find(|cycle| is_interesting_signal(*cycle))
}

/// Whether the given cycle is interesting
fn is_interesting_signal(cycle: isize) -> bool {
    (cycle - 20) % 40 == 0
}

/// Extracts the given file into a vector of instructions
fn extract(to_extract: &str) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();

    for line in to_extract.lines() {
        let instruction = match line {
            "noop" => Instruction::Noop,
            line => {
                let (_, x) = line.split_once(" ").unwrap();

                Instruction::Addx(x.parse().unwrap())
            }
        };

        instructions.push(instruction)
    }

    instructions
}
