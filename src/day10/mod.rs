//! # Day 10: Adapter Array
//!
//! Patched into the aircraft's data port, you discover weather forecasts of a
//! massive tropical storm. Before you can figure out whether it will impact
//! your vacation plans, however, your device suddenly turns off!
//!
//! Its battery is dead.
//!
//! You'll need to plug it in. There's only one problem: the charging outlet
//! near your seat produces the wrong number of jolts. Always prepared, you
//! make a list of all of the joltage adapters in your bag.
//!
//! Each of your joltage adapters is rated for a specific output joltage (your
//! puzzle input). Any given adapter can take an input 1, 2, or 3 jolts lower
//! than its rating and still produce its rated output joltage.
//!
//! In addition, your device has a built-in joltage adapter rated for 3 jolts
//! higher than the highest-rated adapter in your bag. (If your adapter list
//! were 3, 9, and 6, your device's built-in adapter would be rated for 12
//! jolts.)
//!
//! Treat the charging outlet near your seat as having an effective joltage
//! rating of 0.
//!
//! Since you have some time to kill, you might as well test all of your
//! adapters. Wouldn't want to get to your resort and realize you can't even
//! charge your device!
//!
//! If you use every adapter in your bag at once, what is the distribution of
//! joltage differences between the charging outlet, the adapters, and your
//! device?
//!
//! For example, suppose that in your bag, you have adapters with the following
//! joltage ratings:
//!
//! ```text
//! 16
//! 10
//! 15
//! 5
//! 1
//! 11
//! 7
//! 19
//! 6
//! 12
//! 4
//! ```
//!
//! With these adapters, your device's built-in joltage adapter would be rated
//! for 19 + 3 = 22 jolts, 3 higher than the highest-rated adapter.
//!
//! Because adapters can only connect to a source 1-3 jolts lower than its
//! rating, in order to use every adapter, you'd need to choose them like this:
//!
//! * The charging outlet has an effective rating of 0 jolts, so the only
//!   adapters that could connect to it directly would need to have a joltage
//!   rating of 1, 2, or 3 jolts. Of these, only one you have is an adapter
//!   rated 1 jolt (difference of 1).
//! * From your 1-jolt rated adapter, the only choice is your 4-jolt rated
//!   adapter (difference of 3).
//! * From the 4-jolt rated adapter, the adapters rated 5, 6, or 7 are valid
//!   choices. However, in order to not skip any adapters, you have to pick the
//!   adapter rated 5 jolts (difference of 1).
//! * Similarly, the next choices would need to be the adapter rated 6 and then
//!   the adapter rated 7 (with difference of 1 and 1).
//! * The only adapter that works with the 7-jolt rated adapter is the one
//!   rated 10 jolts (difference of 3).
//! * From 10, the choices are 11 or 12; choose 11 (difference of 1) and then 12
//!   (difference of 1).
//! * After 12, only valid adapter has a rating of 15 (difference of 3), then 16
//!   (difference of 1), then 19 (difference of 3).
//! * Finally, your device's built-in adapter is always 3 higher than the
//!   highest adapter, so its rating is 22 jolts (always a difference of 3).
//!
//! In this example, when using every adapter, there are 7 differences of 1 jolt
//! and 5 differences of 3 jolts.
//!
//! Here is a larger example:
//!
//! ```text
//! 28
//! 33
//! 18
//! 42
//! 31
//! 14
//! 46
//! 20
//! 48
//! 47
//! 24
//! 23
//! 49
//! 45
//! 19
//! 38
//! 39
//! 11
//! 1
//! 32
//! 25
//! 35
//! 8
//! 17
//! 7
//! 9
//! 4
//! 2
//! 34
//! 10
//! 3
//! ```
//!
//! In this larger example, in a chain that uses all of the adapters, there are
//! 22 differences of 1 jolt and 10 differences of 3 jolts.
//!
//! Find a chain that uses all of your adapters to connect the charging outlet
//! to your device's built-in adapter and count the joltage differences between
//! the charging outlet, the adapters, and your device. What is the number of
//! 1-jolt differences multiplied by the number of 3-jolt differences?
//!
//! ## Part 2
//!
//! To completely determine whether you have enough adapters, you'll need to
//! figure out how many different ways they can be arranged. Every arrangement
//! needs to connect the charging outlet to your device. The previous rules
//! about when adapters can successfully connect still apply.
//!
//! The first example above (the one that starts with 16, 10, 15) supports the
//! following arrangements:
//!
//! ```text
//! (0), 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, (22)
//! (0), 1, 4, 5, 6, 7, 10, 12, 15, 16, 19, (22)
//! (0), 1, 4, 5, 7, 10, 11, 12, 15, 16, 19, (22)
//! (0), 1, 4, 5, 7, 10, 12, 15, 16, 19, (22)
//! (0), 1, 4, 6, 7, 10, 11, 12, 15, 16, 19, (22)
//! (0), 1, 4, 6, 7, 10, 12, 15, 16, 19, (22)
//! (0), 1, 4, 7, 10, 11, 12, 15, 16, 19, (22)
//! (0), 1, 4, 7, 10, 12, 15, 16, 19, (22)
//! ```
//!
//! (The charging outlet and your device's built-in adapter are shown in
//! parentheses.) Given the adapters from the first example, the total number of
//! arrangements that connect the charging outlet to your device is 8.
//!
//! The second example above (the one that starts with 28, 33, 18) has many
//! arrangements. Here are a few:
//!
//! ```text
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, (52)
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 49, (52)
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 46, 48, 49, (52)
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 46, 49, (52)
//!
//! (0), 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31,
//! 32, 33, 34, 35, 38, 39, 42, 45, 47, 48, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 46, 48, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 46, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 47, 48, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 47, 49, (52)
//!
//! (0), 3, 4, 7, 10, 11, 14, 17, 20, 23, 25, 28, 31, 34, 35, 38, 39, 42, 45,
//! 48, 49, (52)
//! ```
//!
//! In total, this set of adapters can connect the charging outlet to your
//! device in 19208 distinct arrangements.
//!
//! You glance back down at your bag and try to remember why you brought so many
//! adapters; there must be more than a trillion valid ways to arrange them!
//! Surely, there must be an efficient way to count the arrangements.
//!
//! What is the total number of distinct ways you can arrange the adapters to
//! connect the charging outlet to your device?
//!
//! [Advent of Code 2020 - Day 10](https://adventofcode.com/2020/day/10)

