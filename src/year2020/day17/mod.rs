//! # Day 17: Conway Cubes
//!
//! As your flight slowly drifts through the sky, the Elves at the Mythical
//! Information Bureau at the North Pole contact you. They'd like some help
//! debugging a malfunctioning experimental energy source aboard one of their
//! super-secret imaging satellites.
//!
//! The experimental energy source is based on cutting-edge technology: a set of
//! Conway Cubes contained in a pocket dimension! When you hear it's having
//! problems, you can't help but agree to take a look.
//!
//! The pocket dimension contains an infinite 3-dimensional grid. At every
//! integer 3-dimensional coordinate (x,y,z), there exists a single cube which
//! is either active or inactive.
//!
//! In the initial state of the pocket dimension, almost all cubes start
//! inactive. The only exception to this is a small flat region of cubes (your
//! puzzle input); the cubes in this region start in the specified active (#) or
//! inactive (.) state.
//!
//! The energy source then proceeds to boot up by executing six cycles.
//!
//! Each cube only ever considers its neighbors: any of the 26 other cubes where
//! any of their coordinates differ by at most 1. For example, given the cube at
//! `x=1,y=2,z=3`, its neighbors include the cube at `x=2,y=2,z=2`, the cube at
//! `x=0,y=2,z=3`, and so on.
//!
//! During a cycle, all cubes simultaneously change their state according to the
//! following rules:
//!
//! * If a cube is active and exactly 2 or 3 of its neighbors are also active,
//!   the cube remains active. Otherwise, the cube becomes inactive.
//! * If a cube is inactive but exactly 3 of its neighbors are active, the cube
//!   becomes active. Otherwise, the cube remains inactive.
//!
//! The engineers responsible for this experimental energy source would like you
//! to simulate the pocket dimension and determine what the configuration of
//! cubes should be at the end of the six-cycle boot process.
//!
//! For example, consider the following initial state:
//!
//! ```text
//! .#.
//! ..#
//! ###
//! ```
//!
//! Even though the pocket dimension is 3-dimensional, this initial state
//! represents a small 2-dimensional slice of it. (In particular, this initial
//! state defines a 3x3x1 region of the 3-dimensional space.)
//!
//! Simulating a few cycles from this initial state produces the following
//! configurations, where the result of each cycle is shown layer-by-layer at
//! each given z coordinate (and the frame of view follows the active cells in
//! each cycle):
//!
//! ```text
//! Before any cycles:
//!
//! z=0
//! .#.
//! ..#
//! ###
//!
//!
//! After 1 cycle:
//!
//! z=-1
//! #..
//! ..#
//! .#.
//!
//! z=0
//! #.#
//! .##
//! .#.
//!
//! z=1
//! #..
//! ..#
//! .#.
//!
//!
//! After 2 cycles:
//!
//! z=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1
//! ..#..
//! .#..#
//! ....#
//! .#...
//! .....
//!
//! z=0
//! ##...
//! ##...
//! #....
//! ....#
//! .###.
//!
//! z=1
//! ..#..
//! .#..#
//! ....#
//! .#...
//! .....
//!
//! z=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//!
//! After 3 cycles:
//!
//! z=-2
//! .......
//! .......
//! ..##...
//! ..###..
//! .......
//! .......
//! .......
//!
//! z=-1
//! ..#....
//! ...#...
//! #......
//! .....##
//! .#...#.
//! ..#.#..
//! ...#...
//!
//! z=0
//! ...#...
//! .......
//! #......
//! .......
//! .....##
//! .##.#..
//! ...#...
//!
//! z=1
//! ..#....
//! ...#...
//! #......
//! .....##
//! .#...#.
//! ..#.#..
//! ...#...
//!
//! z=2
//! .......
//! .......
//! ..##...
//! ..###..
//! .......
//! .......
//! .......
//! ```
//!
//! After the full six-cycle boot process completes, 112 cubes are left in the
//! active state.
//!
//! Starting with your given initial configuration, simulate six cycles. How
//! many cubes are left in the active state after the sixth cycle?
//!
//! ## Part 2
//!
//! For some reason, your simulated results don't match what the experimental
//! energy source engineers expected. Apparently, the pocket dimension actually
//! has four spatial dimensions, not three.
//!
//! The pocket dimension contains an infinite 4-dimensional grid. At every
//! integer 4-dimensional coordinate (x,y,z,w), there exists a single cube
//! (really, a hypercube) which is still either active or inactive.
//!
//! Each cube only ever considers its neighbors: any of the 80 other cubes where
//! any of their coordinates differ by at most 1. For example, given the cube at
//! `x=1,y=2,z=3,w=4`, its neighbors include the cube at `x=2,y=2,z=3,w=3`, the
//! cube at x=0,y=2,z=3,w=4, and so on.
//!
//! The initial state of the pocket dimension still consists of a small flat
//! region of cubes. Furthermore, the same rules for cycle updating still apply:
//! during each cycle, consider the number of active neighbors of each cube.
//!
//! For example, consider the same initial state as in the example above. Even
//! though the pocket dimension is 4-dimensional, this initial state represents
//! a small 2-dimensional slice of it. (In particular, this initial state
//! defines a 3x3x1x1 region of the 4-dimensional space.)
//!
//! Simulating a few cycles from this initial state produces the following
//! configurations, where the result of each cycle is shown layer-by-layer at
//! each given z and w coordinate:
//!
//! ```text
//! Before any cycles:
//!
//! z=0, w=0
//! .#.
//! ..#
//! ###
//!
//!
//! After 1 cycle:
//!
//! z=-1, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=0, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=1, w=-1
//! #..
//! ..#
//! .#.
//!
//! z=-1, w=0
//! #..
//! ..#
//! .#.
//!
//! z=0, w=0
//! #.#
//! .##
//! .#.
//!
//! z=1, w=0
//! #..
//! ..#
//! .#.
//!
//! z=-1, w=1
//! #..
//! ..#
//! .#.
//!
//! z=0, w=1
//! #..
//! ..#
//! .#.
//!
//! z=1, w=1
//! #..
//! ..#
//! .#.
//!
//!
//! After 2 cycles:
//!
//! z=-2, w=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1, w=-2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=-2
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=1, w=-2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=-2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-2, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-1, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=-1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-2, w=0
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=-1, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=0
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=0
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=-2, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-1, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=1, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=1
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=-2, w=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//!
//! z=-1, w=2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=0, w=2
//! ###..
//! ##.##
//! #...#
//! .#..#
//! .###.
//!
//! z=1, w=2
//! .....
//! .....
//! .....
//! .....
//! .....
//!
//! z=2, w=2
//! .....
//! .....
//! ..#..
//! .....
//! .....
//! ```
//!
//! After the full six-cycle boot process completes, 848 cubes are left in the
//! active state.
//!
//! Starting with your given initial configuration, simulate six cycles in a
//! 4-dimensional space. How many cubes are left in the active state after the
//! sixth cycle?
//!
//! [Advent of Code 2020 - Day 17](https://adventofcode.com/2020/day/17)

