//! # Day 12: Rain Risk
//!
//! Your ferry made decent progress toward the island, but the storm came in
//! faster than anyone expected. The ferry needs to take evasive actions!
//!
//! Unfortunately, the ship's navigation computer seems to be malfunctioning;
//! rather than giving a route directly to safety, it produced extremely
//! circuitous instructions. When the captain uses the PA system to ask if
//! anyone can help, you quickly volunteer.
//!
//! The navigation instructions (your puzzle input) consists of a sequence of
//! single-character actions paired with integer input values. After staring at
//! them for a few minutes, you work out what they probably mean:
//!
//! * Action N means to move north by the given value.
//! * Action S means to move south by the given value.
//! * Action E means to move east by the given value.
//! * Action W means to move west by the given value.
//! * Action L means to turn left the given number of degrees.
//! * Action R means to turn right the given number of degrees.
//! * Action F means to move forward by the given value in the direction the
//!   ship is currently facing.
//!
//! The ship starts by facing east. Only the L and R actions change the
//! direction the ship is facing. (That is, if the ship is facing east and the
//! next instruction is N10, the ship would move north 10 units, but would still
//! move east if the following action were F.)
//!
//! For example:
//!
//! ```text
//! F10
//! N3
//! F7
//! R90
//! F11
//! ```
//!
//! These instructions would be handled as follows:
//!
//! * F10 would move the ship 10 units east (because the ship starts by facing
//!   east) to east 10, north 0.
//! * N3 would move the ship 3 units north to east 10, north 3.
//! * F7 would move the ship another 7 units east (because the ship is still
//!   facing east) to east 17, north 3.
//! * R90 would cause the ship to turn right by 90 degrees and face south; it
//!   remains at east 17, north 3.
//! * F11 would move the ship 11 units south to east 17, south 8.
//!
//! At the end of these instructions, the ship's Manhattan distance (sum of the
//! absolute values of its east/west position and its north/south position) from
//! its starting position is 17 + 8 = 25.
//!
//! Figure out where the navigation instructions lead. What is the Manhattan
//! distance between that location and the ship's starting position?
//!
//! ## Part 2
//!
//! Before you can give the destination to the captain, you realize that the
//! actual action meanings were printed on the back of the instructions the
//! whole time.
//!
//! Almost all of the actions indicate how to move a waypoint which is relative
//! to the ship's position:
//!
//! * Action N means to move the waypoint north by the given value.
//! * Action S means to move the waypoint south by the given value.
//! * Action E means to move the waypoint east by the given value.
//! * Action W means to move the waypoint west by the given value.
//! * Action L means to rotate the waypoint around the ship left
//!   (counter-clockwise) the given number of degrees.
//! * Action R means to rotate the waypoint around the ship right
//!   (clockwise) the given number of degrees.
//! * Action F means to move forward to the waypoint a number of times equal to
//!   the given value.
//!
//! The waypoint starts 10 units east and 1 unit north relative to the ship. The
//! waypoint is relative to the ship; that is, if the ship moves, the waypoint
//! moves with it.
//!
//! For example, using the same instructions as above:
//!
//! * F10 moves the ship to the waypoint 10 times (a total of 100 units east and
//!   10 units north), leaving the ship at east 100, north 10. The waypoint
//!   stays 10 units east and 1 unit north of the ship.
//! * N3 moves the waypoint 3 units north to 10 units east and 4 units north of
//!   the ship. The ship remains at east 100, north 10.
//! * F7 moves the ship to the waypoint 7 times (a total of 70 units east and 28
//!   units north), leaving the ship at east 170, north 38. The waypoint stays
//!   10 units east and 4 units north of the ship.
//! * R90 rotates the waypoint around the ship clockwise 90 degrees, moving it
//!   to 4 units east and 10 units south of the ship. The ship remains at east
//!   170, north 38.
//! * F11 moves the ship to the waypoint 11 times (a total of 44 units east and
//!   110 units south), leaving the ship at east 214, south 72. The waypoint
//!   stays 4 units east and 10 units south of the ship.
//!
//! After these operations, the ship's Manhattan distance from its starting
//! position is 214 + 72 = 286.
//!
//! Figure out where the navigation instructions actually lead. What is the
//! Manhattan distance between that location and the ship's starting position?
//!
//! [Advent of Code 2020 - Day 12](https://adventofcode.com/2020/day/12)

use std::ops::{Add, AddAssign};
use std::str::FromStr;

