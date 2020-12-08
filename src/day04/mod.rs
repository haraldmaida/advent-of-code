//! # Day 4: Secure Container
//!
//! You arrive at the Venus fuel depot only to discover it's protected by a
//! password. The Elves had written the password on a sticky note, but someone
//! threw it out.
//!
//! However, they do remember a few key facts about the password:
//!
//! * It is a six-digit number.
//! * The value is within the range given in your puzzle input.
//! * Two adjacent digits are the same (like 22 in 122345).
//! * Going from left to right, the digits never decrease; they only ever
//!   increase or stay the same (like 111123 or 135679).
//!
//! Other than the range rule, the following are true:
//!
//! * 111111 meets these criteria (double 11, never decreases).
//! * 223450 does not meet these criteria (decreasing pair of digits 50).
//! * 123789 does not meet these criteria (no double).
//!
//! How many different passwords within the range given in your puzzle input
//! meet these criteria?
//!
//! [Advent of Code 2019 - Day 4](https://adventofcode.com/2019/day/4)

use std::iter::FromIterator;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[aoc_generator(day4)]
pub fn parse_range(input: &str) -> RangeInclusive<u32> {
    let mut parts = input.trim().split('-');
    let min_part = parts
        .next()
        .unwrap_or_else(|| panic!("no range given: {:?}", input));
    let max_part = parts
        .next()
        .unwrap_or_else(|| panic!("no range given: {:?}", input));
    let min = u32::from_str(min_part)
        .unwrap_or_else(|err| panic!("minimum is not a number: {:?}; reason: {:?}", min_part, err));
    let max = u32::from_str(max_part)
        .unwrap_or_else(|err| panic!("maximum is not a number: {:?}; reason: {:?}", min_part, err));
    min..=max
}

#[derive(Debug)]
pub struct DigitCodeGenerator {
    length: usize,
    upper_bound: Vec<char>,
    first_yield: bool,
    current: Vec<char>,
}

impl DigitCodeGenerator {
    pub fn new(lower_bound: impl Into<String>, upper_bound: impl Into<String>) -> Self {
        let mut upper_bound: Vec<char> = upper_bound.into().chars().collect();
        let length = upper_bound.len();
        let mut lower_bound: Vec<char> = lower_bound.into().chars().collect();
        for i in 0..lower_bound.len() - 1 {
            let c1 = lower_bound[i];
            let c2 = lower_bound[i + 1];
            if c1 > c2 {
                lower_bound[i + 1] = c1;
            }
        }
        for i in 0..upper_bound.len() - 1 {
            let c1 = upper_bound[i];
            let c2 = upper_bound[i + 1];
            if c1 > c2 {
                upper_bound[i] = next_lower_digit(c1).0;
                for j in i + 1..upper_bound.len() {
                    upper_bound[j] = '9'
                }
                break;
            }
        }
        Self {
            length,
            upper_bound,
            first_yield: true,
            current: lower_bound,
        }
    }
}

impl From<RangeInclusive<u32>> for DigitCodeGenerator {
    fn from(range: RangeInclusive<u32>) -> Self {
        Self::new(range.start().to_string(), range.end().to_string())
    }
}

impl Iterator for DigitCodeGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.upper_bound || self.current > self.upper_bound {
            return None;
        }
        if self.first_yield && has_two_same_adjacent_digits(&self.current) {
            self.first_yield = false;
            return Some(String::from_iter(self.current.iter()));
        }
        loop {
            for i in (0..self.current.len()).rev() {
                let (d2, carry) = next_higher_digit(self.current[i]);
                self.current[i] = d2;
                if !carry {
                    for j in i..self.current.len() {
                        self.current[j] = d2;
                    }
                    break;
                }
            }
            if has_two_same_adjacent_digits(&self.current) {
                break;
            }
        }
        Some(String::from_iter(self.current.iter()))
    }
}

fn next_lower_digit(digit: char) -> (char, bool) {
    match digit {
        '0' => ('9', true),
        '1' => ('0', false),
        '2' => ('1', false),
        '3' => ('2', false),
        '4' => ('3', false),
        '5' => ('4', false),
        '6' => ('5', false),
        '7' => ('6', false),
        '8' => ('7', false),
        '9' => ('8', false),
        _ => panic!("not a digit: {:?}", digit),
    }
}

fn next_higher_digit(digit: char) -> (char, bool) {
    match digit {
        '0' => ('1', false),
        '1' => ('2', false),
        '2' => ('3', false),
        '3' => ('4', false),
        '4' => ('5', false),
        '5' => ('6', false),
        '6' => ('7', false),
        '7' => ('8', false),
        '8' => ('9', false),
        '9' => ('0', true),
        _ => panic!("not a digit: {:?}", digit),
    }
}

fn has_two_same_adjacent_digits(digits: &[char]) -> bool {
    digits
        .iter()
        .zip(digits.iter().skip(1))
        .any(|(c1, c2)| c1 == c2)
}

#[aoc(day4, part1)]
pub fn number_of_possible_passwords_in_range(range: &RangeInclusive<u32>) -> usize {
    let digit_code_generator = DigitCodeGenerator::from(range.clone());

    digit_code_generator.count()
}

#[cfg(test)]
mod tests;
