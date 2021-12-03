use super::*;

const INPUT: &str = include_str!("../../../input/2020/day1.txt");

mod product_of_two_incorrect_expenses {
    use super::*;

    #[test]
    fn example1() {
        let report = vec![1721, 979, 366, 299, 675, 1456];

        let product = product_of_two_incorrect_expenses(&report);

        assert_eq!(product, 514579);
    }
}

mod product_of_three_incorrect_expenses {
    use super::*;

    #[test]
    fn example1() {
        let report = vec![1721, 979, 366, 299, 675, 1456];

        let product = product_of_three_incorrect_expenses(&report);

        assert_eq!(product, 241861950);
    }
}

mod part1 {
    use super::*;

    #[test]
    fn answer() {
        let report = parse(INPUT);

        let product = product_of_two_incorrect_expenses(&report);

        assert_eq!(product, 788739);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn answer() {
        let report = parse(INPUT);

        let product = product_of_three_incorrect_expenses(&report);

        assert_eq!(product, 178724430);
    }
}
