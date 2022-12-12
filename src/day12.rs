use std::collections::{HashSet, VecDeque};

pub fn solve() {
    let (map, start, goal) = parse_map(SAMPLE);
    let mut path = Directions::new();
    let steps = min_steps(start, goal, &map, &mut path);

    println!("{}", steps)
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

struct Directions {
    positions: VecDeque<Position>,
}

impl Directions {
    fn new() -> Self {
        Self {
            positions: VecDeque::new(),
        }
    }

    /// Cycle detection?
    fn already_visited(&self, position: Position) -> bool {
        self.positions.contains(&position)
    }

    fn push(&mut self, position: Position) {
        self.positions.push_back(position);
    }

    fn pop(&mut self) -> Option<Position> {
        self.positions.pop_back()
    }
}

fn min_steps(
    current: Position,
    goal: Position,
    map: &HashSet<Position>,
    path: &mut Directions,
) -> usize {
    if current == goal {
        return 0;
    }

    let mut min_so_far = usize::MAX - 1;

    // North
    if let Some(north) = map.get(&current.go_north()) {
        min_so_far = min_steps_dir(current, *north, goal, map, path, min_so_far);
    }

    // South
    if let Some(south) = map.get(&current.go_south()) {
        min_so_far = min_steps_dir(current, *south, goal, map, path, min_so_far);
    }

    // East
    if let Some(east) = map.get(&current.go_east()) {
        min_so_far = min_steps_dir(current, *east, goal, map, path, min_so_far);
    }

    // West
    if let Some(west) = map.get(&current.go_west()) {
        min_so_far = min_steps_dir(current, *west, goal, map, path, min_so_far);
    }

    min_so_far
}

fn min_steps_dir(
    current: Position,
    next: Position,
    goal: Position,
    map: &HashSet<Position>,
    path: &mut Directions,
    min_so_far: usize,
) -> usize {
    // The next position exists on the map. It must be reachable and
    // unvisited to qualify as visitable.
    if !current.can_move_to(next) || path.already_visited(next) {
        return min_so_far;
    }

    path.push(next);

    let steps = 1 + min_steps(next, goal, map, path);

    path.pop();

    if steps == 1 {
        return 1;
    }

    if steps < min_so_far {
        return steps;
    }

    min_so_far
}

// NB think about N/S/E/W rather than up/down/right/left
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    row: isize,
    col: isize,
    height: Height,
}

impl Position {
    fn new(row: isize, col: isize, height: Height) -> Self {
        Position { row, col, height }
    }

    fn go_north(&self) -> Self {
        let north = self.row - 1;
        Position::new(north, self.col, self.height)
    }

    fn go_south(&self) -> Self {
        let south = self.row + 1;
        Position::new(south, self.col, self.height)
    }

    fn go_east(&self) -> Self {
        let east = self.col + 1;
        Position::new(self.row, east, self.height)
    }

    fn go_west(&self) -> Self {
        let west = self.col - 1;
        Position::new(self.row, west, self.height)
    }

    fn can_move_to(&self, other: Position) -> bool {
        self.height.can_move_to(other.height)
    }

    fn is_start(c: char) -> bool {
        c == 'S'
    }

    fn is_goal(c: char) -> bool {
        c == 'E'
    }
}

/// Represents a height value that can be compared and ordered
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Height {
    value: u8,
}

impl Height {
    fn can_move_to(&self, other: Height) -> bool {
        self.value >= other.value - 1
    }
}

impl From<char> for Height {
    fn from(c: char) -> Self {
        let value = match c {
            'S' => 'a',
            'E' => 'z',
            _ => c,
        };

        let value = value as u8;

        Height { value }
    }
}

const SAMPLE: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
