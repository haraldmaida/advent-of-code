use super::*;

const INPUT: &str = include_str!("../../../input/2021/day4.txt");

const EXAMPLE1: &str = "
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

mod parse_input {
    use super::*;

    #[test]
    fn parse_example1() {
        let game = parse(EXAMPLE1);

        assert_eq!(
            game,
            Game {
                drawn_numbers: vec![
                    7u32, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18,
                    20, 8, 19, 3, 26, 1
                ],
                boards: vec![
                    Board::try_from(
                        &[
                            22u32, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18,
                            5, 1, 12, 20, 15, 19
                        ][..]
                    )
                    .unwrap(),
                    Board::try_from(
                        &[
                            3u32, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24,
                            4, 14, 21, 16, 12, 6,
                        ][..]
                    )
                    .unwrap(),
                    Board::try_from(
                        &[
                            14u32, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13,
                            6, 5, 2, 0, 12, 3, 7,
                        ][..]
                    )
                    .unwrap(),
                ],
            }
        )
    }
}

mod part1 {
    use super::*;

    #[test]
    fn score_of_first_winning_board_example1() {
        let game = parse(EXAMPLE1);

        let score = score_of_first_winning_board(&game);

        assert_eq!(score, 4512);
    }

    #[test]
    fn answer() {
        let game = parse(INPUT);

        let score = score_of_first_winning_board(&game);

        assert_eq!(score, 10374);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn score_of_last_winning_board_example1() {
        let game = parse(EXAMPLE1);

        let score = score_of_last_winning_board(&game);

        assert_eq!(score, 1924);
    }

    #[test]
    fn answer() {
        let game = parse(INPUT);

        let score = score_of_last_winning_board(&game);

        assert_eq!(score, 24742);
    }
}
