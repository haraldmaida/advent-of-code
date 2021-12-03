//! # Day 2: Password Philosophy
//!
//! Your flight departs in a few days from the coastal airport; the easiest way
//! down to the coast from here is via toboggan.
//!
//! The shopkeeper at the North Pole Toboggan Rental Shop is having a bad day.
//! "Something's wrong with our computers; we can't log in!" You ask if you can
//! take a look.
//!
//! Their password database seems to be a little corrupted: some of the
//! passwords wouldn't have been allowed by the Official Toboggan Corporate
//! Policy that was in effect when they were chosen.
//!
//! To try to debug the problem, they have created a list (your puzzle input)
//! of passwords (according to the corrupted database) and the corporate policy
//! when that password was set.
//!
//! For example, suppose you have the following list:
//!
//! ```text
//! 1-3 a: abcde
//! 1-3 b: cdefg
//! 2-9 c: ccccccccc
//! ```
//!
//! Each line gives the password policy and then the password. The password
//! policy indicates the lowest and highest number of times a given letter must
//! appear for the password to be valid. For example, 1-3 a means that the
//! password must contain a at least 1 time and at most 3 times.
//!
//! In the above example, 2 passwords are valid. The middle password, cdefg, is
//! not; it contains no instances of b, but needs at least 1. The first and
//! third passwords are valid: they contain one a or nine c, both within the
//! limits of their respective policies.
//!
//! How many passwords are valid according to their policies?
//!
//! ## Part 2
//!
//! While it appears you validated the passwords correctly, they don't seem to
//! be what the Official Toboggan Corporate Authentication System is expecting.
//!
//! The shopkeeper suddenly realizes that he just accidentally explained the
//! password policy rules from his old job at the sled rental place down the
//! street! The Official Toboggan Corporate Policy actually works a little
//! differently.
//!
//! Each policy actually describes two positions in the password, where 1 means
//! the first character, 2 means the second character, and so on. (Be careful;
//! Toboggan Corporate Policies have no concept of "index zero"!) Exactly one
//! of these positions must contain the given letter. Other occurrences of the
//! letter are irrelevant for the purposes of policy enforcement.
//!
//! Given the same example list from above:
//!
//! * 1-3 a: abcde is valid: position 1 contains a and position 3 does not.
//! * 1-3 b: cdefg is invalid: neither position 1 nor position 3 contains b.
//! * 2-9 c: ccccccccc is invalid: both position 2 and position 9 contain c.
//!
//! How many passwords are valid according to the new interpretation of the
//! policies?
//!
//! [Advent of Code 2020 - Day 2](https://adventofcode.com/2020/day/2)

use std::cmp::Ordering;
use std::iter::FromIterator;
use std::mem;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Policy {
    min: usize,
    max: usize,
    character: char,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parts {
    Min,
    Max,
    Character,
    Password,
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<(Policy, String)> {
    let mut policies = Vec::new();
    let mut passwords = Vec::new();

    let mut min = String::new();
    let mut max = String::new();
    let mut password = String::new();

    let mut part = Parts::Min;
    let mut characters = input.chars().chain("\n".chars());
    while let Some(c) = characters.next() {
        match c {
            '-' => part = Parts::Max,
            ':' => part = Parts::Password,
            '\n' => {
                passwords.push(mem::replace(&mut password, String::new()));
                part = Parts::Min;
            }
            _ => match part {
                Parts::Min => {
                    if c.is_ascii_digit() {
                        min.push(c)
                    }
                }
                Parts::Max => {
                    if c.is_ascii_digit() {
                        max.push(c)
                    } else if c.is_whitespace() {
                        part = Parts::Character;
                    }
                }
                Parts::Character => {
                    if !c.is_whitespace() {
                        policies.push(Policy {
                            min: usize::from_str(&min).unwrap_or_else(|err| {
                                panic!("not a valid min value {:?}, reason: {:?}", min, err)
                            }),
                            max: usize::from_str(&max).unwrap_or_else(|err| {
                                panic!("no a valid max value {:?}, reason: {:?}", max, err)
                            }),
                            character: c,
                        });
                        min.clear();
                        max.clear();
                        part = Parts::Password;
                    }
                }
                Parts::Password => {
                    if !c.is_whitespace() {
                        password.push(c);
                    }
                }
            },
        }
    }
    Vec::from_iter(policies.into_iter().zip(passwords.into_iter()))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyViolation {
    ToViewChars(char, usize),
    ToManyChars(char, usize),
}

fn verify_policy(policy: Policy, password: &str) -> Result<(), PolicyViolation> {
    let count = password.chars().filter(|c| *c == policy.character).count();
    if count < policy.min {
        Err(PolicyViolation::ToViewChars(policy.character, count))
    } else if count > policy.max {
        Err(PolicyViolation::ToManyChars(policy.character, count))
    } else {
        Ok(())
    }
}

#[aoc(day2, part1)]
pub fn count_valid_passwords(input: &[(Policy, String)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| verify_policy(*policy, password).is_ok())
        .count()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyV2Violation {
    NoMatch,
    MultipleMatches,
}

fn verify_new_policy(policy: Policy, password: &str) -> Result<(), PolicyV2Violation> {
    let count = password
        .chars()
        .enumerate()
        .filter(|(i, c)| *c == policy.character && (*i == policy.min - 1 || *i == policy.max - 1))
        .count();
    match 1.cmp(&count) {
        Ordering::Equal => Ok(()),
        Ordering::Less => Err(PolicyV2Violation::NoMatch),
        Ordering::Greater => Err(PolicyV2Violation::MultipleMatches),
    }
}

#[aoc(day2, part2)]
pub fn count_valid_passwords_part2(input: &[(Policy, String)]) -> usize {
    input
        .iter()
        .filter(|(policy, password)| verify_new_policy(*policy, password).is_ok())
        .count()
}

#[cfg(test)]
mod tests;
