use super::*;

const INPUT: &str = include_str!("../../input/2020/day11.txt");

const EXAMPLE1: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL
";

#[test]
fn parse_initial_seat_layout_in_example1() {
    let seat_layout = parse_initial_seat_layout(EXAMPLE1);

    assert_eq!(
        seat_layout,
        vec![
            [L, F, L, L, F, L, L, F, L, L],
            [L, L, L, L, L, L, L, F, L, L],
            [L, F, L, F, L, F, F, L, F, F],
            [L, L, L, L, F, L, L, F, L, L],
            [L, F, L, L, F, L, L, F, L, L],
            [L, F, L, L, L, L, L, F, L, L],
            [F, F, L, F, L, F, F, F, F, F],
            [L, L, L, L, L, L, L, L, L, L],
            [L, F, L, L, L, L, L, L, F, L],
            [L, F, L, L, L, L, L, F, L, L],
        ]
    );
}

#[test]
fn parse_initial_seat_layout_in_puzzle_input() {
    let seat_layout = parse_initial_seat_layout(INPUT);

    assert_eq!(seat_layout.len(), 90);
}

#[test]
fn one_generation_after_initial_layout_in_example1() {
    let generation0 = parse_initial_seat_layout(EXAMPLE1);

    let mut generation1 = generation0.clone();
    next_generation_part1(&generation0, &mut generation1);

    assert_eq!(
        generation1,
        vec![
            [O, F, O, O, F, O, O, F, O, O],
            [O, O, O, O, O, O, O, F, O, O],
            [O, F, O, F, O, F, F, O, F, F],
            [O, O, O, O, F, O, O, F, O, O],
            [O, F, O, O, F, O, O, F, O, O],
            [O, F, O, O, O, O, O, F, O, O],
            [F, F, O, F, O, F, F, F, F, F],
            [O, O, O, O, O, O, O, O, O, O],
            [O, F, O, O, O, O, O, O, F, O],
            [O, F, O, O, O, O, O, F, O, O],
        ]
    )
}

#[test]
fn number_of_occupied_seats_for_part1_in_example1() {
    let seat_layout = parse_initial_seat_layout(EXAMPLE1);

    let num_occupied = number_of_occupied_seats_part1(&seat_layout);

    assert_eq!(num_occupied, 37);
}

#[test]
fn number_of_occupied_seats_for_part1_in_puzzle_input() {
    let seat_layout = parse_initial_seat_layout(INPUT);

    let num_occupied = number_of_occupied_seats_part1(&seat_layout);

    assert_eq!(num_occupied, 2324);
}

#[test]
fn number_of_occupied_seats_for_part2_in_example1() {
    let seat_layout = parse_initial_seat_layout(EXAMPLE1);

    let num_occupied = number_of_occupied_seats_part2(&seat_layout);

    assert_eq!(num_occupied, 26);
}

#[test]
fn number_of_occupied_seats_for_part2_in_puzzle_input() {
    let seat_layout = parse_initial_seat_layout(INPUT);

    let num_occupied = number_of_occupied_seats_part2(&seat_layout);

    assert_eq!(num_occupied, 2068);
}
