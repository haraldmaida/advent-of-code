use super::*;

const INPUT: &str = include_str!("../../../input/2021/day14.txt");

const EXAMPLE1: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = parse(EXAMPLE1);

        let result = solve_part1(&input);

        assert_eq!(result, 1588);
    }

    #[test]
    fn answer() {
        let input = parse(INPUT);

        let result = solve_part1(&input);

        assert_eq!(result, 2745);
    }
}
