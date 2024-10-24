use std::collections::HashSet;
use std::fs;
type Position = (isize, isize);
type Direction = Position;
type Motion = (Direction, isize);
type Motions = Vec<Motion>;
type TailHistory = HashSet<Position>;

const START_POSITION: Position = (0, 0);

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let motions = extract(&file);

    let tail_positions = find_amount_of_tail_positions(&motions);

    println!("{}", tail_positions)
}

/// Finds the exact amount of positions the tail will visit while moving along with the head.
fn find_amount_of_tail_positions(motions: &Motions) -> usize {
    let mut head: Position = START_POSITION;
    let mut tail: Position = START_POSITION;
    let mut tail_history: TailHistory = HashSet::from([START_POSITION]);

    for motion in motions.iter() {
        (head, tail, tail_history) = do_motion(motion, head, tail, tail_history);
    }

    tail_history.len()
}

// Moves the head and tail according to the described motion.
fn do_motion(motion: &Motion, mut head: Position, mut tail: Position, mut tail_history: TailHistory) -> (Position, Position, TailHistory) {
    for _ in 0..motion.1 {
        head.0 += motion.0.0;
        head.1 += motion.0.1;
        tail = find_tail_position(motion, head, tail);
        tail_history.insert(tail);
    }

    (head, tail, tail_history)
}

/// Finds the position of the tail so it is maximum 1 square away from the head.
fn find_tail_position(motion: &Motion, head: Position, mut tail: Position) -> Position {
    let distance = ((head.0 - tail.0).unsigned_abs(), (head.1 - tail.1).unsigned_abs());

    if distance.0 <= 1 && distance.1 <= 1 { return tail; }

    tail.0 += motion.0.0;
    tail.1 += motion.0.1;

    // Put the tail straight next to the head
    // This works because the head only moves in 1 axis at a time
    // The axis the head does not move in is where the tail should be moved
    match motion.0 {
        (0, _) => { tail.0 = head.0; }
        (_, 0) => { tail.1 = head.1; }
        _ => {}
    }

    tail
}

/// Extracts the file into a vec of directions and amount of steps in that direction.
fn extract(to_extract: &str) -> Motions {
    let mut instructions: Motions = Vec::new();

    for line in to_extract.lines() {
        let (direction_type, steps) = line.split_once(" ").unwrap();

        let direction: Direction = match direction_type {
            "R" => (1, 0),
            "L" => (-1, 0),
            "U" => (0, 1),
            "D" => (0, -1),
            _ => (0, 0)
        };

        let steps: isize = steps.parse().unwrap();

        instructions.push((direction, steps))
    }

    instructions
}
