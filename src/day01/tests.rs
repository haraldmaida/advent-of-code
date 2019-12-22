use super::*;

const INPUT: &str = include_str!("../../input/2019/day1.txt");

mod calculate_fuel {
    use super::*;

    #[test]
    fn example1() {
        let fuel = calculate_fuel(&Mass::new(12));

        assert_eq!(fuel, Fuel(2));
    }

    #[test]
    fn example2() {
        let fuel = calculate_fuel(&Mass::new(14));

        assert_eq!(fuel, Fuel(2));
    }

    #[test]
    fn example3() {
        let fuel = calculate_fuel(&Mass::new(1969));

        assert_eq!(fuel, Fuel(654));
    }

    #[test]
    fn example4() {
        let fuel = calculate_fuel(&Mass::new(100_756));

        assert_eq!(fuel, Fuel(33_583));
    }
}

mod part1 {
    use super::*;

    #[test]
    fn answer() {
        let fuel = fuel_requirements(&parse(INPUT));

        assert_eq!(fuel, Fuel(3_478_233));
    }
}
