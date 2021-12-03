use super::*;
use proptest::prelude::*;
use Color::*;

const INPUT: &str = include_str!("../../../input/2020/day20.txt");

const EXAMPLE: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...
";

const EXAMPLE_IMG: &str = "\
#...##.#.. ..###..### #.#.#####.
..#.#..#.# ###...#.#. .#..######
.###....#. ..#....#.. ..#.......
###.##.##. .#.#.#..## ######....
.###.##### ##...#.### ####.#..#.
.##.#....# ##.##.###. .#...#.##.
#...###### ####.#...# #.#####.##
.....#..## #...##..#. ..#.###...
#.####...# ##..#..... ..#.......
#.##...##. ..##.#..#. ..#.###...

#.##...##. ..##.#..#. ..#.###...
##..#.##.. ..#..###.# ##.##....#
##.####... .#.####.#. ..#.###..#
####.#.#.. ...#.##### ###.#..###
.#.####... ...##..##. .######.##
.##..##.#. ....#...## #.#.#.#...
....#..#.# #.#.#.##.# #.###.###.
..#.#..... .#.##.#..# #.###.##..
####.#.... .#..#.##.. .######...
...#.#.#.# ###.##.#.. .##...####

...#.#.#.# ###.##.#.. .##...####
..#.#.###. ..##.##.## #..#.##..#
..####.### ##.#...##. .#.#..#.##
#..#.#..#. ...#.#.#.. .####.###.
.#..####.# #..#.#.#.# ####.###..
.#####..## #####...#. .##....##.
##.##..#.. ..#...#... .####...#.
#.#.###... .##..##... .####.##.#
#...###... ..##...#.. ...#..####
..#.#....# ##.#.#.... ...##.....
";

const TILE_1489: &str = "\
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..
";

const TILE_1489_ROT_90: &str = "
#.#.##...#
#.#..#.#.#
##...####.
.#####..##
#....#....
##.##..#.#
.#....##..
#.###...#.
.##..#....
.#..#.....
";

const TILE_1489_ROT_180: &str = "\
..#.##.###
##.##.##..
.##...#.##
..#.#.#...
#.#.#.#..#
.#...#####
...#...#..
...##..##.
..#...##..
....#.#.##
";

const TILE_1489_ROT_270: &str = "
.....#..#.
....#..##.
.#...###.#
..##....#.
#.#..##.##
....#....#
##..#####.
.####...##
#.#.#..#.#
#...##.#.#
";

const TILE_1489_FLIP_HORIZONTAL: &str = "\
....#.#.##
..#...##..
...##..##.
...#...#..
.#...#####
#.#.#.#..#
..#.#.#...
.##...#.##
##.##.##..
..#.##.###
";

const TILE_1489_FLIP_VERTICAL: &str = "\
###.##.#..
..##.##.##
##.#...##.
...#.#.#..
#..#.#.#.#
#####...#.
..#...#...
.##..##...
..##...#..
##.#.#....
";

fn any_color() -> impl Strategy<Value = Color> {
    prop_oneof! {
        Just(Black),
        Just(White),
    }
}

fn any_tile() -> impl Strategy<Value = Tile> {
    let pixels = [
        [
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
        ],
        [
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
        ],
        [
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
        ],
        [
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
        ],
        [
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
        ],
        [
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
        ],
        [
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
        ],
        [
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
        ],
        [
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
        ],
        [
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
            any_color(),
        ],
    ]
    .boxed();
    (1u16..=9999, pixels).prop_map(|(id, pixels)| Tile { id, pixels })
}

fn any_edge() -> impl Strategy<Value = Edge> {
    [
        any_color(),
        any_color(),
        any_color(),
        any_color(),
        any_color(),
        any_color(),
        any_color(),
        any_color(),
        any_color(),
        any_color(),
    ]
    .prop_map(|pixels| Edge(pixels))
}

fn any_edges() -> impl Strategy<Value = Edges> {
    [any_edge(), any_edge(), any_edge(), any_edge()].prop_map(|edges| Edges(edges))
}

