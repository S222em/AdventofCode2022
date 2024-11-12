use std::fs;

/// Stores all information for a single Monkey
struct Monkey<'a> {
    items: Vec<usize>,
    items_inspected: usize,
    operation: Vec<&'a str>,
    test: Vec<&'a str>,
    test_true: usize,
    test_false: usize,
}

impl<'a> Monkey<'a> {
    // Parses the specified number
    // If number is "old" it is replaced with the current item
    pub fn _parse_operation_number(item: usize, number: &str) -> usize {
        match number {
            "old" => item,
            number => number.parse().unwrap()
        }
    }

    // Performs the operation of the monkey
    // Returns the worry level after inspection
    pub fn inspect(&mut self, item: usize) -> usize {
        let left = Self::_parse_operation_number(item, self.operation[0]);
        let right = Self::_parse_operation_number(item, self.operation[2]);

        let result = match self.operation[1] {
            "/" => left / right,
            "*" => left * right,
            "+" => left + right,
            "-" => left - right,
            _ => panic!("Unknown operator")
        };

        self.items_inspected += 1;

        result / 3
    }

    // Performs the monkeys test
    // Returns the monkey to throw too
    pub fn throw(&self, item: usize) -> usize {
        let result = match self.test[0] {
            "divisible" => item % self.test[1].parse::<usize>().unwrap() == 0,
            _ => panic!("Unknown test")
        };

        match result {
            true => self.test_true,
            false => self.test_false
        }
    }
}

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let monkeys = extract(&file);

    let monkeys = do_rounds(monkeys);

    let monkey_business = get_monkey_business(monkeys);

    println!("{}", monkey_business)
}

/// Returns the amount of monkey business
/// Calculated by multiplying the 2 highest amounts
/// of items inspected
fn get_monkey_business(mut monkeys: Vec<Monkey>) -> usize {
    monkeys.sort_by_key(|monkey| monkey.items_inspected);

    let last = monkeys.len() - 1;

    monkeys[last].items_inspected * monkeys[last - 1].items_inspected
}

/// Performs 20 rounds of monkeys being stupid
fn do_rounds(mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
    for _ in 0..20 {
        monkeys = do_round(monkeys)
    }

    monkeys
}

/// Performs a single round
fn do_round(mut monkeys: Vec<Monkey>) -> Vec<Monkey> {
    for i in 0..monkeys.len() {
        for _ in 0..monkeys[i].items.len() {
            let item = monkeys[i].items.remove(0);
            let inspection_result = monkeys[i].inspect(item);
            let throw_to = monkeys[i].throw(inspection_result);
            monkeys[throw_to].items.push(inspection_result);
        }
    }

    monkeys
}

// Trims unneeded info from the line
fn trim_line(line: &str) -> &str {
    let (_, right) = line.split_once(":").unwrap();
    right.trim()
}

// Extracts the items from the line
fn extract_items(line: &str) -> Vec<usize> {
    trim_line(line)
        .split(", ")
        .map(|str| str.parse().unwrap())
        .collect()
}

/// Extracts the operation from the line
fn extract_operation(line: &str) -> Vec<&str> {
    trim_line(line)
        .split(" ")
        .skip(2)
        .collect()
}

/// Extracts the test from the line
fn extract_test(line: &str) -> Vec<&str> {
    let contents: Vec<&str> = trim_line(line)
        .split(" ")
        .collect();

    Vec::from([contents[0], contents[2]])
}

/// Extracts where the monkey should throw in case of x test result
fn extract_test_result(line: &str) -> usize {
    let contents: Vec<&str> = line.split(" ").collect();

    contents.last().unwrap().parse().unwrap()
}

/// Extracts the file into monkeys
fn extract(file: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let sections: Vec<&str> = file.split("\n\n").collect();

    for section in sections {
        // Skip 1 as we do not need the monkey header
        let lines: Vec<&str> = section.lines().skip(1).collect();
        let items: Vec<usize> = extract_items(lines[0]);
        let operation: Vec<&str> = extract_operation(lines[1]);
        let test: Vec<&str> = extract_test(lines[2]);
        let test_true: usize = extract_test_result(lines[3]);
        let test_false: usize = extract_test_result(lines[4]);

        let monkey = Monkey {
            items,
            items_inspected: 0,
            operation,
            test,
            test_true,
            test_false,
        };

        monkeys.push(monkey)
    }

    monkeys
}
