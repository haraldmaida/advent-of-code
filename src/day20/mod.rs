//! # Day 20: Jurassic Jigsaw
//!
//! The high-speed train leaves the forest and quickly carries you south. You
//! can even see a desert in the distance! Since you have some spare time, you
//! might as well see if there was anything interesting in the image the Mythical
//! Information Bureau satellite captured.
//!
//! After decoding the satellite messages, you discover that the data actually
//! contains many small images created by the satellite's camera array. The
//! camera array consists of many cameras; rather than produce a single square
//! image, they produce many smaller square image tiles that need to be
//! reassembled back into a single image.
//!
//! Each camera in the camera array returns a single monochrome image tile with
//! a random unique ID number. The tiles (your puzzle input) arrived in a random
//! order.
//!
//! Worse yet, the camera array appears to be malfunctioning: each image tile
//! has been rotated and flipped to a random orientation. Your first task is to
//! reassemble the original image by orienting the tiles so they fit together.
//!
//! To show how the tiles should be reassembled, each tile's image data includes
//! a border that should line up exactly with its adjacent tiles. All tiles have
//! this border, and the border lines up exactly when the tiles are both
//! oriented correctly. Tiles at the edge of the image also have this border,
//! but the outermost edges won't line up with any other tiles.
//!
//! For example, suppose you have the following nine tiles:
//!
//! ```text
//! Tile 2311:
//! ..##.#..#.
//! ##..#.....
//! #...##..#.
//! ####.#...#
//! ##.##.###.
//! ##...#.###
//! .#.#.#..##
//! ..#....#..
//! ###...#.#.
//! ..###..###
//!
//! Tile 1951:
//! #.##...##.
//! #.####...#
//! .....#..##
//! #...######
//! .##.#....#
//! .###.#####
//! ###.##.##.
//! .###....#.
//! ..#.#..#.#
//! #...##.#..
//!
//! Tile 1171:
//! ####...##.
//! #..##.#..#
//! ##.#..#.#.
//! .###.####.
//! ..###.####
//! .##....##.
//! .#...####.
//! #.##.####.
//! ####..#...
//! .....##...
//!
//! Tile 1427:
//! ###.##.#..
//! .#..#.##..
//! .#.##.#..#
//! #.#.#.##.#
//! ....#...##
//! ...##..##.
//! ...#.#####
//! .#.####.#.
//! ..#..###.#
//! ..##.#..#.
//!
//! Tile 1489:
//! ##.#.#....
//! ..##...#..
//! .##..##...
//! ..#...#...
//! #####...#.
//! #..#.#.#.#
//! ...#.#.#..
//! ##.#...##.
//! ..##.##.##
//! ###.##.#..
//!
//! Tile 2473:
//! #....####.
//! #..#.##...
//! #.##..#...
//! ######.#.#
//! .#...#.#.#
//! .#########
//! .###.#..#.
//! ########.#
//! ##...##.#.
//! ..###.#.#.
//!
//! Tile 2971:
//! ..#.#....#
//! #...###...
//! #.#.###...
//! ##.##..#..
//! .#####..##
//! .#..####.#
//! #..#.#..#.
//! ..####.###
//! ..#.#.###.
//! ...#.#.#.#
//!
//! Tile 2729:
//! ...#.#.#.#
//! ####.#....
//! ..#.#.....
//! ....#..#.#
//! .##..##.#.
//! .#.####...
//! ####.#.#..
//! ##.####...
//! ##..#.##..
//! #.##...##.
//!
//! Tile 3079:
//! #.#.#####.
//! .#..######
//! ..#.......
//! ######....
//! ####.#..#.
//! .#...#.##.
//! #.#####.##
//! ..#.###...
//! ..#.......
//! ..#.###...
//! ```
//!
//! By rotating, flipping, and rearranging them, you can find a square
//! arrangement that causes all adjacent borders to line up:
//!
//! ```text
//! #...##.#.. ..###..### #.#.#####.
//! ..#.#..#.# ###...#.#. .#..######
//! .###....#. ..#....#.. ..#.......
//! ###.##.##. .#.#.#..## ######....
//! .###.##### ##...#.### ####.#..#.
//! .##.#....# ##.##.###. .#...#.##.
//! #...###### ####.#...# #.#####.##
//! .....#..## #...##..#. ..#.###...
//! #.####...# ##..#..... ..#.......
//! #.##...##. ..##.#..#. ..#.###...
//!
//! #.##...##. ..##.#..#. ..#.###...
//! ##..#.##.. ..#..###.# ##.##....#
//! ##.####... .#.####.#. ..#.###..#
//! ####.#.#.. ...#.##### ###.#..###
//! .#.####... ...##..##. .######.##
//! .##..##.#. ....#...## #.#.#.#...
//! ....#..#.# #.#.#.##.# #.###.###.
//! ..#.#..... .#.##.#..# #.###.##..
//! ####.#.... .#..#.##.. .######...
//! ...#.#.#.# ###.##.#.. .##...####
//!
//! ...#.#.#.# ###.##.#.. .##...####
//! ..#.#.###. ..##.##.## #..#.##..#
//! ..####.### ##.#...##. .#.#..#.##
//! #..#.#..#. ...#.#.#.. .####.###.
//! .#..####.# #..#.#.#.# ####.###..
//! .#####..## #####...#. .##....##.
//! ##.##..#.. ..#...#... .####...#.
//! #.#.###... .##..##... .####.##.#
//! #...###... ..##...#.. ...#..####
//! ..#.#....# ##.#.#.... ...##.....
//! ```
//!
//! For reference, the IDs of the above tiles are:
//!
//! ```text
//! 1951    2311    3079
//! 2729    1427    2473
//! 2971    1489    1171
//! ```
//!
//! To check that you've assembled the image correctly, multiply the IDs of the
//! four corner tiles together. If you do this with the assembled tiles from the
//! example above, you get `1951 * 3079 * 2971 * 1171 = 20899048083289`.
//!
//! Assemble the tiles into an image. What do you get if you multiply together
//! the IDs of the four corner tiles?
//!
//!
//!
//! [Advent of Code 2020 - Day 20](https://adventofcode.com/2020/day/20)

