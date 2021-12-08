use super::*;

const INPUT: &str = include_str!("../../../input/2021/day7.txt");

const EXAMPLE1: &str = "
16,1,2,0,4,2,7,1,2,14
";

mod part1 {
    use super::*;

    #[test]
    fn least_amount_of_fuel_linear_example1() {
        let positions = parse(EXAMPLE1);

        let fuel_consumption = least_amount_of_fuel_linear(&positions);

        assert_eq!(fuel_consumption, 37);
    }

    #[test]
    fn answer() {
        let population = parse(INPUT);

        let fuel_consumption = least_amount_of_fuel_linear(&population);

        assert_eq!(fuel_consumption, 329389);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn least_amount_of_fuel_exponential_example1() {
        let positions = parse(EXAMPLE1);

        let fuel_consumption = least_amount_of_fuel_exponential(&positions);

        assert_eq!(fuel_consumption, 168);
    }

    #[test]
    fn answer() {
        let population = parse(INPUT);

        let fuel_consumption = least_amount_of_fuel_exponential(&population);

        assert_eq!(fuel_consumption, 86397080);
    }
}
