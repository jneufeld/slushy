use std::collections::HashSet;

use crate::day12::dijkstra::shortest_path;

use self::{height::Height, position::Position};

mod dijkstra;
mod height;
mod position;

pub fn solve() {
    let input = PUZZLE;

    let (map, start, goal) = parse_map(input);
    let path = shortest_path(start, goal, &map);

    // Subtract one because the first position in the path is the starting
    // position. That doesn't count as a step.
    match path {
        Some(path) => println!("steps: {}", path.len() - 1),
        None => println!("no path found"),
    }
}

/// Returns the set of squares parsed, the starting position, and the goal
/// position
fn parse_map(input: &str) -> (HashSet<Position>, Position, Position) {
    let mut map = HashSet::new();

    let mut start = None;
    let mut goal = None;

    for (row_num, line) in input.split('\n').enumerate() {
        for (col_num, character) in line.chars().enumerate() {
            let height = Height::from(character);
            let position = Position::new(row_num as isize, col_num as isize, height);

            if start.is_none() && Position::is_start(character) {
                start = Some(position);
            }

            if goal.is_none() && Position::is_goal(character) {
                goal = Some(position);
            }

            map.insert(position);
        }
    }

    let start = start.unwrap();
    let goal = goal.unwrap();

    (map, start, goal)
}

const SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

