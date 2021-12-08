use super::*;

const INPUT: &str = include_str!("../../../input/2021/day7.txt");

const EXAMPLE1: &str = "
16,1,2,0,4,2,7,1,2,14
";

mod part1 {
    use super::*;

    #[test]
    fn least_amount_of_fuel_example1() {
        let positions = parse(EXAMPLE1);

        let fuel_consumption = least_amount_of_fuel(&positions);

        assert_eq!(fuel_consumption, 37);
    }

    #[test]
    fn answer() {
        let population = parse(INPUT);

        let fuel_consumption = least_amount_of_fuel(&population);

        assert_eq!(fuel_consumption, 329389);
    }
}
