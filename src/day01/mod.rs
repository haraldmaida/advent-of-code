//! # Day 1: Report Repair
//!
//! Before you leave, the Elves in accounting just need you to fix your expense
//! report (your puzzle input); apparently, something isn't quite adding up.
//!
//! Specifically, they need you to find the two entries that sum to 2020 and
//! then multiply those two numbers together.
//!
//! For example, suppose your expense report contained the following:
//!
//! ```text
//! 1721
//! 979
//! 366
//! 299
//! 675
//! 1456
//! ```
//!
//! In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying
//! them together produces 1721 * 299 = 514579, so the correct answer is 514579.
//!
//! Of course, your expense report is much larger. Find the two entries that sum
//! to 2020; what do you get if you multiply them together?
//!
//! ## Part 2
//!
//! The Elves in accounting are thankful for your help; one of them even offers
//! you a starfish coin they had left over from a past vacation. They offer you
//! a second one if you can find three numbers in your expense report that meet
//! the same criteria.
//!
//! Using the above example again, the three entries that sum to 2020 are 979,
//! 366, and 675. Multiplying them together produces the answer, 241861950.
//!
//! In your expense report, what is the product of the three entries that sum
//! to 2020?
//!
//! [Advent of Code 2020 - Day 1](https://adventofcode.com/2020/day/1)

use std::str::FromStr;

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|entry| {
            i32::from_str(entry.trim()).unwrap_or_else(|err| {
                panic!(
                    "input str is not a valid i32: {:?}, reason: {:?}",
                    entry, err
                )
            })
        })
        .collect()
}

pub fn find_two_incorrect_expenses(expense_report: &[i32]) -> (i32, i32) {
    expense_report
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i, e1)| {
            let needed = 2020 - e1;
            expense_report.iter().copied().skip(i + 1).find_map(|e2| {
                if e2 == needed {
                    Some((e1, e2))
                } else {
                    None
                }
            })
        })
        .next()
        .expect("no result found!")
}

pub fn find_three_incorrect_expenses(expense_report: &[i32]) -> (i32, i32, i32) {
    expense_report
        .iter()
        .copied()
        .enumerate()
        .flat_map(|(i1, e1)| {
            let needed1 = 2020 - e1;
            expense_report
                .iter()
                .copied()
                .enumerate()
                .skip(i1 + 1)
                .flat_map(move |(i2, e2)| {
                    let needed2 = needed1 - e2;
                    expense_report.iter().copied().skip(i2 + 1).find_map(|e3| {
                        if e3 == needed2 {
                            Some((e1, e2, e3))
                        } else {
                            None
                        }
                    })
                })
        })
        .next()
        .expect("no result found!")
}

#[aoc(day1, part1)]
pub fn product_of_two_incorrect_expenses(expense_report: &[i32]) -> i32 {
    let (e1, e2) = find_two_incorrect_expenses(expense_report);
    e1 * e2
}

#[aoc(day1, part2)]
pub fn product_of_three_incorrect_expenses(expense_report: &[i32]) -> i32 {
    let (e1, e2, e3) = find_three_incorrect_expenses(expense_report);
    e1 * e2 * e3
}

#[cfg(test)]
mod tests;
