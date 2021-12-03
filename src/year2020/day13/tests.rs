use super::*;

const INPUT: &str = include_str!("../../../input/2020/day13.txt");

const EXAMPLE1: &str = "\
939
7,13,x,x,59,x,31,19
";

const EXAMPLE2: &str = "\
2
17,x,13,19
";

const EXAMPLE3: &str = "\
3
67,7,59,61
";

const EXAMPLE4: &str = "\
4
67,x,7,59,61
";

const EXAMPLE5: &str = "\
5
67,7,x,59,61
";

const EXAMPLE6: &str = "\
6
1789,37,47,1889
";

#[test]
fn parse_bus_service_notes_in_example() {
    let notes = parse_bus_service_notes(EXAMPLE1);

    assert_eq!(
        notes,
        BusServiceNotes {
            arrival_time: 939,
            bus_lines: vec![
                BusId::Scheduled(7),
                BusId::Scheduled(13),
                BusId::OutOfService,
                BusId::OutOfService,
                BusId::Scheduled(59),
                BusId::OutOfService,
                BusId::Scheduled(31),
                BusId::Scheduled(19),
            ]
        }
    );
}

#[test]
fn earliest_bus_to_catch_in_example1() {
    let notes = parse_bus_service_notes(EXAMPLE1);

    let earliest_bus = earliest_bus_to_catch(&notes);

    assert_eq!(earliest_bus, Some((BusId::Scheduled(59), 5)));
}

#[test]
fn earliest_bus_result_in_example1() {
    let notes = parse_bus_service_notes(EXAMPLE1);

    let result = earliest_bus_result(&notes);

    assert_eq!(result, Some(295));
}

#[test]
fn earliest_bus_result_in_puzzle_input() {
    let notes = parse_bus_service_notes(INPUT);

    let result = earliest_bus_result(&notes);

    assert_eq!(result, Some(2045));
}

#[test]
fn earliest_time_buses_depart_in_sequence_in_example1() {
    let notes = parse_bus_service_notes(EXAMPLE1);

    let earliest_time = earliest_time_buses_depart_in_sequence(&notes);

    assert_eq!(earliest_time, 1068781);
}

#[test]
fn earliest_time_buses_depart_in_sequence_in_example2() {
    let notes = parse_bus_service_notes(EXAMPLE2);

    let earliest_time = earliest_time_buses_depart_in_sequence(&notes);

    assert_eq!(earliest_time, 3417);
}

#[test]
fn earliest_time_buses_depart_in_sequence_in_example3() {
    let notes = parse_bus_service_notes(EXAMPLE3);

    let earliest_time = earliest_time_buses_depart_in_sequence(&notes);

    assert_eq!(earliest_time, 754018);
}

#[test]
fn earliest_time_buses_depart_in_sequence_in_example4() {
    let notes = parse_bus_service_notes(EXAMPLE4);

    let earliest_time = earliest_time_buses_depart_in_sequence(&notes);

    assert_eq!(earliest_time, 779210);
}

#[test]
fn earliest_time_buses_depart_in_sequence_in_example5() {
    let notes = parse_bus_service_notes(EXAMPLE5);

    let earliest_time = earliest_time_buses_depart_in_sequence(&notes);

    assert_eq!(earliest_time, 1261476);
}

#[test]
fn earliest_time_buses_depart_in_sequence_in_example6() {
    let notes = parse_bus_service_notes(EXAMPLE6);

    let earliest_time = earliest_time_buses_depart_in_sequence(&notes);

    assert_eq!(earliest_time, 1202161486);
}

#[ignore]
#[test]
fn earliest_time_buses_depart_in_sequence_in_puzzle_input() {
    let notes = parse_bus_service_notes(INPUT);

    let earliest_time = earliest_time_buses_depart_in_sequence(&notes);

    assert_eq!(earliest_time, 402251700208309);
}