pub type Mile = i32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Navigate {
    Forward(Mile),
    North(Mile),
    East(Mile),
    South(Mile),
    West(Mile),
    RotateLeft(Angle),
    RotateRight(Angle),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Angle {
    D90,
    D180,
    D270,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move {
    pub distance: Mile,
    pub direction: Direction,
}

impl From<(Mile, Direction)> for Move {
    fn from((distance, direction): (Mile, Direction)) -> Self {
        Self {
            distance,
            direction,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub north: Mile,
    pub east: Mile,
}

impl Position {
    pub fn manhattan_distance(self, other: Self) -> Mile {
        (self.east - other.east).abs() + (self.north - other.north).abs()
    }
}

impl Add<Move> for Position {
    type Output = Self;

    fn add(
        self,
        Move {
            distance,
            direction,
        }: Move,
    ) -> Self::Output {
        match direction {
            Direction::North => Self {
                north: self.north + distance,
                east: self.east,
            },
            Direction::East => Self {
                north: self.north,
                east: self.east + distance,
            },
            Direction::South => Self {
                north: self.north - distance,
                east: self.east,
            },
            Direction::West => Self {
                north: self.north,
                east: self.east - distance,
            },
        }
    }
}

impl AddAssign<Move> for Position {
    fn add_assign(
        &mut self,
        Move {
            distance,
            direction,
        }: Move,
    ) {
        match direction {
            Direction::North => {
                self.north += distance;
            }
            Direction::East => {
                self.east += distance;
            }
            Direction::South => {
                self.north -= distance;
            }
            Direction::West => {
                self.east -= distance;
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn rotate_cw(self, angle: Angle) -> Self {
        match self {
            Direction::North => match angle {
                Angle::D90 => Direction::East,
                Angle::D180 => Direction::South,
                Angle::D270 => Direction::West,
            },
            Direction::East => match angle {
                Angle::D90 => Direction::South,
                Angle::D180 => Direction::West,
                Angle::D270 => Direction::North,
            },
            Direction::South => match angle {
                Angle::D90 => Direction::West,
                Angle::D180 => Direction::North,
                Angle::D270 => Direction::East,
            },
            Direction::West => match angle {
                Angle::D90 => Direction::North,
                Angle::D180 => Direction::East,
                Angle::D270 => Direction::South,
            },
        }
    }

    pub fn rotate_ccw(self, angle: Angle) -> Self {
        match self {
            Direction::North => match angle {
                Angle::D90 => Direction::West,
                Angle::D180 => Direction::South,
                Angle::D270 => Direction::East,
            },
            Direction::East => match angle {
                Angle::D90 => Direction::North,
                Angle::D180 => Direction::West,
                Angle::D270 => Direction::South,
            },
            Direction::South => match angle {
                Angle::D90 => Direction::East,
                Angle::D180 => Direction::North,
                Angle::D270 => Direction::West,
            },
            Direction::West => match angle {
                Angle::D90 => Direction::South,
                Angle::D180 => Direction::East,
                Angle::D270 => Direction::North,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ship {
    pub position: Position,
    pub facing: Direction,
    pub waypoint: Position,
}

impl Default for Ship {
    fn default() -> Self {
        Self {
            position: Position { north: 0, east: 0 },
            facing: Direction::East,
            waypoint: Position { north: 1, east: 10 },
        }
    }
}

impl Ship {
    pub fn move_(&mut self, miles: Mile, direction: Direction) {
        self.position += Move::from((miles, direction))
    }

    pub fn turn_left(&mut self, angle: Angle) {
        self.facing = self.facing.rotate_ccw(angle);
    }

    pub fn turn_right(&mut self, angle: Angle) {
        self.facing = self.facing.rotate_cw(angle);
    }

    pub fn rotate_waypoint_cw(&mut self, angle: Angle) {
        let north = self.waypoint.north;
        let east = self.waypoint.east;
        match angle {
            Angle::D90 => {
                self.waypoint.north = -east;
                self.waypoint.east = north;
            }
            Angle::D180 => {
                self.waypoint.north = -north;
                self.waypoint.east = -east;
            }
            Angle::D270 => {
                self.waypoint.north = east;
                self.waypoint.east = -north;
            }
        }
    }

    pub fn rotate_waypoint_ccw(&mut self, angle: Angle) {
        let north = self.waypoint.north;
        let east = self.waypoint.east;
        match angle {
            Angle::D90 => {
                self.waypoint.north = east;
                self.waypoint.east = -north;
            }
            Angle::D180 => {
                self.waypoint.north = -north;
                self.waypoint.east = -east;
            }
            Angle::D270 => {
                self.waypoint.north = -east;
                self.waypoint.east = north;
            }
        }
    }
}

pub trait AutoPilot {
    fn apply(instruction: Navigate, ship: &mut Ship);

    fn execute(instructions: &[Navigate], ship: &mut Ship) {
        instructions
            .into_iter()
            .for_each(|inst| Self::apply(*inst, ship))
    }
}

#[aoc_generator(day12)]
pub fn parse_navigation_instructions(input: &str) -> Vec<Navigate> {
    let mut instructions = Vec::new();

    for (lno, line) in input.lines().enumerate() {
        if line.is_empty() {
            continue;
        }

        let amount = i32::from_str(&line[1..]).unwrap_or_else(|err| {
            panic!(
                "line {}: not a number {:?}; reason: {}",
                lno + 1,
                &line[1..],
                err
            )
        });

        let instruction = match line.chars().nth(0).unwrap() {
            'N' => Navigate::North(amount),
            'S' => Navigate::South(amount),
            'E' => Navigate::East(amount),
            'W' => Navigate::West(amount),
            'L' => {
                if let Some(angle) = parse_angle(amount) {
                    Navigate::RotateLeft(angle)
                } else {
                    panic!(
                        "line {}: invalid amount for turn left: {:?}",
                        lno + 1,
                        amount
                    );
                }
            }
            'R' => {
                if let Some(angle) = parse_angle(amount) {
                    Navigate::RotateRight(angle)
                } else {
                    panic!(
                        "line {}: invalid amount for turn left: {:?}",
                        lno + 1,
                        amount
                    );
                }
            }
            'F' => Navigate::Forward(amount),
            _ => {
                panic!(
                    "line {}: unrecognized instruction code in '{:?}'",
                    lno + 1,
                    line
                );
            }
        };

        instructions.push(instruction);
    }
    instructions
}

fn parse_angle(amount: i32) -> Option<Angle> {
    match amount {
        90 => Some(Angle::D90),
        180 => Some(Angle::D180),
        270 => Some(Angle::D270),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AutoPilot1;

impl AutoPilot for AutoPilot1 {
    fn apply(instruction: Navigate, ship: &mut Ship) {
        match instruction {
            Navigate::Forward(miles) => ship.move_(miles, ship.facing),
            Navigate::North(miles) => ship.move_(miles, Direction::North),
            Navigate::East(miles) => ship.move_(miles, Direction::East),
            Navigate::South(miles) => ship.move_(miles, Direction::South),
            Navigate::West(miles) => ship.move_(miles, Direction::West),
            Navigate::RotateLeft(angle) => ship.turn_left(angle),
            Navigate::RotateRight(angle) => ship.turn_right(angle),
        }
    }
}

#[aoc(day12, part1)]
pub fn part1_distance_to_final_position(instructions: &[Navigate]) -> Mile {
    let mut ship = Ship::default();
    let start_position = ship.position;
    AutoPilot1::execute(instructions, &mut ship);
    ship.position.manhattan_distance(start_position)
}

#[derive(Debug, Clone, Copy)]
pub struct AutoPilot2;

impl AutoPilot for AutoPilot2 {
    fn apply(instruction: Navigate, ship: &mut Ship) {
        match instruction {
            Navigate::Forward(times) => {
                ship.move_(times * ship.waypoint.north, Direction::North);
                ship.move_(times * ship.waypoint.east, Direction::East);
            }
            Navigate::North(miles) => ship.waypoint += Move::from((miles, Direction::North)),
            Navigate::East(miles) => ship.waypoint += Move::from((miles, Direction::East)),
            Navigate::South(miles) => ship.waypoint += Move::from((miles, Direction::South)),
            Navigate::West(miles) => ship.waypoint += Move::from((miles, Direction::West)),
            Navigate::RotateLeft(angle) => ship.rotate_waypoint_ccw(angle),
            Navigate::RotateRight(angle) => ship.rotate_waypoint_cw(angle),
        }
    }
}

#[aoc(day12, part2)]
pub fn part2_distance_to_final_position(instructions: &[Navigate]) -> Mile {
    let mut ship = Ship::default();
    let start_position = ship.position;
    AutoPilot2::execute(instructions, &mut ship);
    ship.position.manhattan_distance(start_position)
}

#[cfg(test)]
mod tests;
