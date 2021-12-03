use super::*;

const INPUT: &str = include_str!("../../../input/2020/day15.txt");

const EXAMPLE1: &str = "0,3,6";

const EXAMPLE2: &str = "1,3,2";
const EXAMPLE3: &str = "2,1,3";
const EXAMPLE4: &str = "1,2,3";
const EXAMPLE5: &str = "2,3,1";
const EXAMPLE6: &str = "3,2,1";
const EXAMPLE7: &str = "3,1,2";

#[test]
fn parse_starting_numbers_in_example1() {
    let numbers = parse_starting_numbers(EXAMPLE1);

    assert_eq!(numbers, vec![0, 3, 6]);
}

#[test]
fn determine_the_2020th_number_spoken_in_example1() {
    let numbers = parse_starting_numbers(EXAMPLE1);

    let num2020 = determine_the_2020th_number_spoken(&numbers);

    assert_eq!(num2020, 436);
}

#[test]
fn determine_the_2020th_number_spoken_in_example2() {
    let numbers = parse_starting_numbers(EXAMPLE2);

    let num2020 = determine_the_2020th_number_spoken(&numbers);

    assert_eq!(num2020, 1);
}

#[test]
fn determine_the_2020th_number_spoken_in_example3() {
    let numbers = parse_starting_numbers(EXAMPLE3);

    let num2020 = determine_the_2020th_number_spoken(&numbers);

    assert_eq!(num2020, 10);
}

#[test]
fn determine_the_2020th_number_spoken_in_example4() {
    let numbers = parse_starting_numbers(EXAMPLE4);

    let num2020 = determine_the_2020th_number_spoken(&numbers);

    assert_eq!(num2020, 27);
}

#[test]
fn determine_the_2020th_number_spoken_in_example5() {
    let numbers = parse_starting_numbers(EXAMPLE5);

    let num2020 = determine_the_2020th_number_spoken(&numbers);

    assert_eq!(num2020, 78);
}

#[test]
fn determine_the_2020th_number_spoken_in_example6() {
    let numbers = parse_starting_numbers(EXAMPLE6);

    let num2020 = determine_the_2020th_number_spoken(&numbers);

    assert_eq!(num2020, 438);
}

#[test]
fn determine_the_2020th_number_spoken_in_example7() {
    let numbers = parse_starting_numbers(EXAMPLE7);

    let num2020 = determine_the_2020th_number_spoken(&numbers);

    assert_eq!(num2020, 1836);
}

#[test]
fn determine_the_2020th_number_spoken_in_puzzle_input() {
    let numbers = parse_starting_numbers(INPUT);

    let num2020 = determine_the_2020th_number_spoken(&numbers);

    assert_eq!(num2020, 211);
}

#[test]
fn determine_the_30millionsth_number_spoken_in_example1() {
    let numbers = parse_starting_numbers(EXAMPLE1);

    let num30mill = determine_the_30millionsth_number_spoken(&numbers);

    assert_eq!(num30mill, 175594);
}

#[test]
fn determine_the_30millionsth_number_spoken_in_example2() {
    let numbers = parse_starting_numbers(EXAMPLE2);

    let num30mill = determine_the_30millionsth_number_spoken(&numbers);

    assert_eq!(num30mill, 2578);
}

#[test]
fn determine_the_30millionsth_number_spoken_in_example3() {
    let numbers = parse_starting_numbers(EXAMPLE3);

    let num30mill = determine_the_30millionsth_number_spoken(&numbers);

    assert_eq!(num30mill, 3544142);
}

#[test]
fn determine_the_30millionsth_number_spoken_in_example4() {
    let numbers = parse_starting_numbers(EXAMPLE4);

    let num30mill = determine_the_30millionsth_number_spoken(&numbers);

    assert_eq!(num30mill, 261214);
}

#[test]
fn determine_the_30millionsth_number_spoken_in_example5() {
    let numbers = parse_starting_numbers(EXAMPLE5);

    let num30mill = determine_the_30millionsth_number_spoken(&numbers);

    assert_eq!(num30mill, 6895259);
}

#[test]
fn determine_the_30millionsth_number_spoken_in_example6() {
    let numbers = parse_starting_numbers(EXAMPLE6);

    let num30mill = determine_the_30millionsth_number_spoken(&numbers);

    assert_eq!(num30mill, 18);
}

#[test]
fn determine_the_30millionsth_number_spoken_in_example7() {
    let numbers = parse_starting_numbers(EXAMPLE7);

    let num30mill = determine_the_30millionsth_number_spoken(&numbers);

    assert_eq!(num30mill, 362);
}

#[test]
fn determine_the_30millionsth_number_spoken_in_puzzle_input() {
    let numbers = parse_starting_numbers(INPUT);

    let num30mill = determine_the_30millionsth_number_spoken(&numbers);

    assert_eq!(num30mill, 2159626);
}
