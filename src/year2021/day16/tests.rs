use super::*;

const INPUT: &str = include_str!("../../../input/2021/day16.txt");

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = parse("D2FE28");

        let result = solve_part1(&input);

        assert_eq!(result, 6);
    }

    #[test]
    fn example2() {
        let input = parse("38006F45291200");

        let result = solve_part1(&input);

        assert_eq!(result, 9);
    }

    #[test]
    fn example3() {
        let input = parse("EE00D40C823060");

        let result = solve_part1(&input);

        assert_eq!(result, 14);
    }

    #[test]
    fn example4() {
        let input = parse("8A004A801A8002F478");

        let result = solve_part1(&input);

        assert_eq!(result, 16);
    }

    #[test]
    fn example5() {
        let input = parse("620080001611562C8802118E34");

        let result = solve_part1(&input);

        assert_eq!(result, 12);
    }

    #[test]
    fn example6() {
        let input = parse("C0015000016115A2E0802F182340");

        let result = solve_part1(&input);

        assert_eq!(result, 23);
    }

    #[test]
    fn example7() {
        let input = parse("A0016C880162017C3686B18A3D4780");

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

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = parse("C200B40A82");

        let result = solve_part2(&input);

        assert_eq!(result, 3);
    }

    #[test]
    fn example2() {
        let input = parse("04005AC33890");

        let result = solve_part2(&input);

        assert_eq!(result, 54);
    }

    #[test]
    fn example3() {
        let input = parse("880086C3E88112");

        let result = solve_part2(&input);

        assert_eq!(result, 7);
    }

    #[test]
    fn example4() {
        let input = parse("CE00C43D881120");

        let result = solve_part2(&input);

        assert_eq!(result, 9);
    }

    #[test]
    fn example5() {
        let input = parse("D8005AC2A8F0");

        let result = solve_part2(&input);

        assert_eq!(result, 1);
    }

    #[test]
    fn example6() {
        let input = parse("F600BC2D8F");

        let result = solve_part2(&input);

        assert_eq!(result, 0);
    }

    #[test]
    fn example7() {
        let input = parse("9C005AC2F8F0");

        let result = solve_part2(&input);

        assert_eq!(result, 0);
    }

    #[test]
    fn example8() {
        let input = parse("9C0141080250320F1802104A08");
        // "100 111 0 000000001010000 0100001000000000100101000000110010000011110001100000000010000100000100101000001000"

        let result = solve_part2(&input);

        assert_eq!(result, 1);
    }

    #[test]
    fn answer() {
        let input = parse(INPUT);

        let result = solve_part2(&input);

        assert_eq!(result, 18234816469452);
    }
}
