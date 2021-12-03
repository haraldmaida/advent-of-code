use super::*;

mod part1 {
    use super::*;

    #[test]
    fn example1() {
        let input = Spreadsheet::new(vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]]);

        let answer = checksum(&input);

        assert_eq!(answer, 18);
    }
}

mod part2 {
    use super::*;

    #[test]
    fn example1() {
        let input = Spreadsheet::new(vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]]);

        let answer = checksum2(&input);

        assert_eq!(answer, 9);
    }
}
