use super::*;

const INPUT: &str = include_str!("../../../input/2021/day8.txt");

const EXAMPLE1: &str = "
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

const EXAMPLE2: &str =
    "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

mod part1 {
    use super::*;

    #[test]
    fn count_digits_1_4_7_8_example1() {
        let pattern_list = parse(EXAMPLE1);

        let count = count_digits_1_4_7_8(&pattern_list);

        assert_eq!(count, 26);
    }

    #[test]
    fn answer() {
        let pattern_list = parse(INPUT);

        let count = count_digits_1_4_7_8(&pattern_list);

        assert_eq!(count, 272);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn decode_signals_example2() {
        let pattern = parse(EXAMPLE2).iter().next().unwrap().clone();

        let mapping = decode_signals(&pattern.signals);

        assert_eq!(
            mapping,
            Some(HashMap::from_iter([
                ('g', 4),
                ('c', 6),
                ('d', 0),
                ('e', 1),
                ('f', 3),
                ('a', 2),
                ('b', 5)
            ]))
        );
    }

    #[test]
    fn sum_output_values_example2() {
        let pattern = parse(EXAMPLE2);

        let output_sum = sum_output_values(&pattern);

        assert_eq!(output_sum, 5353);
    }

    #[test]
    fn sum_output_values_example1() {
        let pattern_list = parse(EXAMPLE1);

        let output_sum = sum_output_values(&pattern_list);

        assert_eq!(output_sum, 61229);
    }

    #[test]
    fn answer() {
        let pattern_list = parse(INPUT);

        let output_sum = sum_output_values(&pattern_list);

        assert_eq!(output_sum, 1007675);
    }
}
