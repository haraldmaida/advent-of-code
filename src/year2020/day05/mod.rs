//! # Day 5: Binary Boarding
//!
//! You board your plane only to discover a new problem: you dropped your
//! boarding pass! You aren't sure which seat is yours, and all of the flight
//! attendants are busy with the flood of people that suddenly made it through
//! passport control.
//!
//! You write a quick program to use your phone's camera to scan all of the
//! nearby boarding passes (your puzzle input); perhaps you can find your seat
//! through process of elimination.
//!
//! Instead of [zones or groups], this airline uses binary space partitioning to
//! seat people. A seat might be specified like `FBFBBFFRLR`, where F means
//! "front", B means "back", L means "left", and R means "right".
//!
//! The first 7 characters will either be F or B; these specify exactly one of
//! the 128 rows on the plane (numbered 0 through 127). Each letter tells you
//! which half of a region the given seat is in. Start with the whole list of
//! rows; the first letter indicates whether the seat is in the front (0 through
//! 63) or the back (64 through 127). The next letter indicates which half of
//! that region the seat is in, and so on until you're left with exactly one
//! row.
//!
//! For example, consider just the first seven characters of `FBFBBFFRLR`:
//!
//! * Start by considering the whole range, rows 0 through 127.
//! * F means to take the lower half, keeping rows 0 through 63.
//! * B means to take the upper half, keeping rows 32 through 63.
//! * F means to take the lower half, keeping rows 32 through 47.
//! * B means to take the upper half, keeping rows 40 through 47.
//! * B keeps rows 44 through 47.
//! * F keeps rows 44 through 45.
//! * The final F keeps the lower of the two, row 44.
//!
//! The last three characters will be either L or R; these specify exactly one
//! of the 8 columns of seats on the plane (numbered 0 through 7). The same
//! process as above proceeds again, this time with only three steps. L means to
//! keep the lower half, while R means to keep the upper half.
//!
//! For example, consider just the last 3 characters of `FBFBBFFRLR`:
//!
//! * Start by considering the whole range, columns 0 through 7.
//! * R means to take the upper half, keeping columns 4 through 7.
//! * L means to take the lower half, keeping columns 4 through 5.
//! * The final R keeps the upper of the two, column 5.
//!
//! So, decoding `FBFBBFFRLR` reveals that it is the seat at row 44, column 5.
//!
//! Every seat also has a unique seat ID: multiply the row by 8, then add the
//! column. In this example, the seat has ID 44 * 8 + 5 = 357.
//!
//! Here are some other boarding passes:
//!
//! * `BFFFBBFRRR`: row 70, column 7, seat ID 567.
//! * `FFFBBBFRRR`: row 14, column 7, seat ID 119.
//! * `BBFFBBFRLL`: row 102, column 4, seat ID 820.
//!
//! As a sanity check, look through your list of boarding passes. What is the
//! highest seat ID on a boarding pass?
//!
//! ## Part 2
//!
//! Ding! The "fasten seat belt" signs have turned on. Time to find your seat.
//!
//! It's a completely full flight, so your seat should be the only missing
//! boarding pass in your list. However, there's a catch: some of the seats at
//! the very front and back of the plane don't exist on this aircraft, so
//! they'll be missing from your list as well.
//!
//! Your seat wasn't at the very front or back, though; the seats with IDs +1
//! and -1 from yours will be in your list.
//!
//! What is the ID of your seat?
//!
//! [Advent of Code 2020 - Day 5](https://adventofcode.com/2020/day/5)
//! [zones or groups](https://www.youtube.com/watch?v=oAHbLRjF0vo)

use std::convert::TryFrom;

const NUM_ROWS_ON_PLANE: usize = 128;
const NUM_COLUMNS_ON_PLANE: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionError {
    UnrecognizedCharacter(char),
    NotARowPartition(Partition),
    NotAColumnPartition(Partition),
    NotASingleRow(usize, usize),
    NotASingleColumn(usize, usize),
}

impl From<RowPartError> for PartitionError {
    fn from(error: RowPartError) -> Self {
        match error {
            RowPartError::UnrecognizedCharacter(c) => PartitionError::UnrecognizedCharacter(c),
        }
    }
}

