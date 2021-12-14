//! # Day 13: Transparent Origami
//!
//! You reach another volcanically active part of the cave. It would be nice if
//! you could do some kind of thermal imaging so you could tell ahead of time
//! which caves are too hot to safely enter.
//!
//! Fortunately, the submarine seems to be equipped with a thermal camera! When
//! you activate it, you are greeted with:
//!
//! Congratulations on your purchase! To activate this infrared thermal imaging
//! camera system, please enter the code found on page 1 of the manual.
//! Apparently, the Elves have never used this feature. To your surprise, you
//! manage to find the manual; as you go to open it, page 1 falls out. It's a
//! large sheet of transparent paper! The transparent paper is marked with
//! random dots and includes instructions on how to fold it up (your puzzle
//! input). For example:
//!
//! ```text
//! 6,10
//! 0,14
//! 9,10
//! 0,3
//! 10,4
//! 4,11
//! 6,0
//! 6,12
//! 4,1
//! 0,13
//! 10,12
//! 3,4
//! 3,0
//! 8,4
//! 1,10
//! 2,14
//! 8,10
//! 9,0
//!
//! fold along y=7
//! fold along x=5
//! ```
//!
//! The first section is a list of dots on the transparent paper. 0,0 represents
//! the top-left coordinate. The first value, x, increases to the right. The
//! second value, y, increases downward. So, the coordinate 3,0 is to the right
//! of 0,0, and the coordinate 0,7 is below 0,0. The coordinates in this example
//! form the following pattern, where # is a dot on the paper and . is an empty,
//! unmarked position:
//!
//! ```text
//! ...#..#..#.
//! ....#......
//! ...........
//! #..........
//! ...#....#.#
//! ...........
//! ...........
//! ...........
//! ...........
//! ...........
//! .#....#.##.
//! ....#......
//! ......#...#
//! #..........
//! #.#........
//! ```
//!
//! Then, there is a list of fold instructions. Each instruction indicates a
//! line on the transparent paper and wants you to fold the paper up (for
//! horizontal y=... lines) or left (for vertical x=... lines). In this example,
//! the first fold instruction is fold along y=7, which designates the line
//! formed by all of the positions where y is 7 (marked here with -):
//!
//! ```text
//! ...#..#..#.
//! ....#......
//! ...........
//! #..........
//! ...#....#.#
//! ...........
//! ...........
//! -----------
//! ...........
//! ...........
//! .#....#.##.
//! ....#......
//! ......#...#
//! #..........
//! #.#........
//! ```
//!
//! Because this is a horizontal line, fold the bottom half up. Some of the dots
//! might end up overlapping after the fold is complete, but dots will never
//! appear exactly on a fold line. The result of doing this fold looks like
//! this:
//!
//! ```text
//! #.##..#..#.
//! #...#......
//! ......#...#
//! #...#......
//! .#.#..#.###
//! ...........
//! ...........
//! ```
//!
//! Now, only 17 dots are visible.
//!
//! Notice, for example, the two dots in the bottom left corner before the
//! transparent paper is folded; after the fold is complete, those dots appear
//! in the top left corner (at 0,0 and 0,1). Because the paper is transparent,
//! the dot just below them in the result (at 0,3) remains visible, as it can be
//! seen through the transparent paper.
//!
//! Also notice that some dots can end up overlapping; in this case, the dots
//! merge together and become a single dot.
//!
//! The second fold instruction is fold along x=5, which indicates this line:
//!
//! ```text
//! #.##.|#..#.
//! #...#|.....
//! .....|#...#
//! #...#|.....
//! .#.#.|#.###
//! .....|.....
//! .....|.....
//! ```
//!
//! Because this is a vertical line, fold left:
//!
//! ```text
//! #####
//! #...#
//! #...#
//! #...#
//! #####
//! .....
//! .....
//! ```
//!
//! The instructions made a square!
//!
//! The transparent paper is pretty big, so for now, focus on just completing
//! the first fold. After the first fold in the example above, 17 dots are
//! visible - dots that end up overlapping after the fold is completed count as
//! a single dot.
//!
//! How many dots are visible after completing just the first fold instruction
//! on your transparent paper?
//!
//! ## Part Two
//!
//! Finish folding the transparent paper according to the instructions. The
//! manual says the code is always eight capital letters.
//!
//! What code do you use to activate the infrared thermal imaging camera system?
//!
//! [Advent of Code 2021 - Day 13](https://adventofcode.com/2021/day/13)

