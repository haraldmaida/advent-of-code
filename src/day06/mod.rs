//! # Day 6: Custom Customs
//!
//! As your flight approaches the regional airport where you'll switch to a much
//! larger plane, customs declaration forms are distributed to the passengers.
//!
//! The form asks a series of 26 yes-or-no questions marked a through z. All you
//! need to do is identify the questions for which anyone in your group answers
//! "yes". Since your group is just you, this doesn't take very long.
//!
//! However, the person sitting next to you seems to be experiencing a language
//! barrier and asks if you can help. For each of the people in their group, you
//! write down the questions for which they answer "yes", one per line. For
//! example:
//!
//! ```text
//! abcx
//! abcy
//! abcz
//! ```
//!
//! In this group, there are 6 questions to which anyone answered "yes":
//! a, b, c, x, y, and z. (Duplicate answers to the same question don't count
//! extra; each question counts at most once.)
//!
//! Another group asks for your help, then another, and eventually you've
//! collected answers from every group on the plane (your puzzle input). Each
//! group's answers are separated by a blank line, and within each group, each
//! person's answers are on a single line. For example:
//!
//! ```text
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//!
//! * The first group contains one person who answered "yes" to 3 questions:
//!   a, b, and c.
//! * The second group contains three people; combined, they answered "yes" to
//!   3 questions: a, b, and c.
//! * The third group contains two people; combined, they answered "yes" to
//!   3 questions: a, b, and c.
//! * The fourth group contains four people; combined, they answered "yes" to
//!   only 1 question, a.
//! * The last group contains one person who answered "yes" to only 1 question,
//!   b.
//!
//! In this example, the sum of these counts is `3 + 3 + 3 + 1 + 1 = 11`.
//!
//! For each group, count the number of questions to which anyone answered
//! "yes". What is the sum of those counts?
//!
//! ## Part 2
//!
//! As you finish the last group's customs declaration, you notice that you
//! misread one word in the instructions:
//!
//! You don't need to identify the questions to which anyone answered "yes"; you
//! need to identify the questions to which everyone answered "yes"!
//!
//! Using the same example as above:
//!
//! ```text
//! abc
//!
//! a
//! b
//! c
//!
//! ab
//! ac
//!
//! a
//! a
//! a
//! a
//!
//! b
//! ```
//!
//! This list represents answers from five groups:
//!
//! * In the first group, everyone (all 1 person) answered "yes" to 3 questions:
//!   a, b, and c.
//! * In the second group, there is no question to which everyone answered
//!   "yes".
//! * In the third group, everyone answered yes to only 1 question, a. Since
//!   some people did not answer "yes" to b or c, they don't count.
//! * In the fourth group, everyone answered yes to only 1 question, a.
//! * In the fifth group, everyone (all 1 person) answered "yes" to 1 question,
//!   b.
//!
//! In this example, the sum of these counts is `3 + 0 + 1 + 1 + 1 = 6`.
//!
//! For each group, count the number of questions to which everyone answered
//! "yes". What is the sum of those counts?
//!
//! [Advent of Code 2020 - Day 6](https://adventofcode.com/2020/day/6)

use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::mem;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Compliance(char);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Group {
    compliance_list: Vec<Vec<Compliance>>,
}

#[aoc_generator(day6)]
pub fn parse_compliance_list(input: &str) -> Vec<Group> {
    let mut groups = Vec::new();
    let mut compliance_list = Vec::new();
    let mut answers = Vec::new();
    for c in input.chars().chain("\n\n".chars()) {
        match c {
            '\n' => {
                if answers.is_empty() && !compliance_list.is_empty() {
                    groups.push(Group {
                        compliance_list: mem::replace(&mut compliance_list, Vec::new()),
                    });
                } else {
                    compliance_list.push(mem::replace(&mut answers, Vec::new()));
                }
            }
            _ if c.is_whitespace() => {}
            _ => answers.push(Compliance(c)),
        }
    }
    groups
}

fn distinct_answers(compliance_list: &[Vec<Compliance>]) -> HashSet<Compliance> {
    compliance_list
        .iter()
        .fold(HashSet::new(), |mut distinct, answers| {
            answers.iter().for_each(|answer| {
                distinct.insert(*answer);
            });
            distinct
        })
}

fn common_answers(compliance_list: &[Vec<Compliance>]) -> HashSet<Compliance> {
    let num_members = compliance_list.len();
    HashSet::from_iter(
        compliance_list
            .iter()
            .fold(HashMap::new(), |mut common, answers| {
                answers.iter().for_each(|answer| {
                    *common.entry(*answer).or_insert(0) += 1;
                });
                common
            })
            .into_iter()
            .filter_map(|(compliance, count)| {
                if count == num_members {
                    Some(compliance)
                } else {
                    None
                }
            }),
    )
}

#[aoc(day6, part1)]
pub fn sum_of_anyone_is_compliant(group_list: &[Group]) -> usize {
    group_list
        .iter()
        .map(|group| distinct_answers(&group.compliance_list).len())
        .sum()
}

#[aoc(day6, part2)]
pub fn sum_of_everyone_is_compliant(group_list: &[Group]) -> usize {
    group_list
        .iter()
        .map(|group| common_answers(&group.compliance_list).len())
        .sum()
}

#[cfg(test)]
mod tests;
