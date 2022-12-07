use crate::Solution;

pub fn solve() -> Option<Solution> {
    find_solution(REAL_INPUT)
}

fn find_solution(input: &str) -> Option<Solution> {
    for character in input.chars() {}

    None
}

#[cfg(test)]
mod test {
    #[test]
    fn simple() {
        let inputs = vec![(r"", 0), (r"", 0)];

        for (input, expected) in inputs {
            let solution = find_solution(input);
            let solution = solution.unwrap();
            let solution = solution.value;

            assert_eq!(solution, expected);
        }
    }
}

const REAL_INPUT: &str = r"";
