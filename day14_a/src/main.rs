use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;

type Position = (isize, isize);

enum Material {
    Air,
    Rock,
    Sand,
}

type Locations = HashMap<Position, Material>;

const SAND_START_POSITION: Position = (500, 0);

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let locations = create_pre_defined_locations(&file);

    let sand_poured = pour_sand_until_not_settled(locations);

    println!("{}", sand_poured);
}

/// Keeps pouring sand until a piece of sand does not settle and falls into the abyss
/// Returns the amount of sand that settled
fn pour_sand_until_not_settled(mut locations: Locations) -> usize {
    let lowest_y = get_lowest_y(&locations);

    let mut sand_poured: usize = 0;

    loop {
        let (l, settled) = pour_sand(locations, lowest_y);

        locations = l;

        if !settled { break; }

        sand_poured += 1;
    }

    sand_poured
}

/// Pours a single piece of sand from the start position
/// Returns whether the sand has settled
fn pour_sand(mut locations: Locations, lowest_y: isize) -> (Locations, bool) {
    let mut position = SAND_START_POSITION;

    loop {
        position.1 += 1;

        // The current position is already occupied
        // See if we can move the sand left or right
        // If not the sand has settled
        if locations.contains_key(&position) {
            if !locations.contains_key(&(position.0 - 1, position.1)) {
                position.0 -= 1;
            } else if !locations.contains_key(&(position.0 + 1, position.1)) {
                position.0 += 1;
            } else {
                // As the sand has settled the previous position is now occupied
                locations.insert((position.0, position.1 - 1), Material::Sand);
                break (locations, true);
            }
        }

        if position.1 > lowest_y { break (locations, false); }
    }
}

/// Finds the lowest point in locations
/// If the sand gets below this point we should stop the program
fn get_lowest_y(locations: &Locations) -> isize {
    locations.keys().max_by_key(|position| position.1).unwrap().1
}

/// Creates a position from a given "x,y" str
fn create_position_from_str(str: &str) -> Position {
    let (x, y) = str.split_once(",").unwrap();

    (x.parse().unwrap(), y.parse().unwrap())
}

/// Add all locations in a path to the hashmap
/// Generates all x,y between 2 given points in the path and fills them all with rock
fn create_locations_from_path(path: &str, mut locations: Locations) -> Locations {
    let positions: Vec<_> = path.split(" -> ").collect();

    for window in positions.windows(2) {
        let p1 = create_position_from_str(window[0]);
        let p2 = create_position_from_str(window[1]);

        // Get the start and end of the range between the 2 points
        // We need to fill all the points between the 2 with rock
        let start_x = min(p1.0, p2.0);
        let end_x = max(p1.0, p2.0);
        let start_y = min(p1.1, p2.1);
        let end_y = max(p1.1, p2.1);

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                locations.insert((x, y), Material::Rock);
            }
        }
    }

    locations
}

/// Add all pre-defined locations in the puzzle to the hashmap
fn create_pre_defined_locations(file: &str) -> Locations {
    let mut locations: Locations = HashMap::new();

    for path in file.lines() {
        locations = create_locations_from_path(path, locations);
    }

    locations
}
