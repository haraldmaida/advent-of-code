use super::*;

const INPUT: &str = include_str!("../../../input/2021/day3.txt");

const EXAMPLE1: &str = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

mod parse_input {
    use super::*;

    #[test]
    fn parse_input_example1() {
        let diagnostic_report = parse(EXAMPLE1);
        dbg!(&diagnostic_report);

        assert_eq!(diagnostic_report[0].load::<u32>(), 0b_00100);
        assert_eq!(diagnostic_report[1].load::<u32>(), 0b_11110);
        assert_eq!(diagnostic_report[2].load::<u32>(), 0b_10110);
        assert_eq!(diagnostic_report[3].load::<u32>(), 0b_10111);
        assert_eq!(diagnostic_report[4].load::<u32>(), 0b_10101);
        assert_eq!(diagnostic_report[5].load::<u32>(), 0b_01111);
        assert_eq!(diagnostic_report[6].load::<u32>(), 0b_00111);
        assert_eq!(diagnostic_report[7].load::<u32>(), 0b_11100);
        assert_eq!(diagnostic_report[8].load::<u32>(), 0b_10000);
        assert_eq!(diagnostic_report[9].load::<u32>(), 0b_11001);
        assert_eq!(diagnostic_report[10].load::<u32>(), 0b_00010);
        assert_eq!(diagnostic_report[11].load::<u32>(), 0b_01010);
    }
}

mod part1 {
    use super::*;

    #[test]
    fn power_consumption_example1() {
        let diagnostic_report = parse(EXAMPLE1);

        let power_consumption = power_consumption(&diagnostic_report);

        assert_eq!(power_consumption, 198);
    }

    #[test]
    fn answer() {
        let diagnostic_report = parse(INPUT);

        let power_consumption = power_consumption(&diagnostic_report);

        assert_eq!(power_consumption, 1071734);
    }
}
