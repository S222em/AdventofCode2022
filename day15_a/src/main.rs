use std::collections::HashMap;
use std::fs;

enum Contents {
    Beacon,
    Sensor(isize),
    NoBeacon,
}

type Position = (isize, isize);
type Locations = HashMap<Position, Contents>;

const TARGET_Y: isize = 2000000;

fn main() {
    let file = fs::read_to_string("puzzle.txt").unwrap();

    let locations = create_locations_from_file(&file);

    let locations = mark_no_beacon(locations);

    let count = count_no_beacon_on_target(&locations);

    println!("{}", count);
}

/// Counts amount of Contents No Beacon in the locations
fn count_no_beacon_on_target(locations: &Locations) -> usize {
    let mut count = 0;

    for (position, location) in locations.iter() {
        if position.1 != TARGET_Y || !matches!(location, Contents::NoBeacon) { continue; }
        count += 1;
    }

    count
}

/// Marks all the locations a beacon can not be present
fn mark_no_beacon(mut locations: Locations) -> Locations {
    let sensors: Vec<Position> = locations.iter()
        .filter(|(_, content)| matches!(content, Contents::Sensor(_)))
        .map(|(position, _)| *position).collect();

    for sensor in sensors {
        locations = mark_no_beacon_from(locations, sensor);
    }

    locations
}

/// Marks all the locations a beacon can not be present from the sensor
/// These are all positions in range of the sensor (range is determined by distance from sensor to the closest beacon)
fn mark_no_beacon_from(mut locations: Locations, sensor: Position) -> Locations {
    let Contents::Sensor(sensor_range) = *locations.get(&sensor).unwrap() else { unreachable!() };

    let distance = (TARGET_Y - sensor.1).abs();

    if distance > sensor_range { return locations; }

    let x_sensor_range = sensor_range - distance;
    let x_range = (sensor.0 - x_sensor_range)..=(sensor.0 + x_sensor_range);

    for x in x_range {
        if locations.contains_key(&(x, TARGET_Y)) { continue; }
        locations.insert((x, TARGET_Y), Contents::NoBeacon);
    }

    locations
}

/// Returns the taxicab distance between 2 points
fn get_distance(a: Position, b: Position) -> isize {
    (b.0 - a.0).abs() + (b.1 - a.1).abs()
}

/// Adds the points in a line to the locations
fn create_locations_from_line(line: &str, mut locations: Locations) -> Locations {
    let (sensor_str, beacon_str) = line.split_once(": ").unwrap();

    let sensor: Position = {
        let (x, y) = sensor_str[10..].split_once(", ").unwrap();

        (x[2..].parse().unwrap(), y[2..].parse().unwrap())
    };

    let beacon: Position = {
        let (x, y) = beacon_str[21..].split_once(", ").unwrap();

        (x[2..].parse().unwrap(), y[2..].parse().unwrap())
    };

    let distance = get_distance(sensor, beacon);

    locations.insert(sensor, Contents::Sensor(distance));
    locations.insert(beacon, Contents::Beacon);

    locations
}

/// Adds all the beacons and sensors described to the locations
fn create_locations_from_file(file: &str) -> Locations {
    let mut locations: Locations = HashMap::new();

    for line in file.lines() {
        locations = create_locations_from_line(line, locations);
    }

    locations
}


