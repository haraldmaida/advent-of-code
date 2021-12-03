use super::*;

const INPUT: &str = include_str!("../../../input/2020/day5.txt");

#[test]
fn parse_input() {
    let boarding_passes = parse_boarding_passes(INPUT);

    assert_eq!(boarding_passes.len(), 826);
}

#[test]
fn seat_id_of_example1() {
    let seat = seat_on_boarding_pass("FBFBBFFRLR").unwrap();

    assert_eq!(seat.id, 357);
}

#[test]
fn seat_id_of_example2() {
    let seat = seat_on_boarding_pass("BFFFBBFRRR").unwrap();

    assert_eq!(seat.id, 567);
}

#[test]
fn seat_id_of_example3() {
    let seat = seat_on_boarding_pass("FFFBBBFRRR").unwrap();

    assert_eq!(seat.id, 119);
}

#[test]
fn seat_id_of_example4() {
    let seat = seat_on_boarding_pass("BBFFBBFRLL").unwrap();

    assert_eq!(seat.id, 820);
}

#[test]
fn highest_seat_id_of_puzzle_input() {
    let seat_id = highest_seat_id_on_a_boarding_pass(&parse_boarding_passes(INPUT));

    assert_eq!(seat_id, 874);
}

#[test]
fn find_free_seat_in_puzzle_input() {
    let free_seat = find_free_seat(&parse_boarding_passes(INPUT));

    assert_eq!(free_seat, 594);
}
