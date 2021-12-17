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
//! ## Part Two
//!
//! Now that you know how to find low-risk paths in the cave, you can try to
//! find your way out.
//!
//! The entire cave is actually five times larger in both dimensions than you
//! thought; the area you originally scanned is just one tile in a 5x5 tile area
//! that forms the full map. Your original map tile repeats to the right and
//! downward; each time the tile repeats to the right or downward, all of its
//! risk levels are 1 higher than the tile immediately up or left of it.
//! However, risk levels above 9 wrap back around to 1. So, if your original map
//! had some position with a risk level of 8, then that same position on each of
//! the 25 total tiles would be as follows:
//!
//! ```text
//! 8 9 1 2 3
//! 9 1 2 3 4
//! 1 2 3 4 5
//! 2 3 4 5 6
//! 3 4 5 6 7
//! ```
//!
//! Each single digit above corresponds to the example position with a value of
//! 8 on the top-left tile. Because the full map is actually five times larger
//! in both dimensions, that position appears a total of 25 times, once in each
//! duplicated tile, with the values shown above.
//!
//! Here is the full five-times-as-large version of the first example above,
//! with the original map in the top left corner highlighted:
//!
//! ```text
//! 11637517422274862853338597396444961841755517295286
//! 13813736722492484783351359589446246169155735727126
//! 21365113283247622439435873354154698446526571955763
//! 36949315694715142671582625378269373648937148475914
//! 74634171118574528222968563933317967414442817852555
//! 13191281372421239248353234135946434524615754563572
//! 13599124212461123532357223464346833457545794456865
//! 31254216394236532741534764385264587549637569865174
//! 12931385212314249632342535174345364628545647573965
//! 23119445813422155692453326671356443778246755488935
//! 22748628533385973964449618417555172952866628316397
//! 24924847833513595894462461691557357271266846838237
//! 32476224394358733541546984465265719557637682166874
//! 47151426715826253782693736489371484759148259586125
//! 85745282229685639333179674144428178525553928963666
//! 24212392483532341359464345246157545635726865674683
//! 24611235323572234643468334575457944568656815567976
//! 42365327415347643852645875496375698651748671976285
//! 23142496323425351743453646285456475739656758684176
//! 34221556924533266713564437782467554889357866599146
//! 33859739644496184175551729528666283163977739427418
//! 35135958944624616915573572712668468382377957949348
//! 43587335415469844652657195576376821668748793277985
//! 58262537826937364893714847591482595861259361697236
//! 96856393331796741444281785255539289636664139174777
//! 35323413594643452461575456357268656746837976785794
//! 35722346434683345754579445686568155679767926678187
//! 53476438526458754963756986517486719762859782187396
//! 34253517434536462854564757396567586841767869795287
//! 45332667135644377824675548893578665991468977611257
//! 44961841755517295286662831639777394274188841538529
//! 46246169155735727126684683823779579493488168151459
//! 54698446526571955763768216687487932779859814388196
//! 69373648937148475914825958612593616972361472718347
//! 17967414442817852555392896366641391747775241285888
//! 46434524615754563572686567468379767857948187896815
//! 46833457545794456865681556797679266781878137789298
//! 64587549637569865174867197628597821873961893298417
//! 45364628545647573965675868417678697952878971816398
//! 56443778246755488935786659914689776112579188722368
//! 55172952866628316397773942741888415385299952649631
//! 57357271266846838237795794934881681514599279262561
//! 65719557637682166874879327798598143881961925499217
//! 71484759148259586125936169723614727183472583829458
//! 28178525553928963666413917477752412858886352396999
//! 57545635726865674683797678579481878968159298917926
//! 57944568656815567976792667818781377892989248891319
//! 75698651748671976285978218739618932984172914319528
//! 56475739656758684176786979528789718163989182927419
//! 67554889357866599146897761125791887223681299833479
//! ```
//!
//! Equipped with the full map, you can now find a path from the top left corner
//! to the bottom right corner with the lowest total risk.
//!
//! The total risk of this path is 315 (the starting position is still never
//! entered, so its risk is not counted).
//!
//! Using the full map, what is the lowest total risk of any path from the top
//! left to the bottom right?
//!
//! [Advent of Code 2021 - Day 15](https://adventofcode.com/2021/day/15)

