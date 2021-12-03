//! # Day 7: Handy Haversacks
//!
//! You land at the regional airport in time for your next flight. In fact, it
//! looks like you'll even have time to grab some food: all flights are
//! currently delayed due to issues in luggage processing.
//!
//! Due to recent aviation regulations, many rules (your puzzle input) are being
//! enforced about bags and their contents; bags must be color-coded and must
//! contain specific quantities of other color-coded bags. Apparently, nobody
//! responsible for these regulations considered how long they would take to
//! enforce!
//!
//! For example, consider the following rules:
//!
//! * light red bags contain 1 bright white bag, 2 muted yellow bags.
//! * dark orange bags contain 3 bright white bags, 4 muted yellow bags.
//! * bright white bags contain 1 shiny gold bag.
//! * muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
//! * shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
//! * dark olive bags contain 3 faded blue bags, 4 dotted black bags.
//! * vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
//! * faded blue bags contain no other bags.
//! * dotted black bags contain no other bags.
//!
//! These rules specify the required contents for 9 bag types. In this example,
//! every faded blue bag is empty, every vibrant plum bag contains 11 bags
//! (5 faded blue and 6 dotted black), and so on.
//!
//! You have a shiny gold bag. If you wanted to carry it in at least one other
//! bag, how many different bag colors would be valid for the outermost bag?
//! (In other words: how many colors can, eventually, contain at least one shiny
//! gold bag?)
//!
//! In the above rules, the following options would be available to you:
//!
//! * A bright white bag, which can hold your shiny gold bag directly.
//! * A muted yellow bag, which can hold your shiny gold bag directly, plus some
//!   other bags.
//! * A dark orange bag, which can hold bright white and muted yellow bags,
//!   either of which could then hold your shiny gold bag.
//! * A light red bag, which can hold bright white and muted yellow bags, either
//!   of which could then hold your shiny gold bag.
//!
//! So, in this example, the number of bag colors that can eventually contain at
//! least one shiny gold bag is 4.
//!
//! How many bag colors can eventually contain at least one shiny gold bag?
//! (The list of rules is quite long; make sure you get all of it.)
//!
//! # Part 2
//!
//! It's getting pretty expensive to fly these days - not because of ticket
//! prices, but because of the ridiculous number of bags you need to buy!
//!
//! Consider again your shiny gold bag and the rules from the above example:
//!
//! * faded blue bags contain 0 other bags.
//! * dotted black bags contain 0 other bags.
//! * vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted
//!   black bags.
//! * dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black
//!   bags.
//!
//! So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags
//! within it) plus 2 vibrant plum bags (and the 11 bags within each of those):
//! `1 + 1*7 + 2 + 2*11 = 32` bags!
//!
//! Of course, the actual rules have a small chance of going several levels
//! deeper than this example; be sure to count all of the bags, even if the
//! nesting becomes topologically impractical!
//!
//! Here's another example:
//!
//! * shiny gold bags contain 2 dark red bags.
//! * dark red bags contain 2 dark orange bags.
//! * dark orange bags contain 2 dark yellow bags.
//! * dark yellow bags contain 2 dark green bags.
//! * dark green bags contain 2 dark blue bags.
//! * dark blue bags contain 2 dark violet bags.
//! * dark violet bags contain no other bags.
//!
//! In this example, a single shiny gold bag must contain 126 other bags.
//!
//! How many individual bags are required inside your single shiny gold bag?
//!
//! [Advent of Code 2020 - Day 7](https://adventofcode.com/2020/day/7)

use std::collections::HashMap;
use std::iter::FromIterator;
use std::str::FromStr;
use std::{fmt, mem};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorCode<'a> {
    modifier: &'a str,
    color: &'a str,
}

impl<'a> fmt::Display for ColorCode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.modifier, self.color)
    }
}

