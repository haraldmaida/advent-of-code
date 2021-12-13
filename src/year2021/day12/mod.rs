//! # Day 12: Passage Pathing
//!
//! With your submarine's subterranean subsystems subsisting suboptimally, the
//! only way you're getting out of this cave anytime soon is by finding a path
//! yourself. Not just a path - the only way to know if you've found the best
//! path is to find all of them.
//!
//! Fortunately, the sensors are still mostly working, and so you build a rough
//! map of the remaining caves (your puzzle input). For example:
//!
//! ```text
//! start-A
//! start-b
//! A-c
//! A-b
//! b-d
//! A-end
//! b-end
//! ```
//!
//! This is a list of how all of the caves are connected. You start in the cave
//! named start, and your destination is the cave named end. An entry like b-d
//! means that cave b is connected to cave d - that is, you can move between
//! them.
//!
//! So, the above cave system looks roughly like this:
//!
//! ```text
//!     start
//!     /   \
//! c--A-----b--d
//!     \   /
//!      end
//! ```
//!
//! Your goal is to find the number of distinct paths that start at start, end
//! at end, and don't visit small caves more than once. There are two types of
//! caves: big caves (written in uppercase, like A) and small caves (written in
//! lowercase, like b). It would be a waste of time to visit any small cave more
//! than once, but big caves are large enough that it might be worth visiting
//! them multiple times. So, all paths you find should visit small caves at most
//! once, and can visit big caves any number of times.
//!
//! Given these rules, there are 10 paths through this example cave system:
//!
//! ```text
//! start,A,b,A,c,A,end
//! start,A,b,A,end
//! start,A,b,end
//! start,A,c,A,b,A,end
//! start,A,c,A,b,end
//! start,A,c,A,end
//! start,A,end
//! start,b,A,c,A,end
//! start,b,A,end
//! start,b,end
//! ```
//!
//! (Each line in the above list corresponds to a single path; the caves visited
//! by that path are listed in the order they are visited and separated by
//! commas.)
//!
//! Note that in this cave system, cave d is never visited by any path: to do
//! so, cave b would need to be visited twice (once on the way to cave d and a
//! second time when returning from cave d), and since cave b is small, this is
//! not allowed.
//!
//! Here is a slightly larger example:
//!
//! ```text
//! dc-end
//! HN-start
//! start-kj
//! dc-start
//! dc-HN
//! LN-dc
//! HN-end
//! kj-sa
//! kj-HN
//! kj-dc
//! ```
//!
//! The 19 paths through it are as follows:
//!
//! ```text
//! start,HN,dc,HN,end
//! start,HN,dc,HN,kj,HN,end
//! start,HN,dc,end
//! start,HN,dc,kj,HN,end
//! start,HN,end
//! start,HN,kj,HN,dc,HN,end
//! start,HN,kj,HN,dc,end
//! start,HN,kj,HN,end
//! start,HN,kj,dc,HN,end
//! start,HN,kj,dc,end
//! start,dc,HN,end
//! start,dc,HN,kj,HN,end
//! start,dc,end
//! start,dc,kj,HN,end
//! start,kj,HN,dc,HN,end
//! start,kj,HN,dc,end
//! start,kj,HN,end
//! start,kj,dc,HN,end
//! start,kj,dc,end
//! ```
//!
//! Finally, this even larger example has 226 paths through it:
//!
//! ```text
//! fs-end
//! he-DX
//! fs-he
//! start-DX
//! pj-DX
//! end-zg
//! zg-sl
//! zg-pj
//! pj-he
//! RW-he
//! fs-DX
//! pj-RW
//! zg-RW
//! start-pj
//! he-WI
//! zg-he
//! pj-fs
//! start-RW
//! ```
//!
//! How many paths through this cave system are there that visit small caves at
//! most once?
//!
//! ## Part Two
//!
//! After reviewing the available paths, you realize you might have time to
//! visit a single small cave twice. Specifically, big caves can be visited any
//! number of times, a single small cave can be visited at most twice, and the
//! remaining small caves can be visited at most once. However, the caves named
//! start and end can only be visited exactly once each: once you leave the
//! start cave, you may not return to it, and once you reach the end cave, the
//! path must end immediately.
//!
//! Now, the 36 possible paths through the first example above are:
//!
//! ```text
//! start,A,b,A,b,A,c,A,end
//! start,A,b,A,b,A,end
//! start,A,b,A,b,end
//! start,A,b,A,c,A,b,A,end
//! start,A,b,A,c,A,b,end
//! start,A,b,A,c,A,c,A,end
//! start,A,b,A,c,A,end
//! start,A,b,A,end
//! start,A,b,d,b,A,c,A,end
//! start,A,b,d,b,A,end
//! start,A,b,d,b,end
//! start,A,b,end
//! start,A,c,A,b,A,b,A,end
//! start,A,c,A,b,A,b,end
//! start,A,c,A,b,A,c,A,end
//! start,A,c,A,b,A,end
//! start,A,c,A,b,d,b,A,end
//! start,A,c,A,b,d,b,end
//! start,A,c,A,b,end
//! start,A,c,A,c,A,b,A,end
//! start,A,c,A,c,A,b,end
//! start,A,c,A,c,A,end
//! start,A,c,A,end
//! start,A,end
//! start,b,A,b,A,c,A,end
//! start,b,A,b,A,end
//! start,b,A,b,end
//! start,b,A,c,A,b,A,end
//! start,b,A,c,A,b,end
//! start,b,A,c,A,c,A,end
//! start,b,A,c,A,end
//! start,b,A,end
//! start,b,d,b,A,c,A,end
//! start,b,d,b,A,end
//! start,b,d,b,end
//! start,b,end
//! ```
//!
//! The slightly larger example above now has 103 paths through it, and the even
//! larger example now has 3509 paths through it.
//!
//! Given these new rules, how many paths through this cave system are there?
//!
//! [Advent of Code 2021 - Day 12](https://adventofcode.com/2021/day/12)

