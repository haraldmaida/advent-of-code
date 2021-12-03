use super::*;
use proptest::prelude::*;

use std::collections::HashSet;
use std::iter::FromIterator;

const INPUT: &str = include_str!("../../../input/2020/day10.txt");

const EXAMPLE1: &str = "\
16
10
15
5
1
11
7
19
6
12
4
";

const EXAMPLE2: &str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";

#[test]
fn parse_adapter_joltages_in_example1() {
    let joltages = parse_adapter_joltages(EXAMPLE1);

    assert_eq!(
        joltages,
        vec![
            Adapter { rating: 16 },
            Adapter { rating: 10 },
            Adapter { rating: 15 },
            Adapter { rating: 5 },
            Adapter { rating: 1 },
            Adapter { rating: 11 },
            Adapter { rating: 7 },
            Adapter { rating: 19 },
            Adapter { rating: 6 },
            Adapter { rating: 12 },
            Adapter { rating: 4 },
        ]
    );
}

#[test]
fn parse_adapter_joltages_in_example2() {
    let joltages = parse_adapter_joltages(EXAMPLE2);

    assert_eq!(
        joltages,
        vec![
            Adapter { rating: 28 },
            Adapter { rating: 33 },
            Adapter { rating: 18 },
            Adapter { rating: 42 },
            Adapter { rating: 31 },
            Adapter { rating: 14 },
            Adapter { rating: 46 },
            Adapter { rating: 20 },
            Adapter { rating: 48 },
            Adapter { rating: 47 },
            Adapter { rating: 24 },
            Adapter { rating: 23 },
            Adapter { rating: 49 },
            Adapter { rating: 45 },
            Adapter { rating: 19 },
            Adapter { rating: 38 },
            Adapter { rating: 39 },
            Adapter { rating: 11 },
            Adapter { rating: 1 },
            Adapter { rating: 32 },
            Adapter { rating: 25 },
            Adapter { rating: 35 },
            Adapter { rating: 8 },
            Adapter { rating: 17 },
            Adapter { rating: 7 },
            Adapter { rating: 9 },
            Adapter { rating: 4 },
            Adapter { rating: 2 },
            Adapter { rating: 34 },
            Adapter { rating: 10 },
            Adapter { rating: 3 },
        ]
    );
}

#[test]
fn parse_adapter_joltages_in_puzzle_input() {
    let joltages = parse_adapter_joltages(INPUT);

    assert_eq!(joltages.len(), 93);
    assert_eq!(
        joltages.len(),
        HashSet::<u32>::from_iter(joltages.iter().map(|adapter| adapter.rating)).len()
    );
}

proptest! {
    #[test]
    fn adapter_cannot_connect_to_source_of_same_rating(
        rating in (0u32..=u32::MAX)
    ) {
        assert!(!Adapter::new(rating).can_connect_to_source(&Adapter::new(rating)));
    }

    #[test]
    fn adapter_can_connect_to_source_with_rating_1_lower(
        rating in (1u32..=u32::MAX)
    ) {
        assert!(Adapter::new(rating).can_connect_to_source(&Adapter::new(rating - 1)));
    }

    #[test]
    fn adapter_can_connect_to_source_with_rating_2_lower(
        rating in (2u32..=u32::MAX)
    ) {
        assert!(Adapter::new(rating).can_connect_to_source(&Adapter::new(rating - 2)));
    }

    #[test]
    fn adapter_can_connect_to_source_with_rating_3_lower(
        rating in (3u32..=u32::MAX)
    ) {
        assert!(Adapter::new(rating).can_connect_to_source(&Adapter::new(rating - 3)));
    }

    #[test]
    fn adapter_cannot_connect_to_source_with_rating_4_lower_or_more(
        (rating, difference) in (4u32..=u32::MAX).prop_flat_map(|rating| (Just(rating), (4..=rating)))
    ) {
        assert!(!Adapter::new(rating).can_connect_to_source(&Adapter::new(rating - difference)));
    }
}

