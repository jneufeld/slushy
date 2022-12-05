use std::collections::VecDeque;

use lazy_static::lazy_static;
use regex::Regex;

/// Day 5 feels like Towers of Hanoi. It's the first challenge I bumped into
/// Rust's borrow checker, and the first where the solution uses hard-coded
/// values. In other words, I have to admit it took me a little longer.
pub fn solve() {
    // Each stack of crates is a tower
    let mut stacks = parse_stacks();
    let actions = parse_actions();

    for action in actions {
        // The temporary stack preserves the ordering when moving items from a
        // source stack to the target when there are multiple items. E.g. when
        // moving `123` the items are pushed down so the order is `321`. Popping
        // items over to the target stack reverses the order back to `123`.
        let mut temp = VecDeque::new();

        // Pop from source to temp stack as many times as directed by the action
        for _ in 0..action.count {
            // TODO lazy and dangerous to switch between number and index
            let elem = stacks[action.from - 1].pop_back().unwrap();
            temp.push_back(elem);
        }

        // Pop from temp to target stack as many times as directed by the action
        for _ in 0..action.count {
            let elem = temp.pop_back().unwrap();
            stacks[action.to - 1].push_back(elem);
        }
    }

    for mut stack in stacks {
        let top = stack.pop_back();
        println!("top: {:?}", top);
    }
}

fn parse_stacks() -> Vec<VecDeque<char>> {
    // yikes
    let mut stacks = vec![
        VecDeque::new(), // 1
        VecDeque::new(), // 2
        VecDeque::new(), // 3
        VecDeque::new(), // 4
        VecDeque::new(), // 5
        VecDeque::new(), // 6
        VecDeque::new(), // 7
        VecDeque::new(), // 8
        VecDeque::new(), // 9
    ];

    // NB hard-coded `8` because of the input (`3` for testing)
    for input_line in REAL_STACK_INPUT.split('\n').take(8) {
        let mut chars = input_line.chars();

        // NB hard-coded `9` because of the input (`3` for testing)
        for column in 0..9 {
            // Consume the characters even if they are blanks. This moves the
            // iterator to the contents of the next pile.
            let first = chars.next().unwrap(); // '['
            let second = chars.next().unwrap(); // content
            let _third = chars.next().unwrap(); // ']'

            if first == '[' {
                // Push to the top of the stack
                stacks[column].push_front(second);
            }

            let _blank = chars.next();
        }
    }

    stacks
}

#[derive(Debug, Clone, Copy)]
struct Action {
    count: usize,
    from: usize,
    to: usize,
}