#[test]
fn parse_tile_1489_from_str() {
    let tile = Tile::from_str(TILE_1489).unwrap();

    assert_eq!(tile.id, 0);
    assert_eq!(
        tile.pixels,
        [
            [White, White, Black, White, Black, White, Black, Black, Black, Black],
            [Black, Black, White, White, Black, Black, Black, White, Black, Black],
            [Black, White, White, Black, Black, White, White, Black, Black, Black],
            [Black, Black, White, Black, Black, Black, White, Black, Black, Black],
            [White, White, White, White, White, Black, Black, Black, White, Black],
            [White, Black, Black, White, Black, White, Black, White, Black, White],
            [Black, Black, Black, White, Black, White, Black, White, Black, Black],
            [White, White, Black, White, Black, Black, Black, White, White, Black],
            [Black, Black, White, White, Black, White, White, Black, White, White],
            [White, White, White, Black, White, White, Black, White, Black, Black],
        ]
    );
}

#[test]
fn edges_of_tile_1489() {
    let tile = Tile::from_str(TILE_1489).unwrap();

    let edges = tile.edges();

    assert_eq!(
        edges.0,
        [
            Edge::from([White, White, Black, White, Black, White, Black, Black, Black, Black]),
            Edge::from([Black, Black, Black, Black, Black, White, Black, Black, White, Black]),
            Edge::from([White, White, White, Black, White, White, Black, White, Black, Black]),
            Edge::from([White, Black, Black, Black, White, White, Black, White, Black, White]),
        ]
    );
}

#[test]
fn rotate_tile_1489_by_90deg() {
    let rotated = Tile::from_str(TILE_1489_ROT_90).unwrap();
    let mut tile = Tile::from_str(TILE_1489).unwrap();

    tile.rotate(Angle::D90);

    assert_eq!(tile.pixels, rotated.pixels);
}

#[test]
fn rotate_tile_1489_by_180deg() {
    let rotated = Tile::from_str(TILE_1489_ROT_180).unwrap();
    let mut tile = Tile::from_str(TILE_1489).unwrap();

    tile.rotate(Angle::D180);

    assert_eq!(tile.pixels, rotated.pixels);
}

#[test]
fn rotate_tile_1489_by_270deg() {
    let rotated = Tile::from_str(TILE_1489_ROT_270).unwrap();
    let mut tile = Tile::from_str(TILE_1489).unwrap();

    tile.rotate(Angle::D270);

    assert_eq!(tile.pixels, rotated.pixels);
}

#[test]
fn flip_tile_1489_horizontal() {
    let flipped = Tile::from_str(TILE_1489_FLIP_HORIZONTAL).unwrap();
    let mut tile = Tile::from_str(TILE_1489).unwrap();

    tile.flip(Orientation::Horizontal);

    assert_eq!(tile.pixels, flipped.pixels);
}

#[test]
fn flip_tile_1489_vertical() {
    let flipped = Tile::from_str(TILE_1489_FLIP_VERTICAL).unwrap();
    let mut tile = Tile::from_str(TILE_1489).unwrap();

    tile.flip(Orientation::Vertical);

    assert_eq!(tile.pixels, flipped.pixels);
}

proptest! {
    #[test]
    fn rotating_any_tile_4_times_by_90_degrees_gives_the_original_tile(
        tile in any_tile()
    ) {
        let mut rotated = tile.clone();
        rotated.rotate(Angle::D90);
        rotated.rotate(Angle::D90);
        rotated.rotate(Angle::D90);
        rotated.rotate(Angle::D90);

        prop_assert_eq!(rotated, tile);
    }
}

proptest! {
    #[test]
    fn rotating_any_tile_2_times_by_180_degrees_gives_the_original_tile(
        tile in any_tile()
    ) {
        let mut rotated = tile.clone();
        rotated.rotate(Angle::D180);
        rotated.rotate(Angle::D180);

        prop_assert_eq!(rotated, tile);
    }
}

proptest! {
    #[test]
    fn rotating_any_tile_by_270_degrees_plus_and_by_90_degrees_gives_the_original_tile(
        tile in any_tile()
    ) {
        let mut rotated = tile.clone();
        rotated.rotate(Angle::D270);
        rotated.rotate(Angle::D90);

        prop_assert_eq!(rotated, tile);
    }
}

