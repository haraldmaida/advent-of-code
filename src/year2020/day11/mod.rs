//! # Day 11: Seating System
//!
//! Your plane lands with plenty of time to spare. The final leg of your
//! journey is a ferry that goes directly to the tropical island where you can
//! finally start your vacation. As you reach the waiting area to board the
//! ferry, you realize you're so early, nobody else has even arrived yet!
//!
//! By modeling the process people use to choose (or abandon) their seat in the
//! waiting area, you're pretty sure you can predict the best place to sit. You
//! make a quick map of the seat layout (your puzzle input).
//!
//! The seat layout fits neatly on a grid. Each position is either floor (.), an
//! empty seat (L), or an occupied seat (#). For example, the initial seat
//! layout might look like this:
//!
//! ```text
//! L.LL.LL.LL
//! LLLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLLL
//! L.LLLLLL.L
//! L.LLLLL.LL
//! ```
//!
//! Now, you just need to model the people who will be arriving shortly.
//! Fortunately, people are entirely predictable and always follow a simple set
//! of rules. All decisions are based on the number of occupied seats adjacent
//! to a given seat (one of the eight positions immediately up, down, left,
//! right, or diagonal from the seat). The following rules are applied to every
//! seat simultaneously:
//!
//! * If a seat is empty (L) and there are no occupied seats adjacent to it, the
//!   seat becomes occupied.
//! * If a seat is occupied (#) and four or more seats adjacent to it are also
//!   occupied, the seat becomes empty.
//! * Otherwise, the seat's state does not change.
//!
//! Floor (.) never changes; seats don't move, and nobody sits on the floor.
//!
//! After one round of these rules, every seat in the example layout becomes
//! occupied:
//!
//! ```text
//! #.##.##.##
//! #######.##
//! #.#.#..#..
//! ####.##.##
//! #.##.##.##
//! #.#####.##
//! ..#.#.....
//! ##########
//! #.######.#
//! #.#####.##
//! ```
//!
//! After a second round, the seats with four or more occupied adjacent seats
//! become empty again:
//!
//! ```text
//! #.LL.L#.##
//! #LLLLLL.L#
//! L.L.L..L..
//! #LLL.LL.L#
//! #.LL.LL.LL
//! #.LLLL#.##
//! ..L.L.....
//! #LLLLLLLL#
//! #.LLLLLL.L
//! #.#LLLL.##
//! ```
//!
//! This process continues for three more rounds:
//!
//! ```text
//! #.##.L#.##
//! #L###LL.L#
//! L.#.#..#..
//! #L##.##.L#
//! #.##.LL.LL
//! #.###L#.##
//! ..#.#.....
//! #L######L#
//! #.LL###L.L
//! #.#L###.##
//! ```
//!
//! ```text
//! #.#L.L#.##
//! #LLL#LL.L#
//! L.L.L..#..
//! #LLL.##.L#
//! #.LL.LL.LL
//! #.LL#L#.##
//! ..L.L.....
//! #L#LLLL#L#
//! #.LLLLLL.L
//! #.#L#L#.##
//! ```
//!
//! ```text
//! #.#L.L#.##
//! #LLL#LL.L#
//! L.#.L..#..
//! #L##.##.L#
//! #.#L.LL.LL
//! #.#L#L#.##
//! ..L.L.....
//! #L#L##L#L#
//! #.LLLLLL.L
//! #.#L#L#.##
//! ```
//!
//! At this point, something interesting happens: the chaos stabilizes and
//! further applications of these rules cause no seats to change state! Once
//! people stop moving around, you count 37 occupied seats.
//!
//! Simulate your seating area by applying the seating rules repeatedly until no
//! seats change state. How many seats end up occupied?
//!
//! ## Part 2
//!
//! As soon as people start to arrive, you realize your mistake. People don't
//! just care about adjacent seats - they care about the first seat they can see
//! in each of those eight directions!
//!
//! Now, instead of considering just the eight immediately adjacent seats,
//! consider the first seat in each of those eight directions. For example,
//! the empty seat below would see eight occupied seats:
//!
//! ```text
//! .......#.
//! ...#.....
//! .#.......
//! .........
//! ..#L....#
//! ....#....
//! .........
//! #........
//! ...#.....
//! ```
//!
//! The leftmost empty seat below would only see one empty seat, but cannot see
//! any of the occupied ones:
//!
//! ```text
//! .............
//! .L.L.#.#.#.#.
//! .............
//! ```
//!
//! The empty seat below would see no occupied seats:
//!
//! ```text
//! .##.##.
//! #.#.#.#
//! ##...##
//! ...L...
//! ##...##
//! #.#.#.#
//! .##.##.
//! ```
//!
//! Also, people seem to be more tolerant than you expected: it now takes five
//! or more visible occupied seats for an occupied seat to become empty (rather
//! than four or more from the previous rules). The other rules still apply:
//! empty seats that see no occupied seats become occupied, seats matching no
//! rule don't change, and floor never changes.
//!
//! Given the same starting layout as above, these new rules cause the seating
//! area to shift around as follows:
//!
//! ```text
//! L.LL.LL.LL
//! LLLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLLL
//! L.LLLLLL.L
//! L.LLLLL.LL
//! ```
//! ```text
//! #.##.##.##
//! #######.##
//! #.#.#..#..
//! ####.##.##
//! #.##.##.##
//! #.#####.##
//! ..#.#.....
//! ##########
//! #.######.#
//! #.#####.##
//! ```
//! ```text
//! #.LL.LL.L#
//! #LLLLLL.LL
//! L.L.L..L..
//! LLLL.LL.LL
//! L.LL.LL.LL
//! L.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLL#
//! #.LLLLLL.L
//! #.LLLLL.L#
//! ```
//! ```text
//! #.L#.##.L#
//! #L#####.LL
//! L.#.#..#..
//! ##L#.##.##
//! #.##.#L.##
//! #.#####.#L
//! ..#.#.....
//! LLL####LL#
//! #.L#####.L
//! #.L####.L#
//! ```
//! ```text
//! #.L#.L#.L#
//! #LLLLLL.LL
//! L.L.L..#..
//! ##LL.LL.L#
//! L.LL.LL.L#
//! #.LLLLL.LL
//! ..L.L.....
//! LLLLLLLLL#
//! #.LLLLL#.L
//! #.L#LL#.L#
//! ```
//! ```text
//! #.L#.L#.L#
//! #LLLLLL.LL
//! L.L.L..#..
//! ##L#.#L.L#
//! L.L#.#L.L#
//! #.L####.LL
//! ..#.#.....
//! LLL###LLL#
//! #.LLLLL#.L
//! #.L#LL#.L#
//! ```
//! ```text
//! #.L#.L#.L#
//! #LLLLLL.LL
//! L.L.L..#..
//! ##L#.#L.L#
//! L.L#.LL.L#
//! #.LLLL#.LL
//! ..#.L.....
//! LLL###LLL#
//! #.LLLLL#.L
//! #.L#LL#.L#
//! ```
//!
//! Again, at this point, people stop shifting around and the seating area
//! reaches equilibrium. Once this occurs, you count 26 occupied seats.
//!
//! Given the new visibility method and the rule change for occupied seats
//! becoming empty, once equilibrium is reached, how many seats end up occupied?
//!
//! [Advent of Code 2020 - Day 11](https://adventofcode.com/2020/day/11)

