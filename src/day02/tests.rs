use super::*;

const INPUT: &str = include_str!("../../input/2020/day2.txt");

const EXAMPLE1: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

mod parse_input {
    use super::*;

    #[test]
    fn parse_example1() {
        let entries = parse(EXAMPLE1);

        assert_eq!(
            entries,
            vec![
                (
                    Policy {
                        min: 1,
                        max: 3,
                        character: 'a',
                    },
                    "abcde".to_string()
                ),
                (
                    Policy {
                        min: 1,
                        max: 3,
                        character: 'b',
                    },
                    "cdefg".to_string()
                ),
                (
                    Policy {
                        min: 2,
                        max: 9,
                        character: 'c',
                    },
                    "ccccccccc".to_string()
                ),
            ]
        )
    }

    #[test]
    fn parse_input() {
        let entries = parse(INPUT);

        assert_eq!(entries.len(), 1000);
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let answer = count_valid_passwords(&parse(EXAMPLE1));

        assert_eq!(answer, 2);
    }

    #[test]
    fn answer() {
        let answer = count_valid_passwords(&parse(INPUT));

        assert_eq!(answer, 467);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let answer = count_valid_passwords_part2(&parse(EXAMPLE1));

        assert_eq!(answer, 1);
    }

    #[test]
    fn answer() {
        let answer = count_valid_passwords_part2(&parse(INPUT));

        assert_eq!(answer, 441);
    }
}
