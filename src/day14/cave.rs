use std::collections::HashSet;

use super::position::{get_line_between, parse_position, Position};

#[derive(Debug)]
pub struct Cave {
    /// Positions of rocks within the cave
    rocks: HashSet<Position>,

    /// The y-value of the cave floor
    floor_level: usize,

    /// Positions of sand at rest within the cave
    resting_sand: HashSet<Position>,
}

impl Cave {
    /// Sand is always dropped from coordinates (500, 0). It must be dropped one
    /// unit at a time per the problem requirements. Later, the problem solves
    /// on how many grains of sand come to rest.
    pub fn drop_sand(&mut self) -> Position {
        // Start the sand at (500, 0) and drop until it comes to rest
        let mut position = Position::new(500, 0);

        loop {
            // Sand falls straight down if possible
            let down = position.down();

            if !self.is_blocked(&down) {
                position = down;
                continue;
            }

            // Else sand falls down and left
            let down_and_left = position.down_and_left();

            if !self.is_blocked(&down_and_left) {
                position = down_and_left;
                continue;
            }

            // Else sand falls down and right
            let down_and_right = position.down_and_right();

            if !self.is_blocked(&down_and_right) {
                position = down_and_right;
                continue;
            }

            self.resting_sand.insert(position);

            break;
        }

        position
    }

    pub fn count_at_rest(&self) -> usize {
        self.resting_sand.len()
    }

    pub fn is_dropping_point(&self, position: &Position) -> bool {
        position.x == 500 && position.y == 0
    }

    /// A position in the cave is blocked if it contains a rock, contains
    /// resting sand, or is on the floor.
    fn is_blocked(&self, position: &Position) -> bool {
        self.floor_level <= position.y
            || self.rocks.contains(position)
            || self.resting_sand.contains(position)
    }
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        let (rocks, lowest) = parse_rocks(input);

        // By problem definition, the floor depth is two levels below the lowest
        // rock. In this coordinate system, that's addition.
        let floor_level = lowest + 2;

        Cave {
            rocks,
            floor_level,
            resting_sand: HashSet::new(),
        }
    }
}

fn parse_rocks(input: &str) -> (HashSet<Position>, usize) {
    let mut rocks: HashSet<Position> = HashSet::new();
    let mut lowest_rock = 0;

    // Deep nesting is a bad sign. For now, documentation will help.
    // Refactoring is needed though.
    for line in input.split('\n') {
        let mut characters = line.chars().peekable();
        let mut previous: Option<Position> = None;

        // Loop on knowledge there is a next character -- but don't consume
        // it yet. The `characters` iterator is passed to parsing functions
        // so it can advance as needed. This is only a control loop.
        while let Some(character) = characters.peek() {
            match character {
                // Consume and skip separators
                ' ' | '-' | '>' => {
                    let _ignore = characters.next();
                }
                // Otherwise parse the position
                _ => {
                    let position = parse_position(&mut characters);

                    if position.y > lowest_rock {
                        lowest_rock = position.y;
                    }

                    rocks.insert(position);

                    // When the input indicates a line between positions it
                    // must be filled in programmatically
                    if let Some(previous) = previous {
                        let positions = get_line_between(&position, &previous);
                        rocks.extend(positions);
                    }

                    previous = Some(position);
                }
            }
        }
    }

    (rocks, lowest_rock)
}