use fxhash::FxBuildHasher;
use hashbrown::HashMap;
use std::convert::TryFrom;
use std::iter::FromIterator;
use std::mem;
use std::str::FromStr;

pub type Id = u16;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Angle {
    D90,
    D180,
    D270,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq)]
pub enum ParseColorError {
    InvalidCharacter(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Black,
    White,
}

impl TryFrom<char> for Color {
    type Error = ParseColorError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Color::Black),
            '#' => Ok(Color::White),
            _ => Err(ParseColorError::InvalidCharacter(value)),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseTileError {
    NotEnoughPixel,
    UnrecognizedColor(ParseColorError),
}

impl From<ParseColorError> for ParseTileError {
    fn from(err: ParseColorError) -> Self {
        ParseTileError::UnrecognizedColor(err)
    }
}

const TILE_LEN: usize = 10;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    pub id: Id,
    pub pixels: [[Color; TILE_LEN]; TILE_LEN],
}

impl FromStr for Tile {
    type Err = ParseTileError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut img = [[Color::Black; TILE_LEN]; TILE_LEN];
        let mut chars = s.chars();
        for row in 0..TILE_LEN {
            for col in 0..TILE_LEN {
                loop {
                    if let Some(c) = chars.next() {
                        if !c.is_whitespace() {
                            let color = Color::try_from(c)?;
                            img[row][col] = color;
                            break;
                        }
                    } else {
                        return Err(ParseTileError::NotEnoughPixel);
                    }
                }
            }
        }
        Ok(Self {
            id: Id::default(),
            pixels: img,
        })
    }
}

impl Tile {
    pub fn edges(&self) -> Edges {
        let [mut top, mut right, mut bottom, mut left] = [[Color::Black; TILE_LEN]; NUM_EDGES];
        for i in 0..TILE_LEN {
            top[i] = self.pixels[0][i];
            right[i] = self.pixels[i][TILE_LEN - 1];
            bottom[i] = self.pixels[TILE_LEN - 1][i];
            left[i] = self.pixels[i][0];
        }
        Edges::from([
            Edge::from(top),
            Edge::from(right),
            Edge::from(bottom),
            Edge::from(left),
        ])
    }

