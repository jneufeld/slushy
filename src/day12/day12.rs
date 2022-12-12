use std::collections::{HashSet, VecDeque};

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