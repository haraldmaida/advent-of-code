use super::*;

const INPUT: &str = include_str!("../../../input/2019/day3.txt");

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let wire1_path = parse_line("R8,U5,L5,D3").unwrap();
        let wire2_path = parse_line("U7,R6,D4,L4").unwrap();

        let result = distance_to_closest_intersection(&[wire1_path, wire2_path]);

        assert_eq!(result, Distance(6));
    }

    #[test]
    fn example2() {
        let wire1_path = parse_line("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap();
        let wire2_path = parse_line("U62,R66,U55,R34,D71,R55,D58,R83").unwrap();

        let result = distance_to_closest_intersection(&[wire1_path, wire2_path]);

        assert_eq!(result, Distance(159));
    }

    #[test]
    fn example3() {
        let wire1_path = parse_line("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap();
        let wire2_path = parse_line("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap();

        let result = distance_to_closest_intersection(&[wire1_path, wire2_path]);

        assert_eq!(result, Distance(135));
    }

    #[test]
    fn answer() {
        let wires = parse(INPUT);

        let result = distance_to_closest_intersection(&wires);

        assert_eq!(result, Distance(1431));
    }
}