use hashbrown::{HashMap, HashSet};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CaveKind {
    Small,
    Big,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cave {
    pub kind: CaveKind,
    pub name: String,
}

impl FromStr for Cave {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = if s.chars().all(|c| c.is_uppercase()) {
            CaveKind::Big
        } else {
            CaveKind::Small
        };
        let name = s.to_string();
        Ok(Self { kind, name })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Connection(pub Cave, pub Cave);

impl FromStr for Connection {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (head, tail) = s
            .split_once('-')
            .ok_or_else(|| format!("not a valid connection"))?;
        let cave1 = head.parse::<Cave>()?;
        let cave2 = tail.parse::<Cave>()?;
        Ok(Connection(cave1, cave2))
    }
}

#[derive(Debug)]
pub struct CaveMap {
    indexes: HashMap<String, usize>,
    caves: HashMap<usize, Cave>,
    adjacent_list: Vec<HashSet<usize>>,
}

impl FromIterator<Connection> for CaveMap {
    fn from_iter<T: IntoIterator<Item = Connection>>(iter: T) -> Self {
        let source = iter.into_iter();
        let mut indexes = HashMap::new();
        let mut caves = HashMap::new();
        let mut adjacent_list = Vec::new();
        for Connection(cave1, cave2) in source {
            let index1 = *indexes.entry(cave1.name.clone()).or_insert_with(|| {
                let index = adjacent_list.len();
                adjacent_list.push(HashSet::new());
                index
            });
            let index2 = *indexes.entry(cave2.name.clone()).or_insert_with(|| {
                let index = adjacent_list.len();
                adjacent_list.push(HashSet::new());
                index
            });

            adjacent_list[index1].insert(index2);
            adjacent_list[index2].insert(index1);

            caves.insert(index1, cave1);
            caves.insert(index2, cave2);
        }
        Self {
            indexes,
            caves,
            adjacent_list,
        }
    }
}

impl CaveMap {
    pub fn find_all_paths(&self, start: &str, end: &str) -> Option<Vec<Vec<Cave>>> {
        let start_index = *self.indexes.get(start)?;
        let end_index = *self.indexes.get(end)?;
        let mut found_paths = Vec::new();
        let mut to_visit = Vec::new();
        to_visit.push((start_index, vec![start_index], HashSet::new()));
        while let Some((current, path, once)) = to_visit.pop() {
            for &neighbor in &self.adjacent_list[current] {
                if neighbor == end_index {
                    let mut found = path.clone();
                    found.push(end_index);
                    found_paths.push(found);
                } else if neighbor == start_index {
                } else {
                    let mut once = once.clone();
                    if self.caves[&neighbor].kind == CaveKind::Big || once.insert(neighbor) {
                        let mut path = path.clone();
                        path.push(neighbor);
                        to_visit.push((neighbor, path, once));
                    }
                }
            }
        }
        Some(
            found_paths
                .iter()
                .map(|path| path.iter().map(|i| self.caves[i].clone()).collect())
                .collect(),
        )
    }

    pub fn find_all_paths2(&self, start: &str, end: &str) -> Option<Vec<Vec<Cave>>> {
        let start_index = *self.indexes.get(start)?;
        let end_index = *self.indexes.get(end)?;
        let mut found_paths = Vec::new();
        let mut to_visit = Vec::new();
        to_visit.push((start_index, vec![start_index], HashSet::new(), 0));
        while let Some((current, path, once, dup_small)) = to_visit.pop() {
            for &neighbor in &self.adjacent_list[current] {
                if neighbor == end_index {
                    let mut found = path.clone();
                    found.push(end_index);
                    found_paths.push(found);
                } else if neighbor == start_index {
                } else {
                    let mut dup_small = dup_small;
                    let mut once = once.clone();
                    if self.caves[&neighbor].kind == CaveKind::Big || once.insert(neighbor) || {
                        dup_small += 1;
                        dup_small <= 1
                    } {
                        let mut path = path.clone();
                        path.push(neighbor);
                        to_visit.push((neighbor, path, once, dup_small));
                    }
                }
            }
        }
        Some(
            found_paths
                .iter()
                .map(|path| path.iter().map(|i| self.caves[i].clone()).collect())
                .collect(),
        )
    }
}

#[aoc_generator(day12)]
pub fn parse(input: &str) -> Vec<Connection> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.parse::<Connection>()
                .expect("not a valid connection entry")
        })
        .collect()
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Connection]) -> usize {
    let cave_map = CaveMap::from_iter(input.iter().cloned());
    cave_map
        .find_all_paths("start", "end")
        .expect("no path found at all")
        .len()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Connection]) -> usize {
    let cave_map = CaveMap::from_iter(input.iter().cloned());
    cave_map
        .find_all_paths2("start", "end")
        .expect("no path found at all")
        .len()
}

#[cfg(test)]
mod tests;
