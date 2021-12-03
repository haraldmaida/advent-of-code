use super::*;

const INPUT: &str = include_str!("../../input/2020/day4.txt");

const EXAMPLE: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
";

#[test]
fn parse_example_input() {
    let records = parse(EXAMPLE);

    let record = &records[0];
    assert_eq!(record.get("byr"), Some(&"1937".into()));
    assert_eq!(record.get("iyr"), Some(&"2017".into()));
    assert_eq!(record.get("eyr"), Some(&"2020".into()));
    assert_eq!(record.get("hgt"), Some(&"183cm".into()));
    assert_eq!(record.get("hcl"), Some(&"#fffffd".into()));
    assert_eq!(record.get("ecl"), Some(&"gry".into()));
    assert_eq!(record.get("pid"), Some(&"860033327".into()));
    assert_eq!(record.get("cid"), Some(&"147".into()));

    let record = &records[1];
    assert_eq!(record.get("byr"), Some(&"1929".into()));
    assert_eq!(record.get("iyr"), Some(&"2013".into()));
    assert_eq!(record.get("eyr"), Some(&"2023".into()));
    assert_eq!(record.get("hgt"), None);
    assert_eq!(record.get("hcl"), Some(&"#cfa07d".into()));
    assert_eq!(record.get("ecl"), Some(&"amb".into()));
    assert_eq!(record.get("pid"), Some(&"028048884".into()));
    assert_eq!(record.get("cid"), Some(&"350".into()));

    let record = &records[2];
    assert_eq!(record.get("byr"), Some(&"1931".into()));
    assert_eq!(record.get("iyr"), Some(&"2013".into()));
    assert_eq!(record.get("eyr"), Some(&"2024".into()));
    assert_eq!(record.get("hgt"), Some(&"179cm".into()));
    assert_eq!(record.get("hcl"), Some(&"#ae17e1".into()));
    assert_eq!(record.get("ecl"), Some(&"brn".into()));
    assert_eq!(record.get("pid"), Some(&"760753108".into()));
    assert_eq!(record.get("cid"), None);

    let record = &records[3];
    assert_eq!(record.get("byr"), None);
    assert_eq!(record.get("iyr"), Some(&"2011".into()));
    assert_eq!(record.get("eyr"), Some(&"2025".into()));
    assert_eq!(record.get("hgt"), Some(&"59in".into()));
    assert_eq!(record.get("hcl"), Some(&"#cfa07d".into()));
    assert_eq!(record.get("ecl"), Some(&"brn".into()));
    assert_eq!(record.get("pid"), Some(&"166559648".into()));
    assert_eq!(record.get("cid"), None);
}

#[test]
fn count_valid_passports_in_example_input() {
    let count = count_valid_passports(&parse(EXAMPLE));

    assert_eq!(count, 2);
}

#[test]
fn count_valid_passports_in_puzzle_input() {
    let count = count_valid_passports(&parse(INPUT));

    assert_eq!(count, 210);
}

#[test]
fn count_valid_passports2_in_puzzle_input() {
    let count = count_valid_passports2(&parse(INPUT));

    assert_eq!(count, 131);
}
