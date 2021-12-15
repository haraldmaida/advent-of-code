//! # Day 14: Extended Polymerization
//!
//! The incredible pressures at this depth are starting to put a strain on your
//! submarine. The submarine has polymerization equipment that would produce
//! suitable materials to reinforce the submarine, and the nearby
//! volcanically-active caves should even have the necessary input elements in
//! sufficient quantities.
//!
//! The submarine manual contains instructions for finding the optimal polymer
//! formula; specifically, it offers a polymer template and a list of pair
//! insertion rules (your puzzle input). You just need to work out what polymer
//! would result after repeating the pair insertion process a few times.
//!
//! For example:
//!
//! ```text
//! NNCB
//!
//! CH -> B
//! HH -> N
//! CB -> H
//! NH -> C
//! HB -> C
//! HC -> B
//! HN -> C
//! NN -> C
//! BH -> H
//! NC -> B
//! NB -> B
//! BN -> B
//! BB -> N
//! BC -> B
//! CC -> N
//! CN -> C
//! ```
//!
//! The first line is the polymer template - this is the starting point of the
//! process.
//!
//! The following section defines the pair insertion rules. A rule like AB -> C
//! means that when elements A and B are immediately adjacent, element C should
//! be inserted between them. These insertions all happen simultaneously.
//!
//! So, starting with the polymer template NNCB, the first step simultaneously
//! considers all three pairs:
//!
//! - The first pair (NN) matches the rule NN -> C, so element C is inserted
//!   between the first N and the second N.
//! - The second pair (NC) matches the rule NC -> B, so element B is inserted
//!   between the N and the C.
//! - The third pair (CB) matches the rule CB -> H, so element H is inserted
//!   between the C and the B.
//!
//! Note that these pairs overlap: the second element of one pair is the first
//! element of the next pair. Also, because all pairs are considered
//! simultaneously, inserted elements are not considered to be part of a pair
//! until the next step.
//!
//! After the first step of this process, the polymer becomes NCNBCHB.
//!
//! Here are the results of a few steps using the above rules:
//!
//! ```text
//! Template:     NNCB
//! After step 1: NCNBCHB
//! After step 2: NBCCNBBBCBHCB
//! After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
//! After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
//! ```
//!
//! This polymer grows quickly. After step 5, it has length 97; After step 10,
//! it has length 3073. After step 10, B occurs 1749 times, C occurs 298 times,
//! H occurs 161 times, and N occurs 865 times; taking the quantity of the most
//! common element (B, 1749) and subtracting the quantity of the least common
//! element (H, 161) produces 1749 - 161 = 1588.
//!
//! Apply 10 steps of pair insertion to the polymer template and find the most
//! and least common elements in the result. What do you get if you take the
//! quantity of the most common element and subtract the quantity of the least
//! common element?
//!
//! # Part Two
//!
//! The resulting polymer isn't nearly strong enough to reinforce the submarine.
//! You'll need to run more steps of the pair insertion process; a total of 40
//! steps should do it.
//!
//! In the above example, the most common element is B (occurring 2192039569602
//! times) and the least common element is H (occurring 3849876073 times);
//! subtracting these produces 2188189693529.
//!
//! Apply 40 steps of pair insertion to the polymer template and find the most
//! and least common elements in the result. What do you get if you take the
//! quantity of the most common element and subtract the quantity of the least
//! common element?
//!
//! [Advent of Code 2021 - Day 14](https://adventofcode.com/2021/day/14)

use hashbrown::HashMap;
use std::fmt::{Display, Formatter};
use std::iter::once;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rule {
    pub pattern: (char, char),
    pub insertion: char,
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(" -> ")
            .ok_or_else(|| format!("invalid rule: {}", s))?;
        let mut pattern_chars = left.chars();
        let c1 = pattern_chars
            .next()
            .ok_or_else(|| format!("no first pattern char: {}", left))?;
        let c2 = pattern_chars
            .next()
            .ok_or_else(|| format!("no second pattern char: {}", left))?;
        let pattern = (c1, c2);
        let insertion = right
            .chars()
            .next()
            .ok_or_else(|| format!("no insertion character: {}", right))?;
        Ok(Self { pattern, insertion })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolymerInstructions {
    pub template: String,
    pub rules: HashMap<(char, char), char>,
}

impl Display for PolymerInstructions {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.template)?;
        writeln!(f, "")?;
        for ((c1, c2), ci) in self.rules.iter() {
            writeln!(f, "{}{} -> {}", c1, c2, ci)?;
        }
        Ok(())
    }
}

