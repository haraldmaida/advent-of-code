use super::*;

const INPUT: &str = include_str!("../../../input/2020/day3.txt");

const EXAMPLE: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

#[test]
fn parse_example_input() {
    let map = parse(EXAMPLE);

    assert_eq!(
        &map.tiles,
        &vec![
            vec![O, O, X, X, O, O, O, O, O, O, O],
            vec![X, O, O, O, X, O, O, O, X, O, O],
            vec![O, X, O, O, O, O, X, O, O, X, O],
            vec![O, O, X, O, X, O, O, O, X, O, X],
            vec![O, X, O, O, O, X, X, O, O, X, O],
            vec![O, O, X, O, X, X, O, O, O, O, O],
            vec![O, X, O, X, O, X, O, O, O, O, X],
            vec![O, X, O, O, O, O, O, O, O, O, X],
            vec![X, O, X, X, O, O, O, X, O, O, O],
            vec![X, O, O, O, X, X, O, O, O, O, X],
            vec![O, X, O, O, X, O, O, O, X, O, X],
        ]
    );
}

#[test]
fn parse_puzzle_input() {
    let map = parse(INPUT);

    assert_eq!(map.tiles.len(), 323);
    assert_eq!(map.tiles[0].len(), 31);
}

#[test]
fn count_trees_on_slope_r3d1_on_example_map() {
    let map = parse(EXAMPLE);

    let num_trees = count_trees_on_slope_r3d1(&map);

    assert_eq!(num_trees, 7);
}

#[test]
fn count_trees_on_slope_r3d1_on_input_map() {
    let map = parse(INPUT);

    let num_trees = count_trees_on_slope_r3d1(&map);

    assert_eq!(num_trees, 209);
}

#[test]
fn product_of_trees_on_multiple_slopes_on_example_map() {
    let map = parse(EXAMPLE);

    let num_trees = product_of_trees_on_multiple_slopes(&map);

    assert_eq!(num_trees, 336);
}

#[test]
fn product_of_trees_on_multiple_slopes_on_input_map() {
    let map = parse(INPUT);

    let num_trees = product_of_trees_on_multiple_slopes(&map);

    assert_eq!(num_trees, 1574890240);
}