lazy_static! {
    static ref ACTION_REGEX: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

fn parse_actions() -> Vec<Action> {
    let mut actions = Vec::new();

    for line in REAL_ACTIONS_INPUT.split('\n') {
        let captures = ACTION_REGEX.captures(line).unwrap();

        let count = captures.get(1).unwrap();
        let count = count.as_str().parse::<usize>().unwrap();

        let from = captures.get(2).unwrap();
        let from = from.as_str().parse::<usize>().unwrap();

        let to = captures.get(3).unwrap();
        let to = to.as_str().parse::<usize>().unwrap();

        actions.push(Action { count, from, to });
    }

    actions
}

const TEST_STACK_INPUT: &str = r"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";

const TEST_ACTIONS_INPUT: &str = r"move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

const REAL_STACK_INPUT: &str = r"[N]     [Q]         [N]            
[R]     [F] [Q]     [G] [M]        
[J]     [Z] [T]     [R] [H] [J]    
[T] [H] [G] [R]     [B] [N] [T]    
[Z] [J] [J] [G] [F] [Z] [S] [M]    
[B] [N] [N] [N] [Q] [W] [L] [Q] [S]
[D] [S] [R] [V] [T] [C] [C] [N] [G]
[F] [R] [C] [F] [L] [Q] [F] [D] [P]
 1   2   3   4   5   6   7   8   9 ";

const REAL_ACTIONS_INPUT: &str = r"move 3 from 9 to 4
move 2 from 5 to 2
move 8 from 1 to 9
move 4 from 7 to 1
move 5 from 3 to 8
move 3 from 3 to 7
move 11 from 8 to 3
move 7 from 3 to 6
move 2 from 5 to 9
move 3 from 1 to 6
move 6 from 2 to 4
move 6 from 7 to 5
move 1 from 6 to 1
move 1 from 9 to 4
move 16 from 4 to 9
move 2 from 1 to 2
move 1 from 4 to 6
move 1 from 3 to 7
move 2 from 2 to 4
move 1 from 7 to 9
move 22 from 9 to 8
move 1 from 6 to 3
move 18 from 8 to 5
move 3 from 8 to 2
move 3 from 2 to 9
move 13 from 6 to 7
move 1 from 6 to 7
move 4 from 3 to 6
move 2 from 6 to 3
move 2 from 3 to 8
move 3 from 7 to 8
move 14 from 5 to 2
move 3 from 2 to 5
move 2 from 8 to 4
move 4 from 8 to 6
move 4 from 6 to 4
move 11 from 5 to 2
move 3 from 9 to 2
move 7 from 2 to 3
move 11 from 7 to 2
move 1 from 5 to 7
move 5 from 6 to 8
move 30 from 2 to 7
move 23 from 7 to 2
move 4 from 3 to 4
move 3 from 9 to 6
move 4 from 8 to 2
move 1 from 8 to 2
move 2 from 7 to 9
move 4 from 2 to 3
move 1 from 5 to 9
move 6 from 4 to 7
move 5 from 3 to 6
move 1 from 3 to 6
move 1 from 9 to 2
move 16 from 2 to 5
move 7 from 7 to 6
move 9 from 2 to 1
move 2 from 1 to 4
move 8 from 5 to 3
move 5 from 7 to 4
move 1 from 9 to 8
move 9 from 3 to 6
move 25 from 6 to 8
move 2 from 9 to 5
move 3 from 4 to 2
move 7 from 4 to 1
move 1 from 8 to 7
move 6 from 5 to 2
move 11 from 8 to 5
move 1 from 7 to 9
move 10 from 1 to 2
move 6 from 5 to 1
move 1 from 4 to 2
move 13 from 8 to 1
move 17 from 1 to 2
move 5 from 1 to 9
move 1 from 8 to 4
move 1 from 1 to 3
move 1 from 3 to 6
move 1 from 9 to 3
move 2 from 4 to 5
move 1 from 4 to 8
move 1 from 9 to 1
move 8 from 5 to 7
move 1 from 8 to 1
move 7 from 7 to 6
move 2 from 1 to 2
move 1 from 3 to 6
move 2 from 5 to 4
move 8 from 2 to 1
move 1 from 9 to 7
move 1 from 5 to 1
move 2 from 7 to 3
move 2 from 3 to 7
move 2 from 7 to 8
move 2 from 1 to 5
move 3 from 9 to 2
move 2 from 8 to 9
move 1 from 9 to 2
move 1 from 9 to 8
move 1 from 8 to 7
move 6 from 6 to 5
move 1 from 6 to 2
move 2 from 4 to 5
move 2 from 6 to 8
move 1 from 7 to 1
move 2 from 8 to 4
move 11 from 2 to 5
move 18 from 5 to 6
move 6 from 2 to 6
move 10 from 2 to 7
move 1 from 4 to 3
move 7 from 2 to 8
move 7 from 1 to 4
move 6 from 7 to 8
move 2 from 7 to 3
move 8 from 4 to 7
move 1 from 1 to 3
move 1 from 2 to 1
move 4 from 7 to 1
move 4 from 1 to 3
move 2 from 3 to 9
move 2 from 5 to 4
move 1 from 2 to 1
move 2 from 1 to 5
move 1 from 3 to 1
move 2 from 5 to 2
move 1 from 2 to 6
move 5 from 7 to 4
move 1 from 1 to 2
move 10 from 8 to 1
move 2 from 2 to 7
move 2 from 7 to 1
move 1 from 7 to 9
move 1 from 5 to 7
move 3 from 8 to 7
move 3 from 3 to 6
move 3 from 7 to 1
move 5 from 1 to 4
move 1 from 7 to 6
move 22 from 6 to 3
move 2 from 6 to 2
move 19 from 3 to 4
move 15 from 4 to 8
move 9 from 8 to 4
move 5 from 6 to 8
move 2 from 2 to 8
move 2 from 9 to 4
move 7 from 1 to 5
move 1 from 1 to 3
move 1 from 9 to 7
move 5 from 8 to 3
move 4 from 8 to 1
move 5 from 1 to 5
move 10 from 4 to 3
move 3 from 4 to 2
move 2 from 8 to 3
move 12 from 4 to 8
move 1 from 7 to 6
move 3 from 2 to 9
move 2 from 4 to 5
move 5 from 3 to 7
move 1 from 7 to 2
move 1 from 1 to 6
move 1 from 7 to 2
move 15 from 3 to 8
move 10 from 5 to 6
move 3 from 7 to 8
move 1 from 5 to 8
move 1 from 2 to 3
move 7 from 6 to 1
move 3 from 5 to 3
move 5 from 3 to 5
move 3 from 5 to 4
move 2 from 4 to 9
move 2 from 3 to 5
move 14 from 8 to 5
move 1 from 9 to 1
move 16 from 5 to 3
move 16 from 3 to 6
move 2 from 9 to 8
move 21 from 6 to 7
move 2 from 1 to 7
move 1 from 2 to 7
move 4 from 1 to 7
move 1 from 4 to 7
move 16 from 8 to 5
move 20 from 7 to 4
move 1 from 9 to 8
move 1 from 7 to 4
move 3 from 8 to 6
move 1 from 9 to 1
move 2 from 1 to 4
move 2 from 5 to 2
move 5 from 4 to 7
move 1 from 6 to 9
move 11 from 7 to 6
move 2 from 7 to 5
move 12 from 6 to 2
move 13 from 2 to 1
move 1 from 2 to 3
move 1 from 8 to 4
move 6 from 4 to 1
move 1 from 6 to 7
move 7 from 4 to 9
move 8 from 9 to 3
move 2 from 8 to 3
move 10 from 5 to 4
move 11 from 1 to 8
move 1 from 1 to 3
move 5 from 1 to 8
move 8 from 5 to 6
move 13 from 8 to 9
move 12 from 3 to 5
move 12 from 5 to 9
move 1 from 7 to 9
move 1 from 1 to 2
move 1 from 1 to 4
move 3 from 8 to 5
move 1 from 2 to 5
move 1 from 4 to 8
move 5 from 6 to 3
move 1 from 8 to 4
move 13 from 4 to 7
move 3 from 7 to 6
move 1 from 1 to 4
move 4 from 4 to 2
move 1 from 6 to 3
move 2 from 5 to 9
move 2 from 5 to 9
move 1 from 4 to 8
move 6 from 9 to 4
move 22 from 9 to 2
move 8 from 7 to 4
move 7 from 2 to 1
move 3 from 3 to 8
move 2 from 6 to 7
move 14 from 4 to 2
move 2 from 6 to 1
move 1 from 8 to 7
move 3 from 3 to 9
move 1 from 8 to 4
move 3 from 1 to 9
move 3 from 9 to 3
move 31 from 2 to 8
move 8 from 8 to 4
move 1 from 9 to 1
move 9 from 4 to 5
move 7 from 5 to 6
move 2 from 5 to 1
move 1 from 2 to 1
move 1 from 7 to 9
move 1 from 2 to 9
move 2 from 6 to 4
move 4 from 7 to 4
move 4 from 9 to 8
move 6 from 4 to 1
move 1 from 3 to 2
move 1 from 3 to 6
move 1 from 9 to 2
move 2 from 2 to 4
move 1 from 9 to 1
move 1 from 3 to 1
move 17 from 1 to 9
move 4 from 6 to 2
move 1 from 9 to 7
move 4 from 9 to 7
move 1 from 8 to 4
move 3 from 7 to 6
move 1 from 4 to 9
move 10 from 8 to 5
move 6 from 6 to 5
move 1 from 7 to 2
move 1 from 1 to 4
move 1 from 4 to 5
move 9 from 8 to 3
move 4 from 3 to 9
move 1 from 4 to 6
move 1 from 6 to 5
move 1 from 4 to 8
move 2 from 3 to 8
move 1 from 3 to 8
move 3 from 8 to 9
move 5 from 2 to 9
move 1 from 7 to 9
move 5 from 8 to 7
move 3 from 8 to 4
move 2 from 8 to 5
move 24 from 9 to 7
move 1 from 3 to 5
move 2 from 9 to 4
move 22 from 7 to 9
move 1 from 3 to 4
move 6 from 4 to 6
move 4 from 6 to 7
move 10 from 5 to 3
move 8 from 3 to 5
move 2 from 3 to 4
move 2 from 4 to 6
move 10 from 7 to 3
move 21 from 9 to 1
move 2 from 3 to 4
move 4 from 3 to 8
move 2 from 4 to 8
move 1 from 7 to 8
move 4 from 6 to 8
move 3 from 5 to 4
move 8 from 8 to 2
move 18 from 1 to 6
move 3 from 4 to 1
move 1 from 2 to 8
move 5 from 1 to 8
move 3 from 3 to 6
move 3 from 2 to 9
move 3 from 8 to 1
move 11 from 5 to 2
move 3 from 8 to 7
move 10 from 2 to 9
move 1 from 7 to 9
move 3 from 8 to 1
move 2 from 7 to 8
move 6 from 9 to 5
move 4 from 2 to 8
move 8 from 5 to 8
move 1 from 3 to 7
move 2 from 5 to 6
move 3 from 1 to 6
move 2 from 1 to 6
move 4 from 9 to 8
move 4 from 9 to 8
move 1 from 9 to 4
move 9 from 6 to 9
move 16 from 6 to 9
move 1 from 4 to 7
move 1 from 2 to 9
move 5 from 8 to 5
move 4 from 5 to 1
move 6 from 1 to 7
move 12 from 8 to 4
move 5 from 8 to 1
move 6 from 9 to 3
move 1 from 1 to 6
move 2 from 5 to 8
move 12 from 4 to 7
move 2 from 8 to 4
move 1 from 4 to 8
move 1 from 7 to 6
move 1 from 4 to 6
move 14 from 7 to 1
move 3 from 3 to 2
move 7 from 9 to 7
move 3 from 3 to 5
move 15 from 1 to 2
move 2 from 5 to 9
move 1 from 8 to 9
move 16 from 9 to 1
move 1 from 5 to 9
move 5 from 6 to 2
move 12 from 7 to 2
move 20 from 2 to 6
move 10 from 2 to 6
move 11 from 1 to 7
move 2 from 7 to 4
move 2 from 2 to 5
move 1 from 2 to 3
move 2 from 5 to 6
move 1 from 9 to 5
move 1 from 5 to 9
move 25 from 6 to 7
move 25 from 7 to 6
move 1 from 3 to 1
move 1 from 2 to 5
move 1 from 4 to 3
move 33 from 6 to 3
move 1 from 9 to 5
move 2 from 3 to 5
move 28 from 3 to 9
move 5 from 1 to 9
move 4 from 1 to 8
move 2 from 3 to 2
move 2 from 8 to 1
move 1 from 4 to 6
move 3 from 5 to 3
move 1 from 2 to 4
move 2 from 2 to 8
move 1 from 6 to 5
move 30 from 9 to 2
move 7 from 2 to 6
move 1 from 1 to 3
move 1 from 1 to 7
move 1 from 5 to 6
move 1 from 5 to 4
move 5 from 7 to 4
move 4 from 7 to 3
move 1 from 3 to 7
move 3 from 8 to 7
move 8 from 3 to 1
move 3 from 1 to 7
move 4 from 1 to 4
move 3 from 9 to 8
move 8 from 6 to 2
move 18 from 2 to 6
move 6 from 7 to 2
move 1 from 1 to 7
move 3 from 4 to 7
move 5 from 4 to 8
move 2 from 8 to 7
move 7 from 2 to 5
move 5 from 2 to 7
move 10 from 7 to 9
move 5 from 5 to 9
move 1 from 3 to 9
move 5 from 2 to 6
move 3 from 7 to 9
move 2 from 5 to 6
move 2 from 2 to 9
move 2 from 8 to 7
move 1 from 4 to 5
move 8 from 9 to 2
move 5 from 6 to 7
move 4 from 9 to 1
move 4 from 8 to 9
move 12 from 9 to 1
move 16 from 1 to 4
move 12 from 6 to 2
move 3 from 7 to 6
move 3 from 7 to 3
move 1 from 9 to 4
move 12 from 4 to 5
move 2 from 4 to 3
move 1 from 7 to 1
move 4 from 4 to 3
move 1 from 8 to 2
move 6 from 3 to 1
move 1 from 1 to 8
move 7 from 2 to 5
move 1 from 8 to 1
move 4 from 5 to 4
move 5 from 5 to 9
move 1 from 3 to 8
move 1 from 9 to 7
move 1 from 8 to 1
move 4 from 5 to 6
move 5 from 5 to 9
move 7 from 9 to 5
move 11 from 6 to 4
move 1 from 9 to 4
move 1 from 9 to 1
move 1 from 7 to 2
move 9 from 4 to 3
move 5 from 1 to 9
move 3 from 5 to 1
move 5 from 9 to 8
move 8 from 3 to 1
move 2 from 5 to 3
move 7 from 2 to 5
move 1 from 6 to 4
move 3 from 5 to 9
move 3 from 6 to 9
move 3 from 2 to 9
move 5 from 3 to 6
move 1 from 9 to 5
move 4 from 8 to 3
move 1 from 8 to 4
move 8 from 1 to 3
move 12 from 3 to 7
move 1 from 2 to 4
move 3 from 2 to 8
move 6 from 7 to 6
move 4 from 5 to 7
move 5 from 9 to 7
move 2 from 9 to 2
move 1 from 9 to 5
move 4 from 5 to 1
move 1 from 5 to 4
move 14 from 7 to 6
move 1 from 1 to 7
move 10 from 4 to 5
move 4 from 1 to 2
move 1 from 4 to 6
move 1 from 7 to 4
move 17 from 6 to 8
move 1 from 5 to 7
move 10 from 5 to 4
move 1 from 2 to 6
move 4 from 2 to 6
move 13 from 6 to 1
move 9 from 4 to 3
move 2 from 2 to 4
move 1 from 6 to 7
move 1 from 4 to 3
move 8 from 3 to 5
move 1 from 3 to 4
move 17 from 1 to 3
move 15 from 3 to 7
move 3 from 4 to 1
move 6 from 8 to 9
move 6 from 9 to 1
move 2 from 3 to 1
move 2 from 5 to 2
move 6 from 7 to 6
move 3 from 6 to 9";