#[aoc_generator(day14)]
pub fn parse(input: &str) -> PolymerInstructions {
    let mut lines = input.trim().lines();
    let template = lines.next().expect("no template in input").to_string();
    lines.next().expect("no empty separator line");
    let rules = lines
        .map(|line| {
            let Rule { pattern, insertion } = line
                .parse::<Rule>()
                .unwrap_or_else(|err| panic!("no valid rule:{}", err));
            (pattern, insertion)
        })
        .collect();
    PolymerInstructions { template, rules }
}

pub fn develop_polymer(num_runs: u16, polymer_instructions: &PolymerInstructions) -> String {
    let template = polymer_instructions.template.to_string();
    (0..num_runs).fold(template, |polymer, _| {
        let first_char = polymer.chars().next().expect("empty polymer");
        once(first_char)
            .chain(
                polymer
                    .chars()
                    .zip(polymer.chars().skip(1))
                    .flat_map(|sequence| {
                        if let Some(insertion) = polymer_instructions.rules.get(&sequence) {
                            vec![*insertion, sequence.1]
                        } else {
                            vec![sequence.1]
                        }
                    }),
            )
            .collect()
    })
}

fn count_elements_in_polymer(polymer: &str) -> HashMap<char, u64> {
    let mut element_counts = HashMap::new();
    for c in polymer.chars() {
        *element_counts.entry(c).or_insert(0) += 1;
    }
    element_counts
}

#[aoc(day14, part1)]
pub fn solve_part1(polymer_instructions: &PolymerInstructions) -> u64 {
    //eprintln!("{}", polymer_instructions);
    let polymer = develop_polymer(10, polymer_instructions);
    let element_counts = count_elements_in_polymer(&polymer);
    let most_common = *element_counts
        .values()
        .max()
        .expect("no character in polymer at all");
    let least_common = *element_counts
        .values()
        .min()
        .expect("no character in polymer at all");
    most_common - least_common
}

fn init_pair_counts(template: &str) -> HashMap<(char, char), u64> {
    let mut pair_count = HashMap::new();
    for pair in template.chars().zip(template.chars().skip(1)) {
        *pair_count.entry(pair).or_insert(0) += 1;
    }
    pair_count
}

fn develop_polymer2(
    num_runs: u16,
    polymer_instructions: &PolymerInstructions,
) -> HashMap<(char, char), u64> {
    let mut pair_counts = init_pair_counts(&polymer_instructions.template);
    for _ in 0..num_runs {
        let mut next_pair_counts = HashMap::new();
        for (pair, count) in pair_counts {
            if let Some(insertion) = polymer_instructions.rules.get(&pair) {
                *next_pair_counts.entry((pair.0, *insertion)).or_insert(0) += count;
                *next_pair_counts.entry((*insertion, pair.1)).or_insert(0) += count;
            } else {
                *next_pair_counts.entry(pair).or_insert(0) += count;
            }
        }
        pair_counts = next_pair_counts;
    }
    pair_counts
}

fn count_elements_in_pairs(pair_counts: &HashMap<(char, char), u64>) -> HashMap<char, u64> {
    let mut element_counts = HashMap::new();
    for (&(e1, e2), &count) in pair_counts {
        *element_counts.entry(e1).or_insert(0) += count;
        *element_counts.entry(e2).or_insert(0) += count;
    }
    element_counts
}

#[aoc(day14, part2)]
pub fn solve_part2(polymer_instructions: &PolymerInstructions) -> u64 {
    let pair_counts = develop_polymer2(40, polymer_instructions);
    let element_counts = count_elements_in_pairs(&pair_counts);
    let most_common = *element_counts
        .values()
        .max()
        .expect("no character in polymer at all");
    let least_common = *element_counts
        .values()
        .min()
        .expect("no character in polymer at all");
    most_common / 2 - least_common / 2 + 1
}

#[cfg(test)]
mod tests;
