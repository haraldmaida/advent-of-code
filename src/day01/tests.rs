use super::*;

const INPUT: &str = include_str!("../../input/2019/day1.txt");

mod calculate_fuel_for_module {
    use super::*;

    #[test]
    fn example1() {
        let fuel = calculate_fuel_for_mass(Mass::new(12));

        assert_eq!(fuel, Fuel(2));
    }

    #[test]
    fn example2() {
        let fuel = calculate_fuel_for_mass(Mass::new(14));

        assert_eq!(fuel, Fuel(2));
    }

    #[test]
    fn example3() {
        let fuel = calculate_fuel_for_mass(Mass::new(1969));

        assert_eq!(fuel, Fuel(654));
    }

    #[test]
    fn example4() {
        let fuel = calculate_fuel_for_mass(Mass::new(100_756));

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

mod part2 {
    use super::*;
    use crate::day01::fuel_requirements_incl_fuel_mass;

    #[test]
    fn example1() {
        let fuel = fuel_requirements_incl_fuel_mass(&[Mass(14)]);

        assert_eq!(fuel, Fuel(2));
    }

    #[test]
    fn example2() {
        let fuel = fuel_requirements_incl_fuel_mass(&[Mass(1969)]);

        assert_eq!(fuel, Fuel(966));
    }

    #[test]
    fn example3() {
        let fuel = fuel_requirements_incl_fuel_mass(&[Mass(100_756)]);

        assert_eq!(fuel, Fuel(50_346));
    }

    #[test]
    fn answer() {
        let fuel = fuel_requirements_incl_fuel_mass(&parse(INPUT));

        assert_eq!(fuel, Fuel(5_214_475));
    }
}
