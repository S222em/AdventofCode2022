use std::fs;

type Grid = Vec<Vec<usize>>;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let grid = extract(&file);

    let visible = find_amount_of_visible_trees(&grid);

    println!("{}", visible);
}

/// Finds the amount of trees that are visible from the edges of the grid
fn find_amount_of_visible_trees(grid: &Grid) -> usize {
    let mut visible: usize = 0;

    for (x, row) in grid.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            if !is_tree_visible(x, y, grid) { continue; }
            visible += 1;
        }
    }

    visible
}

/// Checks whether a single tree is visible
/// Goes in all 4 directions until it finds a tree higher than the target or hits the edge
fn is_tree_visible(x: usize, y: usize, grid: &Grid) -> bool {
    let mut queue: Vec<(isize, isize, isize, isize)> = Vec::new();

    let size = grid[x][y];

    // Add all four directions from the starting point to the queue
    for (vx, vy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        queue.push((x as isize, y as isize, vx, vy))
    }

    while !queue.is_empty() {
        let (qx, qy, vx, vy) = queue.remove(0);

        let nx = qx + vx;
        let ny = qy + vy;

        let nx_out_bounds = nx < 0 || nx.unsigned_abs() >= grid.len();
        let ny_out_bounds = ny < 0 || ny.unsigned_abs() >= grid[0].len();
        if nx_out_bounds || ny_out_bounds { return true; }

        let q_size = grid[nx.unsigned_abs()][ny.unsigned_abs()];

        if q_size >= size { continue; }

        queue.push((nx, ny, vx, vy));
    }

    false
}

/// Extracts the tree sizes from the file and places them in a grid
fn extract(to_extract: &str) -> Grid {
    let mut grid: Grid = Vec::new();

    for line in to_extract.lines() {
        let row: Vec<usize> = line
            .chars()
            .map(|char| char.to_digit(10).unwrap() as usize)
            .collect();

        grid.push(row);
    }


    grid
}
