//! # Day 2: Corruption Checksum
//!
//! ## Part 1
//!
//! As you walk through the door, a glowing humanoid shape yells in your
//! direction. "You there! Your state appears to be idle. Come help us repair
//! the corruption in this spreadsheet - if we take another millisecond, we'll
//! have to display an hourglass cursor!"
//!
//! The spreadsheet consists of rows of apparently-random numbers. To make sure
//! the recovery process is on the right track, they need you to calculate the
//! spreadsheet's checksum. For each row, determine the difference between the
//! largest value and the smallest value; the checksum is the sum of all of
//! these differences.
//!
//! For example, given the following spreadsheet:
//!
//! ```text
//! | 5 | 1 | 9 | 5 |
//! | 7 | 5 | 3 |   |
//! | 2 | 4 | 6 | 8 |
//! ```
//!
//! * The first row's largest and smallest values are 9 and 1, and their
//!   difference is 8.
//! * The second row's largest and smallest values are 7 and 3, and their
//!   difference is 4.
//! * The third row's difference is 6.
//!
//! In this example, the spreadsheet's checksum would be 8 + 4 + 6 = 18.
//!
//! What is the checksum for the spreadsheet in your puzzle input?
//!
//! ## Part 2
//!
//! "Based on what we're seeing, it looks like all the User wanted is some
//! information about the evenly divisible values in the spreadsheet.
//! Unfortunately, none of us are equipped for that kind of calculation - most
//! of us specialize in bitwise operations."
//!
//! It sounds like the goal is to find the only two numbers in each row where
//! one evenly divides the other - that is, where the result of the division
//! operation is a whole number. They would like you to find those numbers on
//! each line, divide them, and add up each line's result.
//!
//! For example, given the following spreadsheet:
//!
//! ```text
//! 5 9 2 8
//! 9 4 7 3
//! 3 8 6 5
//! ```
//!
//! * In the first row, the only two numbers that evenly divide are 8 and 2;
//!   the result of this division is 4.
//! * In the second row, the two numbers are 9 and 3; the result is 3.
//! * In the third row, the result is 2.
//! * In this example, the sum of the results would be 4 + 3 + 2 = 9.
//!
//! What is the sum of each row's result in your puzzle input?
//!
//! [Advent of Code 2017 - Day 2](https://adventofcode.com/2017/day/2)

pub type Cell = i32;

pub struct Spreadsheet {
    cells: Vec<Vec<Cell>>,
}

impl AsRef<Self> for Spreadsheet {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl Spreadsheet {
    pub fn new(cells: Vec<Vec<Cell>>) -> Self {
        Self { cells }
    }
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Spreadsheet {
    Spreadsheet {
        cells: input
            .trim()
            .lines()
            .map(|line| {
                line.split("\t")
                    .map(|c| c.trim().parse().unwrap())
                    .collect()
            })
            .collect(),
    }
}

#[aoc(day2, part1)]
pub fn checksum(spreadsheet: &Spreadsheet) -> i32 {
    spreadsheet
        .cells
        .iter()
        .map(|row| row.iter().max().unwrap_or(&0) - row.iter().min().unwrap_or(&0))
        .sum()
}

#[aoc(day2, part2)]
pub fn checksum2(spreadsheet: &Spreadsheet) -> i32 {
    let mut row_values = Vec::with_capacity(spreadsheet.cells.len());
    for row in &spreadsheet.cells {
        'row: for (idx, cell1) in row.iter().enumerate() {
            for cell2 in row.iter().skip(idx + 1) {
                let value1 = cell1.max(cell2);
                let value2 = cell1.min(cell2);
                if value1 % value2 == 0 {
                    row_values.push(value1 / value2);
                    break 'row;
                }
            }
        }
    }
    row_values.into_iter().sum()
}

#[cfg(test)]
mod tests;
