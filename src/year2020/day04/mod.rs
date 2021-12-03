//! # Day 4: Passport Processing
//!
//! You arrive at the airport only to realize that you grabbed your North Pole
//! Credentials instead of your passport. While these documents are extremely
//! similar, North Pole Credentials aren't issued by a country and therefore
//! aren't actually valid documentation for travel in most of the world.
//!
//! It seems like you're not the only one having problems, though; a very long
//! line has formed for the automatic passport scanners, and the delay could
//! upset your travel itinerary.
//!
//! Due to some questionable network security, you realize you might be able to
//! solve both of these problems at the same time.
//!
//! The automatic passport scanners are slow because they're having trouble
//! detecting which passports have all required fields. The expected fields are
//! as follows:
//!
//! ```text
//! byr (Birth Year)
//! iyr (Issue Year)
//! eyr (Expiration Year)
//! hgt (Height)
//! hcl (Hair Color)
//! ecl (Eye Color)
//! pid (Passport ID)
//! cid (Country ID)
//! ```
//!
//! Passport data is validated in batch files (your puzzle input). Each passport
//! is represented as a sequence of key:value pairs separated by spaces or
//! newlines. Passports are separated by blank lines.
//!
//! Here is an example batch file containing four passports:
//!
//! ```text
//! ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
//! byr:1937 iyr:2017 cid:147 hgt:183cm
//!
//! iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
//! hcl:#cfa07d byr:1929
//!
//! hcl:#ae17e1 iyr:2013
//! eyr:2024
//! ecl:brn pid:760753108 byr:1931
//! hgt:179cm
//!
//! hcl:#cfa07d eyr:2025 pid:166559648
//! iyr:2011 ecl:brn hgt:59in
//! ```
//!
//! The first passport is valid - all eight fields are present. The second
//! passport is invalid - it is missing hgt (the Height field).
//!
//! The third passport is interesting; the only missing field is cid, so it
//! looks like data from North Pole Credentials, not a passport at all! Surely,
//! nobody would mind if you made the system temporarily ignore missing cid
//! fields. Treat this "passport" as valid.
//!
//! The fourth passport is missing two fields, cid and byr. Missing cid is fine,
//! but missing any other field is not, so this passport is invalid.
//!
//! According to the above rules, your improved system would report 2 valid
//! passports.
//!
//! Count the number of valid passports - those that have all required fields.
//! Treat cid as optional. In your batch file, how many passports are valid?
//!
//!
//! # Part 2
//!
//! The line is moving more quickly now, but you overhear airport security
//! talking about how passports with invalid data are getting through. Better
//! add some data validation, quick!
//!
//! You can continue to ignore the cid field, but each other field has strict
//! rules about what values are valid for automatic validation:
//!
//! ```text
//! byr (Birth Year) - four digits; at least 1920 and at most 2002.
//! iyr (Issue Year) - four digits; at least 2010 and at most 2020.
//! eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
//! hgt (Height) - a number followed by either cm or in:
//! If cm, the number must be at least 150 and at most 193.
//! If in, the number must be at least 59 and at most 76.
//! hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
//! ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
//! pid (Passport ID) - a nine-digit number, including leading zeroes.
//! cid (Country ID) - ignored, missing or not.
//! ```
//!
//! Your job is to count the passports where all required fields are both
//! present and valid according to the above rules. Here are some example
//! values:
//!
//! ```text
//! byr valid:   2002
//! byr invalid: 2003
//!
//! hgt valid:   60in
//! hgt valid:   190cm
//! hgt invalid: 190in
//! hgt invalid: 190
//!
//! hcl valid:   #123abc
//! hcl invalid: #123abz
//! hcl invalid: 123abc
//!
//! ecl valid:   brn
//! ecl invalid: wat
//!
//! pid valid:   000000001
//! pid invalid: 0123456789
//! ```
//!
//! Here are some invalid passports:
//!
//! ```text
//! eyr:1972 cid:100
//! hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
//!
//! iyr:2019
//! hcl:#602927 eyr:1967 hgt:170cm
//! ecl:grn pid:012533040 byr:1946
//!
//! hcl:dab227 iyr:2012
//! ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
//!
//! hgt:59cm ecl:zzz
//! eyr:2038 hcl:74454a iyr:2023
//! pid:3556412378 byr:2007
//! ```
//!
//! Here are some valid passports:
//!
//! ```text
//! pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
//! hcl:#623a2f
//!
//! eyr:2029 ecl:blu cid:129 byr:1989
//! iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm
//!
//! hcl:#888785
//! hgt:164cm byr:2001 iyr:2015 cid:88
//! pid:545766238 ecl:hzl
//! eyr:2022
//!
//! iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
//! ```
//!
//! Count the number of valid passports - those that have all required fields
//! and valid values. Continue to treat cid as optional. In your batch file,
//! how many passports are valid?
//!
//! [Advent of Code 2020 - Day 4](https://adventofcode.com/2020/day/4)

use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::mem;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Year(i16);

impl TryFrom<&str> for Year {
    type Error = ParseIntError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        i16::from_str(value).map(|val| Year(val))
    }
}

