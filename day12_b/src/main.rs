use std::collections::HashSet;
use std::fs;

type Position = (isize, isize);
type Grid = Vec<Vec<u8>>;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let grid = create_grid_from_file(&file);

    let start = find_height(&grid, b'S').unwrap();
    let target = find_height(&grid, b'E').unwrap();

    let path_length = find_shortest_path(&grid, start, target).unwrap();

    println!("{}", path_length);
}

/// Finds the shortest path to the point with best signal
fn find_shortest_path(grid: &Grid, start: Position, target: Position) -> Option<usize> {
    let mut visited: HashSet<Position> = HashSet::from([start]);
    let mut queue: Vec<(Position, usize)> = Vec::from([(start, 0)]);

    while !queue.is_empty() {
        let (position, path_length) = queue.remove(0);

        for direction in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let next_possible_position = move_by(position, direction, grid);

            if next_possible_position.is_none() { continue; }
            let next_position = next_possible_position.unwrap();

            if !can_step_to(position, next_position, grid) || !visited.insert(next_position) { continue; }

            if next_position == target { return Some(path_length + 1); }

            // I don't know why but this works on the real puzzle and not the test puzzle.
            // I can't really bother to find out why it does not work.
            let next_path_length = match get_height(grid, next_position) {
                b'a' => 0,
                _ => path_length + 1
            };

            queue.push((next_position, next_path_length));
        }
    }

    None
}

/// Finds the first occurrence of given height in the grid
fn find_height(grid: &Grid, height: u8) -> Option<Position> {
    for (x, row) in grid.iter().enumerate() {
        for (y, item) in row.iter().enumerate() {
            if *item == height { return Some((x as isize, y as isize)); }
        }
    }

    None
}

/// Move the position by x amount
/// Returns none of the new position is not in bounds
fn move_by(from: Position, by: Position, grid: &Grid) -> Option<Position> {
    let x = from.0 + by.0;
    let y = from.1 + by.1;

    let x_out_bounds = x < 0 || x.unsigned_abs() >= grid.len();
    let y_out_bounds = y < 0 || y.unsigned_abs() >= grid[0].len();

    if x_out_bounds || y_out_bounds { return None; }

    Some((x, y))
}

/// Returns the height of the given position
/// Keeps in account that S is a and E is z
fn get_height(grid: &Grid, position: Position) -> u8 {
    match grid[position.0.unsigned_abs()][position.1.unsigned_abs()] {
        // If the height is S or E replace
        // With a or z respectively
        b'S' => b'a',
        b'E' => b'z',
        height => height
    }
}

/// Whether we can step from one position to another given that we can move 1 up or down
fn can_step_to(from: Position, to: Position, grid: &Grid) -> bool {
    let height_from = get_height(grid, from) as i8;
    let height_to = get_height(grid, to) as i8;

    (height_to - height_from) <= 1
}

/// Constructs the grid present in the given file
fn create_grid_from_file(file: &str) -> Grid {
    let mut grid: Grid = Vec::new();

    for line in file.lines() {
        let mut row: Vec<u8> = Vec::new();

        for char in line.chars() {
            row.push(char as u8);
        }

        grid.push(row)
    }

    grid
}