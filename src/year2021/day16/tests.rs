use super::*;

const INPUT: &str = include_str!("../../../input/2021/day16.txt");

const EXAMPLE1: &str = "D2FE28";
const EXAMPLE2: &str = "38006F45291200";
const EXAMPLE3: &str = "EE00D40C823060";
const EXAMPLE4: &str = "8A004A801A8002F478";
const EXAMPLE5: &str = "620080001611562C8802118E34";
const EXAMPLE6: &str = "C0015000016115A2E0802F182340";
const EXAMPLE7: &str = "A0016C880162017C3686B18A3D4780";

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = parse(EXAMPLE1);

        let result = solve_part1(&input);

        assert_eq!(result, 6);
    }

    #[test]
    fn example2() {
        let input = parse(EXAMPLE2);

        let result = solve_part1(&input);

        assert_eq!(result, 9);
    }

    #[test]
    fn example3() {
        let input = parse(EXAMPLE3);

        let result = solve_part1(&input);

        assert_eq!(result, 14);
    }

    #[test]
    fn example4() {
        let input = parse(EXAMPLE4);

        let result = solve_part1(&input);

        assert_eq!(result, 16);
    }

    #[test]
    fn example5() {
        let input = parse(EXAMPLE5);

        let result = solve_part1(&input);

        assert_eq!(result, 12);
    }

    #[test]
    fn example6() {
        let input = parse(EXAMPLE6);

        let result = solve_part1(&input);

        assert_eq!(result, 23);
    }

    #[test]
    fn example7() {
        let input = parse(EXAMPLE7);

        let result = solve_part1(&input);

        assert_eq!(result, 31);
    }

    #[test]
    fn answer() {
        let input = parse(INPUT);

        let result = solve_part1(&input);

        assert_eq!(result, 986);
    }
}
