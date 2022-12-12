use super::height::Height;

/// Stores a position on the map. Contains unique coordinates and the height at
/// that position.
///
/// Row and column coordinates grow from the Cartesian origin (0, 0) down and
/// right. This is because of the input format. Instead of thinking in terms of
/// Cartesian coordinates, think of compass directions: north, west, south,
/// east.
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub row: isize,
    pub col: isize,
    height: Height,
}

impl Eq for Position {}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl std::hash::Hash for Position {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
    }
}

impl Position {
    pub fn new(row: isize, col: isize, height: Height) -> Self {
        Position { row, col, height }
    }

    pub fn go_north(&self) -> Self {
        let north = self.row - 1;
        Position::new(north, self.col, self.height)
    }

    pub fn go_south(&self) -> Self {
        let south = self.row + 1;
        Position::new(south, self.col, self.height)
    }

    pub fn go_east(&self) -> Self {
        let east = self.col + 1;
        Position::new(self.row, east, self.height)
    }

    pub fn go_west(&self) -> Self {
        let west = self.col - 1;
        Position::new(self.row, west, self.height)
    }

    pub fn can_move_to(&self, other: Position) -> bool {
        self.height.can_move_to(other.height)
    }

    pub fn get_height(&self) -> u8 {
        self.height.get_value()
    }

    pub fn is_start(c: char) -> bool {
        c == 'S'
    }

    pub fn is_goal(c: char) -> bool {
        c == 'E'
    }
}
