use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

use super::{position::Position, zone::Zone};

/// Coverage, conceptually, describes borders. Think of fences drawn on a map.
/// Every sensor and beacon provided by the input can geometrically describe
/// a square fence somewhere on the map.
///
/// This data structure provides an API to ask questions about points on the
/// map: `is_occupied()` and `contains()`. The idea of fences on a map is useful
/// at this level, but as we go deeper we'll need to provide a more mathematical
/// descripition.
pub struct Coverage {
    /// Stores the locations of sensors in this `Coverage`
    sensors: HashSet<Position>,

    /// Stores the locations of beacons in this `Coverage`
    beacons: HashSet<Position>,

    /// Coverage zones describe the points they contain according to the sensor
    /// and beacon input
    zones: Vec<Zone>,

    /// The minimum reach defines the smallest x and y values covered. No point
    /// outside these bounds is contained by this report.
    min_reach: Position,

    /// The maximum reach defines the largest x and y values covered. No point
    /// outside these bounds is contained by this report.
    max_reach: Position,
}

impl Coverage {
    /// A stateful property: `true` when a sensor or beacon is at the given
    /// position
    pub fn is_occupied(&self, point: Position) -> bool {
        self.sensors.contains(&point) || self.beacons.contains(&point)
    }

    /// A geometric property: `true` when the given point is within some `Zone`.
    /// In short, a zone can be thought of as a square in Cartesian coordinates.
    /// This basically checks if a point is inside any of the squares in this
    /// `Coverage`.
    ///
    /// The documentation for `Zone` and `Position` elaborates on these these
    /// geometric ideas.
    pub fn contains(&self, point: Position) -> bool {
        for zone in self.zones.iter() {
            if zone.contains(point) {
                return true;
            }
        }

        false
    }

    pub fn get_min_reach(&self) -> Position {
        self.min_reach
    }

    pub fn get_max_reach(&self) -> Position {
        self.max_reach
    }
}

impl From<&str> for Coverage {
    fn from(input: &str) -> Self {
        let mut sensors = HashSet::new();
        let mut beacons = HashSet::new();
        let mut zones = Vec::new();

        // NB the initial values are effectively negative infinity (e.g. the
        // minimum reach's negative infinity is a maximum value). These values
        // get overwritten while parsing. Oddly, the `min_reach` position will
        // contain the left-bottom-most point, i.e. the third quadrant.
        let mut min_reach = Position::new(isize::MAX, isize::MAX);
        let mut max_reach = Position::new(isize::MIN, isize::MIN);

        for line in input.split('\n') {
            let (sensor, beacon) = parse_positions(line);

            sensors.insert(sensor);
            beacons.insert(beacon);

            let zone = Zone::new(sensor, beacon);
            zones.push(zone);

            if zone.get_up_reach() > max_reach.y {
                max_reach.y = zone.get_up_reach();
            }

            if zone.get_down_reach() < min_reach.y {
                min_reach.y = zone.get_down_reach();
            }

            if zone.get_right_reach() > max_reach.x {
                max_reach.x = zone.get_right_reach();
            }

            if zone.get_left_reach() < min_reach.x {
                min_reach.x = zone.get_left_reach();
            }
        }

        Coverage {
            sensors,
            beacons,
            zones,
            min_reach,
            max_reach,
        }
    }
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(
        r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
    )
    .unwrap();
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
