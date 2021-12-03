//! # Day 16: Ticket Translation
//!
//! As you're walking to yet another connecting flight, you realize that one of
//! the legs of your re-routed trip coming up is on a high-speed train. However,
//! the train ticket you were given is in a language you don't understand. You
//! should probably figure out what it says before you get to the train station
//! after the next flight.
//!
//! Unfortunately, you can't actually read the words on the ticket. You can,
//! however, read the numbers, and so you figure out the fields these tickets
//! must have and the valid ranges for values in those fields.
//!
//! You collect the rules for ticket fields, the numbers on your ticket, and the
//! numbers on other nearby tickets for the same train service (via the airport
//! security cameras) together into a single document you can reference (your
//! puzzle input).
//!
//! The rules for ticket fields specify a list of fields that exist somewhere on
//! the ticket and the valid ranges of values for each field. For example, a
//! rule like class: 1-3 or 5-7 means that one of the fields in every ticket is
//! named class and can be any value in the ranges 1-3 or 5-7 (inclusive, such
//! that 3 and 5 are both valid in this field, but 4 is not).
//!
//! Each ticket is represented by a single line of comma-separated values. The
//! values are the numbers on the ticket in the order they appear; every ticket
//! has the same format. For example, consider this ticket:
//!
//! ```text
//! .--------------------------------------------------------.
//! | ????: 101    ?????: 102   ??????????: 103     ???: 104 |
//! |                                                        |
//! | ??: 301  ??: 302             ???????: 303      ??????? |
//! | ??: 401  ??: 402           ???? ????: 403    ????????? |
//! '--------------------------------------------------------'
//! ```
//!
//! Here, ? represents text in a language you don't understand. This ticket
//! might be represented as `101,102,103,104,301,302,303,401,402,403;` of
//! course, the actual train tickets you're looking at are much more
//! complicated. In any case, you've extracted just the numbers in such a way
//! that the first number is always the same specific field, the second number
//! is always a different specific field, and so on - you just don't know what
//! each position actually means!
//!
//! Start by determining which tickets are completely invalid; these are tickets
//! that contain values which aren't valid for any field. Ignore your ticket
//! for now.
//!
//! For example, suppose you have the following notes:
//!
//! ```text
//! class: 1-3 or 5-7
//! row: 6-11 or 33-44
//! seat: 13-40 or 45-50
//!
//! your ticket:
//! 7,1,14
//!
//! nearby tickets:
//! 7,3,47
//! 40,4,50
//! 55,2,20
//! 38,6,12
//! ```
//!
//! It doesn't matter which position corresponds to which field; you can
//! identify invalid nearby tickets by considering only whether tickets contain
//! values that are not valid for any field. In this example, the values on the
//! first nearby ticket are all valid for at least one field. This is not true
//! of the other three nearby tickets: the values 4, 55, and 12 are are not
//! valid for any field. Adding together all of the invalid values produces your
//! ticket scanning error rate: 4 + 55 + 12 = 71.
//!
//! Consider the validity of the nearby tickets you scanned. What is your ticket
//! scanning error rate?
//!
//! ## Part 2
//!
//! Now that you've identified which tickets contain invalid values, discard
//! those tickets entirely. Use the remaining valid tickets to determine which
//! field is which.
//!
//! Using the valid ranges for each field, determine what order the fields
//! appear on the tickets. The order is consistent between all tickets: if seat
//! is the third field, it is the third field on every ticket, including your
//! ticket.
//!
//! For example, suppose you have the following notes:
//!
//! ```text
//! class: 0-1 or 4-19
//! row: 0-5 or 8-19
//! seat: 0-13 or 16-19
//!
//! your ticket:
//! 11,12,13
//!
//! nearby tickets:
//! 3,9,18
//! 15,1,5
//! 5,14,9
//! ```
//!
//! Based on the nearby tickets in the above example, the first position must be
//! row, the second position must be class, and the third position must be seat;
//! you can conclude that in your ticket, class is 12, row is 11, and seat is
//! 13.
//!
//! Once you work out which field is which, look for the six fields on your
//! ticket that start with the word departure. What do you get if you multiply
//! those six values together?
//!
//! [Advent of Code 2020 - Day 16](https://adventofcode.com/2020/day/16)

use hashbrown::HashMap;
use std::collections::hash_map::RandomState;
use std::collections::HashSet;
use std::iter::FromIterator;
use std::ops::RangeInclusive;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Field {
    pub label: String,
    pub ranges: [RangeInclusive<u32>; 2],
}