use fxhash::FxBuildHasher;
use hashbrown::HashSet;
use std::hash::Hash;
use std::iter::FromIterator;
use std::mem;
use std::ops::Deref;

pub type Coord = i64;

pub type D3 = [Coord; 3];
pub type D4 = [Coord; 4];

pub type Position3D = Position<D3>;
pub type Position4D = Position<D4>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position<D>(D);

impl<D> Default for Position<D>
where
    D: Default,
{
    fn default() -> Self {
        Position(D::default())
    }
}

impl<D> From<D> for Position<D> {
    fn from(coords: D) -> Self {
        Position(coords)
    }
}

impl<D> Deref for Position<D> {
    type Target = D;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Active,
    Inactive,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PocketDimension<D>
where
    D: Eq + Hash,
{
    active_cubes: HashSet<Position<D>, FxBuildHasher>,
    next_gen: HashSet<Position<D>, FxBuildHasher>,
}

impl From<&PocketDimension<D3>> for PocketDimension<D4> {
    fn from(value: &PocketDimension<D3>) -> Self {
        let active_cubes = HashSet::from_iter(value.active_cubes.iter().map(|p3| {
            (0..p3.0.len()).fold(Position::from([0; 4]), |mut p4, i| {
                p4.0[i] = p3.0[i];
                p4
            })
        }));
        let next_gen = HashSet::from_iter(value.next_gen.iter().map(|p3| {
            (0..p3.0.len()).fold(Position::from([0; 4]), |mut p4, i| {
                p4.0[i] = p3.0[i];
                p4
            })
        }));
        Self {
            active_cubes,
            next_gen,
        }
    }
}

impl<D> PocketDimension<D>
where
    D: Eq + Hash,
{
    pub fn get(&self, position: Position<D>) -> State {
        if self.active_cubes.contains(&position) {
            State::Active
        } else {
            State::Inactive
        }
    }

    pub fn num_active(&self) -> usize {
        self.active_cubes.len()
    }

    pub fn neighbors(&self, position: Position<D>) -> Neighbors<'_, D> {
        Neighbors {
            active_cubes: &self.active_cubes,
            position,
            current: 0,
        }
    }
}