const PUZZLE: &str = "abccccccccccccccccccccccccccaaaaaaaaacccccccccccaaacccccccccccccccccccccccccaaaaaaaaccccccccaaaaaaccaaccccccccccccccccccccccccaaaaacaacaaaacccccccccccccccccccccccccccccccccccccaaaaa
abccaaacccccccccccccccccccccaaaaaaaaacccccccccaaaaaacccccccccccccccccccccaaaaaaaaaaaccccccccaaaaaaccaaaaaacccaacaaccccccccccccaaaaaaaacaaaaaaccccccccccccccccccccccccccccccccccaaaaaa
abccaaaaccccccccccccccccccccaaaaaaaaccccccccccaaaaaaccccaaaaaccccccccccccaaaaaaaaaaacccccccccaaaaaccaaaaaccccaaaacccccccccccccccaaaaacccaaaaaccccccccccccccccccccaaccccccccccccaaaaaa
abccaaaacccccccaaaccccccccccaaaaaaacccccccccccaaaaaaccccaaaaaccccccccccccacaaaaaaaaaacccccccaaaaacaaaaaaacccaaaaaccccccccccccccaaaaacccaaaaaccccccccccccccccccccaaaaccccccccccccccaaa
abccaaaccccccaaaaaacccccccccaccaaaccccccccccccaaaaacccccaaaaaaccccaaaacccccaaaaaaaaaaaccccccaaaaacaaaaaaacccaaaaaacccccccccccccaacaaaccaccaacccccccccccccccaaaccaaaaccccccccccccccaaa
abcccccccccccaaaaaacccccccccccccaaacccccccccccaaaaacccccaaaaaaccccaaaaccccccaaaaacaaaaccccccccccccccaaaaaaccacaaaaccccaaaaacccccccaaccccccccccccccccccccccaaaackkkaccccccccccccccccaa
abcccccccccccaaaaaacccccccccccccccaaacccccccccccccccccccaaaaaaccccaaaacccccaaaaaccccaaccccccccccccccaaccaaccccaaccccccaaaaaccccccccccccccccccccccccccccccccaakkkkkkkccccccccccccccccc
abaccccccccccaaaaaccccccccccccccccaaaaccccccccccccccccccccaaaccccccaaccccccccaaaccccccccccccccccccccaacccccccccccccccaaaaaacccccccccccccccccccccaaacccccccccjkkkkkkkkccccccccaacccccc
abaccccccccccaaaaacccccaacccccccccaaaaccccccccccccaacccccccccccccccccccccccccaaacccccccccccccccccccccaaccccccaccaccccaaaaaaaaaccaccccccccccccccaaaaccccccccjjkkoopkkkkaccaacaaacccccc
abaccccccccccccccccaaaaaacccccccccaaacccccccccccccaaaaaacccccccccccccccccccccccccccccccccccccccccccccaaaaaaccaaaaccccaaaaaaaaacaaccccccccccccccaaaacccccccjjjkoooppkkkaccaaaaaaaacccc
abcccccccccccccccccaaaaaaaaccccccccccccccccaccccccaaaaaaccccccccccccccccccccccccaaccaacccccccccccccccaaaaaacaaaaacccccaaacccaaaaacccccccccccccccaaacccccjjjjjoooppppkklccaaaaaaaacccc
abcccccccccccccccccaaaaaaaacccccccccccccccaaacccaaaaaaaccccaacccacccccccccccccccaaaaaaccccccccaaaccaaaaaaaccaaaaaaccccccccaaaaaacccccccccccccccccccccjjjjjjjoooouuppplllccaccaaaacccc
abccccccccccccccccccaaaaaaaccccaacccccaaacaaacccaaaaaaaccccaaacaacccccccccccccccaaaaaacccccccccaaaaaaaaaaaacaaaaaaccccccccaaaaaaaaccccccccccccccccciijjjjjjooouuuupppllllcccccccccccc
abccccccccccccccccccaaaaaccccccaacccccaaaaaaaaaaccaaaaaaccccaaaaaccccccccccccccaaaaaaacccccccaaaaaaaaaaaaaacccaaccccccccccaacaaaaacccccccccccccccciiiijoooooouuuuuuppplllllcccccccccc
abcccccccccccccccccaaaaaacccaacaaaaacccaaaaaaaaaccaaccaaccaaaaaacccccccccccccccaaaaaaaaccccccaaaaacccaaaaaacccaccccccccccccccaacccccccccccccccccciiiinnoooooouuxuuuupppplllllcccccccc
abcccccccccccccccccccccaacccaaaaaaaaccccaaaaaaccccaaccccccaaaaaaaaccaaaccccccccaaaaaaaacccccccaaaaaccaacaaaaaaaacccaaccccccccacccccccccaaaccccccciiinnnnntttuuuxxuuuppppqqllllccccccc
abccccccccccccaacccccccccccccaaaaaccccccaaaaaacccccaaaccccaaaaaaaaccaaaacccccccccaaaccccccccccaaccaccccccaaaaaacccaaaaaaccccccccccccccaaaaccccaaiiinnnntttttuuxxxxuuvpqqqqqllmmcccccc
abccccccccccaaaaaaccccccccccccaaaaaccccaaaaaaacccccaaacccccccaacccccaaaaccccaaccccaacccccccccccccccccccccaaaaaaccccaaaaaccccccccccccccaaaaccccaaiiinnnttttxxxxxxxyuvvvvvqqqqmmmcccccc
abccaaacccccaaaaaacccccccccccaaacaaccccaaacaaacccccaaaaaaacccaccccccaaaccccaaaacccccccccccccccccccccccccaaaaaaaacaaaaaaacccccccccaaacccaaaccccaaaiinnntttxxxxxxxxyyyyvvvvqqqmmmcccccc
abcaaaacccccaaaaaccccccccccccaaaccaccccccccccacccaaaaaaaaaaccccccccccccccccaaaaccccccccccccccccccccccccaaaaaaaaaaaaaaaaaaccccccccaaaaaacccccaaaaaiiinnnttxxxxxxxyyyyyyvvvqqqmmmcccccc
SbcaaaaccccccaaaaacccccaaaccccccaaacccccccccaaccaaaaaaaaaaaacccccccccccccccaaaaccccccccccccccccccccccccaaaaaaaaaaaaaaaaaacccccccaaaaaaacccccaaaaaiiinnntttxxxxEzzyyyyvvvqqqmmmdddcccc
abccaaaccccccaaaaacccccaaaaccccaaaaaaccccccaaaaccaaaaaaacaaacccccaaccccccccccccccccccccccccccccccccccccacaaaaacccccaaacacccccccaaaaaaaccccccaaaaaahhhnnntttxxxyyyyyyvvvvqqmmmmdddcccc
abcccccccccccccccccccccaaaaccccaaaaaaccccccaaaaccccaaaaaaaaaaaaaaaacacccccccccccccccccccccccccccccccccccccaaaacccccaaccccccccccaaaaaaaccccccccaaaahhhnnnnttxxxyyyyyvvvqqqqmmmdddccccc
abcccccccccccccccaacaacaaacccccaaaaaaccccccaaaacccaaaaaaaaaaaaaaaaaaacccccccccccccccccaacaaccccccccccccccccaaccccccccccccccccccccaaaaaacccccccaaccchhhmmmttxwyyyyyyvvrqqqmmmddddccccc
abcccccccccccccccaaaacccccccccccaaaaacccccccccccaaaaaaaaaaaaaaccaaaaccccaacccccaacccccaaaaaccccccccccccccccccccccccccccccccaaaaccaaaaaacccccccaaccahhhmmssswwywwwyyyvvrqmmmmdddcccccc
abccccccccccccccaaaaacccccccccccaacaacccccccccccaaaaaacaaaaaacccaaaaaaacaaccccaaaccccccaaaaacccccccacccccccccccccccccccccccaaaaccaacccccccccccaaaaahhhmmsswwwwwwwwywwvrrnnmdddccccccc
abccccccccccccccaaaaaaccaaccccccccccccccccccccccaaaaaaaaaaaaacccacaacaaaaaccccaaacaaacaaaaaacaaacaaacccccccacccaaccccaaccccaaaacccccccccaaaccccaaaahhhmmssswwwwswwwwwwrrnnndddccccccc
abaaccccccccccccacaaaacaaaaaaaccccccccccaacccccccaaaaaaaacaaaccccccccaaaaaaaaaaaaaaaacaaaacccaaaaaaacccccccaacaaaaaaaaacccccaaccccccccccaaacccaaaaahhhmmsssswsssrrwwwrrrnneddaccccccc
abaaccccccccaaccccaaccccaaaaacccccccccccaacccccccaaaacccccccacccccccaaaaaaaaaaaaaaaaacccaaccccaaaaaacccccccaaaaacaaaaaaacccccccccccccaaaaaaaaaaaaaahhhmmssssssssrrrrrrrrnneedaaaacccc
abacccccccccaaaaccccccaaaaaaaccccccccaaaaaaaacccccccccccccccccccccccaaaaaaaacaaaaaacccaaacccccaaaaaaaacccccaaaaaacaaaaaaaccccccccccccaaaaaaaaaaaaaahhhmmmsssssllrrrrrrrnnneeeaaaacccc
abaaacccccaaaaaaccccccaaaaaaaacccaaccaaaaaaaaccccaaaaaccccccccccccccaaaaaacccaaaaaaccccaaaccaaaaaaaaaaccccaaaaaaaaaaaaaaaccccccccccccccaaaaaccccaachhgmmmmmlllllllrrrrrnnneeeaaaacccc
abaaacccccaaaaaccccaccaaaaaaaacaaaaaaccaaaaccccccaaaaacccccccccccccccccaaacccaaaaaaaccaaaaaaaaaaaaaaaaccccaaaaaaaaaaaaacccccccccccccccaaaaaaccccaaccgggmmmllllllllllnnnnneeeaaaaccccc
abcccccccccaaaaacccaaacaaaacaacaaaaaacaaaaaccccccaaaaaaccccccccccccccccccccccaaacaaacccaaaaaaaacaaaccccccccccaaccaaaaaacccccccccccccccaaaaaacaacccccgggggmlllfffflllnnnnneeeaaaaccccc
abcccccccccaaccacccaaaaaaacccccaaaaaacaacaaacccccaaaaaaccccccccccccccccccccccaccaaccaaaaaaaaacccaaaccccccccccaaccccccaacccccaaaaccccccaccaaaaaaccccccggggggggfffffflnnneeeeeacaaacccc
abaaaccccccccccccccaaaaaacccccccaaaaacacccaaaacccaaaaaacccccccccccccccccccaaacccaaccaaaaaaaaacccaaccccccccccccccccccccccccccaaaaccccaaaccaaaaaacccccccgggggggfffffffffeeeeeaacccccccc
abaaaaacccccccccccaaaaaaaaccccccaaaacccccccaaacccccaacccccccaaacccccccccccaaaacaaacccaaaaaaaacccccccccccccccccccccccccccccccaaaaccccaaacccaaaaaaacccccccccgccaaaafffffeeeeccccccccccc
abaaaaaccccccaaccaaaaaaaaaacccccaaacaaaaaacaaaccccccccccccaaaaaccccccccccaaaaacaaacccccaaaaaaacccccccccccccccccccccccccccccccaacccccaaaaaaaaaaaaaccccccccccccaaaacaafffecccccccccccaa
abaaaacccaaacaaccaaaaaaaaaacccccaaaaaaaaaaaaaaaaacccccccccaaaaaaccccccccccaaaaaaaaaaacaaacccacccaacccccccccaaaaccccaaacccccccccccaaaaaaaaaaaaaaacccccccccccccaaaccccaaccccccccccccaaa
abaaaaacccaaaaacccccaaacaaacccccaaaaaaccaaaaaaaaacccccccccaaaaaaccaaccacccaaaaaaaaaaaaaaaccccccaaacccccccccaaaaccccaaaaccccccccccaaaaaaaaaaaaaaccccccccccccccaaaccccccccccccccccccaaa
abaaaaacccaaaaaaacccaaaccccccccaaaaaaaaccaaaaaaaccccccccccaaaaacccaaaaaccccaaaaaaaaaaaaacccccaaaaaaaaccccccaaaaccccaaaacccccccccccaaaaaaacccaaaccccccccccccccaaacccccccccccccccaaaaaa
abcccccccaaaaaaaaccccaacccccccaaaaaaaaaaaaaaaaacccccccccccaaaaaccaaaaaacccccaaaaaaaaaaaaaccccaaaaaaaacccccccaacccccaaacccccccccccccaaaaaaacccccccccccccccccccccccccccccccccccccaaaaaa";