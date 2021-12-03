//! # Day 15: Rambunctious Recitation
//!
//! You catch the airport shuttle and try to book a new flight to your vacation
//! island. Due to the storm, all direct flights have been cancelled, but a
//! route is available to get around the storm. You take it.
//!
//! While you wait for your flight, you decide to check in with the Elves back
//! at the North Pole. They're playing a memory game and are ever so excited to
//! explain the rules!
//!
//! In this game, the players take turns saying numbers. They begin by taking
//! turns reading from a list of starting numbers (your puzzle input). Then,
//! each turn consists of considering the most recently spoken number:
//!
//! * If that was the first time the number has been spoken, the current player
//!   says 0.
//! * Otherwise, the number had been spoken before; the current player announces
//!   how many turns apart the number is from when it was previously spoken.
//!
//! So, after the starting numbers, each turn results in that player speaking
//! aloud either 0 (if the last number is new) or an age (if the last number is
//! a repeat).
//!
//! For example, suppose the starting numbers are 0,3,6:
//!
//! * Turn 1: The 1st number spoken is a starting number, 0.
//! * Turn 2: The 2nd number spoken is a starting number, 3.
//! * Turn 3: The 3rd number spoken is a starting number, 6.
//! * Turn 4: Now, consider the last number spoken, 6. Since that was the first
//!   time the number had been spoken, the 4th number spoken is 0.
//! * Turn 5: Next, again consider the last number spoken, 0. Since it had been
//!   spoken before, the next number to speak is the difference between the turn
//!   number when it was last spoken (the previous turn, 4) and the turn number
//!   of the time it was most recently spoken before then (turn 1). Thus, the
//!   5th number spoken is 4 - 1, 3.
//! * Turn 6: The last number spoken, 3 had also been spoken before, most
//!   recently on turns 5 and 2. So, the 6th number spoken is 5 - 2, 3.
//! * Turn 7: Since 3 was just spoken twice in a row, and the last two turns
//!   are 1 turn apart, the 7th number spoken is 1.
//! * Turn 8: Since 1 is new, the 8th number spoken is 0.
//! * Turn 9: 0 was last spoken on turns 8 and 4, so the 9th number spoken is
//!   the difference between them, 4.
//! * Turn 10: 4 is new, so the 10th number spoken is 0.
//!
//! (The game ends when the Elves get sick of playing or dinner is ready,
//! whichever comes first.)
//!
//! Their question for you is: what will be the 2020th number spoken? In the
//! example above, the 2020th number spoken will be 436.
//!
//! Here are a few more examples:
//!
//! * Given the starting numbers 1,3,2, the 2020th number spoken is 1.
//! * Given the starting numbers 2,1,3, the 2020th number spoken is 10.
//! * Given the starting numbers 1,2,3, the 2020th number spoken is 27.
//! * Given the starting numbers 2,3,1, the 2020th number spoken is 78.
//! * Given the starting numbers 3,2,1, the 2020th number spoken is 438.
//! * Given the starting numbers 3,1,2, the 2020th number spoken is 1836.
//!
//! Given your starting numbers, what will be the 2020th number spoken?
//!
//! # Part 2
//!
//! Impressed, the Elves issue you a challenge: determine the 30000000th number
//! spoken. For example, given the same starting numbers as above:
//!
//! * Given 0,3,6, the 30000000th number spoken is 175594.
//! * Given 1,3,2, the 30000000th number spoken is 2578.
//! * Given 2,1,3, the 30000000th number spoken is 3544142.
//! * Given 1,2,3, the 30000000th number spoken is 261214.
//! * Given 2,3,1, the 30000000th number spoken is 6895259.
//! * Given 3,2,1, the 30000000th number spoken is 18.
//! * Given 3,1,2, the 30000000th number spoken is 362.
//!
//! Given your starting numbers, what will be the 30000000th number spoken?
//!
//! [Advent of Code 2020 - Day 15](https://adventofcode.com/2020/day/15)

use fxhash::FxBuildHasher;
use hashbrown::HashMap;
use std::str::FromStr;

pub type Number = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct History {
    turns: [u32; 2],
}

impl History {
    pub fn new() -> Self {
        Self { turns: [0; 2] }
    }

    pub fn age(&self) -> Option<u32> {
        if self.turns[1] == 0 {
            None
        } else {
            Some(self.turns[0] - self.turns[1])
        }
    }

    pub fn spoken_at_turn(&mut self, turn: u32) {
        debug_assert!(turn > 0, "turn must be greater 0");
        self.turns[1] = self.turns[0];
        self.turns[0] = turn;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Memory {
    history: HashMap<Number, History, FxBuildHasher>,
    last_number: Number,
    last_turn: u32,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            history: HashMap::with_hasher(FxBuildHasher::default()),
            last_number: 0,
            last_turn: 0,
        }
    }

    pub fn add_spoken_number(&mut self, turn: u32, number: Number) {
        self.last_turn = turn;
        self.last_number = number;
        self.history
            .entry(number)
            .or_insert_with(History::new)
            .spoken_at_turn(turn);
    }

    pub fn last_spoken_number(&self) -> Number {
        self.last_number
    }

    pub fn last_turn(&self) -> u32 {
        self.last_turn
    }
}

impl Iterator for Memory {
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        let next_number = self.history[&self.last_number].age().unwrap_or(0);
        self.add_spoken_number(self.last_turn + 1, next_number);
        Some(next_number)
    }
}

#[aoc_generator(day15)]
pub fn parse_starting_numbers(input: &str) -> Vec<u32> {
    input
        .split(',')
        .map(|s| {
            u32::from_str(s.trim()).unwrap_or_else(|err| panic!("not a number {:?}; {}", s, err))
        })
        .collect()
}

#[aoc(day15, part1)]
pub fn determine_the_2020th_number_spoken(numbers: &[u32]) -> u32 {
    let mut memory = Memory::new();
    numbers
        .iter()
        .enumerate()
        .for_each(|(turn, num)| memory.add_spoken_number(turn as u32 + 1, *num));
    let remaining_turns = 2020 - memory.last_turn();
    memory.take(remaining_turns as usize).last().unwrap_or(0)
}

#[aoc(day15, part2)]
pub fn determine_the_30millionsth_number_spoken(numbers: &[u32]) -> u32 {
    let mut memory = Memory::new();
    numbers
        .iter()
        .enumerate()
        .for_each(|(turn, num)| memory.add_spoken_number(turn as u32 + 1, *num));
    let remaining_turns = 30_000_000 - memory.last_turn();
    memory.take(remaining_turns as usize).last().unwrap_or(0)
}

#[cfg(test)]
mod tests;