impl PocketDimension<D3> {
    pub fn region(&self) -> Region<D3> {
        let (mut min_pos, mut max_pos) = if self.active_cubes.is_empty() {
            ([0; 3], [0; 3])
        } else {
            ([Coord::max_value(); 3], [Coord::min_value(); 3])
        };
        for pos in self.active_cubes.iter() {
            for d in 0..pos.len() {
                if pos[d] < min_pos[d] {
                    min_pos[d] = pos[d];
                }
                if pos[d] > max_pos[d] {
                    max_pos[d] = pos[d];
                }
            }
        }
        for d in 0..3 {
            min_pos[d] -= 1;
            max_pos[d] += 1;
        }
        Region {
            max_pos,
            min_pos,
            current: min_pos,
        }
    }

    pub fn evolve(&mut self) {
        self.next_gen.clear();
        for curr_pos in self.region() {
            match self
                .neighbors(curr_pos)
                .filter(|s| *s == State::Active)
                .count()
            {
                3 => {
                    self.next_gen.insert(curr_pos);
                }
                2 => {
                    if self.active_cubes.contains(&curr_pos) {
                        self.next_gen.insert(curr_pos);
                    }
                }
                _ => {}
            }
        }
        mem::swap(&mut self.active_cubes, &mut self.next_gen);
    }
}

impl PocketDimension<D4> {
    pub fn region(&self) -> Region<D4> {
        let (mut min_pos, mut max_pos) = if self.active_cubes.is_empty() {
            ([0; 4], [0; 4])
        } else {
            ([Coord::max_value(); 4], [Coord::min_value(); 4])
        };
        for pos in self.active_cubes.iter() {
            for d in 0..pos.len() {
                if pos[d] < min_pos[d] {
                    min_pos[d] = pos[d];
                }
                if pos[d] > max_pos[d] {
                    max_pos[d] = pos[d];
                }
            }
        }
        for d in 0..4 {
            min_pos[d] -= 1;
            max_pos[d] += 1;
        }
        Region {
            max_pos,
            min_pos,
            current: min_pos,
        }
    }

    pub fn evolve(&mut self) {
        self.next_gen.clear();
        for curr_pos in self.region() {
            match self
                .neighbors(curr_pos)
                .filter(|s| *s == State::Active)
                .count()
            {
                3 => {
                    self.next_gen.insert(curr_pos);
                }
                2 => {
                    if self.active_cubes.contains(&curr_pos) {
                        self.next_gen.insert(curr_pos);
                    }
                }
                _ => {}
            }
        }
        mem::swap(&mut self.active_cubes, &mut self.next_gen);
    }
}

