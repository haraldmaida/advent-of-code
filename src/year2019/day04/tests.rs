use super::*;

const INPUT: &str = include_str!("../../../../input/2019/day4.txt");

#[test]
fn parse_range_in_puzzel_input() {
    let range = parse_range(INPUT);

    assert_eq!(range, 136818..=685979);
}

#[test]
fn construct_digit_code_generator_from_136818_to_685979() {
    let code_gen = DigitCodeGenerator::from(136818..=685979);

    assert_eq!(code_gen.length, 6);
    assert_eq!(&code_gen.upper_bound, &vec!['6', '7', '9', '9', '9', '9']);
    assert_eq!(&code_gen.current, &vec!['1', '3', '6', '8', '8', '8']);
}

#[test]
fn generate_digit_codes_with_range_211_to_425() {
    let code_gen = DigitCodeGenerator::from(211..=425);

    let digit_codes: Vec<String> = code_gen
        .filter(|code| has_two_same_adjacent_digits(code))
        .collect();

    assert_eq!(
        digit_codes,
        vec![
            "222".to_string(),
            "223".to_string(),
            "224".to_string(),
            "225".to_string(),
            "226".to_string(),
            "227".to_string(),
            "228".to_string(),
            "229".to_string(),
            "233".to_string(),
            "244".to_string(),
            "255".to_string(),
            "266".to_string(),
            "277".to_string(),
            "288".to_string(),
            "299".to_string(),
            "333".to_string(),
            "334".to_string(),
            "335".to_string(),
            "336".to_string(),
            "337".to_string(),
            "338".to_string(),
            "339".to_string(),
            "344".to_string(),
            "355".to_string(),
            "366".to_string(),
            "377".to_string(),
            "388".to_string(),
            "399".to_string(),
        ]
    );
}

#[test]
fn number_of_possible_passwords_with_double_in_puzzle_input() {
    let num_possible_passwords = number_of_possible_passwords_with_double(&parse_range(INPUT));

    assert_eq!(num_possible_passwords, 1919);
}

#[test]
fn number_of_possible_passwords_with_lonely_double_in_puzzle_input() {
    let num_possible_passwords =
        number_of_possible_passwords_with_lonely_double(&parse_range(INPUT));

    assert_eq!(num_possible_passwords, 1291);
}
