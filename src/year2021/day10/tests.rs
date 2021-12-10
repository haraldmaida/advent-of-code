use super::*;

const INPUT: &str = include_str!("../../../input/2021/day10.txt");

const EXAMPLE1: &str = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

mod part1 {
    use super::*;

    #[test]
    fn total_syntax_error_score_example1() {
        let code_lines = parse(EXAMPLE1);

        let result = total_syntax_error_score(&code_lines);

        assert_eq!(result, 26397);
    }

    #[test]
    fn answer() {
        let code_lines = parse(INPUT);

        let result = total_syntax_error_score(&code_lines);

        assert_eq!(result, 362271);
    }
}
