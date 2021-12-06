use super::*;

const INPUT: &str = include_str!("../../../input/2021/day6.txt");

const EXAMPLE1: &str = "
3,4,3,1,2
";

mod part1 {
    use super::*;

    #[test]
    fn count_lanternfish_after_80_days_example1() {
        let population = parse(EXAMPLE1);

        let count = count_lanternfish_after_80_days(&population);

        assert_eq!(count, 5934);
    }

    #[test]
    fn answer() {
        let population = parse(INPUT);

        let count = count_lanternfish_after_80_days(&population);

        assert_eq!(count, 396210);
    }
}
