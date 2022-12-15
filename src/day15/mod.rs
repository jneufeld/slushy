use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

pub fn solve() {
    let input = SAMPLE;
    let report = Report::from(input);

    let magic_y = 10;

    let count = report
        .scanned
        .iter()
        .filter(|position| position.y == magic_y)
        .count();

    println!("count at y={}: {}", magic_y, count);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Self {
        Position { x, y }
    }

    fn get_scanned(&self, beacon: Position) -> HashSet<Position> {
        let mut scanned = HashSet::new();

        let distance = self.get_manhattan_distance(beacon);

        println!(
            "distance between {:?} and {:?} is {}",
            self, beacon, distance
        );

        for x in 0..distance + 1 {
            let left_x = self.x - x as isize;
            let right_x = self.x + x as isize;

            for y in 0..distance + 1 - x {
                let up_y = self.y + y as isize;
                let down_y = self.y - y as isize;

                //println!(
                //    "\tadding ({}, {}), ({}, {}), ({}, {}), and ({}, {})",
                //    left_x,
                //    up_y,
                //    left_x,
                //    down_y,
                //    right_x,
                //    up_y,
                //    right_x,
                //    down_y
                //);

                scanned.insert(Position::new(left_x, up_y));
                scanned.insert(Position::new(left_x, down_y));

                scanned.insert(Position::new(right_x, up_y));
                scanned.insert(Position::new(right_x, down_y));
            }
        }

        scanned
    }

    fn get_manhattan_distance(&self, other: Position) -> usize {
        let x_distance = self.x.abs_diff(other.x);
        let y_distance = self.y.abs_diff(other.y);

        x_distance + y_distance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Square {
    top_left: Position,
    bottom_right: Position,
}

impl Square {
    fn new(sensor: Position, beacon: Position) -> Self {
        let distance = sensor.get_manhattan_distance(beacon) as isize;

        let top_left = Position::new(sensor.x + distance, sensor.y + distance);
        let bottom_right =
            Position::new(sensor.x - distance, sensor.y - distance);

        Square {
            top_left,
            bottom_right,
        }
    }
}

struct Report {
    sensors: HashSet<Position>,
    beacons: HashSet<Position>,
    scanned: HashSet<Position>,
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
    )
    .unwrap();
}

impl From<&str> for Report {
    fn from(input: &str) -> Self {
        let mut sensors = HashSet::new();
        let mut beacons = HashSet::new();
        let mut scanned = HashSet::new();

        for line in input.split('\n') {
            let (sensor, beacon) = parse_positions(line);

            sensors.insert(sensor);
            beacons.insert(beacon);

            scanned.extend(sensor.get_scanned(beacon));

            print!("sensor: {:?}\nbeacon: {:?}\n\n", sensor, beacon);
        }

        Report {
            sensors,
            beacons,
            scanned,
        }
    }
}

fn parse_positions(line: &str) -> (Position, Position) {
    let captures = LINE_REGEX.captures(line).unwrap();

    let sensor_x = captures.get(1).unwrap().as_str().parse().unwrap();
    let sensor_y = captures.get(2).unwrap().as_str().parse().unwrap();

    let sensor = Position::new(sensor_x, sensor_y);

    let beacon_x = captures.get(3).unwrap().as_str().parse().unwrap();
    let beacon_y = captures.get(4).unwrap().as_str().parse().unwrap();

    let beacon = Position::new(beacon_x, beacon_y);

    (sensor, beacon)
}

const SAMPLE: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

const PUZZLE: &str = r"Sensor at x=3772068, y=2853720: closest beacon is at x=4068389, y=2345925
Sensor at x=78607, y=2544104: closest beacon is at x=-152196, y=4183739
Sensor at x=3239531, y=3939220: closest beacon is at x=3568548, y=4206192
Sensor at x=339124, y=989831: closest beacon is at x=570292, y=1048239
Sensor at x=3957534, y=2132743: closest beacon is at x=3897332, y=2000000
Sensor at x=1882965, y=3426126: closest beacon is at x=2580484, y=3654136
Sensor at x=1159443, y=3861139: closest beacon is at x=2580484, y=3654136
Sensor at x=2433461, y=287013: closest beacon is at x=2088099, y=-190228
Sensor at x=3004122, y=3483833: closest beacon is at x=2580484, y=3654136
Sensor at x=3571821, y=799602: closest beacon is at x=3897332, y=2000000
Sensor at x=2376562, y=1539540: closest beacon is at x=2700909, y=2519581
Sensor at x=785113, y=1273008: closest beacon is at x=570292, y=1048239
Sensor at x=1990787, y=38164: closest beacon is at x=2088099, y=-190228
Sensor at x=3993778, y=3482849: closest beacon is at x=4247709, y=3561264
Sensor at x=3821391, y=3986080: closest beacon is at x=3568548, y=4206192
Sensor at x=2703294, y=3999015: closest beacon is at x=2580484, y=3654136
Sensor at x=1448314, y=2210094: closest beacon is at x=2700909, y=2519581
Sensor at x=3351224, y=2364892: closest beacon is at x=4068389, y=2345925
Sensor at x=196419, y=3491556: closest beacon is at x=-152196, y=4183739
Sensor at x=175004, y=138614: closest beacon is at x=570292, y=1048239
Sensor at x=1618460, y=806488: closest beacon is at x=570292, y=1048239
Sensor at x=3974730, y=1940193: closest beacon is at x=3897332, y=2000000
Sensor at x=2995314, y=2961775: closest beacon is at x=2700909, y=2519581
Sensor at x=105378, y=1513086: closest beacon is at x=570292, y=1048239
Sensor at x=3576958, y=3665667: closest beacon is at x=3568548, y=4206192
Sensor at x=2712265, y=2155055: closest beacon is at x=2700909, y=2519581";
