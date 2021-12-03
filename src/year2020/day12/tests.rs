use super::*;

const INPUT: &str = include_str!("../../../input/2020/day12.txt");

const EXAMPLE: &str = "\
F10
N3
F7
R90
F11
";

#[test]
fn parse_navigation_instructions_in_example() {
    let instructions = parse_navigation_instructions(EXAMPLE);

    assert_eq!(
        instructions,
        vec![
            Navigate::Forward(10),
            Navigate::North(3),
            Navigate::Forward(7),
            Navigate::RotateRight(Angle::D90),
            Navigate::Forward(11)
        ]
    );
}

#[test]
fn parse_navigation_instructions_in_puzzle_input() {
    let instructions = parse_navigation_instructions(INPUT);

    assert_eq!(instructions.len(), 759);
}

#[test]
fn ship_starts_at_position_0_0_facing_east_by_default() {
    let ship = Ship::default();

    assert_eq!(
        ship,
        Ship {
            position: Position { north: 0, east: 0 },
            facing: Direction::East,
            waypoint: Position { north: 1, east: 10 },
        }
    )
}

#[test]
fn execute_instructions_according_rules_part1_in_example() {
    let instructions = parse_navigation_instructions(EXAMPLE);
    let mut ship = Ship::default();

    AutoPilot1::execute(&instructions, &mut ship);

    assert_eq!(
        ship,
        Ship {
            position: Position {
                north: -8,
                east: 17
            },
            facing: Direction::South,
            waypoint: Position { north: 1, east: 10 }
        }
    )
}

#[test]
fn part1_distance_to_final_position_in_example() {
    let instructions = parse_navigation_instructions(EXAMPLE);

    let distance = part1_distance_to_final_position(&instructions);

    assert_eq!(distance, 25);
}

#[test]
fn part1_distance_to_final_position_in_puzzle_input() {
    let instructions = parse_navigation_instructions(INPUT);

    let distance = part1_distance_to_final_position(&instructions);

    assert_eq!(distance, 923);
}

#[test]
fn execute_instructions_according_rules_part2_in_example() {
    let instructions = parse_navigation_instructions(EXAMPLE);
    let mut ship = Ship::default();

    AutoPilot2::execute(&instructions, &mut ship);

    assert_eq!(
        ship,
        Ship {
            position: Position {
                north: -72,
                east: 214
            },
            facing: Direction::East,
            waypoint: Position {
                north: -10,
                east: 4
            }
        }
    )
}

#[test]
fn part2_distance_to_final_position_in_example() {
    let instructions = parse_navigation_instructions(EXAMPLE);

    let distance = part2_distance_to_final_position(&instructions);

    assert_eq!(distance, 286);
}

#[test]
fn part2_distance_to_final_position_in_puzzle_input() {
    let instructions = parse_navigation_instructions(INPUT);

    let distance = part2_distance_to_final_position(&instructions);

    assert_eq!(distance, 24769);
}