#[derive(Debug)]
pub struct Region<D> {
    max_pos: D,
    min_pos: D,
    current: D,
}

impl Iterator for Region<D3> {
    type Item = Position<D3>;

    fn next(&mut self) -> Option<Self::Item> {
        let last_i = self.current.len() - 1;
        if self.current[last_i] > self.max_pos[last_i] {
            return None;
        }
        let position = Position::from(self.current);
        for i in 0..self.current.len() {
            self.current[i] += 1;
            if self.current[i] <= self.max_pos[i] || i == last_i {
                break;
            }
            self.current[i] = self.min_pos[i];
        }
        Some(position)
    }
}

impl Iterator for Region<D4> {
    type Item = Position<D4>;

    fn next(&mut self) -> Option<Self::Item> {
        let last_i = self.current.len() - 1;
        if self.current[last_i] > self.max_pos[last_i] {
            return None;
        }
        let position = Position::from(self.current);
        for i in 0..self.current.len() {
            self.current[i] += 1;
            if self.current[i] <= self.max_pos[i] || i == last_i {
                break;
            }
            self.current[i] = self.min_pos[i];
        }
        Some(position)
    }
}

#[derive(Debug)]
pub struct Neighbors<'a, D>
where
    D: Eq + Hash,
{
    active_cubes: &'a HashSet<Position<D>, FxBuildHasher>,
    position: Position<D>,
    current: usize,
}

const NEIGHBORS_3D: [D3; 26] = [
    [-1, -1, -1],
    [0, -1, -1],
    [1, -1, -1],
    [-1, 0, -1],
    [0, 0, -1],
    [1, 0, -1],
    [-1, 1, -1],
    [0, 1, -1],
    [1, 1, -1],
    [-1, -1, 0],
    [0, -1, 0],
    [1, -1, 0],
    [-1, 0, 0],
    [1, 0, 0],
    [-1, 1, 0],
    [0, 1, 0],
    [1, 1, 0],
    [-1, -1, 1],
    [0, -1, 1],
    [1, -1, 1],
    [-1, 0, 1],
    [0, 0, 1],
    [1, 0, 1],
    [-1, 1, 1],
    [0, 1, 1],
    [1, 1, 1],
];

impl<'a> Iterator for Neighbors<'a, D3> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= NEIGHBORS_3D.len() {
            return None;
        }
        let neighbor = Position3D::from([
            self.position[0] + NEIGHBORS_3D[self.current][0],
            self.position[1] + NEIGHBORS_3D[self.current][1],
            self.position[2] + NEIGHBORS_3D[self.current][2],
        ]);
        self.current += 1;
        let state = if self.active_cubes.contains(&neighbor) {
            State::Active
        } else {
            State::Inactive
        };
        Some(state)
    }
}

