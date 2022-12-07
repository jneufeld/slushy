use crate::Solution;

use lazy_static::lazy_static;
use regex::Regex;

pub fn solve() -> Option<Solution> {
    find_solution(REAL_INPUT)
}

#[derive(Debug)]
struct Entry {
    name: String,
    kind: Kind,
    size: Option<usize>,
    level: usize,
}

impl Entry {
    fn directory(name: String, level: usize) -> Entry {
        Entry {
            name: name,
            kind: Kind::Dir,
            size: None,
            level: level,
        }
    }

    fn file(name: String, level: usize) -> Entry {
        Entry {
            name: name,
            kind: Kind::Dir,
            size: None,
            level: level,
        }
    }
}

#[derive(Debug)]
enum Kind {
    Dir,
    File,
}

enum State {
    Start,
    Name,
    Type,
    Size,
    Accept,
}

lazy_static! {
    static ref DIR_REGEX: Regex = Regex::new(
        r"(:x)
        -\s         # marker
        (\S+)\s     # name
        \(dir\)     # dir
    "
    )
    .unwrap();
    static ref FILE_REGEX: Regex = Regex::new(
        r"(:x)
        -\s                     # marker
        (\S+)\s                 # name
        \(file, size=(\d+)\)    # file size
    "
    )
    .unwrap();
}

fn find_solution(input: &str) -> Option<Solution> {
    for line in input.split('\n') {
        let directory_captures = DIR_REGEX.captures(line).unwrap();

        let name = directory_captures.get(1).unwrap();
        let name = String::from(name.as_str());
        let level = indent_level(line);

        let entry = Entry::directory(name, level);

        println!("entry: {:?}", entry);
    }

    None
}

fn indent_level(line: &str) -> usize {
    let mut spaces = 0;

    for character in line.chars() {
        if character != ' ' {
            break;
        }

        spaces += 1;
    }

    spaces / 2
}

#[cfg(test)]
mod test {
    use crate::day7::find_solution;

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

const REAL_INPUT: &str = r"- / (dir)
  - a (dir)
    - e (dir)
      - i (file, size=584)
    - f (file, size=29116)
    - g (file, size=2557)
    - h.lst (file, size=62596)
  - b.txt (file, size=14848514)
  - c.dat (file, size=8504156)
  - d (dir)
    - j (file, size=4060174)
    - d.log (file, size=8033020)
    - d.ext (file, size=5626152)
    - k (file, size=7214296)";
