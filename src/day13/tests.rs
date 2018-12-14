use super::*;

const INPUT: &str = include_str!("../../input/2018/day13.txt");

const EXAMPLE1_INPUT: &str = r#"/->-\
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/
"#;

mod parse {
    use super::*;

    #[test]
    fn parse_example1() {
        let carts_n_tracks = parse(EXAMPLE1_INPUT);
        //eprintln!("{:?}", carts_n_tracks);
        assert_eq!(carts_n_tracks.to_string(), EXAMPLE1_INPUT);
    }

    #[test]
    fn parse_input() {
        let _carts_n_tracks = parse(INPUT);
        // just testing for not panicking
    }
}

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let carts_n_tracks = parse(EXAMPLE1_INPUT);

        let answer = location_of_first_crash(&carts_n_tracks);

        assert_eq!(answer, Position::new(7, 3));
    }

    #[test]
    fn answer() {
        let carts_n_tracks = parse(INPUT);

        let answer = location_of_first_crash(&carts_n_tracks);

        assert_eq!(answer, Position::new(65, 73));
    }
}
