//! # Day 16: Packet Decoder
//!
//! As you leave the cave and reach open waters, you receive a transmission from
//! the Elves back on the ship.
//!
//! The transmission was sent using the Buoyancy Interchange Transmission System
//! (BITS), a method of packing numeric expressions into a binary sequence. Your
//! submarine's computer has saved the transmission in hexadecimal (your puzzle
//! input).
//!
//! The first step of decoding the message is to convert the hexadecimal
//! representation into binary. Each character of hexadecimal corresponds to
//! four bits of binary data:
//!
//! ```text
//! 0 = 0000
//! 1 = 0001
//! 2 = 0010
//! 3 = 0011
//! 4 = 0100
//! 5 = 0101
//! 6 = 0110
//! 7 = 0111
//! 8 = 1000
//! 9 = 1001
//! A = 1010
//! B = 1011
//! C = 1100
//! D = 1101
//! E = 1110
//! F = 1111
//! ```
//!
//! The BITS transmission contains a single packet at its outermost layer which
//! itself contains many other packets. The hexadecimal representation of this
//! packet might encode a few extra 0 bits at the end; these are not part of the
//! transmission and should be ignored.
//!
//! Every packet begins with a standard header: the first three bits encode the
//! packet version, and the next three bits encode the packet type ID. These two
//! values are numbers; all numbers encoded in any packet are represented as
//! binary with the most significant bit first. For example, a version encoded
//! as the binary sequence 100 represents the number 4.
//!
//! Packets with type ID 4 represent a literal value. Literal value packets
//! encode a single binary number. To do this, the binary number is padded with
//! leading zeroes until its length is a multiple of four bits, and then it is
//! broken into groups of four bits. Each group is prefixed by a 1 bit except
//! the last group, which is prefixed by a 0 bit. These groups of five bits
//! immediately follow the packet header. For example, the hexadecimal string
//! `D2FE28` becomes:
//!
//! ```text
//! 110100101111111000101000
//! VVVTTTAAAAABBBBBCCCCC
//! ```
//!
//! Below each bit is a label indicating its purpose:
//!
//! - The three bits labeled V (110) are the packet version, 6.
//! - The three bits labeled T (100) are the packet type ID, 4, which means the
//!   packet is a literal value.
//! - The five bits labeled A (10111) start with a 1 (not the last group, keep
//!   reading) and contain the first four bits of the number, 0111.
//! - The five bits labeled B (11110) start with a 1 (not the last group, keep
//!   reading) and contain four more bits of the number, 1110.
//! - The five bits labeled C (00101) start with a 0 (last group, end of packet)
//!   and contain the last four bits of the number, 0101.
//! - The three unlabeled 0 bits at the end are extra due to the hexadecimal
//!   representation and should be ignored.
//!
//! So, this packet represents a literal value with binary representation
//! `011111100101`, which is 2021 in decimal.
//!
//! Every other type of packet (any packet with a type ID other than 4)
//! represent an operator that performs some calculation on one or more
//! sub-packets contained within. Right now, the specific operations aren't
//! important; focus on parsing the hierarchy of sub-packets.
//!
//! An operator packet contains one or more packets. To indicate which
//! subsequent binary data represents its sub-packets, an operator packet can
//! use one of two modes indicated by the bit immediately after the packet
//! header; this is called the length type ID:
//!
//! - If the length type ID is 0, then the next 15 bits are a number that
//!   represents the total length in bits of the sub-packets contained by this packet.
//! - If the length type ID is 1, then the next 11 bits are a number that
//!   represents the number of sub-packets immediately contained by this packet.
//!
//! Finally, after the length type ID bit and the 15-bit or 11-bit field, the
//! sub-packets appear.
//!
//! For example, here is an operator packet (hexadecimal string
//! `38006F45291200`) with length type ID 0 that contains two sub-packets:
//!
//! ```text
//! 00111000000000000110111101000101001010010001001000000000
//! VVVTTTILLLLLLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBBBBBB
//! ```
//!
//! - The three bits labeled V (001) are the packet version, 1.
//! - The three bits labeled T (110) are the packet type ID, 6, which means the
//!   packet is an operator.
//! - The bit labeled I (0) is the length type ID, which indicates that the
//!   length is a 15-bit number representing the number of bits in the
//!   sub-packets.
//! - The 15 bits labeled L (000000000011011) contain the length of the
//!   sub-packets in bits, 27.
//! - The 11 bits labeled A contain the first sub-packet, a literal value
//!   representing the number 10.
//! - The 16 bits labeled B contain the second sub-packet, a literal value
//!   representing the number 20.
//!
//! After reading 11 and 16 bits of sub-packet data, the total length indicated
//! in L (27) is reached, and so parsing of this packet stops.
//!
//! As another example, here is an operator packet (hexadecimal string
//! `EE00D40C823060`) with length type ID 1 that contains three sub-packets:
//!
//! ```text
//! 11101110000000001101010000001100100000100011000001100000
//! VVVTTTILLLLLLLLLLLAAAAAAAAAAABBBBBBBBBBBCCCCCCCCCCC
//! ```
//!
//! - The three bits labeled V (111) are the packet version, 7.
//! - The three bits labeled T (011) are the packet type ID, 3, which means the
//!   packet is an operator.
//! - The bit labeled I (1) is the length type ID, which indicates that the
//!   length is a 11-bit number representing the number of sub-packets.
//! - The 11 bits labeled L (00000000011) contain the number of sub-packets, 3.
//! - The 11 bits labeled A contain the first sub-packet, a literal value
//!   representing the number 1.
//! - The 11 bits labeled B contain the second sub-packet, a literal value
//!   representing the number 2.
//! - The 11 bits labeled C contain the third sub-packet, a literal value
//!   representing the number 3.
//!
//! After reading 3 complete sub-packets, the number of sub-packets indicated in
//! L (3) is reached, and so parsing of this packet stops.
//!
//! For now, parse the hierarchy of the packets throughout the transmission and
//! add up all of the version numbers.
//!
//! Here are a few more examples of hexadecimal-encoded transmissions:
//!
//! - `8A004A801A8002F478` represents an operator packet (version 4) which
//!   contains an operator packet (version 1) which contains an operator packet
//!   (version 5) which contains a literal value (version 6); this packet has a
//!   version sum of 16.
//! - `620080001611562C8802118E34` represents an operator packet (version 3)
//!   which contains two sub-packets; each sub-packet is an operator packet that
//!   contains two literal values. This packet has a version sum of 12.
//! - `C0015000016115A2E0802F182340` has the same structure as the previous
//!   example, but the outermost packet uses a different length type ID. This
//!   packet has a version sum of 23.
//! - `A0016C880162017C3686B18A3D4780` is an operator packet that contains an
//!   operator packet that contains an operator packet that contains five
//!   literal values; it has a version sum of 31.
//!
//! Decode the structure of your hexadecimal-encoded BITS transmission; what do
//! you get if you add up the version numbers in all packets?
//!
//! ## Part Two
//!
//! Now that you have the structure of your transmission decoded, you can
//! calculate the value of the expression it represents.
//!
//! Literal values (type ID 4) represent a single number as described above.
//! The remaining type IDs are more interesting:
//!
//! - Packets with type ID 0 are sum packets - their value is the sum of the
//!   values of their sub-packets. If they only have a single sub-packet, their
//!   value is the value of the sub-packet.
//! - Packets with type ID 1 are product packets - their value is the result of
//!   multiplying together the values of their sub-packets. If they only have a
//!   single sub-packet, their value is the value of the sub-packet.
//! - Packets with type ID 2 are minimum packets - their value is the minimum of
//!   the values of their sub-packets.
//! - Packets with type ID 3 are maximum packets - their value is the maximum of
//!   the values of their sub-packets.
//! - Packets with type ID 5 are greater than packets - their value is 1 if the
//!   value of the first sub-packet is greater than the value of the second
//!   sub-packet; otherwise, their value is 0. These packets always have exactly
//!   two sub-packets.
//! - Packets with type ID 6 are less than packets - their value is 1 if the
//!   value of the first sub-packet is less than the value of the second
//!   sub-packet; otherwise, their value is 0. These packets always have exactly
//!   two sub-packets.
//! - Packets with type ID 7 are equal to packets - their value is 1 if the
//!   value of the first sub-packet is equal to the value of the second
//!   sub-packet; otherwise, their value is 0. These packets always have exactly
//!   two sub-packets.
//!
//! Using these rules, you can now work out the value of the outermost packet in
//! your BITS transmission.
//!
//! For example:
//!
//! - `C200B40A82` finds the sum of 1 and 2, resulting in the value 3.
//! - `04005AC33890` finds the product of 6 and 9, resulting in the value 54.
//! - `880086C3E88112` finds the minimum of 7, 8, and 9, resulting in the value 7.
//! - `CE00C43D881120` finds the maximum of 7, 8, and 9, resulting in the value 9.
//! - `D8005AC2A8F0` produces 1, because 5 is less than 15.
//! - `F600BC2D8F` produces 0, because 5 is not greater than 15.
//! - `9C005AC2F8F0` produces 0, because 5 is not equal to 15.
//! - `9C0141080250320F1802104A08` produces 1, because 1 + 3 = 2 * 2.
//!
//! What do you get if you evaluate the expression represented by your
//! hexadecimal-encoded BITS transmission?
//!
//! [Advent of Code 2021 - Day 16](https://adventofcode.com/2021/day/16)

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ast(Entry);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Entry {
    Number(i64),
    Sum(Vec<Entry>),
    Product(Vec<Entry>),
    Minimum(Vec<Entry>),
    Maximum(Vec<Entry>),
    GreaterThan(Box<Entry>, Box<Entry>),
    LessThan(Box<Entry>, Box<Entry>),
    EqualTo(Box<Entry>, Box<Entry>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub version: u8,
    pub type_id: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Length {
    TotalLength(usize),
    NumberOfSubPackets(u32),
}

fn hex_to_binary(chr: char) -> Result<&'static str, String> {
    match chr {
        '0' => Ok("0000"),
        '1' => Ok("0001"),
        '2' => Ok("0010"),
        '3' => Ok("0011"),
        '4' => Ok("0100"),
        '5' => Ok("0101"),
        '6' => Ok("0110"),
        '7' => Ok("0111"),
        '8' => Ok("1000"),
        '9' => Ok("1001"),
        'A' => Ok("1010"),
        'B' => Ok("1011"),
        'C' => Ok("1100"),
        'D' => Ok("1101"),
        'E' => Ok("1110"),
        'F' => Ok("1111"),
        _ => Err(format!("not a hexadecimal character: {}", chr)),
    }
}

#[aoc_generator(day16)]
pub fn parse(input: &str) -> String {
    let line = input
        .lines()
        .filter(|line| !line.is_empty())
        .next()
        .expect("no input line");
    let mut binary_string = String::new();
    for chr in line.chars() {
        let bit_str = hex_to_binary(chr).unwrap_or_else(|err| panic!("{}", err));
        binary_string.push_str(bit_str);
    }
    binary_string
}

fn decode_version(input: &str) -> (u8, &str, usize) {
    let (version_str, rest) = input.split_at(3);
    let version = u8::from_str_radix(version_str, 2)
        .unwrap_or_else(|err| panic!("not an integer: {} - {}", version_str, err));
    (version, rest, 3)
}

fn decode_type_id(input: &str) -> (u8, &str, usize) {
    let (type_id_str, rest) = input.split_at(3);
    let type_id = u8::from_str_radix(type_id_str, 2)
        .unwrap_or_else(|err| panic!("not an integer: {} - {}", type_id_str, err));
    (type_id, rest, 3)
}

fn decode_header(input: &str) -> (Header, &str, usize) {
    let (version, rest, consumed1) = decode_version(input);
    let (type_id, rest, consumed2) = decode_type_id(rest);
    (Header { version, type_id }, rest, consumed1 + consumed2)
}

fn decode_number(input: &str) -> (i64, &str, usize) {
    let mut consumed = 0;
    let mut input = input;
    let mut number = 0;
    let mut keep_reading = true;
    while keep_reading {
        let (group, rest) = input.split_at(5);
        consumed += 5;
        let (more, digit) = group.split_at(1);
        number = number * 16
            + i64::from_str_radix(digit, 2)
                .unwrap_or_else(|err| panic!("not a binary number: {} - {}", digit, err));
        input = rest;
        keep_reading = more == "1";
    }
    (number, input, consumed)
}

fn decode_length(input: &str) -> (Length, &str, usize) {
    let (length_type, rest) = input.split_at(1);
    match length_type {
        "0" => {
            let (length_str, rest) = rest.split_at(15);
            let total_length = usize::from_str_radix(length_str, 2)
                .unwrap_or_else(|err| panic!("not a valid length: {} - {}", length_str, err));
            (Length::TotalLength(total_length), rest, 16)
        }
        "1" => {
            let (num_subpackets_str, rest) = rest.split_at(11);
            let num_subpackets = u32::from_str_radix(num_subpackets_str, 2).unwrap_or_else(|err| {
                panic!(
                    "not a valid number of subpackets: {} - {}",
                    num_subpackets_str, err
                )
            });
            (Length::NumberOfSubPackets(num_subpackets), rest, 12)
        }
        _ => panic!("invalid length type: {}", length_type),
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(message: &str) -> u32 {
    let mut open = Vec::new();
    let (header, rest, _) = decode_header(message);
    let mut version_sum = 0;
    open.push((header, rest));
    while let Some((header, input)) = open.pop() {
        version_sum += header.version as u32;
        let rest = match header.type_id {
            4 => {
                let (_number, rest, _) = decode_number(input);
                rest
            }
            _ => {
                let (_length, rest, _) = decode_length(input);
                rest
            }
        };
        if rest.len() > 10 {
            let (header, rest, _) = decode_header(rest);
            open.push((header, rest));
        }
    }
    version_sum
}

fn decode_operation(header: Header, data: Vec<Entry>) -> Result<Entry, String> {
    match header.type_id {
        0 => Ok(Entry::Sum(data)),
        1 => Ok(Entry::Product(data)),
        2 => Ok(Entry::Minimum(data)),
        3 => Ok(Entry::Maximum(data)),
        5 => {
            let mut data = data.into_iter();
            let data1 = data
                .next()
                .ok_or_else(|| format!("no operand for greater than operation"))?;
            let data2 = data
                .next()
                .ok_or_else(|| format!("missing second operand for greater than operation"))?;
            if data.next().is_some() {
                return Err(format!("more than 2 operands for greater than operation"));
            }
            Ok(Entry::GreaterThan(Box::new(data1), Box::new(data2)))
        }
        6 => {
            let mut data = data.into_iter();
            let data1 = data
                .next()
                .ok_or_else(|| format!("no operand for less than operation"))?;
            let data2 = data
                .next()
                .ok_or_else(|| format!("missing second operand for less than operation"))?;
            if data.next().is_some() {
                return Err(format!("more than 2 operands for less than operation"));
            }
            Ok(Entry::LessThan(Box::new(data1), Box::new(data2)))
        }
        7 => {
            let mut data = data.into_iter();
            let data1 = data
                .next()
                .ok_or_else(|| format!("no operand for equal to operation"))?;
            let data2 = data
                .next()
                .ok_or_else(|| format!("missing second operand for equal to operation"))?;
            if data.next().is_some() {
                return Err(format!("more than 2 operands for equal to operation"));
            }
            Ok(Entry::EqualTo(Box::new(data1), Box::new(data2)))
        }
        _ => Err(format!("unrecognized type id: {}", header.type_id)),
    }
}

fn update_remaining_total_length(open: &mut [(Header, Length, Vec<Entry>)], consumed: usize) {
    open.iter_mut()
        .for_each(|(_, remaining, _)| match remaining {
            Length::TotalLength(len) => *len -= consumed,
            Length::NumberOfSubPackets(_) => {}
        });
}

fn close_packets(open: &mut Vec<(Header, Length, Vec<Entry>)>) -> Result<Option<Entry>, String> {
    while let Some((header, mut remaining, data)) = open.pop() {
        let end_of_packet = match &mut remaining {
            Length::TotalLength(len) => *len == 0,
            Length::NumberOfSubPackets(num) => {
                *num -= 1;
                *num == 0
            }
        };
        if end_of_packet {
            let operation = decode_operation(header, data)?;
            if let Some((_, _, parent)) = open.last_mut() {
                parent.push(operation);
            } else {
                return Ok(Some(operation));
            }
        } else {
            open.push((header, remaining, data));
            break;
        }
    }
    Ok(None)
}

pub fn decode_message(message: &str) -> Result<Ast, String> {
    let mut open: Vec<(Header, Length, Vec<Entry>)> = Vec::new();
    let mut input = message;
    while input.len() > 10 {
        let (header, rest, consumed1) = decode_header(input);
        match header.type_id {
            4 => {
                let (number, rest, consumed2) = decode_number(rest);
                input = rest;
                update_remaining_total_length(&mut open, consumed1 + consumed2);
                if let Some((_, _, data)) = open.last_mut() {
                    data.push(Entry::Number(number));
                }
                if let Some(root) = close_packets(&mut open)? {
                    return Ok(Ast(root));
                }
            }
            _ => {
                let (length, rest, consumed2) = decode_length(rest);
                input = rest;
                update_remaining_total_length(&mut open, consumed1 + consumed2);
                open.push((header, length, Vec::new()));
            }
        }
    }
    Err(format!("no operation in message"))
}

fn evaluate_operation(operator: &Entry, params: &[i64]) -> i64 {
    match operator {
        Entry::Number(_) => unreachable!(),
        Entry::Sum(_) => params.iter().copied().sum(),
        Entry::Product(_) => params.iter().copied().product(),
        Entry::Minimum(_) => params.iter().copied().min().unwrap_or(0),
        Entry::Maximum(_) => params.iter().copied().max().unwrap_or(0),
        Entry::GreaterThan(_, _) => {
            if params[0] > params[1] {
                1
            } else {
                0
            }
        }
        Entry::LessThan(_, _) => {
            if params[0] < params[1] {
                1
            } else {
                0
            }
        }
        Entry::EqualTo(_, _) => {
            if params[0] == params[1] {
                1
            } else {
                0
            }
        }
    }
}

pub fn evaluate_ast(ast: &Ast) -> i64 {
    let mut open: Vec<(&Entry, Option<usize>)> = Vec::new();
    let mut closed: Vec<(&Entry, Option<usize>, Vec<i64>)> = Vec::new();
    open.push((&ast.0, None));
    while let Some((entry, parent)) = open.pop() {
        match entry {
            Entry::Number(number) => {
                if let Some(index) = parent {
                    closed[index].2.push(*number);
                } else {
                    return *number;
                }
            }
            Entry::Sum(entries) => {
                let current = Some(closed.len());
                open.extend(entries.iter().rev().map(|e| (e, current)));
                closed.push((entry, parent, Vec::new()));
            }
            Entry::Product(entries) => {
                let current = Some(closed.len());
                open.extend(entries.iter().rev().map(|e| (e, current)));
                closed.push((entry, parent, Vec::new()));
            }
            Entry::Minimum(entries) => {
                let current = Some(closed.len());
                open.extend(entries.iter().rev().map(|e| (e, current)));
                closed.push((entry, parent, Vec::new()));
            }
            Entry::Maximum(entries) => {
                let current = Some(closed.len());
                open.extend(entries.iter().rev().map(|e| (e, current)));
                closed.push((entry, parent, Vec::new()));
            }
            Entry::GreaterThan(entry1, entry2) => {
                let current = Some(closed.len());
                open.push((entry2, current));
                open.push((entry1, current));
                closed.push((entry, parent, Vec::new()));
            }
            Entry::LessThan(entry1, entry2) => {
                let current = Some(closed.len());
                open.push((entry2, current));
                open.push((entry1, current));
                closed.push((entry, parent, Vec::new()));
            }
            Entry::EqualTo(entry1, entry2) => {
                let current = Some(closed.len());
                open.push((entry2, current));
                open.push((entry1, current));
                closed.push((entry, parent, Vec::new()));
            }
        }
    }
    while let Some((operator, parent, params)) = closed.pop() {
        let result = evaluate_operation(operator, &params);
        if let Some(index) = parent {
            closed[index].2.insert(0, result);
        } else {
            return result;
        }
    }
    unreachable!()
}

#[aoc(day16, part2)]
pub fn solve_part2(message: &str) -> i64 {
    let ast = decode_message(message).unwrap_or_else(|err| panic!("{}", err));
    evaluate_ast(&ast)
}

#[cfg(test)]
mod tests;
