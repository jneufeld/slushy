use core::panic;
use std::{
    collections::VecDeque,
    fmt::{Display, Formatter},
};

use indexmap::IndexMap;

// Here only for convenience of testing in the terminal
const ROUNDS: usize = 10_000;
const INPUT: &str = PUZZLE;

/// Part two only
pub fn solve() {
    let mut monkies = parse_monkies(INPUT);

    // A few cues pushed me towards LCM and this calculation. First, the test
    // operation is division. Second, even in the sample input, the operand is
    // prime. That pattern is made glaringly obvious in the puzzle input. Those
    // ideas plus the need to keep sizes bounded (operations are monotonically
    // increasing) were the tricky part of the problem.
    let least_common_multiple: u128 = monkies
        .iter()
        .map(|(_, monkey)| monkey.test.divisible_by)
        .product();

    // I don't think `modulus` is the correct terminology :shrug: Is it
    // `modulo` like "the positive integers modulo n"? I've been out of school
    // too long to remember.
    monkies
        .iter_mut()
        .for_each(|(_, monkey)| monkey.operation.modulus = Some(least_common_multiple));

    let num_monkeys = monkies.len();

    // Monkey around :shrug:
    for _ in 1..ROUNDS + 1 {
        for idx in 0..num_monkeys {
            while let Some((item, target)) = monkies.get_mut(&idx).unwrap().inspect() {
                monkies.entry(target.0).and_modify(|m| m.catch(item));
            }
        }
    }

    // Sort by how many items were inspected
    monkies.sort_by(|_, m1, _, m2| m2.inspected.cmp(&m1.inspected));

    // Asking the important questions: what's the degree of this monkey
    // business?
    let mut top_two = monkies.iter().take(2);

    let (_, first) = top_two.next().unwrap();
    let (_, second) = top_two.next().unwrap();

    let monkey_business = first.inspected * second.inspected;

    println!("monkey business: {}", monkey_business);
}

/// States for parsing (think state machine)
enum State {
    Monkey,
    Items,
    Operation,
    TestDivisor,
    TestTruth,
    TestFalse,
}

fn parse_monkies(input: &str) -> IndexMap<usize, Monkey> {
    let mut monkies = IndexMap::new();

    let mut state = State::Monkey;
    let mut monkey_number = 0;
    let mut current_monkey = Monkey::default();

    for line in input.split('\n') {
        if line.trim().is_empty() {
            continue;
        }

        match state {
            State::Monkey => {
                // Must be first step of this state (this of this as a `Start`
                // state). The current line doesn't yet matter but if it is
                // needed later, it contains the monkey number.
                current_monkey = Monkey::default();
                state = State::Items;
            }
            State::Items => {
                current_monkey.items = Items::from(line);
                state = State::Operation;
            }
            State::Operation => {
                current_monkey.operation = Operation::from(line);
                state = State::TestDivisor;
            }
            State::TestDivisor => {
                current_monkey.test.with_divisible_by(line);
                state = State::TestTruth;
            }
            State::TestTruth => {
                current_monkey.test.with_truth(line);
                state = State::TestFalse;
            }
            State::TestFalse => {
                current_monkey.test.with_false(line);

                // Must be final step of this state (this of this as an `Accept`
                // state)
                monkies.insert(monkey_number, current_monkey.clone());

                monkey_number += 1;
                state = State::Monkey;
            }
        }
    }

    monkies
}

#[derive(Debug, Default, Clone, Hash, PartialEq)]
struct Monkey {
    items: Items,
    operation: Operation,
    test: Test,
    inspected: usize,
}

impl Monkey {
    pub fn inspect(&mut self) -> Option<(WorryLevel, ThrowTo)> {
        let worry_level = self.items.items.pop_back()?; // .items.item.items -_-
        let worry_level = worry_level.modify_with(&self.operation);

        let throw_to = self.get_throw_target(&worry_level);

        self.inspected += 1;

        Some((worry_level, throw_to))
    }

    pub fn catch(&mut self, item: WorryLevel) {
        self.items.items.push_front(item);
    }

    fn get_throw_target(&self, worry_level: &WorryLevel) -> ThrowTo {
        if worry_level.value % self.test.divisible_by == 0 {
            self.test.when_true
        } else {
            self.test.when_false
        }
    }
}

impl Display for Monkey {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "items (inspected {}): {:?}",
            self.inspected, self.items
        )
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, PartialOrd, Eq, Ord)]
struct WorryLevel {
    value: u128,
}

impl WorryLevel {
    fn modify_with(self, operation: &Operation) -> Self {
        let operator = &operation.operator;
        let operand = &operation.operand;

        let updated_level = match (operator, operand) {
            (Operator::Addition, Operand::Value(operand)) => self.value + operand,
            (Operator::Addition, Operand::Variable(_)) => self.value + self.value,
            (Operator::Multiplication, Operand::Value(operand)) => self.value * operand,
            (Operator::Multiplication, Operand::Variable(_)) => self.value * self.value,
        };

        match operation.modulus {
            Some(modulus) => WorryLevel::from(updated_level % modulus),
            None => panic!("no modulus provided"),
        }
    }
}