impl Field {
    pub fn is_valid(&self, value: u32) -> bool {
        self.ranges.iter().any(|r| r.contains(&value))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    pub values: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketNotes {
    pub fields: Vec<Field>,
    pub my_ticket: Ticket,
    pub nearby_tickets: Vec<Ticket>,
}

#[aoc_generator(day16)]
pub fn parse_ticket_notes(input: &str) -> TicketNotes {
    let mut fields = Vec::new();
    let mut my_ticket = None;
    let mut nearby_tickets = Vec::new();

    let mut lines = input.trim_end().lines();
    while let Some(line) = lines.next() {
        if line.trim().is_empty() {
            break;
        }
        let mut line_split = line.split(':');
        let label = line_split.next().unwrap();
        let mut range_split = line_split.next().unwrap().split(" or ");
        let mut range1 = range_split.next().unwrap().split('-');
        let mut range2 = range_split.next().unwrap().split('-');
        let range1_min = u32::from_str(range1.next().unwrap().trim()).unwrap();
        let range1_max = u32::from_str(range1.next().unwrap().trim()).unwrap();
        let range2_min = u32::from_str(range2.next().unwrap().trim()).unwrap();
        let range2_max = u32::from_str(range2.next().unwrap().trim()).unwrap();
        let field = Field {
            label: label.to_string(),
            ranges: [range1_min..=range1_max, range2_min..=range2_max],
        };
        fields.push(field);
    }

    while let Some(line) = lines.next() {
        if line.starts_with("your ticket:") {
            break;
        }
    }
    if let Some(line) = lines.next() {
        let values = line
            .split(',')
            .map(|s| {
                u32::from_str(s).unwrap_or_else(|err| panic!("not a valid number {:?}; {}", s, err))
            })
            .collect();
        my_ticket = Some(Ticket { values })
    }

    while let Some(line) = lines.next() {
        if line.starts_with("nearby tickets:") {
            break;
        }
    }
    while let Some(line) = lines.next() {
        let values = line
            .split(',')
            .map(|s| {
                u32::from_str(s).unwrap_or_else(|err| panic!("not a valid number {:?}; {}", s, err))
            })
            .collect();
        nearby_tickets.push(Ticket { values })
    }

    TicketNotes {
        fields,
        my_ticket: my_ticket.unwrap(),
        nearby_tickets,
    }
}

fn find_invalid_values_in_ticket(ticket: &Ticket, fields: &[Field]) -> Vec<u32> {
    ticket
        .values
        .iter()
        .filter(|val| fields.iter().all(|field| !field.is_valid(**val)))
        .copied()
        .collect()
}

fn find_invalid_values_in_nearby_tickets(ticket_notes: &TicketNotes) -> Vec<u32> {
    ticket_notes
        .nearby_tickets
        .iter()
        .map(|ticket| find_invalid_values_in_ticket(ticket, &ticket_notes.fields))
        .flatten()
        .collect()
}

#[aoc(day16, part1)]
pub fn ticket_scanning_error_rate(ticket_notes: &TicketNotes) -> u32 {
    find_invalid_values_in_nearby_tickets(ticket_notes)
        .into_iter()
        .sum()
}

fn determine_field_positions(ticket_notes: &TicketNotes) -> HashMap<String, usize> {
    let valid_nearby_tickets = ticket_notes
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            !ticket.values.iter().any(|val| {
                ticket_notes
                    .fields
                    .iter()
                    .all(|field| !field.is_valid(*val))
            })
        })
        .collect::<Vec<_>>();

    let num_fields = ticket_notes.fields.len();

    let mut invalid_positions: HashMap<usize, HashSet<String>> = HashMap::with_capacity(num_fields);
    for ticket in &valid_nearby_tickets {
        for (position, value) in ticket.values.iter().enumerate() {
            let position_entry = invalid_positions
                .entry(position)
                .or_insert_with(HashSet::new);
            for field in &ticket_notes.fields {
                if !field.is_valid(*value) {
                    position_entry.insert(field.label.clone());
                }
            }
        }
    }

    debug_assert_eq!(invalid_positions.len(), num_fields);

    let mut field_positions = HashMap::with_capacity(num_fields);
    let mut remaining_fields = HashSet::<_, RandomState>::from_iter(
        ticket_notes.fields.iter().map(|fld| fld.label.clone()),
    );
    while !remaining_fields.is_empty() {
        for (position, invalid_fields) in &invalid_positions {
            let mut valid_fields = remaining_fields
                .difference(invalid_fields)
                .cloned()
                .collect::<Vec<_>>();
            if valid_fields.len() == 1 {
                remaining_fields.remove(&valid_fields[0]);
                field_positions.insert(valid_fields.remove(0), *position);
                break;
            }
        }
    }

    field_positions
}

#[aoc(day16, part2)]
pub fn product_of_departure_values_in_my_ticket(ticket_notes: &TicketNotes) -> u64 {
    let field_positions = determine_field_positions(ticket_notes);
    ticket_notes
        .fields
        .iter()
        .filter_map(|field| {
            if field.label.starts_with("departure") {
                Some(u64::from(
                    ticket_notes.my_ticket.values[field_positions[&field.label]],
                ))
            } else {
                None
            }
        })
        .product()
}

#[cfg(test)]
mod tests;
