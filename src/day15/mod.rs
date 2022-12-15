use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

pub fn solve() {
    let input = SAMPLE;
    let report = Report::from(input);
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
    squares: Vec<Square>,
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
        let mut squares: Vec<Square> = Vec::new();

        for line in input.split('\n') {
            let captures = LINE_REGEX.captures(line).unwrap();

            let sensor_x = captures.get(1).unwrap().as_str().parse().unwrap();
            let sensor_y = captures.get(2).unwrap().as_str().parse().unwrap();

            let sensor = Position::new(sensor_x, sensor_y);

            sensors.insert(sensor);

            let beacon_x = captures.get(3).unwrap().as_str().parse().unwrap();
            let beacon_y = captures.get(4).unwrap().as_str().parse().unwrap();

            let beacon = Position::new(beacon_x, beacon_y);

            beacons.insert(beacon);

            let square = Square::new(sensor, beacon);

            squares.push(square);

            print!(
                "sensor: {:?}\nbeacon: {:?}\nsquare: {:?}\n\n",
                sensor, beacon, square
            );
        }

        Report {
            sensors,
            beacons,
            squares,
        }
    }
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