impl From<u128> for WorryLevel {
    fn from(value: u128) -> Self {
        WorryLevel { value }
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq)]
struct Items {
    items: VecDeque<WorryLevel>,
}

impl From<&str> for Items {
    fn from(s: &str) -> Self {
        let mut items = VecDeque::new();

        let input = s.trim();
        let (_ignore, numbers) = input.split_at(16);

        let mut number = String::new();

        for character in numbers.chars() {
            match character {
                ',' => {
                    let number = number.parse::<u128>();
                    let number = WorryLevel::from(number.unwrap());
                    items.push_front(number);
                }
                ' ' => number = String::new(),
                _ => number.push(character),
            }
        }

        let last_number = number.parse::<u128>().unwrap();

        items.push_front(WorryLevel::from(last_number));

        Items { items }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Default)]
struct Operation {
    operator: Operator,
    operand: Operand,
    modulus: Option<u128>,
}

impl From<&str> for Operation {
    fn from(s: &str) -> Self {
        let input = s.trim();
        let ignore_len = "Operation: new = old ".len();

        // The value of `s` current holds `<OPERATOR> <OPERAND>`
        let (_ignore, operation) = input.split_at(ignore_len);

        let mut operation = operation.chars().peekable();

        // Consuming an iterator over the characterse one time yields the
        // character that maps to an `Operator` type. Parsing, yada yada.
        let operator: char = operation.next().unwrap();
        let operator = Operator::from(operator);

        let _ignore_space = operation.next();

        let operand: String = operation.collect();
        let operand = Operand::from(operand);

        Operation {
            operator,
            operand,
            modulus: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
enum Operator {
    Addition,
    Multiplication,
}

impl Default for Operator {
    // Utter nonsense but that only exposes the poor naming :shrug:
    fn default() -> Self {
        Operator::Addition
    }
}

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '+' => Operator::Addition,
            '*' => Operator::Multiplication,
            _ => panic!("unexpected operator"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq)]
enum Operand {
    Value(u128),
    Variable(String),
}

impl Default for Operand {
    // Utter nonsense but that only exposes the poor naming :shrug:
    fn default() -> Self {
        Operand::Value(0)
    }
}

impl From<String> for Operand {
    fn from(s: String) -> Self {
        match s.parse() {
            Ok(value) => Operand::Value(value),
            Err(_) => Operand::Variable(s),
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
struct ThrowTo(usize);

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
struct Test {
    divisible_by: u128,
    when_true: ThrowTo,
    when_false: ThrowTo,
}

impl Default for Test {
    fn default() -> Self {
        Test {
            divisible_by: 0,
            when_true: ThrowTo(0),
            when_false: ThrowTo(0),
        }
    }
}

impl Test {
    fn with_divisible_by(&mut self, line: &str) {
        let input = line.trim();
        let ignore_len = "Test: divisible by ".len();
        let (_ignore, number) = input.split_at(ignore_len);
        let number = number.parse::<u128>().unwrap();

        self.divisible_by = number;
    }

    fn with_truth(&mut self, line: &str) {
        let input = line.trim();
        let ignore_len = "If true: throw to monkey ".len();
        let (_ignore, number) = input.split_at(ignore_len);
        let number = number.parse::<usize>().unwrap();

        self.when_true = ThrowTo(number);
    }

    fn with_false(&mut self, line: &str) {
        let input = line.trim();
        let ignore_len = "If false: throw to monkey ".len();
        let (_ignore, number) = input.split_at(ignore_len);
        let number = number.parse::<usize>().unwrap();

        self.when_false = ThrowTo(number);
    }
}

const PUZZLE: &str = r"Monkey 0:
  Starting items: 98, 70, 75, 80, 84, 89, 55, 98
  Operation: new = old * 2
  Test: divisible by 11
    If true: throw to monkey 1
    If false: throw to monkey 4

Monkey 1:
  Starting items: 59
  Operation: new = old * old
  Test: divisible by 19
    If true: throw to monkey 7
    If false: throw to monkey 3

Monkey 2:
  Starting items: 77, 95, 54, 65, 89
  Operation: new = old + 6
  Test: divisible by 7
    If true: throw to monkey 0
    If false: throw to monkey 5

Monkey 3:
  Starting items: 71, 64, 75
  Operation: new = old + 2
  Test: divisible by 17
    If true: throw to monkey 6
    If false: throw to monkey 2

Monkey 4:
  Starting items: 74, 55, 87, 98
  Operation: new = old * 11
  Test: divisible by 3
    If true: throw to monkey 1
    If false: throw to monkey 7

Monkey 5:
  Starting items: 90, 98, 85, 52, 91, 60
  Operation: new = old + 7
  Test: divisible by 5
    If true: throw to monkey 0
    If false: throw to monkey 4

Monkey 6:
  Starting items: 99, 51
  Operation: new = old + 1
  Test: divisible by 13
    If true: throw to monkey 5
    If false: throw to monkey 2

Monkey 7:
  Starting items: 98, 94, 59, 76, 51, 65, 75
  Operation: new = old + 5
  Test: divisible by 2
    If true: throw to monkey 3
    If false: throw to monkey 6";
