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
//! [Advent of Code 2019 - Day 3](https://adventofcode.com/2019/day/3)

use crate::day03::wire_grid::{Direction, Distance, Move, Point, Segment};
use std::iter::FromIterator;
use std::str::FromStr;

mod wire_grid {
    use std::fmt::{self, Display};
    use std::ops::Add;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Distance(i32);

    impl Display for Distance {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Distance {
        pub const fn zero() -> Self {
            Distance(0)
        }

        pub fn new(value: i32) -> Self {
            Distance(value)
        }

        pub fn val(self) -> i32 {
            self.0
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Move {
        direction: Direction,
        distance: Distance,
    }

    impl Move {
        pub fn new(direction: Direction, distance: Distance) -> Self {
            Self {
                direction,
                distance,
            }
        }

        pub fn into_segment(self) -> Segment {
            match self.direction {
                Direction::Up => Segment(
                    Point { x: 0, y: 0 },
                    Point {
                        x: 0,
                        y: self.distance.0,
                    },
                ),
                Direction::Down => Segment(
                    Point { x: 0, y: 0 },
                    Point {
                        x: 0,
                        y: -self.distance.0,
                    },
                ),
                Direction::Left => Segment(
                    Point { x: 0, y: 0 },
                    Point {
                        x: -self.distance.0,
                        y: 0,
                    },
                ),
                Direction::Right => Segment(
                    Point { x: 0, y: 0 },
                    Point {
                        x: self.distance.0,
                        y: 0,
                    },
                ),
            }
        }

        pub fn direction(self) -> Direction {
            self.direction
        }

        pub fn distance(self) -> Distance {
            self.distance
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        pub const fn zero() -> Self {
            Self { x: 0, y: 0 }
        }

        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }

        pub fn x(self) -> i32 {
            self.x
        }

        pub fn y(self) -> i32 {
            self.y
        }

        pub fn manhattan_distance(self, other: Point) -> Distance {
            Distance((self.x - other.x).abs() + (self.y - other.y).abs())
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Segment(pub Point, pub Point);

    impl Add<Point> for Segment {
        type Output = Segment;

        fn add(self, rhs: Point) -> Self::Output {
            Segment(
                Point {
                    x: self.0.x + rhs.x,
                    y: self.0.y + rhs.y,
                },
                Point {
                    x: self.1.x + rhs.x,
                    y: self.0.y + rhs.y,
                },
            )
        }
    }

    impl Segment {
        /// http://geomalgorithms.com/a05-_intersect-1.html
        pub fn intersection(self, other: Segment) -> Option<Point> {
            self.list_points().into_iter().find_map(|point1| {
                other
                    .list_points()
                    .into_iter()
                    .find(|point2| *point2 == point1)
            })
        }

        fn list_points(self) -> Vec<Point> {
            if self.0.x == self.1.x {
                let (min_y, max_y) = if self.0.y <= self.1.y {
                    (self.0.y, self.1.y)
                } else {
                    (self.1.y, self.0.y)
                };

                (min_y..=max_y).map(|y| Point { x: self.0.x, y }).collect()
            } else {
                let (min_x, max_x) = if self.0.x <= self.1.x {
                    (self.0.x, self.1.x)
                } else {
                    (self.1.x, self.0.x)
                };

                (min_x..=max_x).map(|x| Point { x, y: self.0.y }).collect()
            }
        }
    }
}

#[derive(Debug)]
pub enum ParseMoveError {
    InvalidMove(String),
    InvalidDirection(char),
    InvalidDistance(String),
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Vec<Vec<Move>> {
    input
        .trim()
        .lines()
        .map(parse_line)
        .collect::<Result<_, ParseMoveError>>()
        .unwrap_or_else(|err| panic!("invalid input: {:?}", err))
}

fn parse_line(line: &str) -> Result<Vec<Move>, ParseMoveError> {
    line.trim().split(',').map(parse_move).collect()
}

fn parse_move(input: &str) -> Result<Move, ParseMoveError> {
    let mut chars = input.trim().chars();
    match chars.next() {
        Some('U') => Ok(Direction::Up),
        Some('D') => Ok(Direction::Down),
        Some('L') => Ok(Direction::Left),
        Some('R') => Ok(Direction::Right),
        Some(chr) => Err(ParseMoveError::InvalidDirection(chr)),
        None => Err(ParseMoveError::InvalidMove(input.into())),
    }
    .and_then(|direction| {
        let str_val = String::from_iter(chars);
        if let Ok(distance) = i32::from_str(&str_val) {
            Ok(Move::new(direction, Distance::new(distance)))
        } else {
            Err(ParseMoveError::InvalidDistance(str_val.into()))
        }
    })
}

fn map_to_segments(line: &[Move]) -> Vec<Segment> {
    let mut segments = Vec::with_capacity(line.len());
    let mut last_point = Point::zero();
    for a_move in line {
        let next_point = match a_move.direction() {
            Direction::Up => Point::new(last_point.x(), last_point.y() + a_move.distance().val()),
            Direction::Down => Point::new(last_point.x(), last_point.y() - a_move.distance().val()),
            Direction::Left => Point::new(last_point.x() - a_move.distance().val(), last_point.y()),
            Direction::Right => {
                Point::new(last_point.x() + a_move.distance().val(), last_point.y())
            }
        };
        segments.push(Segment(last_point, next_point));
        last_point = next_point;
    }
    segments
}

#[aoc(day3, part1)]
pub fn distance_to_closest_intersection(input: &[Vec<Move>]) -> Distance {
    let wire1 = map_to_segments(&input[0]);
    let wire2 = map_to_segments(&input[1]);

    find_intersections(&wire1, &wire2)
        .iter()
        .filter(|point| **point != Point::zero())
        .map(|point| Point::zero().manhattan_distance(*point))
        .min()
        .unwrap_or(Distance::zero())
}

fn find_intersections(wire1: &[Segment], wire2: &[Segment]) -> Vec<Point> {
    wire1
        .into_iter()
        .flat_map(|segment1| {
            wire2
                .into_iter()
                .filter_map(move |segment2| segment2.intersection(*segment1))
        })
        .collect()
}

#[cfg(test)]
mod tests;
