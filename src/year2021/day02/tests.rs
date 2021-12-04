use super::*;

const INPUT: &str = include_str!("../../../input/2021/day2.txt");

const EXAMPLE1: &str = "
forward 5
down 5
forward 8
up 3
down 8
forward 2
";

mod part1 {
    use super::*;

    #[test]
    fn simulate_navigation_example1() {
        let route = parse(EXAMPLE1);

        let position = simulate_navigation::<Position>(&route);

        assert_eq!(position, Position { x: 15, depth: 10 });
    }

    #[test]
    fn answer() {
        let route = parse(INPUT);

        let product = product_of_final_position(&route);

        assert_eq!(product, 1480518);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn simulate_navigation_example1() {
        let route = parse(EXAMPLE1);

        let position = simulate_navigation::<AimedPosition>(&route);

        assert_eq!(
            position,
            AimedPosition {
                x: 15,
                depth: 60,
                aim: 10
            }
        );
    }

    #[test]
    fn answer() {
        let route = parse(INPUT);

        let product = product_of_final_position_variant2(&route);

        assert_eq!(product, 1282809906);
    }
}
