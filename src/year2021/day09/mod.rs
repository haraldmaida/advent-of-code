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
//! [Advent of Code 2021 - Day 9](https://adventofcode.com/2021/day/9)

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

fn low_points(heightmap: &[Vec<u32>]) -> Vec<u32> {
    let mut low_points = Vec::new();
    let y_len = heightmap.len();
    let y_max = y_len - 1;
    for y in 0..y_len {
        let x_len = heightmap[y].len();
        let x_max = x_len - 1;
        for x in 0..x_len {
            let h = heightmap[y][x];
            let mut neighbors = Vec::with_capacity(4);
            if y > 0 {
                neighbors.push(heightmap[y - 1][x]);
            }
            if y < y_max {
                neighbors.push(heightmap[y + 1][x]);
            }
            if x > 0 {
                neighbors.push(heightmap[y][x - 1]);
            }
            if x < x_max {
                neighbors.push(heightmap[y][x + 1]);
            }
            if neighbors.iter().all(|n| *n > h) {
                low_points.push(h);
            }
        }
    }
    low_points
}

fn risk_level(height: u32) -> u32 {
    1 + height
}

#[aoc(day9, part1)]
pub fn sum_risk_level_at_low_points(heightmap: &[Vec<u32>]) -> u32 {
    low_points(heightmap).into_iter().map(risk_level).sum()
}

#[cfg(test)]
mod tests;
