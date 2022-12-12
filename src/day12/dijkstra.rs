use std::collections::{HashMap, HashSet, VecDeque};

use super::position::Position;

pub fn shortest_path(
    start: Position,
    goal: Position,
    map: &HashSet<Position>,
) -> Option<VecDeque<Position>> {
    // The frontier is the group of positions we are interested in exploring.
    // Initially, this only includes the starting position. Think of it as a
    // queue.
    let mut frontier = VecDeque::new();
    frontier.push_back(start);

    // Track the shortest path by setting the predecessor of each node
    let mut predecessors = HashMap::new();

    // The graph score of each node is the currently known shortest path to that
    // node. The starting node, by definition, has distance 0.
    let mut graph_score = HashMap::new();
    graph_score.insert(start, 0);

    // The guess score is a combination of the heuristic and current graph
    // score.
    let mut guess_score = HashMap::new();
    guess_score.insert(start, heuristic(start));

    while !frontier.is_empty() {
        let current = frontier.pop_front().unwrap();

        if current == goal {
            let path = build_path(current, predecessors);
            return Some(path);
        }

        for neighbour in get_neighbours(current, map) {
            let tentative_graph_score = 1 + graph_score.get(&current).unwrap();
            let current_score = graph_score
                .get(&neighbour)
                .unwrap_or(&usize::MAX)
                .to_owned();

            if tentative_graph_score < current_score {
                predecessors.insert(neighbour, current);
                graph_score.insert(neighbour, tentative_graph_score);
                guess_score.insert(neighbour, tentative_graph_score + heuristic(neighbour));

                if !frontier.contains(&neighbour) {
                    frontier.push_back(neighbour);
                    frontier.make_contiguous().sort_by(|n1, n2| {
                        guess_score
                            .get(n1)
                            .unwrap()
                            .cmp(guess_score.get(n2).unwrap())
                    });
                }
            }
        }
    }

    None
}

fn build_path(current: Position, predecessors: HashMap<Position, Position>) -> VecDeque<Position> {
    let mut path = VecDeque::new();
    path.push_back(current);

    let mut previous = predecessors.get(&current);

    while previous.is_some() {
        let node = previous.unwrap().to_owned();
        path.push_front(node);

        previous = predecessors.get(&node);
    }

    path
}

fn heuristic(node: Position) -> usize {
    let highest = b'z';
    let current = node.get_height();

    (highest - current) as usize
}

fn get_neighbours(node: Position, map: &HashSet<Position>) -> Vec<Position> {
    let mut neighbours = Vec::new();

    if let Some(north) = can_go(node, node.go_north(), map) {
        neighbours.push(north);
    }

    if let Some(south) = can_go(node, node.go_south(), map) {
        neighbours.push(south);
    }

    if let Some(east) = can_go(node, node.go_east(), map) {
        neighbours.push(east);
    }

    if let Some(west) = can_go(node, node.go_west(), map) {
        neighbours.push(west);
    }

    neighbours
}

fn can_go(from: Position, destination: Position, map: &HashSet<Position>) -> Option<Position> {
    // If the destination isn't on the map then it is unreachable
    if let Some(destination) = map.get(&destination) {
        // The destination is on the map. It is reachable according to height
        // rules defined in `Height`.
        if from.can_move_to(*destination) {
            return Some(destination.to_owned());
        }
    }

    None
}

// v..v<<<<
// >v.vv<<^
// .>vv>E^^
// ..v>>>^^
// ..>>>>>^"
