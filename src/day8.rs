pub fn solve() {
    //let (_, visibility_cols) = parse_tree_heights();

    let num_rows = INPUT.split('\n').next().unwrap().len();
    let num_cols = INPUT.split('\n').count();
    let limits = Position::new(Column(num_cols - 1), Row(num_rows - 1));

    let mut total_visible = 0;

    for row_idx in 0..num_rows {
        for col_idx in 0..num_cols {
            let position = Position::new(Column(col_idx), Row(row_idx));

            if is_visible(position, limits) {
                total_visible += 1;
            }
        }
    }

    println!("total visible: {}", total_visible);
}

fn parse_tree_heights() -> (Vec<Vec<u8>>, Vec<Vec<bool>>) {
    let mut tree_height_columns: Vec<Vec<u8>> = Vec::new();
    let mut tree_visibility_columns: Vec<Vec<bool>> = Vec::new();

    // Iterate through the input line-by-line. These are the rows.
    for line in INPUT.split('\n') {
        // Each row is a line of trees. The input shows the height of the tree
        // as a digit [0-9] where larger values indicate higher trees. To solve
        // the problem, track the height and a boolean indicating if it's
        // visible (to be defined).
        let mut tree_height_row: Vec<u8> = Vec::new();
        let mut tree_visibility_row: Vec<bool> = Vec::new();

        for character in line.chars() {
            // The character '0' has an ASCII value of 48. To get the integer,
            // here a `u8`, subtract 48 from the character's ASCII value to get
            // its literal value -- the height.
            let height = character as u8 - 48;

            tree_height_row.push(height);

            // For now, don't worry about whether the tree is visible. This is
            // easier to answer later on. It is sufficient to think that each
            // tree has both a height and can be visible or not.
            let not_yet_visible = false;

            tree_visibility_row.push(not_yet_visible);
        }

        // Every value in the row has been processed. Now it can be added to the
        // list of columns.
        //
        // NB This order is arbitrary; but if access in the other order is
        // required, the data structures much be switched.
        tree_height_columns.push(tree_height_row);
        tree_visibility_columns.push(tree_visibility_row);
    }

    (tree_height_columns, tree_visibility_columns)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Column(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Row(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Position {
    /// An x-coordinate
    col_idx: Column,
    /// An y-coordinate
    row_idx: Row,
}

impl Position {
    fn new(col_idx: Column, row_idx: Row) -> Position {
        Position { col_idx, row_idx }
    }

    fn is_on_inside_edge(&self) -> bool {
        self.col_idx == Column(0) || self.row_idx == Row(0)
    }

    fn is_on_outside_edge(&self, another: &Position) -> bool {
        self.eq(another)
    }

    fn go_up(&self) -> Position {
        let up = Column(self.col_idx.0 - 1);
        Position {
            col_idx: up,
            row_idx: self.row_idx,
        }
    }

    fn go_down(&self) -> Position {
        let down = Column(self.col_idx.0 + 1);
        Position {
            col_idx: down,
            row_idx: self.row_idx,
        }
    }

    fn go_right(&self) -> Position {
        let right = Row(self.row_idx.0 + 1);
        Position {
            col_idx: self.col_idx,
            row_idx: right,
        }
    }

    fn go_left(&self) -> Position {
        let left = Row(self.row_idx.0 - 1);
        Position {
            col_idx: self.col_idx,
            row_idx: left,
        }
    }
}

fn is_visible(position: Position, limits: Position) -> bool {
    // Visible at the edges of known space
    if position.is_on_inside_edge() || position.is_on_outside_edge(&limits) {
        return true;
    }

    // Taller than all trees to the edges of known space
    return taller_than_all(position, limits);
}

fn taller_than_all(position: Position, limits: Position) -> bool {
    // If on the edge of space, this tree can truly said to be taller than its
    // neighbour. Seeeeessh.
    if position.is_on_inside_edge() || position.is_on_outside_edge(&limits) {
        return true;
    }

    // If taller than UP, DOWN, RIGHT, LEFT then continue that direction to see
    // if that's true all the way to the edges of known space.
    if taller_than_all(position.go_up(), limits)
        || taller_than_all(position.go_down(), limits)
        || taller_than_all(position.go_right(), limits)
        || taller_than_all(position.go_left(), limits)
    {
        return true;
    }

    // This sad tree is not visible from outside the universe
    false
}

const INPUT: &str = r"30373
25512
65332
33549
35390";

fn set_borders_visible(height_cols: &Vec<Vec<u8>>, visibility_cols: &mut Vec<Vec<bool>>) {
    let num_rows = INPUT.split('\n').next().unwrap().len();
    let num_cols = INPUT.split('\n').count();

    let mut largest_in_column = Vec::new();

    for _ in 0..num_cols {
        largest_in_column.push(0);
    }

    for row_idx in 0..num_rows {
        let mut largest_in_row = 0 as u8;

        for col_idx in 0..num_cols {
            let column_height = height_cols.get(col_idx).unwrap();
            let tree_height = *column_height.get(row_idx).unwrap();

            // A tree is only visible over any previous tree if its height is
            // monotonically increasing
            if tree_height > largest_in_row {
                let column_visibility = visibility_cols.get_mut(col_idx).unwrap();
                let tree_visibility = column_visibility.get_mut(row_idx).unwrap();

                *tree_visibility = true;

                println!(
                    "({}, {}) biggest in row was {} now {} (visible)",
                    col_idx, row_idx, largest_in_row, tree_height,
                );

                largest_in_row = tree_height;
            }

            let mut largest_in_col = *largest_in_column.get_mut(col_idx).unwrap() as u8;

            if tree_height > largest_in_col {
                let column_visibility = visibility_cols.get_mut(col_idx).unwrap();
                let tree_visibility = column_visibility.get_mut(row_idx).unwrap();

                *tree_visibility = true;

                println!(
                    "({}, {}) biggest in col was {} now {} (visible)",
                    col_idx, row_idx, largest_in_col, tree_height,
                );

                largest_in_col = tree_height;
            }

            let col_visibility = visibility_cols.get(col_idx).unwrap();
            let is_already_visible = *col_visibility.get(row_idx).unwrap();

            if is_already_visible {
                continue;
            }

            // The first and last row and column are entirely visible
            if row_idx == 0
                || row_idx == (num_rows - 1)
                || col_idx == 0
                || col_idx == (num_cols - 1)
            {
                let column_visibility = visibility_cols.get_mut(col_idx).unwrap();
                let tree_visibility = column_visibility.get_mut(row_idx).unwrap();

                *tree_visibility = true;

                continue;
            }
        }

        largest_in_row = 0;
    }
}
