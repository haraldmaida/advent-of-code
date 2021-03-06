//! # Day 3: Crossed Wires
//!
//! The gravity assist was successful, and you're well on your way to the Venus
//! refuelling station. During the rush back on Earth, the fuel management
//! system wasn't completely installed, so that's next on the priority list.
//!
//! Opening the front panel reveals a jumble of wires. Specifically, two wires
//! are connected to a central port and extend outward on a grid. You trace the
//! path each wire takes as it leaves the central port, one wire per line of
//! text (your puzzle input).
//!
//! The wires twist and turn, but the two wires occasionally cross paths. To fix
//! the circuit, you need to find the intersection point closest to the central
//! port. Because the wires are on a grid, use the Manhattan distance for this
//! measurement. While the wires do technically cross right at the central port
//! where they both start, this point does not count, nor does a wire count as
//! crossing with itself.
//!
//! For example, if the first wire's path is R8,U5,L5,D3, then starting from the
//! central port (o), it goes right 8, up 5, left 5, and finally down 3:
//!
//! ```text
//! ...........
//! ...........
//! ...........
//! ....+----+.
//! ....|....|.
//! ....|....|.
//! ....|....|.
//! .........|.
//! .o-------+.
//! ...........
//! ```
//!
//! Then, if the second wire's path is U7,R6,D4,L4, it goes up 7, right 6,
//! down 4, and left 4:
//!
//! ```text
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//! ```
//!
//! These wires cross at two locations (marked X), but the lower-left one is
//! closer to the central port: its distance is 3 + 3 = 6.
//!
//! Here are a few more examples:
//!
//! * ```text
//!   R75,D30,R83,U83,L12,D49,R71,U7,L72
//!   U62,R66,U55,R34,D71,R55,D58,R83
//!   ```
//!   = distance 159
//! * ```text
//!   R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//!   U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
//!   ```
//!   = distance 135
//!
//! What is the Manhattan distance from the central port to the closest
//! intersection?
//!
//! ## Part Two
//!
//! It turns out that this circuit is very timing-sensitive; you actually need
//! to minimize the signal delay.
//!
//! To do this, calculate the number of steps each wire takes to reach each
//! intersection; choose the intersection where the sum of both wires' steps is
//! lowest. If a wire visits a position on the grid multiple times, use the
//! steps value from the first time it visits that position when calculating the
//! total value of a specific intersection.
//!
//! The number of steps a wire takes is the total number of grid squares the
//! wire has entered to get to that location, including the intersection being
//! considered. Again consider the example from above:
//!
//! ```text
//! ...........
//! .+-----+...
//! .|.....|...
//! .|..+--X-+.
//! .|..|..|.|.
//! .|.-X--+.|.
//! .|..|....|.
//! .|.......|.
//! .o-------+.
//! ...........
//! ```
//!
//! In the above example, the intersection closest to the central port is
//! reached after `8+5+5+2 = 20` steps by the first wire and `7+6+4+3 = 20`
//! steps by the second wire for a total of `20+20 = 40` steps.
//!
//! However, the top-right intersection is better: the first wire takes only
//! `8+5+2 = 15` and the second wire takes only `7+6+2 = 15`, a total of
//! `15+15 = 30` steps.
//!
//! Here are the best steps for the extra examples from above:
//!
//! * ```text
//!   R75,D30,R83,U83,L12,D49,R71,U7,L72
//!   U62,R66,U55,R34,D71,R55,D58,R83
//!   ```
//!   = 610 steps
//! * ```text
//!   R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
//!   U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
//!   ```
//!   = 410 steps
//!
//! What is the fewest combined steps the wires must take to reach an
//! intersection?
//!
//! [Advent of Code 2019 - Day 3](https://adventofcode.com/2019/day/3)

use crate::year2019::day03::wire_grid::{Direction, Distance, Move, Point};
use hashbrown::{HashMap, HashSet};

