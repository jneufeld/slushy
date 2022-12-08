use crate::Solution;

use lazy_static::lazy_static;
use regex::Regex;

pub fn solve() -> Option<Solution> {
    match find_solution(TEST_INPUT) {
        Ok(solution) => Some(solution),
        Err(why) => {
            eprintln!("ERROR: {:?}", why);
            None
        }
    }
}

#[derive(Debug, Clone)]
struct FileSystemEntry {
    kind: Kind,
    name: String,
    size: usize,
    dirs: Vec<FileSystemEntry>,
    files: Vec<FileSystemEntry>,
}

impl FileSystemEntry {
    fn new(kind: Kind, name: String, size: usize) -> FileSystemEntry {
        let dirs = Vec::new();
        let files = Vec::new();

        FileSystemEntry {
            kind,
            name,
            size,
            dirs,
            files,
        }
    }
}

impl Default for FileSystemEntry {
    fn default() -> Self {
        FileSystemEntry::new(Kind::Dir, String::from("DEFAULT"), 0)
    }
}

#[derive(Debug, Copy, Clone)]
enum Kind {
    Dir,
    File,
}

fn find_solution(input: &str) -> anyhow::Result<Solution> {
    let root = parse_directory(input);
    let solution = Solution::new(root.size);

    Ok(solution)
}

fn parse_directory(input: &str) -> FileSystemEntry {
    // The problem now is splitting at the correct point: the first line
    // contains the current directory's name so must be included. Parsing isn't
    // complete for an arbitrary number of lines. It ends with a similar marker
    // as the first: `$ cd`.
    //
    // In other words, I want to split at the SECOND instance of `$ cd`.

    // Split the input at the next `cd` command, i.e. where another directory
    // will be parsed.
    let index = find_split_index(input);
    let (needed, remaining) = input.split_at(index);

    println!("needed:\n'{}'\nremaining:\n'{}'\n", needed, remaining);

    // Parse the file contents of this directory (i.e. one-level of depth and
    // only files.
    let mut current_directory = parse_files(needed);

    loop {
        // Break when sub-directory parsing is complete.
        if remaining.is_empty() || remaining.starts_with("$ cd ..") {
            break;
        }

        // Recursively parse sub-directories.
        let sub_directory = parse_directory(remaining);
        current_directory.dirs.push(sub_directory);
    }

    // Update size of files and sub-directories.
    for file in &current_directory.files {
        current_directory.size += file.size;
    }

    for dir in &current_directory.dirs {
        current_directory.size += dir.size;
    }

    current_directory
}

/// Starting at `$ cd <name>` find the index of the next `$ cd` pattern
fn find_split_index(input: &str) -> usize {
    let mut index = 0;

    for i in 0..input.len() {
        let c = input[i];
    }

    index
}

fn parse_files(input: &str) -> FileSystemEntry {
    let mut lines = input.split('\n');

    let name_line = lines.next().unwrap();
    let name = parse_dir_name(name_line);

    let mut directory = FileSystemEntry::new(Kind::Dir, name, 0);

    let _ignore_ls_command = lines.next();

    for line in lines {
        if line.starts_with("$ cd") {
            break;
        }

        if line.starts_with("dir") {
            continue;
        }

        let file = parse_file(line);

        directory.files.push(file);
    }

    directory
}

lazy_static! {
    static ref DIR_NAME_REGEX: Regex = Regex::new(r"cd (.+)$").unwrap();
    static ref FILE_REGEX: Regex = Regex::new(r"(\d+) (.+)$").unwrap();
}

/// E.g. `$ cd /` or `$ cd e`
fn parse_dir_name(line: &str) -> String {
    let captures = DIR_NAME_REGEX.captures(line);

    match captures {
        Some(capture) => {
            let name = capture.get(1).unwrap();
            let name = name.as_str();

            let result = String::from(name);

            println!("parsed name: '{}'\nfrom: '{}'\n", result, line);

            result
        }
        None => panic!("unable to find name from '{}'", line),
    }
}

/// E.g. `14848514 b.txt` or `4060174 j`
fn parse_file(line: &str) -> FileSystemEntry {
    let captures = FILE_REGEX.captures(line);

    match captures {
        Some(capture) => {
            let size = capture.get(1).unwrap();
            let size = size.as_str().parse::<usize>().unwrap();

            let name = capture.get(2).unwrap();
            let name = String::from(name.as_str());

            let result = FileSystemEntry::new(Kind::File, name, size);

            println!("parsed file: {:?}\nfrom: {}\n", result, line);

            result
        }
        None => panic!("unable to find size and name from {}", line),
    }
}

const TEST_INPUT: &str = r"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

const REAL_INPUT: &str = r"";