impl TryFrom<&String> for Year {
    type Error = ParseIntError;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        i16::from_str(value).map(|val| Year(val))
    }
}

impl PartialEq<i16> for Year {
    fn eq(&self, other: &i16) -> bool {
        self.0 == *other
    }
}

impl PartialOrd<i16> for Year {
    fn partial_cmp(&self, other: &i16) -> Option<Ordering> {
        self.0.partial_cmp(other)
    }
}

impl Year {
    pub fn val(self) -> i16 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_hex(value: &str) -> Option<Color> {
        if value.len() != 7 {
            return None;
        }
        if &value[0..1] != "#" {
            return None;
        }
        let r = u8::from_str_radix(&value[1..3], 16);
        let g = u8::from_str_radix(&value[3..5], 16);
        let b = u8::from_str_radix(&value[5..7], 16);
        if let (Ok(r), Ok(g), Ok(b)) = (r, g, b) {
            Some(Color { r, g, b })
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EyeColor {
    Amber,
    Blue,
    Brown,
    Grey,
    Green,
    Hazelnut,
    Other,
}

impl EyeColor {
    pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "amb" => Some(EyeColor::Amber),
            "blu" => Some(EyeColor::Blue),
            "brn" => Some(EyeColor::Brown),
            "gry" => Some(EyeColor::Grey),
            "grn" => Some(EyeColor::Green),
            "hzl" => Some(EyeColor::Hazelnut),
            "oth" => Some(EyeColor::Other),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passport {
    byr: Year,
    iyr: Year,
    eyr: Year,
    hgt: u16,
    hcl: Color,
    ecl: EyeColor,
    pid: String,
    cid: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
    Key,
    Value,
}

#[aoc_generator(day4)]
pub fn parse(input: &str) -> Vec<HashMap<String, String>> {
    let mut records = Vec::new();
    let mut entry = HashMap::new();
    let mut key = String::new();
    let mut value = String::new();
    let mut token = Token::Key;
    let mut new_lines = 0;

    for c in input.chars().chain("\n\n".chars()) {
        match c {
            ':' => token = Token::Value,
            '\n' => {
                if token == Token::Value {
                    entry.insert(
                        mem::replace(&mut key, String::new()),
                        mem::replace(&mut value, String::new()),
                    );
                    token = Token::Key;
                }
                new_lines += 1;
                if new_lines >= 2 {
                    new_lines = 0;
                    records.push(mem::replace(&mut entry, HashMap::new()));
                }
            }
            _ if c.is_whitespace() => {
                if token == Token::Value {
                    entry.insert(
                        mem::replace(&mut key, String::new()),
                        mem::replace(&mut value, String::new()),
                    );
                }
                token = Token::Key;
            }
            _ => {
                match token {
                    Token::Key => key.push(c),
                    Token::Value => value.push(c),
                }
                new_lines = 0;
            }
        }
    }
    records
}

fn validate_required_fields(passport: &HashMap<String, String>) -> bool {
    passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")
    // && passport.contains_key("cid") <-- cid is treated as optional
}

#[aoc(day4, part1)]
pub fn count_valid_passports(passports: &[HashMap<String, String>]) -> usize {
    passports
        .iter()
        .filter(|passport| validate_required_fields(passport))
        .count()
}

fn validate_passport_fields(passport: &HashMap<String, String>) -> bool {
    passport
        .get("byr")
        .map(|byr| {
            Year::try_from(byr)
                .map(|yr| yr.val() >= 1920 && yr.val() <= 2002)
                .unwrap_or(false)
        })
        .unwrap_or(false)
        && passport
            .get("iyr")
            .map(|iyr| {
                Year::try_from(iyr)
                    .map(|yr| yr.val() >= 2010 && yr.val() <= 2020)
                    .unwrap_or(false)
            })
            .unwrap_or(false)
        && passport
            .get("eyr")
            .map(|eyr| {
                Year::try_from(eyr)
                    .map(|yr| yr >= 2020 && yr <= 2030)
                    .unwrap_or(false)
            })
            .unwrap_or(false)
        && passport
            .get("pid")
            .map(|pid| pid.chars().filter(|c| c.is_ascii_digit()).count() == 9)
            .unwrap_or(false)
        && passport
            .get("ecl")
            .map(|ecl| EyeColor::from_code(&ecl).is_some())
            .unwrap_or(false)
        && passport
            .get("hcl")
            .map(|hcl| Color::from_hex(hcl).is_some())
            .unwrap_or(false)
        && passport
            .get("hgt")
            .map(|hgt| {
                if hgt.len() > 2 {
                    let unit = &hgt[hgt.len() - 2..];
                    u16::from_str(&hgt[0..hgt.len() - 2])
                        .map(|val| match unit {
                            "cm" => val >= 150 && val <= 193,
                            "in" => val >= 59 && val <= 76,
                            _ => false,
                        })
                        .unwrap_or(false)
                } else {
                    false
                }
            })
            .unwrap_or(false)
}

#[aoc(day4, part2)]
pub fn count_valid_passports2(passports: &[HashMap<String, String>]) -> usize {
    passports
        .iter()
        .filter(|passport| validate_passport_fields(passport))
        .count()
}

#[cfg(test)]
mod tests;
