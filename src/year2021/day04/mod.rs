//! # Day 4: Giant Squid
//!
//! You're already almost 1.5km (almost a mile) below the surface of the ocean,
//! already so deep that you can't see any sunlight. What you can see, however,
//! is a giant squid that has attached itself to the outside of your submarine.
//!
//! Maybe it wants to play bingo?
//!
//! Bingo is played on a set of boards each consisting of a 5x5 grid of numbers.
//! Numbers are chosen at random, and the chosen number is marked on all boards
//! on which it appears. (Numbers may not appear on all boards.) If all numbers
//! in any row or any column of a board are marked, that board wins. (Diagonals
//! don't count.)
//!
//! The submarine has a bingo subsystem to help passengers (currently, you and
//! the giant squid) pass the time. It automatically generates a random order in
//! which to draw numbers and a random set of boards (your puzzle input). For
//! example:
//!
//! ```text
//! 7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1
//!
//! 22 13 17 11  0
//!  8  2 23  4 24
//! 21  9 14 16  7
//!  6 10  3 18  5
//!  1 12 20 15 19
//!
//!  3 15  0  2 22
//!  9 18 13 17  5
//! 19  8  7 25 23
//! 20 11 10 24  4
//! 14 21 16 12  6
//!
//! 14 21 17 24  4
//! 10 16 15  9 19
//! 18  8 23 26 20
//! 22 11 13  6  5
//!  2  0 12  3  7
//! ```
//!
//! After the first five numbers are drawn (7, 4, 9, 5, and 11), there are no
//! winners, but the boards are marked as follows (shown here adjacent to each
//! other to save space):
//!
//! ```text
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! After the next six numbers are drawn (17, 23, 2, 0, 14, and 21), there are still no winners:
//!
//! ```text
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! Finally, 24 is drawn:
//!
//! ```text
//! 22 13 17 11  0         3 15  0  2 22        14 21 17 24  4
//!  8  2 23  4 24         9 18 13 17  5        10 16 15  9 19
//! 21  9 14 16  7        19  8  7 25 23        18  8 23 26 20
//!  6 10  3 18  5        20 11 10 24  4        22 11 13  6  5
//!  1 12 20 15 19        14 21 16 12  6         2  0 12  3  7
//! ```
//!
//! At this point, the third board wins because it has at least one complete row
//! or column of marked numbers (in this case, the entire top row is marked:
//! `14 21 17 24 4`).
//!
//! The score of the winning board can now be calculated. Start by finding the
//! sum of all unmarked numbers on that board; in this case, the sum is 188.
//! Then, multiply that sum by the number that was just called when the board
//! won, 24, to get the final score, `188 * 24 = 4512`.
//!
//! To guarantee victory against the giant squid, figure out which board will
//! win first. What will your final score be if you choose that board?
//!
//! [Advent of Code 2021 - Day 4](https://adventofcode.com/2021/day/4)

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    value: u32,
    marked: bool,
}

impl From<u32> for Cell {
    fn from(value: u32) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

impl Cell {
    pub fn mark(&mut self) {
        self.marked = true;
    }

    pub fn is_marked(&self) -> bool {
        self.marked
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    cells: [Cell; Board::ROW_LEN * Board::NUM_ROWS],
}

impl TryFrom<&[u32]> for Board {
    type Error = String;

    fn try_from(values: &[u32]) -> Result<Self, Self::Error> {
        const BOARD_SIZE: usize = Board::ROW_LEN * Board::NUM_ROWS;
        if values.len() != BOARD_SIZE {
            Err(format!(
                "number of numbers for board ({}) does not match board size ({})",
                values.len(),
                BOARD_SIZE
            ))
        } else {
            let mut cells = [Cell {
                value: 0,
                marked: false,
            }; BOARD_SIZE];

            values
                .iter()
                .enumerate()
                .for_each(|(idx, &number)| cells[idx].value = number);
            Ok(Self { cells })
        }
    }
}

impl Board {
    pub const ROW_LEN: usize = 5;
    pub const NUM_ROWS: usize = 5;

    pub fn check_number(&mut self, number: u32) -> Option<()> {
        self.cells
            .iter_mut()
            .find(|cell| cell.value == number)
            .map(|cell| cell.mark())
    }

    pub fn is_bingo(&self) -> bool {
        (0..Board::NUM_ROWS).any(|row| {
            !(0..Board::ROW_LEN).any(|col| !self.cells[row * Board::ROW_LEN + col].is_marked())
        }) || (0..Board::ROW_LEN).any(|col| {
            !(0..Board::NUM_ROWS).any(|row| !self.cells[col + row * Board::ROW_LEN].is_marked())
        })
    }

    pub fn score(&self, number: u32) -> u32 {
        number
            * self
                .cells
                .iter()
                .filter(|cell| !cell.is_marked())
                .map(|cell| cell.value)
                .sum::<u32>()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub drawn_numbers: Vec<u32>,
    pub boards: Vec<Board>,
}

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Game {
    let mut drawn_numbers = None;
    let mut boards = Vec::new();
    let mut current_board = Vec::new();
    for (idx, line) in input.lines().enumerate() {
        if drawn_numbers.is_none() {
            if line.is_empty() {
                continue;
            }
            drawn_numbers = Some(
                line.split(',')
                    .map(|num_str| {
                        num_str.parse::<u32>().expect(&format!(
                            "invalid number in drawn numbers line: {}",
                            idx + 1
                        ))
                    })
                    .collect(),
            );
        } else {
            if line.is_empty() {
                if !current_board.is_empty() {
                    boards.push(
                        Board::try_from(&current_board[..])
                            .expect(&format!("invalid board at line: {}", idx + 1)),
                    );
                }
                current_board = Vec::new();
            }
            current_board.extend(
                line.split(' ')
                    .filter(|part| !part.is_empty())
                    .map(|num_str| {
                        num_str
                            .parse::<u32>()
                            .expect(&format!("invalid number in board at line: {}", idx + 1))
                    }),
            );
        }
    }
    if !current_board.is_empty() {
        boards.push(
            Board::try_from(&current_board[..]).expect(&format!("invalid board at end of input")),
        );
    }
    Game {
        drawn_numbers: drawn_numbers.expect("no drawn numbers in input"),
        boards,
    }
}

#[aoc(day4, part1)]
pub fn score_of_first_winning_board(game: &Game) -> u32 {
    let mut winning_board = None;
    let mut boards = game.boards.clone();
    'bingo: for &number in &game.drawn_numbers {
        for board in &mut boards {
            board.check_number(number);
            if board.is_bingo() {
                dbg!(&winning_board);
                winning_board = Some((number, board.clone()));
                break 'bingo;
            }
        }
    }
    winning_board
        .map(|(last_number, board)| board.score(last_number))
        .expect("no winning board at all!")
}

#[cfg(test)]
mod tests;
