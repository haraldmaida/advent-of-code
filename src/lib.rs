//! # Advent of Code 2021
//!
//!
//! [Advent of Code 2021](https://adventofcode.com/2021)

#![deny(unsafe_code, unstable_features)]
#![warn(
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    rust_2018_idioms,
    rust_2021_incompatible_closure_captures,
    rust_2021_incompatible_or_patterns,
    rust_2021_prefixes_incompatible_syntax,
    rust_2021_prelude_collisions,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_doc_tests,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications,
    variant_size_differences
)]
//#![warn(unused_crate_dependencies)] //TODO uncomment eventually
//#![warn(missing_docs)] //TODO uncomment eventually

#[macro_use]
extern crate aoc_runner_derive;

pub mod day01;

aoc_lib! { year = 2021 }