use std::mem;

pub const F: Tile = Tile::Floor;
pub const L: Tile = Tile::EmptySeat;
pub const O: Tile = Tile::OccupiedSeat;

const NEIGHBORS: [Neighbor; 8] = [
    Neighbor::North,
    Neighbor::NorthEast,
    Neighbor::East,
    Neighbor::SouthEast,
    Neighbor::South,
    Neighbor::SouthWest,
    Neighbor::West,
    Neighbor::NorthWest,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Floor,
    EmptySeat,
    OccupiedSeat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Neighbor {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

#[aoc_generator(day11)]
pub fn parse_initial_seat_layout(input: &str) -> Vec<Vec<Tile>> {
    let mut rows = Vec::new();
    let mut current_row = Vec::new();
    for c in input.chars() {
        match c {
            '.' => current_row.push(Tile::Floor),
            'L' => current_row.push(Tile::EmptySeat),
            '#' => current_row.push(Tile::OccupiedSeat),
            '\n' => {
                if !current_row.is_empty() {
                    rows.push(mem::replace(&mut current_row, Vec::new()));
                }
            }
            _ => panic!("unrecognized tile {:?}", c),
        }
    }
    rows
}

fn count_tiles(state: Tile, layout: &[Vec<Tile>]) -> usize {
    let mut count = 0;
    for row in layout {
        for tile in row {
            if *tile == state {
                count += 1
            }
        }
    }
    count
}

fn next_generation_part1(prev_layout: &[Vec<Tile>], next_layout: &mut [Vec<Tile>]) {
    for row in 0..prev_layout.len() {
        for col in 0..prev_layout[row].len() {
            let prev_state = prev_layout[row][col];
            let new_state = match prev_state {
                Tile::Floor => prev_state,
                Tile::EmptySeat => {
                    if 0 == count_neighbors_part1(Tile::OccupiedSeat, row, col, prev_layout) {
                        Tile::OccupiedSeat
                    } else {
                        prev_state
                    }
                }
                Tile::OccupiedSeat => {
                    if 4 <= count_neighbors_part1(Tile::OccupiedSeat, row, col, prev_layout) {
                        Tile::EmptySeat
                    } else {
                        prev_state
                    }
                }
            };
            next_layout[row][col] = new_state;
        }
    }
}

fn count_neighbors_part1(state: Tile, row: usize, col: usize, layout: &[Vec<Tile>]) -> usize {
    let mut count = 0;
    let num_rows = layout.len();
    let num_cols = layout[row].len();
    for neighbor in &NEIGHBORS {
        match neighbor {
            Neighbor::North => {
                if row > 0 && layout[row - 1][col] == state {
                    count += 1;
                }
            }
            Neighbor::NorthEast => {
                if row > 0 && col < num_cols - 1 && layout[row - 1][col + 1] == state {
                    count += 1;
                }
            }
            Neighbor::East => {
                if col < num_cols - 1 && layout[row][col + 1] == state {
                    count += 1;
                }
            }
            Neighbor::SouthEast => {
                if row < num_rows - 1 && col < num_cols - 1 && layout[row + 1][col + 1] == state {
                    count += 1;
                }
            }
            Neighbor::South => {
                if row < num_rows - 1 && layout[row + 1][col] == state {
                    count += 1;
                }
            }
            Neighbor::SouthWest => {
                if row < num_rows - 1 && col > 0 && layout[row + 1][col - 1] == state {
                    count += 1;
                }
            }
            Neighbor::West => {
                if col > 0 && layout[row][col - 1] == state {
                    count += 1;
                }
            }
            Neighbor::NorthWest => {
                if row > 0 && col > 0 && layout[row - 1][col - 1] == state {
                    count += 1;
                }
            }
        }
    }
    count
}

fn next_generation_part2(prev_layout: &[Vec<Tile>], next_layout: &mut [Vec<Tile>]) {
    for row in 0..prev_layout.len() {
        for col in 0..prev_layout[row].len() {
            let prev_state = prev_layout[row][col];
            let new_state = match prev_state {
                Tile::Floor => prev_state,
                Tile::EmptySeat => {
                    if 0 == count_neighbors_part2(Tile::OccupiedSeat, row, col, prev_layout) {
                        Tile::OccupiedSeat
                    } else {
                        prev_state
                    }
                }
                Tile::OccupiedSeat => {
                    if 5 <= count_neighbors_part2(Tile::OccupiedSeat, row, col, prev_layout) {
                        Tile::EmptySeat
                    } else {
                        prev_state
                    }
                }
            };
            next_layout[row][col] = new_state;
        }
    }
}

fn count_neighbors_part2(state: Tile, row: usize, col: usize, layout: &[Vec<Tile>]) -> usize {
    let mut count = 0;
    let num_rows = layout.len();
    let num_cols = layout[row].len();
    for neighbor in &NEIGHBORS {
        match neighbor {
            Neighbor::North => {
                for ri in (0..row).rev() {
                    match layout[ri][col] {
                        s if s == state => {
                            count += 1;
                            break;
                        }
                        Tile::Floor => {}
                        _ => break,
                    }
                }
            }
            Neighbor::NorthEast => {
                if col < num_cols - 1 {
                    for (ri, ci) in (0..row).rev().zip(col + 1..num_cols) {
                        match layout[ri][ci] {
                            s if s == state => {
                                count += 1;
                                break;
                            }
                            Tile::Floor => {}
                            _ => break,
                        }
                    }
                }
            }
            Neighbor::East => {
                if col < num_cols - 1 {
                    for ci in col + 1..num_cols {
                        match layout[row][ci] {
                            s if s == state => {
                                count += 1;
                                break;
                            }
                            Tile::Floor => {}
                            _ => break,
                        }
                    }
                }
            }
            Neighbor::SouthEast => {
                if row < num_rows - 1 && col < num_cols - 1 {
                    for (ri, ci) in (row + 1..num_rows).zip(col + 1..num_cols) {
                        match layout[ri][ci] {
                            s if s == state => {
                                count += 1;
                                break;
                            }
                            Tile::Floor => {}
                            _ => break,
                        }
                    }
                }
            }
            Neighbor::South => {
                if row < num_rows - 1 {
                    for ri in row + 1..num_rows {
                        match layout[ri][col] {
                            s if s == state => {
                                count += 1;
                                break;
                            }
                            Tile::Floor => {}
                            _ => break,
                        }
                    }
                }
            }
            Neighbor::SouthWest => {
                if row < num_rows - 1 {
                    for (ri, ci) in (row + 1..num_rows).zip((0..col).rev()) {
                        match layout[ri][ci] {
                            s if s == state => {
                                count += 1;
                                break;
                            }
                            Tile::Floor => {}
                            _ => break,
                        }
                    }
                }
            }
            Neighbor::West => {
                for ci in (0..col).rev() {
                    match layout[row][ci] {
                        s if s == state => {
                            count += 1;
                            break;
                        }
                        Tile::Floor => {}
                        _ => break,
                    }
                }
            }
            Neighbor::NorthWest => {
                for (ri, ci) in (0..row).rev().zip((0..col).rev()) {
                    match layout[ri][ci] {
                        s if s == state => {
                            count += 1;
                            break;
                        }
                        Tile::Floor => {}
                        _ => break,
                    }
                }
            }
        }
    }
    count
}

#[aoc(day11, part1)]
pub fn number_of_occupied_seats_part1(initial_layout: &[Vec<Tile>]) -> usize {
    let mut prev_layout = initial_layout.to_vec();
    let mut next_layout = initial_layout.to_vec();
    loop {
        next_generation_part1(&prev_layout, &mut next_layout);
        if next_layout == prev_layout {
            break;
        }
        mem::swap(&mut prev_layout, &mut next_layout);
    }
    count_tiles(Tile::OccupiedSeat, &next_layout)
}

#[aoc(day11, part2)]
pub fn number_of_occupied_seats_part2(initial_layout: &[Vec<Tile>]) -> usize {
    let mut prev_layout = initial_layout.to_vec();
    let mut next_layout = initial_layout.to_vec();
    loop {
        next_generation_part2(&prev_layout, &mut next_layout);
        if next_layout == prev_layout {
            break;
        }
        mem::swap(&mut prev_layout, &mut next_layout);
    }
    count_tiles(Tile::OccupiedSeat, &next_layout)
}

#[cfg(test)]
mod tests;
