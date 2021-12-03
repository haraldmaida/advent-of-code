//! # Advent of Code
//!
//! Solutions for the [Advent of Code] puzzles.
//!
//! [Advent of Code](https://adventofcode.com)

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
    unused_qualifications
)]
//#![warn(variant_size_differences)]
//#![warn(unused_crate_dependencies)] //TODO uncomment eventually
//#![warn(missing_docs)] //TODO uncomment eventually

#[macro_use]
extern crate aoc_runner_derive;

#[cfg(feature = "event2017")]
pub mod year2017;

#[cfg(feature = "event2018")]
pub mod year2018;

#[cfg(feature = "event2019")]
pub mod year2019;

#[cfg(feature = "event2020")]
pub mod year2020;

#[cfg(feature = "event2021")]
pub mod year2021;

#[cfg(feature = "event2015")]
aoc_lib! { year = 2015 }

#[cfg(feature = "event2016")]
aoc_lib! { year = 2016 }

#[cfg(feature = "event2017")]
aoc_lib! { year = 2017 }

#[cfg(feature = "event2018")]
aoc_lib! { year = 2018 }

#[cfg(feature = "event2019")]
aoc_lib! { year = 2019 }

#[cfg(feature = "event2020")]
aoc_lib! { year = 2020 }

#[cfg(any(
    feature = "event2021",
    not(any(
        feature = "event2020",
        feature = "event2019",
        feature = "event2018",
        feature = "event2017",
        feature = "event2016",
        feature = "event2015"
    ))
))]
aoc_lib! { year = 2021 }