    pub fn rotate(&mut self, angle: Angle) {
        match angle {
            Angle::D90 => {
                let mut rotated = [[Color::Black; TILE_LEN]; TILE_LEN];
                for row in 0..TILE_LEN {
                    let rot_col = TILE_LEN - 1 - row;
                    for col in 0..TILE_LEN {
                        rotated[col][rot_col] = self.pixels[row][col]
                    }
                }
                mem::swap(&mut self.pixels, &mut rotated);
            }
            Angle::D180 => {
                let mut rotated = [[Color::Black; TILE_LEN]; TILE_LEN];
                for row in 0..TILE_LEN {
                    let rot_row = TILE_LEN - 1 - row;
                    for col in 0..TILE_LEN {
                        let rot_col = TILE_LEN - 1 - col;
                        rotated[rot_row][rot_col] = self.pixels[row][col]
                    }
                }
                mem::swap(&mut self.pixels, &mut rotated);
            }
            Angle::D270 => {
                let mut rotated = [[Color::Black; TILE_LEN]; TILE_LEN];
                for col in 0..TILE_LEN {
                    let rot_row = TILE_LEN - 1 - col;
                    for row in 0..TILE_LEN {
                        rotated[rot_row][row] = self.pixels[row][col]
                    }
                }
                mem::swap(&mut self.pixels, &mut rotated);
            }
        }
    }

