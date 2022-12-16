use super::position::Position;

/// A zone is the space covered by a sensor and beacon. Internally, this space
/// is represented by a triange (and therefore a square). This data structure
/// provides a fast interface for determining if it contains a point via
/// `contains`.
///
/// A sensor and beacon determine a square. The sensor's point and its Manhattan
/// distance to the beacon describe triangles. Any of four trianges can be
/// generated from the point and distance. Rotating any triange to another
/// quadrant describes the square in that quadrant.
#[derive(Debug, Clone, Copy)]
pub struct Zone {
    point: Position,
    distance: usize,
}

impl Zone {
    /// Create a zone of coverage determined by the positions of a sensor and
    /// beacon
    pub fn new(sensor: Position, beacon: Position) -> Self {
        let distance = sensor.get_manhattan_distance(beacon);

        Zone {
            point: sensor,
            distance,
        }
    }

    /// The zone contains a point if that point is inside the square of this
    /// zone's coverage. When the point is outside the borders of this zone,
    /// it is not contained.
    pub fn contains(&self, point: Position) -> bool {
        // If the point's x and y values are outside this zone then it can't
        // be contained. This is trivially false.
        if !self.is_within_x(point) || !self.is_within_y(point) {
            return false;
        }

        // When the point can possibly be contained, check its distance. This
        // might not be obvious at first -- it wasn't for me. But thinking about
        // how a square is described by a triangle (rotations) and the
        // definition of Manhattan Distance led me here.
        let distance = self.point.get_manhattan_distance(point);

        self.distance >= distance
    }

    pub fn get_up_reach(&self) -> isize {
        self.point.y + self.distance as isize
    }

    pub fn get_down_reach(&self) -> isize {
        self.point.y - self.distance as isize
    }

    pub fn get_right_reach(&self) -> isize {
        self.point.x + self.distance as isize
    }

    pub fn get_left_reach(&self) -> isize {
        self.point.x - self.distance as isize
    }

    fn is_within_x(&self, other: Position) -> bool {
        let right_reach = self.get_right_reach();
        let left_reach = self.get_left_reach();

        other.x <= right_reach && other.x >= left_reach
    }

    fn is_within_y(&self, other: Position) -> bool {
        let up_reach = self.get_up_reach();
        let down_reach = self.get_down_reach();

        other.y <= up_reach && other.y >= down_reach
    }
}
