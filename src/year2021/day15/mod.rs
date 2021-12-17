//! # Day 15: Chiton
//!
//! You've almost reached the exit of the cave, but the walls are getting closer
//! together. Your submarine can barely still fit, though; the main problem is
//! that the walls of the cave are covered in chitons, and it would be best not
//! to bump any of them.
//!
//! The cavern is large, but has a very low ceiling, restricting your motion to
//! two dimensions. The shape of the cavern resembles a square; a quick scan of
//! chiton density produces a map of risk level throughout the cave (your puzzle
//! input). For example:
//!
//! ```text
//! 1163751742
//! 1381373672
//! 2136511328
//! 3694931569
//! 7463417111
//! 1319128137
//! 1359912421
//! 3125421639
//! 1293138521
//! 2311944581
//! ```
//!
//! You start in the top left position, your destination is the bottom right
//! position, and you cannot move diagonally. The number at each position is its
//! risk level; to determine the total risk of an entire path, add up the risk
//! levels of each position you enter (that is, don't count the risk level of
//! your starting position unless you enter it; leaving it adds no risk to your
//! total).
//!
//! Your goal is to find a path with the lowest total risk. In this example, a
//! path with the lowest total risk is highlighted here:
//!
//! ```text
//! 1163751742
//! 1381373672
//! 2136511328
//! 3694931569
//! 7463417111
//! 1319128137
//! 1359912421
//! 3125421639
//! 1293138521
//! 2311944581
//! ```
//!
//! The total risk of this path is 40 (the starting position is never entered,
//! so its risk is not counted).
//!
//! What is the lowest total risk of any path from the top left to the bottom right?
//!
//! [Advent of Code 2021 - Day 15](https://adventofcode.com/2021/day/15)

use hashbrown::HashMap;
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Risk {
    pub level: u32,
}

impl Risk {
    const ZERO: Risk = Risk { level: 0 };
    const MAX: Risk = Risk { level: u32::MAX };
}

impl Add for Risk {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            level: self.level + rhs.level,
        }
    }
}

impl TryFrom<char> for Risk {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        value
            .to_digit(10)
            .map(|level| Self { level })
            .ok_or_else(|| format!("invalid digit {}", value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RiskMap {
    /// the risk levels of each point on a grid
    levels: HashMap<Point, Risk>,
    top_left: Point,
    bottom_right: Point,
}

impl FromStr for RiskMap {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut levels = HashMap::new();
        let mut x_max = 0;
        let mut y_max = 0;
        for (y, line) in s.lines().filter(|line| !line.is_empty()).enumerate() {
            y_max = y;
            for (x, c) in line.chars().enumerate() {
                x_max = x;
                let risk = Risk::try_from(c)?;
                levels.insert(
                    Point {
                        x: x as i32,
                        y: y as i32,
                    },
                    risk,
                );
            }
        }
        Ok(Self {
            levels,
            top_left: Point { x: 0, y: 0 },
            bottom_right: Point {
                x: x_max as i32,
                y: y_max as i32,
            },
        })
    }
}

impl Display for RiskMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in self.top_left.y..=self.bottom_right.y {
            for x in self.top_left().x..=self.bottom_right.x {
                write!(f, "{}", self.levels[&Point { x, y }].level)?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

impl RiskMap {
    pub fn top_left(&self) -> Point {
        self.top_left
    }

    pub fn bottom_right(&self) -> Point {
        self.bottom_right
    }

    pub fn get_point(&self, point: Point) -> Option<Point> {
        self.levels.get(&point).map(|_| point)
    }

    pub fn risk(&self, point: Point) -> Option<Risk> {
        self.levels.get(&point).copied()
    }
}

fn neighbors(Point { x, y }: Point, map: &RiskMap) -> Vec<Point> {
    const NEIGHBOR_OFFSETS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    NEIGHBOR_OFFSETS
        .iter()
        .filter_map(|offset| {
            map.get_point(Point {
                x: x + offset.0,
                y: y + offset.1,
            })
        })
        .collect()
}

fn find_path(start: Point, goal: Point, risk_map: &RiskMap) -> (Vec<Point>, Risk) {
    let mut open = Vec::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut risk_so_far: HashMap<Point, Risk> = HashMap::new();
    risk_so_far.insert(start, Risk::ZERO);
    open.push((start, Risk::ZERO));
    while let Some(index) = open
        .iter()
        .enumerate()
        .min_by_key(|(_, (_, risk))| risk)
        .map(|(idx, _)| idx)
    {
        let (current, risk) = open.remove(index);
        if current == goal {
            break;
        }

        for neighbor in neighbors(current, risk_map) {
            let new_risk = risk + risk_map.risk(neighbor).unwrap();
            if new_risk < *risk_so_far.get(&neighbor).unwrap_or(&Risk::MAX) {
                risk_so_far.insert(neighbor, new_risk);
                open.push((neighbor, new_risk));
                came_from.insert(neighbor, current);
            }
        }
    }

    let mut path = VecDeque::new();
    path.push_front(goal);
    let mut current = goal;
    while let Some(&step) = came_from.get(&current) {
        path.push_front(step);
        current = step;
    }

    let path = Vec::from_iter(path.into_iter());
    let risk = *risk_so_far.get(&goal).expect("no calculated risk for goal");
    (path, risk)
}

#[aoc_generator(day15)]
pub fn parse(input: &str) -> RiskMap {
    input
        .parse::<RiskMap>()
        .unwrap_or_else(|err| panic!("{}", err))
}

#[aoc(day15, part1)]
pub fn solve_part1(risk_map: &RiskMap) -> u32 {
    eprintln!("{}", risk_map);
    let start = risk_map.top_left();
    let goal = risk_map.bottom_right();
    let (path, total_risk) = find_path(start, goal, risk_map);
    eprintln!("{:?}", &path);
    total_risk.level
}

#[cfg(test)]
mod tests;
