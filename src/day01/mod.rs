//! # Day 1: The Tyranny of the Rocket Equation
//!
//! The Elves quickly load you into a spacecraft and prepare to launch.
//!
//! At the first Go / No Go poll, every Elf is Go until the Fuel Counter-Upper.
//! They haven't determined the amount of fuel required yet.
//!
//! Fuel required to launch a given module is based on its mass. Specifically,
//! to find the fuel required for a module, take its mass, divide by three,
//! round down, and subtract 2.
//!
//! For example:
//!
//! * For a mass of 12, divide by 3 and round down to get 4, then subtract 2 to
//!   get 2.
//! * For a mass of 14, dividing by 3 and rounding down still yields 4, so the
//!   fuel required is also 2.
//! * For a mass of 1969, the fuel required is 654.
//! * For a mass of 100756, the fuel required is 33583.
//!
//! The Fuel Counter-Upper needs to know the total fuel requirement. To find it,
//! individually calculate the fuel needed for the mass of each module (your
//! puzzle input), then add together all the fuel values.
//!
//! What is the sum of the fuel requirements for all of the modules on your
//! spacecraft?
//!
//! [Advent of Code 2019 - Day 1](https://adventofcode.com/2019/day/1)

use std::fmt;
use std::fmt::Display;
use std::ops::Add;
use std::str::FromStr;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Mass(u32);

impl Display for Mass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Mass {
    pub fn new(value: u32) -> Self {
        Self(value)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fuel(u32);

impl Display for Fuel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Fuel {
    pub const fn zero() -> Self {
        Self(0)
    }
}

impl Add<Self> for Fuel {
    type Output = Self;

    fn add(self, rhs: Fuel) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<Mass> {
    input
        .lines()
        .enumerate()
        .map(|(idx, text)| {
            Mass(u32::from_str(text.trim()).unwrap_or_else(|_| {
                panic!(
                    "input text at line {:03} is not an u32 but is {:?}",
                    idx + 1,
                    text
                )
            }))
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn fuel_requirements(input: &[Mass]) -> Fuel {
    input.iter().fold(Fuel::zero(), |acc_fuel, mass| {
        acc_fuel + calculate_fuel(mass)
    })
}

fn calculate_fuel(mass: &Mass) -> Fuel {
    Fuel(mass.0 / 3 - 2)
}
