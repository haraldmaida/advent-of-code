use super::*;

const INPUT: &str = include_str!("../../../input/2020/day9.txt");

const EXAMPLE: &str = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

#[test]
fn parse_example_xmax_code() {
    let xmas_code = parse_xmas_code(EXAMPLE);

    assert_eq!(
        &xmas_code.numbers,
        &vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576
        ]
    );
}

#[test]
fn parse_puzzle_input_xmas_code() {
    let xmas_code = parse_xmas_code(INPUT);

    assert_eq!(xmas_code.numbers.len(), 1000);
}

#[test]
fn find_first_invalid_number_in_example() {
    let xmas_code = parse_xmas_code(EXAMPLE);

    let first_invalid = find_first_invalid_number(5, &xmas_code.numbers);

    assert_eq!(first_invalid, Some(127));
}

#[test]
fn find_first_invalid_number_in_puzzle_input() {
    let xmas_code = parse_xmas_code(INPUT);

    let first_invalid = first_invalid_number(&xmas_code);

    assert_eq!(first_invalid, 542529149);
}

#[test]
fn find_sum_of_a_set_of_numbers_in_example() {
    let xmas_code = parse_xmas_code(EXAMPLE);

    let num_set = find_sum_of_a_set_of_numbers(127, &xmas_code.numbers);

    assert_eq!(num_set, &vec![15, 25, 47, 40][..]);
}

#[test]
fn encryption_weakness_in_puzzle_input() {
    let xmas_code = parse_xmas_code(INPUT);

    let weakness = encryption_weakness(&xmas_code);

    assert_eq!(weakness, 75678618);
}
