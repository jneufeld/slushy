use std::collections::HashSet;

use super::position::{get_line_between, parse_position, Position};

#[derive(Debug)]
pub struct Cave {
    /// Positions of rocks within the cave
    rocks: HashSet<Position>,

    /// The deepest y-value across all rocks. Below this depth, there are no
    /// rocks.
    oblivion_depth: usize,

    // Positions of sand at rest within the cave
    resting_sand: HashSet<Position>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SandPlacement {
    Resting(Position),
    FreeFall,
}

impl Cave {
    /// Sand is always dropped from coordinates (500, 0). It must be dropped one
    /// unit at a time per the problem requirements. Later, the problem solves
    /// on how many grains of sand come to rest. No point in optimizing this by
    /// dropping multiple grains of sand before necessary.
    pub fn drop_sand(&mut self) -> SandPlacement {
        // Start the sand at (500, 0) and drop until it comes to rest
        let mut position = Position::new(500, 0);

        loop {
            // If the sand couldn't fall then it has come to a rest. Record its
            // final position then break to return the result.
            if position.y > self.oblivion_depth {
                return SandPlacement::FreeFall;
            }

            // Sand falls straight down if possible
            if !self.is_blocked(&position.down()) {
                position = position.down();
                continue;
            }

            // Else sand falls down and left
            if !self.is_blocked(&position.down_and_left()) {
                position = position.down_and_left();
                continue;
            }

            // Else sand falls down AND right
            if !self.is_blocked(&position.down_and_right()) {
                position = position.down_and_right();
                continue;
            }

            // Sand cannot move and is not falling into the darkness below. It
            // has come to a rest.
            self.resting_sand.insert(position);
            return SandPlacement::Resting(position);
        }
    }

    pub fn count_at_rest(&self) -> usize {
        self.resting_sand.len()
    }

    /// A position in the cave is blocked if it contains a rock or resting sand
    fn is_blocked(&self, position: &Position) -> bool {
        self.rocks.contains(position) || self.resting_sand.contains(position)
    }
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        let (rocks, lowest) = parse_rocks(input);

        Cave {
            rocks,
            oblivion_depth: lowest,
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
