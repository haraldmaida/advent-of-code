use super::*;

const INPUT: &str = include_str!("../../../input/2020/day17.txt");

const EXAMPLE: &str = "\
.#.
..#
###
";

#[test]
fn parse_conway_cubes_in_example() {
    let cubes = parse_conway_cubes(EXAMPLE);

    assert_eq!(cubes.get(Position3D::from([0, 0, 0])), State::Inactive);
    assert_eq!(cubes.get(Position3D::from([1, 0, 0])), State::Active);
    assert_eq!(cubes.get(Position3D::from([2, 0, 0])), State::Inactive);
    assert_eq!(cubes.get(Position3D::from([0, 1, 0])), State::Inactive);
    assert_eq!(cubes.get(Position3D::from([1, 1, 0])), State::Inactive);
    assert_eq!(cubes.get(Position3D::from([2, 1, 0])), State::Active);
    assert_eq!(cubes.get(Position3D::from([0, 2, 0])), State::Active);
    assert_eq!(cubes.get(Position3D::from([1, 2, 0])), State::Active);
    assert_eq!(cubes.get(Position3D::from([2, 2, 0])), State::Active);
}

#[test]
fn num_active_cubes_after_6_cycle_boot_in_example() {
    let pocket_dimension = parse_conway_cubes(EXAMPLE);

    let num_active = num_active_cubes_after_6_cycle_boot(&pocket_dimension);

    assert_eq!(num_active, 112);
}

#[test]
fn num_active_cubes_after_6_cycle_boot_in_puzzle_input() {
    let pocket_dimension = parse_conway_cubes(INPUT);

    let num_active = num_active_cubes_after_6_cycle_boot(&pocket_dimension);

    assert_eq!(num_active, 242);
}

#[test]
fn num_active_hypercubes_after_6_cycle_boot_in_example() {
    let pocket_dimension = parse_conway_cubes(EXAMPLE);

    let num_active = num_active_hypercubes_after_6_cycle_boot(&pocket_dimension);

    assert_eq!(num_active, 848);
}

#[test]
fn num_active_hypercubes_after_6_cycle_boot_in_puzzle_input() {
    let pocket_dimension = parse_conway_cubes(INPUT);

    let num_active = num_active_hypercubes_after_6_cycle_boot(&pocket_dimension);

    assert_eq!(num_active, 2292);
}
