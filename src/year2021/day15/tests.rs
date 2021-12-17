use super::*;

const INPUT: &str = include_str!("../../../input/2021/day15.txt");

const EXAMPLE1: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = parse(EXAMPLE1);

        let result = solve_part1(&input);

        assert_eq!(result, 40);
    }

    #[test]
    fn answer() {
        let input = parse(INPUT);

        let result = solve_part1(&input);

        assert_eq!(result, 656);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = parse(EXAMPLE1);

        let result = solve_part2(&input);

        assert_eq!(result, 315);
    }

    #[test]
    fn answer() {
        let input = parse(INPUT);

        let result = solve_part2(&input);

        assert_eq!(result, 2979);
    }
}