const NEIGHBORS_4D: [D4; 80] = [
    [-1, -1, -1, -1],
    [0, -1, -1, -1],
    [1, -1, -1, -1],
    [-1, 0, -1, -1],
    [0, 0, -1, -1],
    [1, 0, -1, -1],
    [-1, 1, -1, -1],
    [0, 1, -1, -1],
    [1, 1, -1, -1],
    [-1, -1, 0, -1],
    [0, -1, 0, -1],
    [1, -1, 0, -1],
    [-1, 0, 0, -1],
    [0, 0, 0, -1],
    [1, 0, 0, -1],
    [-1, 1, 0, -1],
    [0, 1, 0, -1],
    [1, 1, 0, -1],
    [-1, -1, 1, -1],
    [0, -1, 1, -1],
    [1, -1, 1, -1],
    [-1, 0, 1, -1],
    [0, 0, 1, -1],
    [1, 0, 1, -1],
    [-1, 1, 1, -1],
    [0, 1, 1, -1],
    [1, 1, 1, -1],
    [-1, -1, -1, 0],
    [0, -1, -1, 0],
    [1, -1, -1, 0],
    [-1, 0, -1, 0],
    [0, 0, -1, 0],
    [1, 0, -1, 0],
    [-1, 1, -1, 0],
    [0, 1, -1, 0],
    [1, 1, -1, 0],
    [-1, -1, 0, 0],
    [0, -1, 0, 0],
    [1, -1, 0, 0],
    [-1, 0, 0, 0],
    [1, 0, 0, 0],
    [-1, 1, 0, 0],
    [0, 1, 0, 0],
    [1, 1, 0, 0],
    [-1, -1, 1, 0],
    [0, -1, 1, 0],
    [1, -1, 1, 0],
    [-1, 0, 1, 0],
    [0, 0, 1, 0],
    [1, 0, 1, 0],
    [-1, 1, 1, 0],
    [0, 1, 1, 0],
    [1, 1, 1, 0],
    [-1, -1, -1, 1],
    [0, -1, -1, 1],
    [1, -1, -1, 1],
    [-1, 0, -1, 1],
    [0, 0, -1, 1],
    [1, 0, -1, 1],
    [-1, 1, -1, 1],
    [0, 1, -1, 1],
    [1, 1, -1, 1],
    [-1, -1, 0, 1],
    [0, -1, 0, 1],
    [1, -1, 0, 1],
    [-1, 0, 0, 1],
    [0, 0, 0, 1],
    [1, 0, 0, 1],
    [-1, 1, 0, 1],
    [0, 1, 0, 1],
    [1, 1, 0, 1],
    [-1, -1, 1, 1],
    [0, -1, 1, 1],
    [1, -1, 1, 1],
    [-1, 0, 1, 1],
    [0, 0, 1, 1],
    [1, 0, 1, 1],
    [-1, 1, 1, 1],
    [0, 1, 1, 1],
    [1, 1, 1, 1],
];

impl<'a> Iterator for Neighbors<'a, D4> {
    type Item = State;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= NEIGHBORS_4D.len() {
            return None;
        }
        let neighbor = Position4D::from([
            self.position[0] + NEIGHBORS_4D[self.current][0],
            self.position[1] + NEIGHBORS_4D[self.current][1],
            self.position[2] + NEIGHBORS_4D[self.current][2],
            self.position[3] + NEIGHBORS_4D[self.current][3],
        ]);
        self.current += 1;
        let state = if self.active_cubes.contains(&neighbor) {
            State::Active
        } else {
            State::Inactive
        };
        Some(state)
    }
}

#[aoc_generator(day17)]
pub fn parse_conway_cubes(input: &str) -> PocketDimension<D3> {
    let z = 0;
    let mut active_cubes = HashSet::with_hasher(FxBuildHasher::default());
    let mut y = 0;
    let mut x = 0;
    for c in input.chars() {
        match c {
            '.' => x += 1,
            '#' => {
                active_cubes.insert(Position3D::from([x, y, z]));
                x += 1;
            }
            '\n' => {
                x = 0;
                y += 1;
            }
            _ => panic!("unexpected character {:?} at line {} column {} ", c, y, x),
        }
    }
    PocketDimension {
        active_cubes,
        next_gen: HashSet::default(),
    }
}

#[aoc(day17, part1)]
pub fn num_active_cubes_after_6_cycle_boot(pocket_dimension: &PocketDimension<D3>) -> usize {
    let mut pocket_dimension = pocket_dimension.clone();

    (0..6).for_each(|_| pocket_dimension.evolve());

    pocket_dimension.num_active()
}

#[aoc(day17, part2)]
pub fn num_active_hypercubes_after_6_cycle_boot(pocket_dimension: &PocketDimension<D3>) -> usize {
    let mut pocket_dimension = PocketDimension::<D4>::from(pocket_dimension);

    (0..6).for_each(|_| pocket_dimension.evolve());

    pocket_dimension.num_active()
}

#[cfg(test)]
mod tests;
