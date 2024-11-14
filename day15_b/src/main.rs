use std::cmp::{max, min};
use std::collections::HashMap;
use std::fs;

type Position = (isize, isize);

type Range = (isize, isize);
type Ranges = Vec<Range>;
type Occupied = HashMap<isize, Ranges>;
const MIN: isize = 0;
const MAX: isize = 4000000;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let occupied = create_occupied_from_file(&file);

    let position = find_beacon_position(&occupied).unwrap();

    let frequency = get_tuning_frequency(position);

    println!("{}", frequency);
}

/// Returns the tuning frequency corresponding with the position
fn get_tuning_frequency(position: Position) -> isize {
    position.0 * 4000000 + position.1
}

/// Finds the beacons position
fn find_beacon_position(occupied: &Occupied) -> Option<Position> {
    for (row, ranges) in occupied {
        // If the row is fully covered the beacon can't be there
        if ranges[0] == (MIN, MAX) { continue; }

        // Get the y value from the range
        // If the start of the range is not MIN, start - 1 will be y
        // If the end of the range is not MAX, start + 1 will be y
        // This is true as there can be max one uncovered position and 2 ranges
        // The uncovered position has to be at a start/end of the 2 ranges
        let y = match ranges[0].0 == MIN {
            true => ranges[0].1 + 1,
            false => ranges[0].0 - 1,
        };

        return Some((*row, y));
    }

    None
}

/// Returns the taxicab distance between 2 points
fn get_distance(a: Position, b: Position) -> isize {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

/// Adds a range to the row of ranges
/// Merges ranges if any overlaps
fn add_range_to_row(row: isize, range: (isize, isize), mut occupied: Occupied) -> Occupied {
    let ranges = occupied.entry(row).or_default();

    ranges.push(range);

    // Only one range so there will be nothing that overlaps
    if ranges.len() == 1 { return occupied; }

    for i in 0..(ranges.len() - 1) {
        for j in (i + 1)..ranges.len() {
            if !range_overlap(ranges[i], ranges[j]) { continue; }

            // As the ranges overlap we can simply get the new start and end points with min and max
            let start = min(ranges[i].0, ranges[j].0);
            let end = max(ranges[i].1, ranges[j].1);

            // Remove i and j from the ranges
            ranges.remove(i);
            // -1 here as when we remove i (i < j) index j will shift one left
            ranges.remove(j - 1);

            // Rerun with the new range
            return add_range_to_row(row, (start, end), occupied);
        }
    }

    occupied
}

/// Whether to ranges overlap
fn range_overlap(a: Range, b: Range) -> bool {
    a.0 <= b.1 && b.0 <= a.1
}

/// Calculates the positions (in ranges) where it's not possible to place the beacon (e.g. is occupied)
/// Format is HashMap<The row (or x value), vec of (start, end) ranges>
/// Also makes sure that the program does not exceed the bounds of MIN MAX
fn add_occupied_from_line(line: &str, mut occupied: Occupied) -> Occupied {
    let (sensor_str, beacon_str) = line.split_once(": ").unwrap();

    // Extract the sensor and beacon positions
    let sensor: Position = {
        let (x, y) = sensor_str[10..].split_once(", ").unwrap();

        (x[2..].parse().unwrap(), y[2..].parse().unwrap())
    };

    let beacon: Position = {
        let (x, y) = beacon_str[21..].split_once(", ").unwrap();

        (x[2..].parse().unwrap(), y[2..].parse().unwrap())
    };

    // The taxicab range of the sensor
    // This range is the range no other beacon can be in
    let sensor_range = get_distance(sensor, beacon);

    // The range on the x-axis of the sensor
    let x_range = (max(sensor.0 - sensor_range, MIN), min(sensor.0 + sensor_range, MAX));

    for x in x_range.0..=x_range.1 {
        // Calculate the range of the sensor on the current x position
        // Simply the range of the sensor - the distance to the sensor
        let distance = (x - sensor.0).abs();
        let y_sensor_range = sensor_range - distance;

        // The range of the sensor on the y-axis
        let y_range = (max(sensor.1 - y_sensor_range, MIN), min(sensor.1 + y_sensor_range, MAX));

        // Add this new range to the already present ranges in the current row/x
        // Also merge any ranges if they overlap
        occupied = add_range_to_row(x, y_range, occupied);
    }

    occupied
}

/// Looks at all lines (sensor and beacon) and finds all occupied positions in ranges
fn create_occupied_from_file(file: &str) -> Occupied {
    let mut occupied: Occupied = HashMap::new();

    for line in file.lines() {
        occupied = add_occupied_from_line(line, occupied);
    }

    occupied
}