    pub fn flip(&mut self, orientation: Orientation) {
        match orientation {
            Orientation::Horizontal => {
                for col_a in 0..TILE_LEN / 2 {
                    let col_b = TILE_LEN - 1 - col_a;
                    for row in 0..TILE_LEN {
                        self.pixels[row].swap(col_a, col_b);
                    }
                }
            }
            Orientation::Vertical => {
                for row in 0..TILE_LEN / 2 {
                    self.pixels.swap(row, TILE_LEN - 1 - row);
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeLocation {
    Top,
    Right,
    Bottom,
    Left,
}

const NUM_EDGES: usize = 4;
const EDGE_LOCATIONS: [EdgeLocation; NUM_EDGES] = [
    EdgeLocation::Top,
    EdgeLocation::Right,
    EdgeLocation::Bottom,
    EdgeLocation::Left,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edge([Color; TILE_LEN]);

impl From<[Color; TILE_LEN]> for Edge {
    fn from(pixels: [Color; TILE_LEN]) -> Self {
        Edge(pixels)
    }
}

impl Edge {
    pub fn new(pixels: [Color; TILE_LEN]) -> Self {
        Edge(pixels)
    }

    pub fn flip(self) -> Self {
        let mut pixels = self.0;
        for i in 0..TILE_LEN / 2 {
            pixels.swap(i, TILE_LEN - 1 - i);
        }
        Edge(pixels)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Edges([Edge; NUM_EDGES]);

impl From<[Edge; NUM_EDGES]> for Edges {
    fn from(edge_array: [Edge; 4]) -> Self {
        Edges(edge_array)
    }
}

impl Edges {
    pub fn edge(&self, location: EdgeLocation) -> Edge {
        match location {
            EdgeLocation::Top => self.0[0],
            EdgeLocation::Right => self.0[1],
            EdgeLocation::Bottom => self.0[2],
            EdgeLocation::Left => self.0[3],
        }
    }

    pub fn rotate(self, angle: Angle) -> Self {
        let [top, right, bottom, left] = self.0;
        match angle {
            Angle::D90 => Edges([left.flip(), top, right.flip(), bottom]),
            Angle::D180 => Edges([bottom.flip(), left.flip(), top.flip(), right.flip()]),
            Angle::D270 => Edges([right, bottom.flip(), left, top.flip()]),
        }
    }

    pub fn flip(self, orientation: Orientation) -> Self {
        let [top, right, bottom, left] = self.0;
        match orientation {
            Orientation::Horizontal => Edges([top.flip(), left, bottom.flip(), right]),
            Orientation::Vertical => Edges([bottom, right.flip(), top, left.flip()]),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub tiles: Vec<Vec<Tile>>,
}

#[aoc_generator(day20)]
pub fn parse_image_tiles(input: &str) -> Vec<Tile> {
    let mut tiles = Vec::new();
    let mut lines = input.lines();
    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        if line.starts_with("Tile") {
            let id_str = line
                .chars()
                .skip(5)
                .filter(|c| c.is_ascii_digit())
                .collect::<String>();
            let id = u16::from_str(&id_str).unwrap();
            let mut pixes_str = String::from(lines.next().unwrap());
            for _ in 0..9 {
                pixes_str.push_str(lines.next().unwrap());
            }
            let mut tile = Tile::from_str(&pixes_str).unwrap();
            tile.id = id;
            tiles.push(tile);
        }
    }
    tiles
}

const PROCESSING: [(Option<Angle>, Option<Orientation>); 11] = [
    (Some(Angle::D90), None),
    (Some(Angle::D180), None),
    (Some(Angle::D270), None),
    (Some(Angle::D90), Some(Orientation::Horizontal)),
    (Some(Angle::D180), Some(Orientation::Horizontal)),
    (Some(Angle::D270), Some(Orientation::Horizontal)),
    (Some(Angle::D90), Some(Orientation::Vertical)),
    (Some(Angle::D180), Some(Orientation::Vertical)),
    (Some(Angle::D270), Some(Orientation::Vertical)),
    (None, Some(Orientation::Horizontal)),
    (None, Some(Orientation::Vertical)),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TileEdge {
    id: Id,
    location: EdgeLocation,
    edge: Edge,
    flipped: Option<Orientation>,
    rotated: Option<Angle>,
}

fn reassemble_image(tiles: &[Tile]) -> Image {
    let tile_map =
        HashMap::<_, _, FxBuildHasher>::from_iter(tiles.iter().map(|tile| (tile.id, tile)));

    let original_tile_edges = Vec::from_iter(
        tiles
            .iter()
            .map(|tile| {
                let edges = tile.edges();
                EDGE_LOCATIONS
                    .iter()
                    .map(|loc| TileEdge {
                        id: tile.id,
                        location: *loc,
                        edge: edges.edge(*loc),
                        flipped: None,
                        rotated: None,
                    })
                    .collect::<Vec<_>>()
            })
            .flatten(),
    );

    let mut processed_tile_edges = Vec::with_capacity(original_tile_edges.len());
    for tile in tiles {
        let edges = tile.edges();
        for (rotate, flip) in &PROCESSING {
            let modified_edges = match (rotate, flip) {
                (Some(angle), Some(orientation)) => edges.rotate(*angle).flip(*orientation),
                (Some(angle), None) => edges.rotate(*angle),
                (None, Some(orientation)) => edges.flip(*orientation),
                (None, None) => edges,
            };
            for located in &EDGE_LOCATIONS {
                processed_tile_edges.push(TileEdge {
                    id: tile.id,
                    location: *located,
                    edge: modified_edges.edge(*located),
                    flipped: *flip,
                    rotated: *rotate,
                })
            }
        }
    }

    let tile_edges = Vec::from_iter(
        original_tile_edges
            .into_iter()
            .chain(processed_tile_edges.into_iter()),
    );

    let mut possible_neighbors: Vec<(TileEdge, TileEdge)> = Vec::with_capacity(tile_edges.len());
    for (i, edge1) in tile_edges.iter().enumerate() {
        for edge2 in tile_edges.iter().skip(i + 1) {
            match (edge1.location, edge2.location) {
                (EdgeLocation::Top, EdgeLocation::Bottom) => {
                    if edge1.edge == edge2.edge {
                        possible_neighbors.push((edge2.clone(), edge1.clone()));
                    }
                }
                (EdgeLocation::Bottom, EdgeLocation::Top) => {
                    if edge1.edge == edge2.edge {
                        possible_neighbors.push((edge1.clone(), edge2.clone()));
                    }
                }
                (EdgeLocation::Left, EdgeLocation::Right) => {
                    if edge1.edge == edge2.edge {
                        possible_neighbors.push((edge2.clone(), edge1.clone()));
                    }
                }
                (EdgeLocation::Right, EdgeLocation::Left) => {
                    if edge1.edge == edge2.edge {
                        possible_neighbors.push((edge1.clone(), edge2.clone()));
                    }
                }
                _ => {}
            };
        }
    }

    let edges_without_neighbor = tile_edges
        .iter()
        .filter(|tile| {
            !possible_neighbors
                .iter()
                .any(|(e1, e2)| e1.edge == tile.edge || e2.edge == tile.edge)
        })
        .collect::<Vec<_>>();

    dbg!(
        tile_edges.len(),
        possible_neighbors.len(),
        edges_without_neighbor.len()
    );

    todo!()
}

#[aoc(day20, part1)]
pub fn checksum_of_reassembled_image(tiles: &[Tile]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests;
