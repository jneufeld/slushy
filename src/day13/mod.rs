pub fn solve() {
    let input = SAMPLE;

    for (left, right) in parse_pairs(input) {
        println!("left:  {}\nright: {}", left, right);
        println!("ordered: {}", is_ordered(left, right));
    }
}

fn is_ordered(left: String, right: String) -> bool {
    let mut left = left.chars().peekable();
    let mut right = right.chars().peekable();

    while let Some(_) = left.peek() {
        match (left.next(), right.next()) {
            (None, None) => return true,
            (None, Some(_)) => return true,
            (Some(_), None) => return false,
            (Some(left_char), Some(right_char)) => {
                if left_char == right_char {
                    continue;
                }

                // TODO fake promotion when one is a digit but the other is a
                // bracket by consuming the bracket. This may have undesired
                // consequences later.
                if left_char.is_ascii_digit() && right_char == '[' {
                    let _ignore = right.next();
                    continue;
                }

                if right_char.is_ascii_digit() && left_char == '[' {
                    let _ignore = left.next();
                    continue;
                }

                if left_char.is_ascii_digit() && right_char == '[' {
                    let _ignore = right.next();
                    continue;
                }

                if right_char.is_ascii_digit() && left_char == '[' {
                    let _ignore = left.next();
                    continue;
                }

                // When the characters are both digits but not equal they are
                // compared as integers
                if left_char.is_ascii_digit() && right_char.is_ascii_digit() {
                    if left_char > right_char {
                        return false;
                    }
                }
            }
        }
    }

    true
}

fn parse_pairs(input: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();

    let mut lines = input.split('\n').peekable();

    while let Some(_) = lines.peek() {
        let maybe_first = lines.next().unwrap();

        if maybe_first.is_empty() {
            continue;
        }

        let first = maybe_first.to_string();
        let second = lines.next().unwrap().to_string();

        pairs.push((first, second));
    }

    pairs
}

const SAMPLE: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]";
