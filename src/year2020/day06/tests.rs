use super::*;

const INPUT: &str = include_str!("../../../input/2020/day6.txt");

const EXAMPLE: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b
";

#[test]
fn parse_compliance_list_of_example() {
    let compliance_list = parse_compliance_list(EXAMPLE);

    assert_eq!(
        compliance_list,
        vec![
            Group {
                compliance_list: vec![vec![Compliance('a'), Compliance('b'), Compliance('c')]]
            },
            Group {
                compliance_list: vec![
                    vec![Compliance('a')],
                    vec![Compliance('b')],
                    vec![Compliance('c')]
                ]
            },
            Group {
                compliance_list: vec![
                    vec![Compliance('a'), Compliance('b')],
                    vec![Compliance('a'), Compliance('c')]
                ]
            },
            Group {
                compliance_list: vec![
                    vec![Compliance('a')],
                    vec![Compliance('a')],
                    vec![Compliance('a')],
                    vec![Compliance('a')]
                ]
            },
            Group {
                compliance_list: vec![vec![Compliance('b')]]
            }
        ]
    );
}

#[test]
fn sum_of_anyone_is_compliant_in_example() {
    let sum = sum_of_anyone_is_compliant(&parse_compliance_list(EXAMPLE));

    assert_eq!(sum, 11);
}

#[test]
fn sum_of_anyone_is_compliant_in_puzzle_input() {
    let sum = sum_of_anyone_is_compliant(&parse_compliance_list(INPUT));

    assert_eq!(sum, 7283);
}

#[test]
fn sum_of_everyone_is_compliant_in_example() {
    let sum = sum_of_everyone_is_compliant(&parse_compliance_list(EXAMPLE));

    assert_eq!(sum, 6);
}

#[test]
fn sum_of_everyone_is_compliant_in_puzzle_input() {
    let sum = sum_of_everyone_is_compliant(&parse_compliance_list(INPUT));

    assert_eq!(sum, 3520);
}
