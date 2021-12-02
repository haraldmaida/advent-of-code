use super::*;

const INPUT: &str = include_str!("../../input/2021/day1.txt");

const EXAMPLE1: &str = "
199
200
208
210
200
207
240
269
260
263
";

mod part1 {
    use super::*;

    #[test]
    fn count_increased_depth_example1() {
        let report = parse(EXAMPLE1);

        let count = count_increased_depth(&report);

        assert_eq!(count, 7)
    }

    #[test]
    fn answer() {
        let report = parse(INPUT);

        let product = count_increased_depth(&report);

        assert_eq!(product, 1754);
    }
}
