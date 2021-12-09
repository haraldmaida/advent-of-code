//! # Day 8: Seven Segment Search
//!
//! You barely reach the safety of the cave when the whale smashes into the cave
//! mouth, collapsing it. Sensors indicate another exit to this cave at a much
//! greater depth, so you have no choice but to press on.
//!
//! As your submarine slowly makes its way through the cave system, you notice
//! that the four-digit seven-segment displays in your submarine are
//! malfunctioning; they must have been damaged during the escape. You'll be in
//! a lot of trouble without them, so you'd better figure out what's wrong.
//!
//! Each digit of a seven-segment display is rendered by turning on or off any
//! of seven segments named a through g:
//!
//! ```text
//!   0:      1:      2:      3:      4:
//!  aaaa    ....    aaaa    aaaa    ....
//! b    c  .    c  .    c  .    c  b    c
//! b    c  .    c  .    c  .    c  b    c
//!  ....    ....    dddd    dddd    dddd
//! e    f  .    f  e    .  .    f  .    f
//! e    f  .    f  e    .  .    f  .    f
//!  gggg    ....    gggg    gggg    ....
//!
//!   5:      6:      7:      8:      9:
//!  aaaa    aaaa    aaaa    aaaa    aaaa
//! b    .  b    .  .    c  b    c  b    c
//! b    .  b    .  .    c  b    c  b    c
//!  dddd    dddd    ....    dddd    dddd
//! .    f  e    f  .    f  e    f  .    f
//! .    f  e    f  .    f  e    f  .    f
//!  gggg    gggg    ....    gggg    gggg
//! ```
//!
//! So, to render a 1, only segments c and f would be turned on; the rest would
//! be off. To render a 7, only segments a, c, and f would be turned on.
//!
//! The problem is that the signals which control the segments have been mixed
//! up on each display. The submarine is still trying to display numbers by
//! producing output on signal wires a through g, but those wires are connected
//! to segments randomly. Worse, the wire/segment connections are mixed up
//! separately for each four-digit display! (All of the digits within a display
//! use the same connections, though.)
//!
//! So, you might know that only signal wires b and g are turned on, but that
//! doesn't mean segments b and g are turned on: the only digit that uses two
//! segments is 1, so it must mean segments c and f are meant to be on. With
//! just that information, you still can't tell which wire (b/g) goes to which
//! segment (c/f). For that, you'll need to collect more information.
//!
//! For each display, you watch the changing signals for a while, make a note of
//! all ten unique signal patterns you see, and then write down a single four
//! digit output value (your puzzle input). Using the signal patterns, you
//! should be able to work out which pattern corresponds to which digit.
//!
//! For example, here is what you might see in a single entry in your notes:
//!
//! ```text
//! acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
//! cdfeb fcadb cdfeb cdbaf
//! ```
//!
//! (The entry is wrapped here to two lines so it fits; in your notes, it will
//! all be on a single line.)
//!
//! Each entry consists of ten unique signal patterns, a | delimiter, and
//! finally the four digit output value. Within an entry, the same wire/segment
//! connections are used (but you don't know what the connections actually are).
//! The unique signal patterns correspond to the ten different ways the
//! submarine tries to render a digit using the current wire/segment
//! connections. Because 7 is the only digit that uses three segments, dab in
//! the above example means that to render a 7, signal lines d, a, and b are on.
//! Because 4 is the only digit that uses four segments, eafb means that to
//! render a 4, signal lines e, a, f, and b are on.
//!
//! Using this information, you should be able to work out which combination of
//! signal wires corresponds to each of the ten digits. Then, you can decode the
//! four digit output value. Unfortunately, in the above example, all of the
//! digits in the output value (cdfeb fcadb cdfeb cdbaf) use five segments and
//! are more difficult to deduce.
//!
//! For now, focus on the easy digits. Consider this larger example:
//!
//! ```text
//! be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |
//! fdgacbe cefdb cefbgd gcbe
//! edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |
//! fcgedb cgb dgebacf gc
//! fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |
//! cg cg fdcagb cbg
//! fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |
//! efabcd cedba gadfec cb
//! aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |
//! gecf egdcabf bgf bfgea
//! fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |
//! gebdcfa ecba ca fadegcb
//! dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |
//! cefg dcbef fcge gbcadfe
//! bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |
//! ed bcgafe cdgba cbgef
//! egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |
//! gbdfcae bgc cg cgb
//! gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |
//! fgae cfgab fg bagce
//! ```
//!
//! Because the digits 1, 4, 7, and 8 each use a unique number of segments, you
//! should be able to tell which combinations of signals correspond to those
//! digits. Counting only digits in the output values (the part after | on each
//! line), in the above example, there are 26 instances of digits that use a
//! unique number of segments (highlighted above).
//!
//! In the output values, how many times do digits 1, 4, 7, or 8 appear?
//!
//! ## Part Two
//!
//! Through a little deduction, you should now be able to determine the
//! remaining digits. Consider again the first example above:
//!
//! ```text
//! acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |
//! cdfeb fcadb cdfeb cdbaf
//! ```
//!
//! After some careful analysis, the mapping between signal wires and segments
//! only make sense in the following configuration:
//!
//! ```text
//!  dddd
//! e    a
//! e    a
//!  ffff
//! g    b
//! g    b
//!  cccc
//! ```
//!
//! So, the unique signal patterns would correspond to the following digits:
//!
//! - acedgfb: 8
//! - cdfbe: 5
//! - gcdfa: 2
//! - fbcad: 3
//! - dab: 7
//! - cefabd: 9
//! - cdfgeb: 6
//! - eafb: 4
//! - cagedb: 0
//! - ab: 1
//!
//! Then, the four digits of the output value can be decoded:
//!
//! - cdfeb: 5
//! - fcadb: 3
//! - cdfeb: 5
//! - cdbaf: 3
//!
//! Therefore, the output value for this entry is 5353.
//!
//! Following this same process for each entry in the second, larger example
//! above, the output value of each entry can be determined:
//!
//! - fdgacbe cefdb cefbgd gcbe: 8394
//! - fcgedb cgb dgebacf gc: 9781
//! - cg cg fdcagb cbg: 1197
//! - efabcd cedba gadfec cb: 9361
//! - gecf egdcabf bgf bfgea: 4873
//! - gebdcfa ecba ca fadegcb: 8418
//! - cefg dcbef fcge gbcadfe: 4548
//! - ed bcgafe cdgba cbgef: 1625
//! - gbdfcae bgc cg cgb: 8717
//! - fgae cfgab fg bagce: 4315
//!
//! Adding all of the output values in this larger example produces 61229.
//!
//! For each entry, determine all of the wire/segment connections and decode
//! the four-digit output values. What do you get if you add up all of the
//! output values?
//!
//! [Advent of Code 2021 - Day 8](https://adventofcode.com/2021/day/8)

