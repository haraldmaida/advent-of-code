//! # Day 9: Encoding Error
//!
//! With your neighbor happily enjoying their video game, you turn your
//! attention to an open data port on the little screen in the seat in front of
//! you.
//!
//! Though the port is non-standard, you manage to connect it to your computer
//! through the clever use of several paperclips. Upon connection, the port
//! outputs a series of numbers (your puzzle input).
//!
//! The data appears to be encrypted with the eXchange-Masking Addition System
//! (XMAS) which, conveniently for you, is an old cypher with an important
//! weakness.
//!
//! XMAS starts by transmitting a preamble of 25 numbers. After that, each
//! number you receive should be the sum of any two of the 25 immediately
//! previous numbers. The two numbers will have different values, and there
//! might be more than one such pair.
//!
//! For example, suppose your preamble consists of the numbers 1 through 25 in a
//! random order. To be valid, the next number must be the sum of two of those
//! numbers:
//!
//! * 26 would be a valid next number, as it could be 1 plus 25 (or many other
//!   pairs, like 2 and 24).
//! * 49 would be a valid next number, as it is the sum of 24 and 25.
//! * 100 would not be valid; no two of the previous 25 numbers sum to 100.
//! * 50 would also not be valid; although 25 appears in the previous 25
//!   numbers, the two numbers in the pair must be different.
//!
//! Suppose the 26th number is 45, and the first number (no longer an option, as
//! it is more than 25 numbers ago) was 20. Now, for the next number to be
//! valid, there needs to be some pair of numbers among 1-19, 21-25, or 45 that
//! add up to it:
//!
//! * 26 would still be a valid next number, as 1 and 25 are still within the
//!   previous 25 numbers.
//! * 65 would not be valid, as no two of the available numbers sum to it.
//! * 64 and 66 would both be valid, as they are the result of 19+45 and 21+45
//!   respectively.
//!
//! Here is a larger example which only considers the previous 5 numbers (and
//! has a preamble of length 5):
//!
//! ```text
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this example, after the 5-number preamble, almost every number is the sum
//! of two of the previous 5 numbers; the only number that does not follow this
//! rule is 127.
//!
//! The first step of attacking the weakness in the XMAS data is to find the
//! first number in the list (after the preamble) which is not the sum of two of
//! the 25 numbers before it. What is the first number that does not have this
//! property?
//!
//! # Part 2
//!
//! The final step in breaking the XMAS encryption relies on the invalid number
//! you just found: you must find a contiguous set of at least two numbers in
//! your list which sum to the invalid number from step 1.
//!
//! Again consider the above example:
//!
//! ```text
//! 35
//! 20
//! 15
//! 25
//! 47
//! 40
//! 62
//! 55
//! 65
//! 95
//! 102
//! 117
//! 150
//! 182
//! 127
//! 219
//! 299
//! 277
//! 309
//! 576
//! ```
//!
//! In this list, adding up all of the numbers from 15 through 40 produces the
//! invalid number from step 1, 127. (Of course, the contiguous set of numbers
//! in your actual list might be much longer.)
//!
//! To find the encryption weakness, add together the smallest and largest
//! number in this contiguous range; in this example, these are 15 and 47,
//! producing 62.
//!
//! [Advent of Code 2020 - Day 9](https://adventofcode.com/2020/day/9)

use std::str::FromStr;

#[aoc_generator(day9)]
pub fn parse_xmas_code(input: &str) -> XmasCode {
    let numbers = input
        .lines()
        .enumerate()
        .map(|line| {
            i64::from_str(line.1).unwrap_or_else(|err| {
                panic!("line {}: {:?} no a valid number: {:?}", line.0, line.1, err)
            })
        })
        .collect();
    XmasCode { numbers }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmasCode {
    numbers: Vec<i64>,
}

fn is_valid_number(index: usize, preamble_len: usize, numbers: &[i64]) -> bool {
    assert!(
        index >= preamble_len,
        "index must be greater than or equal the length of the preamble"
    );

    let number = numbers[index];
    numbers
        .iter()
        .enumerate()
        .skip(index - preamble_len)
        .take(preamble_len)
        .any(|(i, n1)| {
            let needed = number - n1;
            numbers
                .iter()
                .skip(i + 1)
                .take(preamble_len - 1)
                .any(|n2| *n2 == needed)
        })
}

fn find_first_invalid_number(preamble_len: usize, numbers: &[i64]) -> Option<i64> {
    numbers
        .iter()
        .enumerate()
        .skip(preamble_len)
        .find(|(i, _)| !is_valid_number(*i, preamble_len, numbers))
        .map(|(_, n)| *n)
}

#[aoc(day9, part1)]
pub fn first_invalid_number(xmas_code: &XmasCode) -> i64 {
    find_first_invalid_number(25, &xmas_code.numbers).expect("no invalid number found")
}

fn find_sum_of_a_set_of_numbers(num: i64, numbers: &[i64]) -> &[i64] {
    for i in 0..numbers.len() - 1 {
        let mut sum = numbers[i];
        for j in i + 1..numbers.len() {
            sum += numbers[j];
            if sum == num {
                return &numbers[i..j + 1];
            }
        }
    }
    static EMPTY: [i64; 0] = [];
    &EMPTY
}

#[aoc(day9, part2)]
pub fn encryption_weakness(xmas_code: &XmasCode) -> i64 {
    let num_set = find_sum_of_a_set_of_numbers(542529149, &xmas_code.numbers);
    let largest_num = num_set.iter().max().unwrap();
    let smallest_num = num_set.iter().min().unwrap();
    smallest_num + largest_num
}

#[cfg(test)]
mod tests;
