use super::*;

const INPUT: &str = include_str!("../../../input/2021/day11.txt");

const EXAMPLE1: &str = "
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

mod part1 {
    use super::*;

    #[test]
    fn part1_example1() {
        let input = parse(EXAMPLE1);

        let result = solve_part1(&input);

        assert_eq!(result, 1656);
    }

    #[test]
    fn answer() {
        let input = parse(INPUT);

        let result = solve_part1(&input);

        assert_eq!(result, 1594);
    }
}