use hashbrown::HashSet;
use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Digit {
    segments: [bool; 7],
}

pub const ZERO: Digit = Digit {
    segments: [true, true, true, false, true, true, true],
};

pub const ONE: Digit = Digit {
    segments: [false, false, true, false, false, true, false],
};

pub const TWO: Digit = Digit {
    segments: [true, false, true, true, true, false, true],
};

pub const THREE: Digit = Digit {
    segments: [true, false, true, true, false, true, true],
};

pub const FOUR: Digit = Digit {
    segments: [false, true, true, true, false, true, false],
};

pub const FIVE: Digit = Digit {
    segments: [true, true, false, true, false, true, true],
};

pub const SIX: Digit = Digit {
    segments: [true, true, false, true, true, true, true],
};

pub const SEVEN: Digit = Digit {
    segments: [true, false, true, false, false, true, false],
};

pub const EIGHT: Digit = Digit {
    segments: [true, true, true, true, true, true, true],
};

pub const NINE: Digit = Digit {
    segments: [true, true, true, true, false, true, true],
};

pub const DIGITS: [Digit; 10] = [ZERO, ONE, TWO, THREE, FOUR, FIVE, SIX, SEVEN, EIGHT, NINE];

impl Digit {
    pub fn to_i64(self) -> i64 {
        match self {
            ZERO => 0,
            ONE => 1,
            TWO => 2,
            THREE => 3,
            FOUR => 4,
            FIVE => 5,
            SIX => 6,
            SEVEN => 7,
            EIGHT => 8,
            NINE => 9,
            _ => panic!("invalid digit"),
        }
    }
}

