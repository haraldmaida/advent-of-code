//! # Day 5: Hydrothermal Venture
//!
//! You come across a field of hydrothermal vents on the ocean floor! These
//! vents constantly produce large, opaque clouds, so it would be best to avoid
//! them if possible.
//!
//! They tend to form in lines; the submarine helpfully produces a list of
//! nearby lines of vents (your puzzle input) for you to review. For example:
//!
//! ```text
//! 0,9 -> 5,9
//! 8,0 -> 0,8
//! 9,4 -> 3,4
//! 2,2 -> 2,1
//! 7,0 -> 7,4
//! 6,4 -> 2,0
//! 0,9 -> 2,9
//! 3,4 -> 1,4
//! 0,0 -> 8,8
//! 5,5 -> 8,2
//! ```
//!
//! Each line of vents is given as a line segment in the format `x1,y1 -> x2,y2`
//! where `x1,y1` are the coordinates of one end the line segment and `x2,y2`
//! are the coordinates of the other end. These line segments include the points
//! at both ends. In other words:
//!
//! - An entry like `1,1 -> 1,3` covers points 1,1, 1,2, and 1,3.
//! - An entry like `9,7 -> 7,7` covers points 9,7, 8,7, and 7,7.
//!
//! For now, only consider horizontal and vertical lines: lines where either
//! `x1 = x2` or `y1 = y2`.
//!
//! So, the horizontal and vertical lines from the above list would produce the
//! following diagram:
//!
//! ```text
//! .......1..
//! ..1....1..
//! ..1....1..
//! .......1..
//! .112111211
//! ..........
//! ..........
//! ..........
//! ..........
//! 222111....
//! ```
//!
//! In this diagram, the top left corner is 0,0 and the bottom right corner is
//! 9,9. Each position is shown as the number of lines which cover that point
//! or `.` if no line covers that point. The top-left pair of 1s, for example,
//! comes from `2,2 -> 2,1`; the very bottom row is formed by the overlapping
//! lines `0,9 -> 5,9` and `0,9 -> 2,9`.
//!
//! To avoid the most dangerous areas, you need to determine the number of
//! points where at least two lines overlap. In the above example, this is
//! anywhere in the diagram with a 2 or larger - a total of 5 points.
//!
//! Consider only horizontal and vertical lines. At how many points do at least
//! two lines overlap?
//!
//! [Advent of Code 2021 - Day 5](https://adventofcode.com/2021/day/5)

use hashbrown::HashSet;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s
            .split_once(',')
            .ok_or_else(|| format!("no valid point coordinates: {}", s))?;
        let x = head.parse::<i32>().map_err(|err| err.to_string())?;
        let y = tail.parse::<i32>().map_err(|err| err.to_string())?;
        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LineSegment {
    pub start: Point,
    pub end: Point,
}

impl FromStr for LineSegment {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s
            .split_once(" -> ")
            .ok_or_else(|| format!("not a valid line segment definition: {}", s))?;
        let start = head.parse::<Point>()?;
        let end = tail.parse::<Point>()?;
        Ok(LineSegment { start, end })
    }
}

impl LineSegment {
    pub fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    pub fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

pub trait Intersects<Rhs = Self> {
    fn intersects(&self, other: &Rhs) -> HashSet<Point>;
}

impl Intersects for LineSegment {
    fn intersects(&self, other: &Self) -> HashSet<Point> {
        let mut intersections = HashSet::new();
        match (
            self.is_horizontal(),
            self.is_vertical(),
            other.is_horizontal(),
            other.is_vertical(),
        ) {
            (true, false, true, false) => {
                if self.start.y == other.start.y {
                    let y = self.start.y;
                    let start = self
                        .start
                        .x
                        .min(self.end.x)
                        .max(other.start.x.min(other.end.x));
                    let end = self
                        .start
                        .x
                        .max(self.end.x)
                        .min(other.start.x.max(other.end.x));
                    intersections.extend((start..=end).map(|x| Point { x, y }));
                }
            }
            (true, false, false, true) => {
                let min_x = self.start.x.min(self.end.x);
                let max_x = self.start.x.max(self.end.x);
                let min_y = other.start.y.min(other.end.y);
                let max_y = other.start.y.max(other.end.y);
                if min_x <= other.start.x
                    && other.start.x <= max_x
                    && min_y <= self.start.y
                    && self.start.y <= max_y
                {
                    intersections.insert(Point {
                        x: other.start.x,
                        y: self.start.y,
                    });
                }
            }
            (false, true, true, false) => {
                let min_x = other.start.x.min(other.end.x);
                let max_x = other.start.x.max(other.end.x);
                let min_y = self.start.y.min(self.end.y);
                let max_y = self.start.y.max(self.end.y);
                if min_x <= self.start.x
                    && self.start.x <= max_x
                    && min_y <= other.start.y
                    && other.start.y <= max_y
                {
                    intersections.insert(Point {
                        x: self.start.x,
                        y: other.start.y,
                    });
                }
            }
            (false, true, false, true) => {
                if self.start.x == other.start.x {
                    let x = self.start.x;
                    let start = self
                        .start
                        .y
                        .min(self.end.y)
                        .max(other.start.y.min(other.end.y));
                    let end = self
                        .start
                        .y
                        .max(self.end.y)
                        .min(other.start.y.max(other.end.y));
                    intersections.extend((start..=end).map(|y| Point { x, y }));
                }
            }
            (true, true, _, _) => unreachable!(),
            (_, _, true, true) => unreachable!(),
            (_, _, _, _) => {
                //ignore diagonal lines according too the puzzle definition
            }
        }
        intersections
    }
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Vec<LineSegment> {
    input
        .lines()
        .enumerate()
        .filter(|(_, line)| !line.is_empty())
        .map(|(idx, line)| {
            line.parse::<LineSegment>().expect(&format!(
                "L{}: invalid line segment: {}",
                idx + 1,
                line
            ))
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn count_points_at_least_two_lines_overlap(vent_lines: &[LineSegment]) -> usize {
    let mut intersections = HashSet::new();
    for (idx, line1) in vent_lines.iter().enumerate() {
        for line2 in vent_lines.iter().skip(idx + 1) {
            intersections.extend(line1.intersects(line2));
        }
    }
    intersections.len()
}

#[cfg(test)]
mod tests;
