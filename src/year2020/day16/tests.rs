use super::*;

const INPUT: &str = include_str!("../../../input/2020/day16.txt");

const EXAMPLE1: &str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12
";

const EXAMPLE2: &str = "\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9
";

#[test]
fn parse_ticket_notes_in_example1() {
    let notes = parse_ticket_notes(EXAMPLE1);

    assert_eq!(
        notes,
        TicketNotes {
            fields: vec![
                Field {
                    label: "class".to_string(),
                    ranges: [1..=3, 5..=7]
                },
                Field {
                    label: "row".to_string(),
                    ranges: [6..=11, 33..=44]
                },
                Field {
                    label: "seat".to_string(),
                    ranges: [13..=40, 45..=50]
                },
            ],
            my_ticket: Ticket {
                values: vec![7, 1, 14]
            },
            nearby_tickets: vec![
                Ticket {
                    values: vec![7, 3, 47]
                },
                Ticket {
                    values: vec![40, 4, 50]
                },
                Ticket {
                    values: vec![55, 2, 20]
                },
                Ticket {
                    values: vec![38, 6, 12]
                },
            ]
        }
    )
}

#[test]
fn parse_ticket_notes_in_puzzle_input() {
    let notes = parse_ticket_notes(INPUT);

    assert_eq!(notes.fields.len(), 20);
    assert_eq!(notes.nearby_tickets.len(), 261 - 25);
}

#[test]
fn all_tickets_have_same_count_of_fields_in_puzzle_input() {
    let notes = parse_ticket_notes(INPUT);

    let my_ticket_len = notes.my_ticket.values.len();

    for ticket in notes.nearby_tickets {
        assert_eq!(ticket.values.len(), my_ticket_len);
    }
}

#[test]
fn ticket_scanning_error_rate_in_example1() {
    let notes = parse_ticket_notes(EXAMPLE1);

    let error_rate = ticket_scanning_error_rate(&notes);

    assert_eq!(error_rate, 71);
}

#[test]
fn ticket_scanning_error_rate_in_puzzle_input() {
    let notes = parse_ticket_notes(INPUT);

    let error_rate = ticket_scanning_error_rate(&notes);

    assert_eq!(error_rate, 19087);
}

#[test]
fn determine_field_positions_in_example2() {
    let notes = parse_ticket_notes(EXAMPLE2);

    let field_positions = determine_field_positions(&notes);

    assert_eq!(field_positions["row"], 0);
    assert_eq!(field_positions["class"], 1);
    assert_eq!(field_positions["seat"], 2);
}

#[test]
fn determine_field_positions_in_puzzle_input() {
    let notes = parse_ticket_notes(INPUT);

    let field_positions = determine_field_positions(&notes);

    assert_eq!(field_positions["departure time"], 0);
    assert_eq!(field_positions["departure track"], 1);
    assert_eq!(field_positions["class"], 2);
    assert_eq!(field_positions["duration"], 3);
    assert_eq!(field_positions["arrival track"], 4);
    assert_eq!(field_positions["zone"], 5);
    assert_eq!(field_positions["train"], 6);
    assert_eq!(field_positions["price"], 7);
    assert_eq!(field_positions["arrival platform"], 8);
    assert_eq!(field_positions["departure platform"], 9);
    assert_eq!(field_positions["arrival station"], 10);
    assert_eq!(field_positions["arrival location"], 11);
    assert_eq!(field_positions["row"], 12);
    assert_eq!(field_positions["route"], 13);
    assert_eq!(field_positions["departure station"], 14);
    assert_eq!(field_positions["departure location"], 15);
    assert_eq!(field_positions["type"], 16);
    assert_eq!(field_positions["departure date"], 17);
    assert_eq!(field_positions["wagon"], 18);
    assert_eq!(field_positions["seat"], 19);
}

#[test]
fn product_of_departure_values_in_my_ticket_in_puzzle_input() {
    let notes = parse_ticket_notes(INPUT);

    let result = product_of_departure_values_in_my_ticket(&notes);

    assert_eq!(result, 1382443095281);
}
