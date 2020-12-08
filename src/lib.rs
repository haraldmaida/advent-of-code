//! # Advent of Code 2019
//!
//! Santa has become stranded at the edge of the Solar System while delivering
//! presents to other planets! To accurately calculate his position in space,
//! safely align his warp drive, and return to Earth in time to save Christmas,
//! he needs you to bring him measurements from fifty stars.
//!
//! Collect stars by solving puzzles. Two puzzles will be made available on each
//! day in the Advent calendar; the second puzzle is unlocked when you complete
//! the first. Each puzzle grants one star. Good luck!
//!
//! [Advent of Code 2019](https://adventofcode.com/2019)

#![deny(unsafe_code)]
#![warn(
    bare_trait_objects,
    broken_intra_doc_links,
    missing_copy_implementations,
    missing_debug_implementations,
    private_doc_tests,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications
)]
//#![warn(missing_docs)] //TODO uncomment eventually

#[macro_use]
extern crate aoc_runner_derive;
#[macro_use]
extern crate log;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

aoc_lib! { year = 2019 }
