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

/// Finds the exact amount of positions the last tail will visit while moving along with the head.
fn find_amount_of_tail_positions(motions: &Motions) -> usize {
    let mut head: Position = START_POSITION;
    let mut tails: Vec<Position> = (0..9).map(|_| START_POSITION).collect();
    let mut tail_history: TailHistory = HashSet::from([START_POSITION]);

    for motion in motions.iter() {
        (head, tails, tail_history) = do_motion(motion, head, tails, tail_history);
    }

    tail_history.len()
}

// Moves the head and tails according to the described motion.
fn do_motion(motion: &Motion, mut head: Position, mut tails: Vec<Position>, mut tail_history: TailHistory) -> (Position, Vec<Position>, TailHistory) {
    for _ in 0..motion.1 {
        head.0 += motion.0.0;
        head.1 += motion.0.1;

        for i in 0..tails.len() {
            tails[i] = find_tail_position(if i == 0 { head } else { tails[i - 1] }, tails[i]);

            if i == tails.len() - 1 { tail_history.insert(tails[i]); }
        }
    }

    (head, tails, tail_history)
}

/// Finds the position of the tail so it is maximum 1 square away from the head.
fn find_tail_position(head: Position, mut tail: Position) -> Position {
    let diff_x = head.0 - tail.0;
    let diff_y = head.1 - tail.1;
    if diff_x.unsigned_abs() <= 1 && diff_y.unsigned_abs() <= 1 { return tail; }

    tail.0 += diff_x.clamp(-1, 1);
    tail.1 += diff_y.clamp(-1, 1);

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