#[test]
fn build_chain_of_adapters_in_example1() {
    let all_adapters = parse_adapter_joltages(EXAMPLE1);

    let adapter_chain = build_chain_of_adapters(&all_adapters);

    assert_eq!(
        adapter_chain,
        vec![
            Adapter { rating: 0 },
            Adapter { rating: 1 },
            Adapter { rating: 4 },
            Adapter { rating: 5 },
            Adapter { rating: 6 },
            Adapter { rating: 7 },
            Adapter { rating: 10 },
            Adapter { rating: 11 },
            Adapter { rating: 12 },
            Adapter { rating: 15 },
            Adapter { rating: 16 },
            Adapter { rating: 19 },
            Adapter { rating: 22 }
        ]
    );
}

#[test]
fn joltage_differences_in_adapter_chain_in_example1() {
    let all_adapters = parse_adapter_joltages(EXAMPLE1);

    let result = joltage_differences_in_adapter_chain(&all_adapters);

    assert_eq!(result, 7 * 5);
}

#[test]
fn joltage_differences_in_adapter_chain_in_example2() {
    let all_adapters = parse_adapter_joltages(EXAMPLE2);

    let result = joltage_differences_in_adapter_chain(&all_adapters);

    assert_eq!(result, 22 * 10);
}

#[test]
fn joltage_differences_in_adapter_chain_in_puzzle_input() {
    let all_adapters = parse_adapter_joltages(INPUT);

    let result = joltage_differences_in_adapter_chain(&all_adapters);

    assert_eq!(result, 1885);
}
//
// #[test]
// fn possible_adapter_arrangements_in_example1() {
//     let all_adapters = parse_adapter_joltages(INPUT);
//
//     let possible_arrangements = possible_adapter_arrangements(&all_adapters);
//
//     assert_eq!(
//         possible_arrangements,
//         vec![
//             vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22],
//             vec![0, 1, 4, 5, 6, 7, 10, 12, 15, 16, 19, 22],
//             vec![0, 1, 4, 5, 7, 10, 11, 12, 15, 16, 19, 22],
//             vec![0, 1, 4, 5, 7, 10, 12, 15, 16, 19, 22],
//             vec![0, 1, 4, 6, 7, 10, 11, 12, 15, 16, 19, 22],
//             vec![0, 1, 4, 6, 7, 10, 12, 15, 16, 19, 22],
//             vec![0, 1, 4, 7, 10, 11, 12, 15, 16, 19, 22],
//             vec![0, 1, 4, 7, 10, 12, 15, 16, 19, 22],
//         ]
//     )
// }

#[test]
fn mark_removable_in_example1() {
    let all_adapters = parse_adapter_joltages(EXAMPLE1);
    let adapter_chain = build_chain_of_adapters(&all_adapters);

    let removable = mark_removable(&indexed_differences(&adapter_chain));

    assert_eq!(
        removable,
        vec![
            (0, false),
            (1, false),
            (2, false),
            (3, true),
            (4, true),
            (5, false),
            (6, false),
            (7, true),
            (8, false),
            (9, false),
            (10, false),
            (11, false),
            (12, false),
        ]
    );
}

#[test]
fn split_into_sub_chains_in_example1() {
    let all_adapters = parse_adapter_joltages(EXAMPLE1);
    let adapter_chain = build_chain_of_adapters(&all_adapters);

    let sub_chains = split_into_sub_chains(&adapter_chain);

    assert_eq!(
        sub_chains,
        vec![
            vec![
                Adapter { rating: 4 },
                Adapter { rating: 5 },
                Adapter { rating: 6 },
                Adapter { rating: 7 },
            ],
            vec![
                Adapter { rating: 10 },
                Adapter { rating: 11 },
                Adapter { rating: 12 },
            ],
        ]
    );
}

#[test]
fn number_of_possible_adapter_arrangements_in_example1() {
    let all_adapters = parse_adapter_joltages(EXAMPLE1);

    let num_arrangements = number_of_possible_adapter_arrangements(&all_adapters);

    assert_eq!(num_arrangements, 8);
}

#[test]
fn number_of_possible_adapter_arrangements_in_example2() {
    let all_adapters = parse_adapter_joltages(EXAMPLE2);

    let num_arrangements = number_of_possible_adapter_arrangements(&all_adapters);

    assert_eq!(num_arrangements, 19208);
}

#[test]
fn number_of_possible_adapter_arrangements_in_puzzle_input() {
    let all_adapters = parse_adapter_joltages(INPUT);

    let num_arrangements = number_of_possible_adapter_arrangements(&all_adapters);

    assert_eq!(num_arrangements, 2024782584832);
}
