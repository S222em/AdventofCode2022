use std::fs;

// Each elf's inventory is split by an empty line (\n\n)
// Then simply iterate over all the lines in the inventory and sum them
// Update the total if the elf's inventory is larger than one already found.
fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let inventories: Vec<&str> = file.split("\n\n").collect();

    let mut max: usize = 0;

    for inventory in inventories {
        // Map the contents of the elf's inventory to an usize and sum them
        let total: usize = inventory.lines().map(|line| line.parse::<usize>().unwrap()).sum();

        if total > max {
            max = total
        }
    }

    println!("{}", max)
}
