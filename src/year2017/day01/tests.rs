use super::*;

mod part1 {
    use super::*;

    #[test]
    fn digits_1122() {
        assert_eq!(digit_sum("1122", 1), 3);
    }

    #[test]
    fn digits_1111() {
        assert_eq!(digit_sum("1111", 1), 4);
    }

    #[test]
    fn digits_1234() {
        assert_eq!(digit_sum("1234", 1), 0);
    }

    #[test]
    fn digits_91212129() {
        assert_eq!(digit_sum("91212129", 1), 9);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn digits_1212() {
        let digits = "1212";

        assert_eq!(digit_sum(digits, digits.len() / 2), 6);
    }

    #[test]
    fn digits_1221() {
        let digits = "1221";

        assert_eq!(digit_sum(digits, digits.len() / 2), 0);
    }

    #[test]
    fn digits_123425() {
        let digits = "123425";

        assert_eq!(digit_sum(digits, digits.len() / 2), 4);
    }

    #[test]
    fn digits_123123() {
        let digits = "123123";

        assert_eq!(digit_sum(digits, digits.len() / 2), 12);
    }

    #[test]
    fn digits_12131415() {
        let digits = "12131415";

        assert_eq!(digit_sum(digits, digits.len() / 2), 4);
    }
}