impl From<ColumnPartError> for PartitionError {
    fn from(error: ColumnPartError) -> Self {
        match error {
            ColumnPartError::UnrecognizedCharacter(c) => PartitionError::UnrecognizedCharacter(c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Partition {
    Row(RowPart),
    Column(ColumnPart),
}

impl From<RowPart> for Partition {
    fn from(value: RowPart) -> Self {
        Partition::Row(value)
    }
}

impl From<ColumnPart> for Partition {
    fn from(value: ColumnPart) -> Self {
        Partition::Column(value)
    }
}

impl TryFrom<char> for Partition {
    type Error = PartitionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        RowPart::try_from(value)
            .map(Partition::from)
            .map_err(PartitionError::from)
            .or_else(|_| {
                ColumnPart::try_from(value)
                    .map(Partition::from)
                    .map_err(PartitionError::from)
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowPartError {
    UnrecognizedCharacter(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RowPart {
    Front,
    Back,
}

impl TryFrom<char> for RowPart {
    type Error = RowPartError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'F' => Ok(RowPart::Front),
            'B' => Ok(RowPart::Back),
            _ => Err(RowPartError::UnrecognizedCharacter(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnPartError {
    UnrecognizedCharacter(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColumnPart {
    Left,
    Right,
}

impl TryFrom<char> for ColumnPart {
    type Error = ColumnPartError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(ColumnPart::Left),
            'R' => Ok(ColumnPart::Right),
            _ => Err(ColumnPartError::UnrecognizedCharacter(value)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Seat {
    id: usize,
}

impl From<Position> for Seat {
    fn from(position: Position) -> Self {
        Self {
            id: position.row * NUM_COLUMNS_ON_PLANE + position.col,
        }
    }
}

fn seat_position(boarding_pass: &str) -> Result<Position, PartitionError> {
    let seat_code = boarding_pass
        .chars()
        .map(Partition::try_from)
        .collect::<Result<Vec<_>, PartitionError>>()?;
    let (row_min, row_max) = seat_code.iter().take(7).try_fold(
        (0usize, NUM_ROWS_ON_PLANE),
        |(min_row, max_row), part| match part {
            Partition::Row(RowPart::Front) => Ok((min_row, min_row + (max_row - min_row) / 2)),
            Partition::Row(RowPart::Back) => Ok((min_row + (max_row - min_row) / 2, max_row)),
            _ => Err(PartitionError::NotARowPartition(*part)),
        },
    )?;
    let (col_min, col_max) = seat_code.iter().skip(7).take(3).try_fold(
        (0usize, NUM_COLUMNS_ON_PLANE),
        |(min_col, max_col), part| match part {
            Partition::Column(ColumnPart::Left) => Ok((min_col, min_col + (max_col - min_col) / 2)),
            Partition::Column(ColumnPart::Right) => {
                Ok((min_col + (max_col - min_col) / 2, max_col))
            }
            _ => Err(PartitionError::NotAColumnPartition(*part)),
        },
    )?;
    if row_min != row_max - 1 {
        Err(PartitionError::NotASingleRow(row_min, row_max))
    } else if col_min != col_max - 1 {
        Err(PartitionError::NotASingleColumn(col_min, col_max))
    } else {
        Ok(Position {
            row: row_min,
            col: col_min,
        })
    }
}

fn seat_on_boarding_pass(boarding_pass: &str) -> Result<Seat, PartitionError> {
    seat_position(boarding_pass).map(Seat::from)
}

#[aoc_generator(day5)]
pub fn parse_boarding_passes(input: &str) -> Vec<String> {
    input
        .split('\n')
        .filter(|bp| bp.len() == 10)
        .map(String::from)
        .collect()
}

#[aoc(day5, part1)]
pub fn highest_seat_id_on_a_boarding_pass(boarding_passes: &[String]) -> usize {
    boarding_passes
        .iter()
        .map(|boarding_pass| {
            seat_on_boarding_pass(boarding_pass)
                .map(|seat| seat.id)
                .unwrap_or_else(|err| panic!("error: {:?}", err))
        })
        .max()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn find_free_seat(boarding_passes: &[String]) -> usize {
    let mut seat_ids = boarding_passes
        .iter()
        .map(|boarding_pass| {
            seat_on_boarding_pass(boarding_pass)
                .map(|seat| seat.id)
                .unwrap_or_else(|err| panic!("error: {:?}", err))
        })
        .collect::<Vec<_>>();
    seat_ids.sort();
    seat_ids
        .iter()
        .zip(seat_ids.iter().skip(1))
        .filter_map(|(s1, s2)| if *s1 + 1 < *s2 { Some(*s1 + 1) } else { None })
        .next()
        .expect("no free seat found")
}

#[cfg(test)]
mod tests;
