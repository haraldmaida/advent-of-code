use super::*;

const INPUT: &str = include_str!("../../../input/2021/day5.txt");

const EXAMPLE1: &str = "
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

mod part1 {
    use super::*;

    #[test]
    fn count_points_at_least_two_lines_overlap_example1() {
        let vent_lines = parse(EXAMPLE1);

        let count = count_points_at_least_two_lines_overlap(&vent_lines);

        assert_eq!(count, 5);
    }

    #[test]
    fn answer() {
        let vent_lines = parse(INPUT);

        let count = count_points_at_least_two_lines_overlap(&vent_lines);

        assert_eq!(count, 7269);
    }
}