impl From<[bool; 7]> for Digit {
    fn from(values: [bool; 7]) -> Self {
        Self { segments: values }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signal {
    segments: [Option<char>; 7],
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern {
    pub signals: [HashSet<char>; 10],
    pub display: [HashSet<char>; 4],
}

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Vec<Pattern> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (head, tail) = line.split_once('|').expect("not a valid input line");
            let mut signals_iter = head.split_whitespace();
            let signals = [
                HashSet::from_iter(signals_iter.next().unwrap().chars()),
                HashSet::from_iter(signals_iter.next().unwrap().chars()),
                HashSet::from_iter(signals_iter.next().unwrap().chars()),
                HashSet::from_iter(signals_iter.next().unwrap().chars()),
                HashSet::from_iter(signals_iter.next().unwrap().chars()),
                HashSet::from_iter(signals_iter.next().unwrap().chars()),
                HashSet::from_iter(signals_iter.next().unwrap().chars()),
                HashSet::from_iter(signals_iter.next().unwrap().chars()),
                HashSet::from_iter(signals_iter.next().unwrap().chars()),
                HashSet::from_iter(signals_iter.next().unwrap().chars()),
            ];
            let mut display_iter = tail.split_whitespace();
            let display = [
                HashSet::from_iter(display_iter.next().unwrap().chars()),
                HashSet::from_iter(display_iter.next().unwrap().chars()),
                HashSet::from_iter(display_iter.next().unwrap().chars()),
                HashSet::from_iter(display_iter.next().unwrap().chars()),
            ];
            Pattern { signals, display }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn count_digits_1_4_7_8(pattern_list: &[Pattern]) -> usize {
    pattern_list
        .iter()
        .map(|pattern| {
            pattern
                .display
                .iter()
                .filter(|digit| {
                    let chars_count = digit.len();
                    chars_count == 2 || chars_count == 3 || chars_count == 4 || chars_count == 7
                })
                .count()
        })
        .sum()
}

fn signal_segment_map(wires: impl IntoIterator<Item = char>) -> HashMap<char, usize> {
    HashMap::from_iter(wires.into_iter().enumerate().map(|(i, c)| (c, i)))
}

fn digit_for_signal(
    signal: &HashSet<char>,
    signal_segment_map: &HashMap<char, usize>,
) -> Option<Digit> {
    let mut digit = [false; 7];
    for c in signal {
        let i = *signal_segment_map.get(&c).unwrap();
        digit[i] = true;
    }
    let digit = Digit::from(digit);
    DIGITS.iter().find(|valid| digit == **valid).copied()
}

fn decode_signals(signals: &[HashSet<char>]) -> HashMap<char, usize> {
    let mut found_mapping = None;
    let one_signal = signals.iter().find(|signal| signal.len() == 2).unwrap();
    let seven_signal = signals.iter().find(|signal| signal.len() == 3).unwrap();
    let char0 = *seven_signal.difference(one_signal).next().unwrap();
    let mut all_wires = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];
    let char0_pos = all_wires.iter().position(|c| *c == char0).unwrap();
    all_wires.remove(char0_pos);
    let k_value = all_wires.len();
    for mut wires in all_wires.into_iter().permutations(k_value) {
        wires.insert(0, char0);
        let mapping = signal_segment_map(wires);
        if signals
            .iter()
            .all(|signal| digit_for_signal(signal, &mapping).is_some())
        {
            found_mapping = Some(mapping);
            break;
        }
    }
    found_mapping.expect("no valid mapping found")
}

#[aoc(day8, part2)]
pub fn sum_output_values(pattern_list: &[Pattern]) -> i64 {
    pattern_list
        .iter()
        .map(|pattern| {
            let mapping = decode_signals(&pattern.signals);
            pattern
                .display
                .iter()
                .rev()
                .map(|signal| digit_for_signal(signal, &mapping).expect("signal not in mapping"))
                .enumerate()
                .fold(0, |acc, (i, digit)| {
                    acc + (digit.to_i64() * 10_i64.pow(i as u32))
                })
        })
        .sum()
}

#[cfg(test)]
mod tests;
