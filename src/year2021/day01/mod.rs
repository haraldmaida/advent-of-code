//! # Day 1: Sonar Sweep
//!
//! As the submarine drops below the surface of the ocean, it automatically
//! performs a sonar sweep of the nearby sea floor. On a small screen, the sonar
//! sweep report (your puzzle input) appears: each line is a measurement of the
//! sea floor depth as the sweep looks further and further away from the
//! submarine.
//!
//! For example, suppose you had the following report:
//!
//! ```text
//! 199
//! 200
//! 208
//! 210
//! 200
//! 207
//! 240
//! 269
//! 260
//! 263
//! ```
//!
//! This report indicates that, scanning outward from the submarine, the sonar
//! sweep found depths of 199, 200, 208, 210, and so on.
//!
//! The first order of business is to figure out how quickly the depth
//! increases, just so you know what you're dealing with - you never know if the
//! keys will get carried into deeper water by an ocean current or a fish or
//! something.
//!
//! To do this, count the number of times a depth measurement increases from the
//! previous measurement. (There is no measurement before the first
//! measurement.) In the example above, the changes are as follows:
//!
//! ```text
//! 199 (N/A - no previous measurement)
//! 200 (increased)
//! 208 (increased)
//! 210 (increased)
//! 200 (decreased)
//! 207 (increased)
//! 240 (increased)
//! 269 (increased)
//! 260 (decreased)
//! 263 (increased)
//! ```
//!
//! In this example, there are 7 measurements that are larger than the previous
//! measurement.
//!
//! How many measurements are larger than the previous measurement?
//!
//! ## Part Two
//!
//! Considering every single measurement isn't as useful as you expected:
//! there's just too much noise in the data.
//!
//! Instead, consider sums of a three-measurement sliding window. Again
//! considering the above example:
//!
//! ```text
//! 199  A
//! 200  A B
//! 208  A B C
//! 210    B C D
//! 200  E   C D
//! 207  E F   D
//! 240  E F G
//! 269    F G H
//! 260      G H
//! 263        H
//! ```
//!
//! Start by comparing the first and second three-measurement windows. The
//! measurements in the first window are marked A (199, 200, 208); their sum is
//! 199 + 200 + 208 = 607. The second window is marked B (200, 208, 210); its
//! sum is 618. The sum of measurements in the second window is larger than the
//! sum of the first, so this first comparison increased.
//!
//! Your goal now is to count the number of times the sum of measurements in
//! this sliding window increases from the previous sum. So, compare A with B,
//! then compare B with C, then C with D, and so on. Stop when there aren't
//! enough measurements left to create a new three-measurement sum.
//!
//! In the above example, the sum of each three-measurement window is as
//! follows:
//!
//! ```text
//! A: 607 (N/A - no previous sum)
//! B: 618 (increased)
//! C: 618 (no change)
//! D: 617 (decreased)
//! E: 647 (increased)
//! F: 716 (increased)
//! G: 769 (increased)
//! H: 792 (increased)
//! ```
//!
//! In this example, there are 5 sums that are larger than the previous sum.
//!
//! Consider sums of a three-measurement sliding window. How many sums are
//! larger than the previous sum?
//!
//! [Advent of Code 2021 - Day 1](https://adventofcode.com/2021/day/1)

use std::ops::{Add, Sub};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueChange {
    Increased,
    Decreased,
    NoChange,
}

pub fn diff_between_elements_in_sequence<T>(sonar_report: &[T]) -> Vec<T>
where
    T: Sub<Output = T> + Copy,
{
    sonar_report
        .iter()
        .copied()
        .zip(sonar_report.iter().skip(1).copied())
        .map(|(e1, e2)| e2 - e1)
        .collect()
}

#[aoc(day1, part1)]
pub fn count_increased_depth(sonar_report: &[i32]) -> usize {
    diff_between_elements_in_sequence(sonar_report)
        .iter()
        .copied()
        .map(|diff| match diff.signum() {
            1 => ValueChange::Increased,
            -1 => ValueChange::Decreased,
            0 => ValueChange::NoChange,
            _ => panic!("signum returns unexpected value {}", diff.signum()),
        })
        .filter(|change| *change == ValueChange::Increased)
        .count()
}

pub fn sum_of_three_measurements<T>(sonar_report: &[T]) -> Vec<T>
where
    T: Add<Output = T> + Copy,
{
    sonar_report
        .iter()
        .copied()
        .zip(sonar_report.iter().skip(1).copied())
        .zip(sonar_report.iter().skip(2).copied())
        .map(|((e1, e2), e3)| e1 + e2 + e3)
        .collect()
}

#[aoc(day1, part2)]
pub fn count_increased_sums_of_three_measurements(sonar_report: &[i32]) -> usize {
    let sum_of_three_measurements = sum_of_three_measurements(sonar_report);
    sum_of_three_measurements
        .iter()
        .copied()
        .zip(sum_of_three_measurements.iter().skip(1).copied())
        .map(|(s1, s2)| match (s2 - s1).signum() {
            1 => ValueChange::Increased,
            -1 => ValueChange::Decreased,
            0 => ValueChange::NoChange,
            _ => panic!("signum returns unexpected value {}", (s2 - s1).signum()),
        })
        .filter(|change| *change == ValueChange::Increased)
        .count()
}

#[cfg(test)]
mod tests;
