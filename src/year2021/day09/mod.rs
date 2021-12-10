//! # Day 9: Smoke Basin
//!
//! These caves seem to be lava tubes. Parts are even still volcanically active;
//! small hydrothermal vents release smoke into the caves that slowly settles
//! like rain.
//!
//! If you can model how the smoke flows through the caves, you might be able to
//! avoid it and be that much safer. The submarine generates a heightmap of the
//! floor of the nearby caves for you (your puzzle input).
//!
//! Smoke flows to the lowest point of the area it's in. For example, consider
//! the following heightmap:
//!
//! ```text
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! Each number corresponds to the height of a particular location, where 9 is
//! the highest and 0 is the lowest a location can be.
//!
//! Your first goal is to find the low points - the locations that are lower
//! than any of its adjacent locations. Most locations have four adjacent
//! locations (up, down, left, and right); locations on the edge or corner of
//! the map have three or two adjacent locations, respectively. (Diagonal
//! locations do not count as adjacent.)
//!
//! In the above example, there are four low points, all highlighted: two are in
//! the first row (a 1 and a 0), one is in the third row (a 5), and one is in
//! the bottom row (also a 5). All other locations on the heightmap have some
//! lower adjacent location, and so are not low points.
//!
//! The risk level of a low point is 1 plus its height. In the above example,
//! the risk levels of the low points are 2, 1, 6, and 6. The sum of the risk
//! levels of all low points in the heightmap is therefore 15.
//!
//! Find all of the low points on your heightmap. What is the sum of the risk
//! levels of all low points on your heightmap?
//!
//! ## Part Two
//!
//! Next, you need to find the largest basins so you know what areas are most
//! important to avoid.
//!
//! A basin is all locations that eventually flow downward to a single low
//! point. Therefore, every low point has a basin, although some basins are very
//! small. Locations of height 9 do not count as being in any basin, and all
//! other locations will always be part of exactly one basin.
//!
//! The size of a basin is the number of locations within the basin, including
//! the low point. The example above has four basins.
//!
//! The top-left basin, size 3:
//!
//! ```text
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! The top-right basin, size 9:
//!
//! ```text
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! The middle basin, size 14:
//!
//! ```text
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! The bottom-right basin, size 9:
//!
//! ```text
//! 2199943210
//! 3987894921
//! 9856789892
//! 8767896789
//! 9899965678
//! ```
//!
//! Find the three largest basins and multiply their sizes together. In the
//! above example, this is 9 * 14 * 9 = 1134.
//!
//! What do you get if you multiply together the sizes of the three largest
//! basins?
//!
//! [Advent of Code 2021 - Day 9](https://adventofcode.com/2021/day/9)

use hashbrown::HashSet;

#[aoc_generator(day9)]
pub fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("not a valid digit"))
                .collect()
        })
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub h: u32,
}

fn low_points(heightmap: &[Vec<u32>]) -> Vec<Point> {
    let mut low_points = Vec::new();
    let y_len = heightmap.len();
    let y_max = y_len - 1;
    for y in 0..y_len {
        let x_len = heightmap[y].len();
        let x_max = x_len - 1;
        for x in 0..x_len {
            let h = heightmap[y][x];
            if y > 0 {
                let h1 = heightmap[y - 1][x];
                if h1 <= h {
                    continue;
                }
            }
            if y < y_max {
                let h1 = heightmap[y + 1][x];
                if h1 <= h {
                    continue;
                }
            }
            if x > 0 {
                let h1 = heightmap[y][x - 1];
                if h1 <= h {
                    continue;
                }
            }
            if x < x_max {
                let h1 = heightmap[y][x + 1];
                if h1 <= h {
                    continue;
                }
            }
            low_points.push(Point { x, y, h });
        }
    }
    low_points
}

fn risk_level(height: u32) -> u32 {
    1 + height
}

#[aoc(day9, part1)]
pub fn sum_risk_level_at_low_points(heightmap: &[Vec<u32>]) -> u32 {
    low_points(heightmap)
        .into_iter()
        .map(|lp| risk_level(lp.h))
        .sum()
}

fn basin_at_low_point(low_point: Point, heightmap: &[Vec<u32>]) -> HashSet<Point> {
    let y_max = heightmap.len() - 1;
    let x_max = heightmap[0].len() - 1;
    let mut basin = HashSet::new();
    let mut to_fill = vec![low_point];
    while let Some(point) = to_fill.pop() {
        basin.insert(point);
        let Point { x, y, h } = point;
        if y > 0 {
            let h1 = heightmap[y - 1][x];
            if h1 > h && h1 != 9 {
                to_fill.push(Point { x, y: y - 1, h: h1 });
            }
        }
        if y < y_max {
            let h1 = heightmap[y + 1][x];
            if h1 > h && h1 != 9 {
                to_fill.push(Point { x, y: y + 1, h: h1 });
            }
        }
        if x > 0 {
            let h1 = heightmap[y][x - 1];
            if h1 > h && h1 != 9 {
                to_fill.push(Point { x: x - 1, y, h: h1 });
            }
        }
        if x < x_max {
            let h1 = heightmap[y][x + 1];
            if h1 > h && h1 != 9 {
                to_fill.push(Point { x: x + 1, y, h: h1 });
            }
        }
    }
    basin
}

#[aoc(day9, part2)]
pub fn multiply_size_of_three_largest_basins(heightmap: &[Vec<u32>]) -> usize {
    let low_points = low_points(heightmap);
    let mut basins = low_points
        .iter()
        .map(|low_point| basin_at_low_point(*low_point, heightmap))
        .collect::<Vec<_>>();
    basins.sort_by(|basin1, basin2| basin2.len().cmp(&basin1.len()));
    basins[0].len() * basins[1].len() * basins[2].len()
}

#[cfg(test)]
mod tests;
