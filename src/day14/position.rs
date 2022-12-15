use std::iter::Peekable;

/// A position is a coordinate given by `x` and `y` integer values. The values
/// start at `0`. `x` grows to right, and `y` grows down (into the cave).
///
/// NB greater y-values indicate greater depth in the cave.
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }

    pub fn down(&self) -> Position {
        let down_one = self.y + 1;

        Position {
            x: self.x,
            y: down_one,
        }
    }

    pub fn down_and_left(&self) -> Position {
        let down_one = self.y + 1;
        let left_one = self.x - 1; // TODO runtime panic

        Position {
            x: left_one,
            y: down_one,
        }
    }

    pub fn down_and_right(&self) -> Position {
        let down_one = self.y + 1;
        let right_one = self.x + 1;

        Position {
            x: right_one,
            y: down_one,
        }
    }
}

pub fn parse_position<I>(characters: &mut Peekable<I>) -> Position
where
    I: Iterator<Item = char>,
{
    let x_value = parse_number(characters);
    let _comma = characters.next();
    let y_value = parse_number(characters);

    Position {
        x: x_value,
        y: y_value,
    }
}

fn parse_number<I>(characters: &mut Peekable<I>) -> usize
where
    I: Iterator<Item = char>,
{
    let mut number = String::new();

    while let Some(character) = characters.next_if(|c| c.is_ascii_digit()) {
        number.push(character);
    }

    number.parse::<usize>().unwrap()
}

pub fn get_line_between(start: &Position, end: &Position) -> Vec<Position> {
    let mut between = Vec::new();

    if start.x != end.x && start.y != end.y {
        panic!("can't draw line unless x or y values are the same");
    }

    if start.x == end.x {
        let x_value = start.x;

        let start_y = std::cmp::min(start.y, end.y) + 1;
        let final_y = std::cmp::max(start.y, end.y);

        for y in start_y..final_y {
            between.push(Position::new(x_value, y));
        }
    } else {
        let y_value = start.y;

        let start_x = std::cmp::min(start.x, end.x) + 1;
        let final_x = std::cmp::max(start.x, end.x);

        for x in start_x..final_x {
            between.push(Position::new(x, y_value));
        }
    }

    between
}