use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Adapter {
    pub rating: u32,
}

impl Adapter {
    pub fn new(rating: u32) -> Self {
        Self { rating }
    }

    pub fn can_connect_to_source(&self, source: &Adapter) -> bool {
        self.rating > source.rating && self.rating - source.rating <= 3
    }
}

const SEAT_CHARGING_OUTLET: Adapter = Adapter { rating: 0 };

#[aoc_generator(day10)]
pub fn parse_adapter_joltages(input: &str) -> Vec<Adapter> {
    input
        .lines()
        .enumerate()
        .map(|line| {
            let rating = u32::from_str(line.1).unwrap_or_else(|err| {
                panic!(
                    "line {}: {:?} is not a valid number: {}",
                    line.0, line.1, err
                )
            });
            Adapter { rating }
        })
        .collect()
}

fn build_chain_of_adapters(all_adapters: &[Adapter]) -> Vec<Adapter> {
    let mut all_adapters = all_adapters.to_vec();
    all_adapters.insert(0, SEAT_CHARGING_OUTLET);
    all_adapters.sort_by_key(|adapter| adapter.rating);
    all_adapters.push(Adapter::new(
        all_adapters.last().map(|a| a.rating).unwrap_or(0) + 3,
    ));
    all_adapters
}

#[aoc(day10, part1)]
pub fn joltage_differences_in_adapter_chain(all_adapters: &[Adapter]) -> usize {
    let adapter_chain = build_chain_of_adapters(all_adapters);
    let (diff_one_count, diff_three_count) = adapter_chain
        .iter()
        .zip(adapter_chain.iter().skip(1))
        .map(|(adapter1, adapter2)| adapter2.rating - adapter1.rating)
        .fold((0, 0), |(ones, threes), diff| match diff {
            1 => (ones + 1, threes),
            3 => (ones, threes + 1),
            _ => (ones, threes),
        });
    diff_one_count * diff_three_count
}

fn indexed_differences(adapter_chain: &[Adapter]) -> Vec<(usize, u32)> {
    adapter_chain
        .iter()
        .zip(adapter_chain.iter().enumerate().skip(1))
        .map(|(a1, (index, a2))| (index, a2.rating - a1.rating))
        .collect()
}

fn mark_removable(indexed_differences: &[(usize, u32)]) -> Vec<(usize, bool)> {
    [(0, false)]
        .iter()
        .copied()
        .chain(
            indexed_differences
                .iter()
                .zip(indexed_differences.iter().skip(1))
                .map(|((i1, diff1), (_, diff2))| (*i1, *diff1 == 1 && *diff2 == 1)),
        )
        .chain([(indexed_differences.len(), false)].iter().copied())
        .collect()
}

fn split_into_sub_chains(adapter_chain: &[Adapter]) -> Vec<Vec<Adapter>> {
    let mut sub_chains = Vec::new();
    let mut prev_index = 0;
    for index in mark_removable(&indexed_differences(adapter_chain))
        .iter()
        .filter_map(|(index, removable)| if !removable { Some(*index) } else { None })
    {
        if index - prev_index > 1 {
            let sub_chain = adapter_chain[prev_index..=index].iter().cloned().collect();
            sub_chains.push(sub_chain);
        }
        prev_index = index;
    }
    sub_chains
}

fn count_adapter_combinations(adapter_chain: &[Adapter]) -> usize {
    let mut count = 1;
    for sub_chain in split_into_sub_chains(adapter_chain) {
        let mut found_chains = HashSet::new();
        let mut open_sub_chains = vec![sub_chain.to_vec()];
        while let Some(sub_chain) = open_sub_chains.pop() {
            if !found_chains.contains(&sub_chain) {
                for i in 1..sub_chain.len() - 1 {
                    if sub_chain[i + 1].rating - sub_chain[i - 1].rating <= 3 {
                        let mut new_sub_chain = sub_chain.clone();
                        new_sub_chain.remove(i);
                        open_sub_chains.push(new_sub_chain);
                    }
                }
                found_chains.insert(sub_chain);
            }
        }
        count *= found_chains.len();
        found_chains.clear()
    }
    count
}

#[aoc(day10, part2)]
pub fn number_of_possible_adapter_arrangements(all_adapters: &[Adapter]) -> usize {
    let adapter_chain = build_chain_of_adapters(all_adapters);
    count_adapter_combinations(&adapter_chain)
}

#[cfg(test)]
mod tests;
