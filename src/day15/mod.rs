use crate::day15::coverage::Coverage;

use self::position::Position;

mod coverage;
mod position;
mod zone;

pub fn solve() {
    let input = PUZZLE_INPUT;

    let coverage = Coverage::from(input);

    let min_x = std::cmp::max(coverage.get_min_reach().x, MIN_X);
    let max_x = std::cmp::min(coverage.get_max_reach().x, MAX_X);

    println!("searching for {} <= x <= {}", min_x, max_x);

    let min_y = std::cmp::max(coverage.get_min_reach().y, MIN_Y);
    let max_y = std::cmp::min(coverage.get_max_reach().y, MAX_Y);

    println!("searching for {} <= y <= {}", min_y, max_y);

    let mut distress_signal = None;

    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            let point = Position::new(x, y);

            if coverage.is_occupied(point) || coverage.contains(point) {
                continue;
            }

            distress_signal = Some(point);

            println!("found a solution at {:?}", point);
        }
    }

    let distress_signal = distress_signal.unwrap();

    println!("distress signal position: {:?}", distress_signal);

    let tuning_frequency = distress_signal.x * X_THINGY + distress_signal.y;

    println!("tuning frequency: {}", tuning_frequency);
}

const MIN_X: isize = 0;
const MAX_X: isize = 4_000_000;

const MIN_Y: isize = 0;
const MAX_Y: isize = 4_000_000;

const X_THINGY: isize = 4_000_000;

const SAMPLE_Y: isize = 10;

const SAMPLE_INPUT: &str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
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

const PUZZLE_Y: isize = 2_000_000;

const PUZZLE_INPUT: &str = r"Sensor at x=3772068, y=2853720: closest beacon is at x=4068389, y=2345925
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
