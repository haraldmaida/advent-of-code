//! # Day 3: Toboggan Trajectory
//!
//! With the toboggan login problems resolved, you set off toward the airport.
//! While travel by toboggan might be easy, it's certainly not safe: there's
//! very minimal steering and the area is covered in trees. You'll need to see
//! which angles will take you near the fewest trees.
//!
//! Due to the local geology, trees in this area only grow on exact integer
//! coordinates in a grid. You make a map (your puzzle input) of the open
//! squares (.) and trees (#) you can see. For example:
//!
//! ```text
//! ..##.......
//! #...#...#..
//! .#....#..#.
//! ..#.#...#.#
//! .#...##..#.
//! ..#.##.....
//! .#.#.#....#
//! .#........#
//! #.##...#...
//! #...##....#
//! .#..#...#.#
//! ```
//!
//! These aren't the only trees, though; due to something you read about once
//! involving arboreal genetics and biome stability, the same pattern repeats to
//! the right many times:
//!
//! ```text
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........#.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...##....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! You start on the open square (.) in the top-left corner and need to reach
//! the bottom (below the bottom-most row on your map).
//!
//! The toboggan can only follow a few specific slopes (you opted for a cheaper
//! model that prefers rational numbers); start by counting all the trees you
//! would encounter for the slope right 3, down 1:
//!
//! From your starting position at the top-left, check the position that is
//! right 3 and down 1. Then, check the position that is right 3 and down 1 from
//! there, and so on until you go past the bottom of the map.
//!
//! The locations you'd check in the above example are marked here with O where
//! there was an open square and X where there was a tree:
//!
//! ```text
//! ..##.........##.........##.........##.........##.........##.......  --->
//! #..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
//! .#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
//! ..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
//! .#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
//! ..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
//! .#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
//! .#........#.#........X.#........#.#........#.#........#.#........#
//! #.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
//! #...##....##...##....##...#X....##...##....##...##....##...##....#
//! .#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
//! ```
//!
//! In this example, traversing the map using this slope would cause you to
//! encounter 7 trees.
//!
//! Starting at the top-left corner of your map and following a slope of right 3
//! and down 1, how many trees would you encounter?
//!
//! ## Part 2
//!
//! Time to check the rest of the slopes - you need to minimize the probability
//! of a sudden arboreal stop, after all.
//!
//! Determine the number of trees you would encounter if, for each of the
//! following slopes, you start at the top-left corner and traverse the map all
//! the way to the bottom:
//!
//! * Right 1, down 1.
//! * Right 3, down 1. (This is the slope you already checked.)
//! * Right 5, down 1.
//! * Right 7, down 1.
//! * Right 1, down 2.
//!
//! In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s)
//! respectively; multiplied together, these produce the answer 336.
//!
//! What do you get if you multiply together the number of trees encountered on
//! each of the listed slopes?
//!
//! [Advent of Code 2020 - Day 3](https://adventofcode.com/2020/day/3)

use std::mem;

pub const O: Tile = Tile::Open;
pub const X: Tile = Tile::Tree;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Open,
    Tree,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map {
    tiles: Vec<Vec<Tile>>,
    width: usize,
}

impl Map {
    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn tile(&self, position: Position) -> Tile {
        self.tiles[position.row][position.col % self.width]
    }

    pub fn walk(&self, slope: Slope, start: Position) -> Walker<'_> {
        Walker {
            map: &self,
            slope,
            start,
            current: start,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Slope {
    pub right: usize,
    pub down: usize,
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Map {
    let mut map_tiles = Vec::with_capacity(8);
    let mut row_tiles = Vec::with_capacity(8);
    for c in input.chars().chain("\n".chars()) {
        match c {
            '.' => {
                row_tiles.push(Tile::Open);
            }
            '#' => {
                row_tiles.push(Tile::Tree);
            }
            '\n' => {
                if !row_tiles.is_empty() {
                    map_tiles.push(mem::replace(&mut row_tiles, Vec::with_capacity(8)));
                }
            }
            _ => {
                if !c.is_whitespace() {
                    panic!("unrecognized character {:?} in input", c);
                }
            }
        }
    }
    let width = map_tiles.get(0).map(|row| row.len()).unwrap_or(0);
    Map {
        tiles: map_tiles,
        width,
    }
}

#[derive(Debug)]
pub struct Walker<'a> {
    map: &'a Map,
    slope: Slope,
    start: Position,
    current: Position,
}

impl<'a> Iterator for Walker<'a> {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.row >= self.map.height() {
            return None;
        }
        let next_tile = self.map.tile(self.current);
        self.current.col += self.slope.right;
        self.current.row += self.slope.down;
        Some(next_tile)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let num_steps = (self.map.height() - self.current.row) / self.slope.down;
        (num_steps, Some(num_steps))
    }
}

fn count_trees_walking_down_the_slope(slope: Slope, map: &Map) -> usize {
    map.walk(slope, Position { row: 0, col: 0 })
        .filter(|tile| *tile == Tile::Tree)
        .count()
}

#[allow(dead_code)]
fn count_trees_on_slope(slope: Slope, map: &Map) -> usize {
    (0..map.height())
        .step_by(slope.down)
        .zip((0..usize::max_value()).step_by(slope.right))
        .map(|(row, col)| map.tile(Position { row, col }))
        .filter(|tile| *tile == Tile::Tree)
        .count()
}

#[aoc(day3, part1)]
pub fn count_trees_on_slope_r3d1(map: &Map) -> usize {
    let slope = Slope { right: 3, down: 1 };
    count_trees_walking_down_the_slope(slope, map)
}

#[aoc(day3, part2)]
pub fn product_of_trees_on_multiple_slopes(map: &Map) -> usize {
    let slopes = [
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    slopes
        .iter()
        .map(|slope| count_trees_walking_down_the_slope(*slope, map))
        .product()
}

#[cfg(test)]
mod tests;
