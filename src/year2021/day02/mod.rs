//! # Day 2: Dive!
//!
//! Now, you need to figure out how to pilot this thing.
//!
//! It seems like the submarine can take a series of commands like forward 1,
//! down 2, or up 3:
//!
//! - forward X increases the horizontal position by X units.
//! - down X increases the depth by X units.
//! - up X decreases the depth by X units.
//!
//! Note that since you're on a submarine, down and up affect your depth, and so
//! they have the opposite result of what you might expect.
//!
//! The submarine seems to already have a planned course (your puzzle input).
//! You should probably figure out where it's going. For example:
//!
//! ```text
//! forward 5
//! down 5
//! forward 8
//! up 3
//! down 8
//! forward 2
//! ```
//!
//! Your horizontal position and depth both start at 0. The steps above would
//! then modify them as follows:
//!
//! - forward 5 adds 5 to your horizontal position, a total of 5.
//! - down 5 adds 5 to your depth, resulting in a value of 5.
//! - forward 8 adds 8 to your horizontal position, a total of 13.
//! - up 3 decreases your depth by 3, resulting in a value of 2.
//! - down 8 adds 8 to your depth, resulting in a value of 10.
//! - forward 2 adds 2 to your horizontal position, a total of 15.
//!
//! After following these instructions, you would have a horizontal position of
//! 15 and a depth of 10. (Multiplying these together produces 150.)
//!
//! Calculate the horizontal position and depth you would have after following
//! the planned course. What do you get if you multiply your final horizontal
//! position by your final depth?
//!
//! [Advent of Code 2021 - Day 2](https://adventofcode.com/2021/day/2)

use std::fmt;
use std::fmt::Display;
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    x: i32,
    depth: i32,
}

impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.x, self.depth)
    }
}

impl Default for Position {
    fn default() -> Self {
        Self { x: 0, depth: 0 }
    }
}

impl Position {
    pub fn navigate(&self, command: Move) -> Self {
        let (x, depth) = match command {
            Move::Forward(distance) => (self.x + distance, self.depth),
            Move::Down(distance) => (self.x, self.depth + distance),
            Move::Up(distance) => (self.x, self.depth - distance),
        };
        Self { x, depth }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Move> {
    input
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(index, line)| {
            parse_move(line).unwrap_or_else(|err| panic!("line {}: {}", index + 1, err))
        })
        .collect()
}

fn parse_move(line: &str) -> Result<Move, String> {
    let parts: Vec<_> = line.split_whitespace().collect();
    let amount_str = parts
        .get(1)
        .ok_or_else(|| format!("invalid command: amount is missing"))?;
    let amount = amount_str
        .parse()
        .map_err(|err: ParseIntError| err.to_string())?;
    let direction_str = parts
        .get(0)
        .ok_or_else(|| format!("invalid command: no direction given"))?;
    match *direction_str {
        "forward" => Ok(Move::Forward(amount)),
        "up" => Ok(Move::Up(amount)),
        "down" => Ok(Move::Down(amount)),
        _ => Err(format!("unknown direction: {}", direction_str)),
    }
}

fn simulate_navigation(route: &[Move]) -> Position {
    route.iter().fold(Position::default(), |position, command| {
        position.navigate(*command)
    })
}

#[aoc(day2, part1)]
pub fn product_of_final_position(route: &[Move]) -> i32 {
    let final_position = simulate_navigation(route);
    final_position.x * final_position.depth
}

#[cfg(test)]
mod tests;