impl<'a> ColorCode<'a> {
    pub fn from_str(s: &'a str) -> Result<Self, ColorCodeParseError<'_>> {
        let mut parser = s.trim().split_whitespace();
        if let Some(modifier) = parser.next() {
            if let Some(color) = parser.next() {
                Ok(Self { modifier, color })
            } else {
                Err(ColorCodeParseError::NoColorSpecified(modifier))
            }
        } else {
            Err(ColorCodeParseError::NoModifierSpecified(s))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorCodeParseError<'a> {
    NoColorSpecified(&'a str),
    NoModifierSpecified(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bag<'a> {
    color_code: ColorCode<'a>,
    contents: Vec<Content<'a>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Content<'a> {
    color_code: ColorCode<'a>,
    quantity: usize,
}

#[aoc_generator(day7)]
pub fn parse_bag_rules(input: &str) -> Vec<Bag<'_>> {
    let mut bag_rules = Vec::with_capacity(8);
    let mut content_list = Vec::new();
    for (line_index, line) in input.split(".\n").enumerate() {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split("bags contain");
        let color_code = parts.next().expect("rule is empty").trim();
        let rule_color = ColorCode::from_str(color_code.trim())
            .unwrap_or_else(|err| panic!("line {}: invalid color code: {:?}", line_index + 1, err));
        let contents = parts
            .next()
            .unwrap_or_else(|| panic!("line {}: no content in rule: {:?}", line_index + 1, line));
        if !contents.contains("no other bags") {
            for content in contents.split(',') {
                let number_index_start =
                    content
                        .find(|c: char| c.is_ascii_digit())
                        .unwrap_or_else(|| {
                            panic!(
                                "line {}: no quantity in rule: {:?}",
                                line_index + 1,
                                content
                            )
                        });
                let number_index_end = number_index_start
                    + content[number_index_start..]
                        .find(char::is_whitespace)
                        .unwrap();
                let quantity = usize::from_str(&content[number_index_start..number_index_end])
                    .unwrap_or_else(|err| {
                        panic!(
                            "line {}: not a number: {:?}; reason: {:?}",
                            line_index + 1,
                            &content[number_index_start..number_index_end],
                            err
                        )
                    });
                let color_code = ColorCode::from_str(&content[number_index_end + 1..])
                    .unwrap_or_else(|err| {
                        panic!(
                            "line {}: not a color code: {:?}; reason: {:?}",
                            line_index + 1,
                            &content[number_index_end + 1..],
                            err
                        )
                    });
                content_list.push(Content {
                    color_code,
                    quantity,
                })
            }
        }
        bag_rules.push(Bag {
            color_code: rule_color,
            contents: mem::replace(&mut content_list, Vec::new()),
        })
    }
    bag_rules
}

impl<'a> Bag<'a> {
    pub fn contains(&self, color_code: &ColorCode<'a>) -> bool {
        self.contents
            .iter()
            .any(|content| content.contains(color_code))
    }
}

impl<'a> Content<'a> {
    pub fn contains(&self, color_code: &ColorCode<'a>) -> bool {
        self.color_code == *color_code
    }
}

fn may_hold_bag_with_color(
    color: ColorCode<'_>,
    bag: &Bag<'_>,
    bag_rules: &HashMap<ColorCode<'_>, &Vec<Content<'_>>>,
) -> bool {
    let mut open = Vec::with_capacity(8);
    open.push(bag.color_code);
    while let Some(color_code) = open.pop() {
        if let Some(contents) = bag_rules.get(&color_code) {
            for content in *contents {
                if content.color_code == color && content.quantity > 0 {
                    return true;
                } else {
                    open.push(content.color_code)
                }
            }
        }
    }
    false
}

const SHINY_GOLD: ColorCode<'_> = ColorCode {
    modifier: "shiny",
    color: "gold",
};

//#[aoc(day7, part1)]
pub fn number_of_bag_colors_containing_shiny_gold_bags(bag_rules: &[Bag<'_>]) -> usize {
    let color_map: HashMap<ColorCode<'_>, &Vec<Content<'_>>> = HashMap::from_iter(
        bag_rules
            .iter()
            .map(|rule| (rule.color_code, &rule.contents)),
    );
    bag_rules
        .iter()
        .filter(|bag| may_hold_bag_with_color(SHINY_GOLD, bag, &color_map))
        .count()
}

fn number_of_bags_inside(
    quantity: usize,
    color: ColorCode<'_>,
    bag_rules: &HashMap<ColorCode<'_>, &Vec<Content<'_>>>,
) -> usize {
    if let Some(contents) = bag_rules.get(&color) {
        if contents.is_empty() {
            return quantity;
        }
        let mut count = quantity;
        for content in *contents {
            count +=
                quantity * number_of_bags_inside(content.quantity, content.color_code, bag_rules);
        }
        count
    } else {
        quantity
    }
}

//#[aoc(day7, part2)]
pub fn number_of_bags_inside_shiny_gold_bag(bag_rules: &[Bag<'_>]) -> usize {
    let color_map: HashMap<ColorCode<'_>, &Vec<Content<'_>>> = HashMap::from_iter(
        bag_rules
            .iter()
            .map(|rule| (rule.color_code, &rule.contents)),
    );
    number_of_bags_inside(1, SHINY_GOLD, &color_map) - 1
}

#[cfg(test)]
mod tests;
