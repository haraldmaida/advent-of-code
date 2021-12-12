use super::*;

const INPUT: &str = include_str!("../../../input/2019/day3.txt");

const EXAMPLE1: &str = "\
R8,U5,L5,D3
U7,R6,D4,L4
";

const EXAMPLE2: &str = "\
R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83
";

const EXAMPLE3: &str = "\
R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7
";

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let wires = parse(EXAMPLE1);

        let result = distance_to_closest_intersection(&wires);

        assert_eq!(result, Distance(6));
    }

    #[test]
    fn example2() {
        let wires = parse(EXAMPLE2);

        let result = distance_to_closest_intersection(&wires);

        assert_eq!(result, Distance(159));
    }

    #[test]
    fn example3() {
        let wires = parse(EXAMPLE3);

        let result = distance_to_closest_intersection(&wires);

        assert_eq!(result, Distance(135));
    }

    #[test]
    fn answer() {
        let wires = parse(INPUT);

        let result = distance_to_closest_intersection(&wires);

        assert_eq!(result, Distance(1431));
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let wires = parse(EXAMPLE1);

        let result = minimal_signal_delay(&wires);

        assert_eq!(result, 30);
    }

    #[test]
    fn example2() {
        let wires = parse(EXAMPLE2);

        let result = minimal_signal_delay(&wires);

        assert_eq!(result, 610);
    }

    #[test]
    fn example3() {
        let wires = parse(EXAMPLE3);

        let result = minimal_signal_delay(&wires);

        assert_eq!(result, 410);
    }

    #[test]
    fn answer() {
        let wires = parse(INPUT);

        let result = minimal_signal_delay(&wires);

        assert_eq!(result, 48012);
    }
}