use hashbrown::HashSet;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s
            .split_once(',')
            .ok_or_else(|| format!("invalid x,y coordinates: {}", s))?;
        let x = left.parse::<i32>().map_err(|err| err.to_string())?;
        let y = right.parse::<i32>().map_err(|err| err.to_string())?;
        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Fold {
    Left(i32),
    Up(i32),
}

impl FromStr for Fold {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, tail) = s
            .split_once("fold along ")
            .ok_or_else(|| format!("invalid fold entry: {}", s))?;
        let (left, right) = tail
            .split_once('=')
            .ok_or_else(|| format!("invalid fold entry: {}", tail))?;
        let position = right.parse::<i32>().map_err(|err| err.to_string())?;
        match left {
            "x" => Ok(Fold::Left(position)),
            "y" => Ok(Fold::Up(position)),
            _ => Err(format!("invalid fold direction: {}", left)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image(HashSet<Point>);

impl FromIterator<Point> for Image {
    fn from_iter<T: IntoIterator<Item = Point>>(iter: T) -> Self {
        Image(HashSet::from_iter(iter.into_iter()))
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        for p in self.0.iter() {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }
        for y in min_y..=max_y {
            let mut line = String::with_capacity((max_x - min_x + 1) as usize);
            for x in min_x..=max_x {
                if self.0.get(&Point { x, y }).is_some() {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            writeln!(f, "{}", line)?;
        }
        Ok(())
    }
}

impl Image {
    pub fn fold(&self, fold: Fold) -> Self {
        Self(match fold {
            Fold::Left(fx) => self
                .0
                .iter()
                .map(|p| {
                    let dx = p.x - fx;
                    let x = if dx > 0 { fx - dx } else { p.x };
                    Point { x, y: p.y }
                })
                .collect(),
            Fold::Up(fy) => self
                .0
                .iter()
                .map(|p| {
                    let dy = p.y - fy;
                    let y = if dy > 0 { fy - dy } else { p.y };
                    Point { x: p.x, y }
                })
                .collect(),
        })
    }

    pub fn dots(&self) -> impl Iterator<Item = Point> + '_ {
        self.0.iter().copied()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManPage1 {
    pub image: Image,
    pub folds: Vec<Fold>,
}

#[aoc_generator(day13)]
pub fn parse(input: &str) -> ManPage1 {
    let mut dots = HashSet::new();
    let mut folds = Vec::new();
    for line in input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
    {
        if line.starts_with("fold") {
            let fold = line
                .parse::<Fold>()
                .unwrap_or_else(|err| panic!("invalid fold entry: {}", err));
            folds.push(fold);
        } else {
            let point = line
                .parse::<Point>()
                .unwrap_or_else(|err| panic!("invalid point entry: {}", err));
            dots.insert(point);
        }
    }
    let image = Image::from_iter(dots);
    ManPage1 { image, folds }
}

#[aoc(day13, part1)]
pub fn solve_part1(man_page1: &ManPage1) -> usize {
    let first_fold = man_page1
        .folds
        .iter()
        .next()
        .copied()
        .expect("no fold instruction");
    let folded_image = man_page1.image.fold(first_fold);
    folded_image.dots().count()
}

#[aoc(day13, part2)]
pub fn solve_part2(man_page1: &ManPage1) -> String {
    let image = man_page1
        .folds
        .iter()
        .fold(man_page1.image.clone(), |image, fold| image.fold(*fold));
    //eprintln!("{}", image);
    image.to_string()
}

#[cfg(test)]
mod tests;
