use super::*;

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = Loc(1);

        let answer = port_distance(&input);

        assert_eq!(answer, 0);
    }

    #[test]
    fn example2() {
        let input = Loc(12);

        let answer = port_distance(&input);

        assert_eq!(answer, 3);
    }

    #[test]
    fn example3() {
        let input = Loc(23);

        let answer = port_distance(&input);

        assert_eq!(answer, 2);
    }
}