use hashbrown::HashMap;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, VecDeque};
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

#[derive(Debug, Clone, Copy)]
struct Node {
    point: Point,
    risk: Risk,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.risk.eq(&other.risk)
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.risk.level.partial_cmp(&self.risk.level)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.level.cmp(&self.risk.level)
    }
}

fn find_path(start: Point, goal: Point, risk_map: &RiskMap) -> (Vec<Point>, Risk) {
    let mut open = BinaryHeap::new();
    let mut visited: HashMap<Point, (Risk, Option<Point>)> = HashMap::new();
    visited.insert(start, (Risk::ZERO, None));
    open.push(Node {
        point: start,
        risk: Risk::ZERO,
    });
    while let Some(Node {
        point: current,
        risk,
    }) = open.pop()
    {
        if current == goal {
            break;
        }

        for neighbor in neighbors(current, risk_map) {
            let new_risk = risk + risk_map.risk(neighbor).unwrap();
            if new_risk
                < *visited
                    .get(&neighbor)
                    .map(|(risk, _)| risk)
                    .unwrap_or(&Risk::MAX)
            {
                visited.insert(neighbor, (new_risk, Some(current)));
                open.push(Node {
                    point: neighbor,
                    risk: new_risk,
                });
            }
        }
    }

    let mut path = VecDeque::new();
    path.push_front(goal);
    let mut current = goal;
    while let Some(&Some(step)) = visited.get(&current).map(|(_, point)| point) {
        path.push_front(step);
        current = step;
    }

    let path = Vec::from_iter(path.into_iter());
    let (risk, _) = *visited.get(&goal).expect("no calculated risk for goal");
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
    // eprintln!("{}", risk_map);
    let start = risk_map.top_left();
    let goal = risk_map.bottom_right();
    let (_path, total_risk) = find_path(start, goal, risk_map);
    // eprintln!("{:?}", &_path);
    total_risk.level
}

fn expand_risk_map(factor: i32, risk_map: &RiskMap) -> RiskMap {
    let width = risk_map.bottom_right.x - risk_map.top_left.x + 1;
    let height = risk_map.bottom_right.y - risk_map.top_left.y + 1;
    let x_max = width * 5 - 1;
    let y_max = height * 5 - 1;
    let mut levels = HashMap::with_capacity(risk_map.levels.len());
    for y_tile in 0..factor {
        for x_tile in 0..factor {
            for (point, risk) in &risk_map.levels {
                let new_point = Point {
                    x: point.x + x_tile * width,
                    y: point.y + y_tile * height,
                };
                let mut new_risk_level = risk.level + x_tile as u32 + y_tile as u32;
                if new_risk_level > 9 {
                    new_risk_level = new_risk_level - 9;
                }
                levels.insert(
                    new_point,
                    Risk {
                        level: new_risk_level,
                    },
                );
            }
        }
    }

    RiskMap {
        levels,
        top_left: Point { x: 0, y: 0 },
        bottom_right: Point { x: x_max, y: y_max },
    }
}

#[aoc(day15, part2)]
pub fn solve_part2(risk_map: &RiskMap) -> u32 {
    let risk_map = expand_risk_map(5, risk_map);
    // eprintln!("{}", &risk_map);
    let start = risk_map.top_left();
    let goal = risk_map.bottom_right();
    let (_path, total_risk) = find_path(start, goal, &risk_map);
    // eprintln!("{:?}", &_path);
    total_risk.level
}

#[cfg(test)]
mod tests;