proptest! {
    #[test]
    fn flipping_any_tile_two_times_horizontal_gives_the_original_tile(
        tile in any_tile()
    ) {
        let mut flipped = tile.clone();
        flipped.flip(Orientation::Horizontal);
        flipped.flip(Orientation::Horizontal);

        prop_assert_eq!(flipped, tile);
    }
}

proptest! {
    #[test]
    fn flipping_any_tile_two_times_vertical_gives_the_original_tile(
        tile in any_tile()
    ) {
        let mut flipped = tile.clone();
        flipped.flip(Orientation::Vertical);
        flipped.flip(Orientation::Vertical);

        prop_assert_eq!(flipped, tile);
    }
}

proptest! {
    #[test]
    fn flipping_any_tile_once_horizontal_and_once_vertical_results_in_same_tile_as_rotating_it_by_180_degrees(
        tile in any_tile()
    ) {
        let mut flipped = tile.clone();
        flipped.flip(Orientation::Horizontal);
        flipped.flip(Orientation::Vertical);

        let mut rotated = tile.clone();
        rotated.rotate(Angle::D180);

        prop_assert_eq!(flipped, rotated);
    }
}

#[test]
fn flip_edge() {
    let edge = Edge::new([
        White, Black, Black, Black, White, White, Black, White, Black, White,
    ]);

    let flipped = edge.flip();

    assert_eq!(
        flipped.0,
        [White, Black, White, Black, White, White, Black, Black, Black, White]
    );
}

proptest! {
    #[test]
    fn rotating_any_edges_4_times_by_90_degress_gives_the_orignal_edges(
        edges in any_edges()
    ) {
        let rotated = edges.clone()
            .rotate(Angle::D90)
            .rotate(Angle::D90)
            .rotate(Angle::D90)
            .rotate(Angle::D90);

        prop_assert_eq!(rotated, edges);
    }
}

proptest! {
    #[test]
    fn rotating_any_edges_2_times_by_180_degress_gives_the_orignal_edges(
        edges in any_edges()
    ) {
        let rotated = edges.clone()
            .rotate(Angle::D180)
            .rotate(Angle::D180);

        prop_assert_eq!(rotated, edges);
    }
}

proptest! {
    #[test]
    fn rotating_any_edges_by_270_degress_and_by_90_degrees_gives_the_orignal_edges(
        edges in any_edges()
    ) {
        let rotated = edges.clone()
            .rotate(Angle::D270)
            .rotate(Angle::D90);

        prop_assert_eq!(rotated, edges);
    }
}

proptest! {
    #[test]
    fn flipping_any_edges_2_times_horizontal_gives_the_orignal_edges(
        edges in any_edges()
    ) {
        let flipped = edges.clone()
            .flip(Orientation::Horizontal)
            .flip(Orientation::Horizontal);

        prop_assert_eq!(flipped, edges);
    }
}

proptest! {
    #[test]
    fn flipping_any_edges_2_times_vertical_gives_the_orignal_edges(
        edges in any_edges()
    ) {
        let flipped = edges.clone()
            .flip(Orientation::Vertical)
            .flip(Orientation::Vertical);

        prop_assert_eq!(flipped, edges);
    }
}

#[test]
fn parse_image_tiles_in_example() {
    let tiles = parse_image_tiles(EXAMPLE);

    assert_eq!(tiles.len(), 9);
    let ids = tiles.iter().map(|tile| tile.id).collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![2311, 1951, 1171, 1427, 1489, 2473, 2971, 2729, 3079]
    );
}

#[test]
fn parse_image_tiles_in_puzzle_input() {
    let tiles = parse_image_tiles(INPUT);

    assert_eq!(tiles.len(), 1728 / 12);
}

#[test]
fn reassemble_image_in_example() {
    let tiles = parse_image_tiles(EXAMPLE);

    let image = reassemble_image(&tiles);

    assert_eq!(image.tiles.len(), 9);
}
