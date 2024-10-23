use std::fs;

type Grid = Vec<Vec<usize>>;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let grid = extract(&file);

    let max_score = find_max_score_of_trees(&grid);

    println!("{}", max_score);
}

fn find_max_score_of_trees(grid: &Grid) -> usize {
    let mut max: usize = 0;

    for (x, row) in grid.iter().enumerate() {
        for (y, _) in row.iter().enumerate() {
            let score = find_viewing_score(x, y, grid);
            if score > max { max = score; }
        }
    }

    max
}

// Finds all the trees visible in each direction from another tree, returning the results multiplied.
// Only goes in 4 directions.
fn find_viewing_score(x: usize, y: usize, grid: &Grid) -> usize {
    let mut queue: Vec<(isize, isize, isize, isize)> = Vec::new();

    let size = grid[x][y];

    // Add all four directions from the starting point to the queue
    for (vx, vy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        queue.push((x as isize, y as isize, vx, vy))
    }

    let mut distances = Vec::new();

    while !queue.is_empty() {
        let (qx, qy, vx, vy) = queue.remove(0);

        let nx = qx + vx;
        let ny = qy + vy;

        let nx_out_bounds = nx < 0 || nx.unsigned_abs() >= grid.len();
        let ny_out_bounds = ny < 0 || ny.unsigned_abs() >= grid[0].len();

        if nx_out_bounds || ny_out_bounds {
            // Calculate the distance from the original tree, as we went out of bounds the last visible tree was one step back.
            let distance = x as isize - qx + y as isize - qy;
            distances.push(distance.unsigned_abs());
            continue;
        }

        if grid[nx.unsigned_abs()][ny.unsigned_abs()] >= size {
            // Calculate the distance from the original tree, this tree is larger than the original, and it's the last one we can see.
            let distance = x as isize - nx + y as isize - ny;
            distances.push(distance.unsigned_abs());
            continue;
        }

        queue.push((nx, ny, vx, vy));
    }

    distances.iter().product()
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
