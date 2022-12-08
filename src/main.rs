mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

#[derive(Debug)]
pub struct Solution {
    value: usize,
}

impl Solution {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
}

fn main() {
    day7::solve();
}
