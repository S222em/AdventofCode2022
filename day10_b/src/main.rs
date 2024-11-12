use std::fs;

enum Instruction {
    Noop,
    Addx(isize),
}

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let instructions = extract(&file);

    let image = render_image(&instructions);

    display_image(image)
}

/// Prints the rendered image to the screen
fn display_image(rows: Vec<String>) {
    for row in rows {
        println!("{}", row)
    }
}

/// Runs the program and renders the image
fn render_image(instructions: &[Instruction]) -> Vec<String> {
    let mut current_cycle: isize = 0;
    let mut register: isize = 1;
    let mut rows: Vec<String> = Vec::from([String::new()]);

    for instruction in instructions.iter() {
        let (cycle, x) = match instruction {
            Instruction::Noop => { (1, 0) }
            Instruction::Addx(x) => (2, *x)
        };

        for sub_cycle in (current_cycle + 1)..=(current_cycle + cycle) {
            let pixel = sub_cycle - 1 - ((rows.len() - 1) * 40) as isize;

            // If the difference between the register and current pixel is less than 1
            // The sprite is in the current pixel so the pixel should be lit
            match (register - pixel).abs() <= 1 {
                true => rows.last_mut().unwrap().push('#'),
                false => rows.last_mut().unwrap().push('.')
            }

            // If we filled the row make a new one
            if rows.last().unwrap().len() >= 40 {
                rows.push(String::new())
            }
        }

        current_cycle += cycle;
        register += x;
    }

    rows
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
