use super::*;

const INPUT: &str = include_str!("../../../input/2021/day9.txt");

const EXAMPLE1: &str = "
2199943210
3987894921
9856789892
8767896789
9899965678
";

mod part1 {
    use super::*;

    #[test]
    fn sum_risk_level_at_low_points_example1() {
        let heightmap = parse(EXAMPLE1);

        let result = sum_risk_level_at_low_points(&heightmap);

        assert_eq!(result, 15);
    }

    #[test]
    fn answer() {
        let heightmap = parse(INPUT);

        let result = sum_risk_level_at_low_points(&heightmap);

        assert_eq!(result, 500);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn multiply_size_of_three_largest_basins_example1() {
        let heightmap = parse(EXAMPLE1);

        let result = multiply_size_of_three_largest_basins(&heightmap);

        assert_eq!(result, 1134);
    }

    #[test]
    fn answer() {
        let heightmap = parse(INPUT);

        let result = multiply_size_of_three_largest_basins(&heightmap);

        assert_eq!(result, 970200);
    }
}
