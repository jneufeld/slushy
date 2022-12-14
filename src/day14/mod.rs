use std::{collections::HashSet, iter::Peekable};

pub fn solve() {
    let input = SAMPLE;
    let cave = Cave::from(input);

    println!("cave: {:?}", cave);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Self {
        Position { x, y }
    }
}

#[derive(Debug)]
struct Cave {
    rocks: HashSet<Position>,

    /// The deepest y-value across all rocks. Below this depth, there are no
    /// rocks.
    oblivion_depth: usize,
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        let mut rocks: HashSet<Position> = HashSet::new();
        let mut lowest_rock = 0;

        for line in input.split('\n') {
            let mut characters = line.chars().peekable();
            let mut previous: Option<Position> = None;

            while let Some(character) = characters.peek() {
                match character {
                    ' ' | '-' | '>' => {
                        let _ignore = characters.next();
                    }
                    _ => {
                        let position = parse_position(&mut characters);

                        if position.y > lowest_rock {
                            lowest_rock = position.y;
                        }

                        rocks.insert(position);

                        if let Some(previous) = previous {
                            let positions =
                                get_line_between(&position, &previous);
                            rocks.extend(positions);
                        }

                        previous = Some(position);
                    }
                }
            }
        }

        Cave {
            rocks,
            oblivion_depth: lowest_rock,
        }
    }
}

fn parse_position<I>(characters: &mut Peekable<I>) -> Position
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

fn get_line_between(start: &Position, end: &Position) -> Vec<Position> {
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

const SAMPLE: &str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
