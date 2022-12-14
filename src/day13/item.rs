pub fn parse_packets(input: &str) -> Vec<(Item, Item)> {
    let mut pairs = Vec::new();

    let mut lines = input.split('\n').peekable();

    while let Some(_) = lines.peek() {
        let maybe_first = lines.next().unwrap();

        if maybe_first.is_empty() {
            continue;
        }

        let first = parse_list(maybe_first);
        let second = parse_list(lines.next().unwrap());

        pairs.push((first, second));
    }

    pairs
}

fn parse_list(text: &str) -> Item {
    let mut all: Item = Item::List(Vec::new());

    let mut parent: Option<Item> = None;
    let mut current: Option<Item> = None;

    while let Some(c) = text.chars().next() {
        match c {
            ',' => continue,
            '[' => match current {
                None => current = Some(all),
                Some(list) => match list {
                    Item::Digit(_) => todo!(),
                    Item::List(list) => {
                    }
                }
                    current = Some(Item::List(Vec::new()));
                }
            },
            ']' => {
                match current {
                    None => panic!("ended list parsing without current list"),
                    Some(item) => match item {
                        Item::Digit(_) => todo!(),
                        Item::List(list) => {
                            current = parent;
                            // TODO pop off stack properly
                        }
                    },
                }
            }
            _ => match current {
                None => panic!("parsing number without list to put it in"),
                Some(item) => match item {
                    Item::Digit(_) => todo!(),
                    Item::List(list) => list.push(Item::from(c)),
                },
            },
        }
    }

    all
}

#[derive(Debug, Clone)]
pub enum Item {
    Digit(usize),
    List(Vec<Item>),
}

#[derive(Debug, Clone, Copy)]
pub enum Smaller {
    Equal,
    Left,
    Right,
}

impl Item {
    pub fn is_ordered(left: &Item, right: &Item) -> Smaller {
        match (left, right) {
            // Digits are trivially compared
            (Item::Digit(left), Item::Digit(right)) => {
                if left < right {
                    Smaller::Left
                } else if left == right {
                    Smaller::Equal
                } else {
                    Smaller::Right
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
                            Smaller::Equal => continue,
                            Smaller::Left => return Smaller::Left,
                            Smaller::Right => return Smaller::Right,
                        }
                    }

                    // If the right list runs out of elements before the left
                    // then these lists are not ordered
                    return Smaller::Right;
                }

                if left.len() < right.len() {
                    return Smaller::Left;
                } else if left.len() == right.len() {
                    return Smaller::Equal;
                }

                return Smaller::Right;
            }
        }
    }

    fn promote(digit: usize) -> Item {
        let digit = Item::Digit(digit);

        Item::List(vec![digit])
    }
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        let digit = c as usize - 48;

        Item::Digit(digit)
    }
}
