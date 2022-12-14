use std::{cmp::Ordering, collections::VecDeque};

pub fn parse_packets(input: &str) -> Vec<Item> {
    let mut packets = Vec::new();

    let mut lines = input.split('\n').peekable();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }

        packets.push(Item::from(line));
    }

    packets
}

#[derive(Debug, Clone)]
pub enum Item {
    Digit(usize),
    List(Vec<Item>),
}

impl Item {
    pub fn is_marker(&self, number: usize) -> bool {
        match self {
            Item::Digit(_) => false,
            Item::List(outer_list) => {
                if outer_list.len() != 1 {
                    return false;
                }

                match outer_list.first().unwrap() {
                    Item::Digit(_) => false,
                    Item::List(inner_list) => {
                        if inner_list.len() == 1 {
                            match inner_list.first().unwrap() {
                                Item::Digit(digit) => return *digit == number,
                                Item::List(_) => return false,
                            }
                        }

                        false
                    }
                }
            }
        }
    }

    pub fn is_ordered(left: &Item, right: &Item) -> Ordering {
        match (left, right) {
            // Digits are trivially compared
            (Item::Digit(left), Item::Digit(right)) => {
                if left < right {
                    Ordering::Less
                } else if left == right {
                    Ordering::Equal
                } else {
                    Ordering::Greater
                }
            }

            // Right digit can be compared after promoting to list of single
            // element
            (Item::List(_), Item::Digit(right_digit)) => {
                let right_promoted = Item::promote(*right_digit);
                Item::is_ordered(left, &right_promoted)
            }

            // Same as above with right/left swapped
            (Item::Digit(left_digit), Item::List(_)) => {
                let left_promoted = Item::promote(*left_digit);
                Item::is_ordered(&left_promoted, right)
            }

            // Lists are ordered by comparing elements then list length
            (Item::List(left), Item::List(right)) => {
                let mut left_items = left.iter();
                let mut right_items = right.iter();

                while let Some(left_item) = left_items.next() {
                    if let Some(right_item) = right_items.next() {
                        match Item::is_ordered(left_item, right_item) {
                            Ordering::Equal => continue,
                            Ordering::Less => return Ordering::Less,
                            Ordering::Greater => return Ordering::Greater,
                        }
                    }

                    // If the right list runs out of elements before the left
                    // then these lists are not ordered
                    return Ordering::Greater;
                }

                if left.len() < right.len() {
                    return Ordering::Less;
                } else if left.len() == right.len() {
                    return Ordering::Equal;
                }

                return Ordering::Greater;
            }
        }
    }

    fn promote(digit: usize) -> Item {
        let digit = Item::Digit(digit);

        Item::List(vec![digit])
    }

    fn new(digit: usize) -> Self {
        Item::Digit(digit)
    }
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        let digit = c as usize - 48;

        Item::Digit(digit)
    }
}

impl From<&str> for Item {
    fn from(line: &str) -> Self {
        // Store nested items on a stack. When a nesting delimiter ('[' or ']')
        // is encountered, manipulate the stack.
        let mut previous: VecDeque<Item> = VecDeque::new();

        // The current list of items is pushed and popped off the stack to track
        // the level of nesting
        let mut current_list_of_items = Vec::new();

        // Parse character by character. The first will always be '['. Consume
        // and ignore this character since the current list has already been
        // setup.
        let mut characters = line.chars().peekable();
        let _ignore = characters.next();

        while let Some(character) = characters.next() {
            // Add digits to the current list. Since this problem allows [0-9]
            // and `10` there's a special case. It's ugly, but it works, and I'm
            // getting fed up with this problem.
            if character.is_ascii_digit() {
                match characters.next_if_eq(&'0') {
                    Some(_) => current_list_of_items.push(Item::new(10)),
                    None => current_list_of_items.push(Item::from(character)),
                }
            }
            // Start of nested list
            else if character == '[' {
                previous.push_back(Item::List(current_list_of_items));
                current_list_of_items = Vec::new();
            }
            // End of nested list
            else if character == ']' {
                match previous.pop_back() {
                    // When the stack of previous lists is empty there is no
                    // stack management required
                    None => {
                        continue;
                    }
                    Some(item) => match item {
                        Item::Digit(_) => {
                            panic!("popped digit off stack of lists")
                        }
                        Item::List(mut list) => {
                            let current =
                                Item::List(current_list_of_items.clone());

                            list.push(current);

                            current_list_of_items = list;
                        }
                    },
                }
            }
        }

        previous.extend(current_list_of_items);

        Item::List(previous.into())
    }
}