mod wire_grid {
    use std::fmt::{self, Display};
    use std::str::FromStr;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    impl FromStr for Direction {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let dir_char = s.chars().next();
            match dir_char {
                Some('U') => Ok(Direction::Up),
                Some('D') => Ok(Direction::Down),
                Some('L') => Ok(Direction::Left),
                Some('R') => Ok(Direction::Right),
                Some(chr) => Err(format!("invalid direction: {}", chr)),
                None => Err(format!("missing direction")),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Distance(pub i32);

    impl FromStr for Distance {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let value = s
                .parse::<i32>()
                .map_err(|err| format!("invalid distance: {}", err))?;
            Ok(Distance(value))
        }
    }

    impl Display for Distance {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Distance {
        pub const ZERO: Self = Distance(0);
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Move {
        pub direction: Direction,
        pub distance: Distance,
    }

    impl FromStr for Move {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut chars = s.chars();
            let direction_str = chars
                .next()
                .map(|c| c.to_string())
                .unwrap_or_else(String::new);
            let direction = direction_str.parse::<Direction>()?;
            let distance_str = String::from_iter(chars);
            let distance = distance_str.parse::<Distance>()?;
            Ok(Self {
                direction,
                distance,
            })
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Point {
        pub const ZERO: Self = Self { x: 0, y: 0 };

        pub fn manhattan_distance(self, other: Point) -> Distance {
            Distance((self.x - other.x).abs() + (self.y - other.y).abs())
        }
    }
}

fn parse_line(line: &str) -> Result<Vec<Move>, String> {
    line.trim().split(',').map(|s| s.parse::<Move>()).collect()
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<Vec<Move>> {
    input
        .trim()
        .lines()
        .map(parse_line)
        .collect::<Result<_, String>>()
        .unwrap_or_else(|err| panic!("invalid input: {:?}", err))
}

fn wire_points(wire: &[Move]) -> HashMap<Point, u32> {
    let mut points = HashMap::new();
    let mut steps = 0;
    let mut last_point = Point::ZERO;
    for a_move in wire {
        for _ in 0..a_move.distance.0 {
            let point = match a_move.direction {
                Direction::Up => Point {
                    x: last_point.x,
                    y: last_point.y + 1,
                },
                Direction::Down => Point {
                    x: last_point.x,
                    y: last_point.y - 1,
                },
                Direction::Left => Point {
                    x: last_point.x - 1,
                    y: last_point.y,
                },
                Direction::Right => Point {
                    x: last_point.x + 1,
                    y: last_point.y,
                },
            };
            steps += 1;
            points.insert(point, steps);
            last_point = point;
        }
    }
    points
}

fn find_intersections(wire1: &[Move], wire2: &[Move]) -> HashSet<Point> {
    wire_points(wire1)
        .keys()
        .copied()
        .collect::<HashSet<_>>()
        .intersection(&wire_points(wire2).keys().copied().collect::<HashSet<_>>())
        .copied()
        .collect()
}

#[aoc(day3, part1)]
pub fn distance_to_closest_intersection(input: &[Vec<Move>]) -> Distance {
    let wire1 = &input[0];
    let wire2 = &input[1];
    find_intersections(wire1, wire2)
        .iter()
        .filter(|point| **point != Point::ZERO)
        .map(|point| Point::ZERO.manhattan_distance(*point))
        .min()
        .unwrap_or(Distance::ZERO)
}

#[aoc(day3, part2)]
pub fn minimal_signal_delay(input: &[Vec<Move>]) -> u32 {
    let wire1 = &input[0];
    let wire2 = &input[1];
    let wire1_point_delay_map = wire_points(&wire1);
    let wire2_point_delay_map = wire_points(&wire2);
    let wire1_points = wire1_point_delay_map
        .keys()
        .copied()
        .collect::<HashSet<_>>();
    let wire2_points = wire2_point_delay_map
        .keys()
        .copied()
        .collect::<HashSet<_>>();
    let intersections = wire1_points.intersection(&wire2_points);
    intersections
        .filter(|point| **point != Point::ZERO)
        .map(|point| {
            wire1_point_delay_map
                .get(point)
                .expect("intersection is not point on wire1")
                + wire2_point_delay_map
                    .get(point)
                    .expect("intersection is not point on wire2")
        })
        .min()
        .expect("no result found")
}

#[cfg(test)]
mod tests;
