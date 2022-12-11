use std::collections::VecDeque;

pub fn solve() {
    let monkies = parse_monkies(SAMPLE);

    for monkey in monkies {
        println!("{:?}", monkey);
    }
}

enum State {
    Monkey,
    Items,
    Operation,
    TestDivisor,
    TestTruth,
    TestFalse,
}

fn parse_monkies(input: &str) -> VecDeque<Monkey> {
    let mut state = State::Monkey;
    let mut monkies = VecDeque::new();
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
                current_monkey.starting_items = StartingItems::from(line);
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
                monkies.push_back(current_monkey.clone());
                state = State::Monkey;
            }
        }
    }

    monkies
}

#[derive(Debug, Default, Clone, Hash, PartialEq)]
struct Monkey {
    starting_items: StartingItems,
    operation: Operation,
    test: Test,
}

#[derive(Debug, Default, Clone, Hash, PartialEq)]
struct StartingItems {
    items: VecDeque<usize>,
}

impl From<&str> for StartingItems {
    fn from(s: &str) -> Self {
        let mut items = VecDeque::new();

        let input = s.trim();
        let (_ignore, numbers) = input.split_at(16);

        let mut number = String::new();

        for character in numbers.chars() {
            match character {
                ',' => items.push_back(number.parse::<usize>().unwrap()),
                ' ' => number = String::new(),
                _ => number.push(character),
            }
        }

        let last_number = number.parse::<usize>().unwrap();

        items.push_back(last_number);

        StartingItems { items }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Default)]
struct Operation {
    operator: Operator,
    operand: Operand,
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

        Operation { operator, operand }
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
    Value(usize),
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
    divisible_by: usize,
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
        let number = number.parse::<usize>().unwrap();

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

const SAMPLE: &str = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

#[cfg(test)]
mod tests {
    use crate::day11::{StartingItems, Test, ThrowTo};

    use super::{parse_monkies, Operand, Operation, Operator, SAMPLE};

    #[test]
    fn sample_parses() {
        let mut monkies = parse_monkies(SAMPLE);

        let first = monkies.pop_front().expect("First monkey wasn't parsed");

        let mut expected_items = StartingItems::default();
        expected_items.items.push_back(79);
        expected_items.items.push_back(98);

        assert_eq!(first.starting_items, expected_items);

        let expected_operator = Operator::Multiplication;
        let expected_operand = Operand::Value(19);

        let expected_operation = Operation {
            operator: expected_operator,
            operand: expected_operand,
        };

        assert_eq!(first.operation, expected_operation);

        let expected_test = Test {
            divisible_by: 23,
            when_true: ThrowTo(2),
            when_false: ThrowTo(3),
        };

        assert_eq!(first.test, expected_test);
    }
}
