use super::*;

const INPUT: &str = include_str!("../../../input/2021/day12.txt");

const EXAMPLE1: &str = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end
";

const EXAMPLE2: &str = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";

const EXAMPLE3: &str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = parse(EXAMPLE1);

        let result = solve_part1(&input);

        assert_eq!(result, 10);
    }

    #[test]
    fn example2() {
        let input = parse(EXAMPLE2);

        let result = solve_part1(&input);

        assert_eq!(result, 19);
    }

    #[test]
    fn example3() {
        let input = parse(EXAMPLE3);

        let result = solve_part1(&input);

        assert_eq!(result, 226);
    }

    #[test]
    fn answer() {
        let input = parse(INPUT);

        let result = solve_part1(&input);

        assert_eq!(result, 3410);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = parse(EXAMPLE1);

        let result = solve_part2(&input);

        assert_eq!(result, 36);
    }

    #[test]
    fn example2() {
        let input = parse(EXAMPLE2);

        let result = solve_part2(&input);

        assert_eq!(result, 103);
    }

    #[test]
    fn example3() {
        let input = parse(EXAMPLE3);

        let result = solve_part2(&input);

        assert_eq!(result, 3509);
    }

    #[test]
    fn answer() {
        let input = parse(INPUT);

        let result = solve_part2(&input);

        assert_eq!(result, 98796);
    }
}
