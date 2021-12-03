use super::*;

const INPUT: &str = include_str!("../../../input/2020/day19.txt");

const EXAMPLE: &str = r#"
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: "a"
5: "b"

ababbb
bababa
abbbab
aaabbb
aaaabbb
"#;

const EXAMPLE2: &str = r#"
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
"#;

#[test]
fn parse_monster_messages_in_example() {
    let monster_messages = parse_monster_messages(EXAMPLE);

    assert_eq!(
        monster_messages,
        MonsterMessages {
            rules: vec![
                Expr::Seq(vec![4, 1, 5]),
                Expr::Or(vec![2, 3], vec![3, 2]),
                Expr::Or(vec![4, 4], vec![5, 5]),
                Expr::Or(vec![4, 5], vec![5, 4]),
                Expr::Char('a'),
                Expr::Char('b'),
            ],
            messages: vec![
                "ababbb".into(),
                "bababa".into(),
                "abbbab".into(),
                "aaabbb".into(),
                "aaaabbb".into(),
            ]
        }
    )
}

#[test]
fn parse_monster_messages_in_puzzle_input() {
    let monster_messages = parse_monster_messages(INPUT);

    assert_eq!(monster_messages.rules.len(), 139);
    assert_eq!(monster_messages.messages.len(), 443);
    assert_eq!(monster_messages.rules[8], Expr::Seq(vec![42]));
    assert_eq!(monster_messages.rules[123], Expr::Char('a'));
    assert_eq!(monster_messages.rules[97], Expr::Char('b'));
}

#[test]
fn first_message_matches_pattern_in_example() {
    let monster_messages = parse_monster_messages(EXAMPLE);
    let pattern = Pattern::new(monster_messages.rules);

    assert!(pattern.matches("ababbb"));
}

#[test]
fn second_message_does_not_match_pattern_in_example() {
    let monster_messages = parse_monster_messages(EXAMPLE);
    let pattern = Pattern::new(monster_messages.rules);

    assert!(!pattern.matches("bababa"));
}

#[test]
fn third_message_matches_pattern_in_example() {
    let monster_messages = parse_monster_messages(EXAMPLE);
    let pattern = Pattern::new(monster_messages.rules);

    assert!(pattern.matches("abbbab"));
}

#[test]
fn fourth_message_does_not_match_pattern_in_example() {
    let monster_messages = parse_monster_messages(EXAMPLE);
    let pattern = Pattern::new(monster_messages.rules);

    assert!(!pattern.matches("aaabbb"));
}

#[test]
fn fifth_message_does_not_match_pattern_in_example() {
    let monster_messages = parse_monster_messages(EXAMPLE);
    let pattern = Pattern::new(monster_messages.rules);

    assert!(!pattern.matches("aaaabbb"));
}

#[test]
fn count_matching_messages_in_example() {
    let monster_messages = parse_monster_messages(EXAMPLE);

    let count = count_matching_messages(&monster_messages);

    assert_eq!(count, 2);
}

#[test]
fn count_matching_messages_in_puzzle_input() {
    let monster_messages = parse_monster_messages(INPUT);

    let count = count_matching_messages(&monster_messages);

    assert_ne!(count, 14);
    assert_eq!(count, 198);
}

#[test]
fn count_matching_messages_in_example2() {
    let monster_messages = parse_monster_messages(EXAMPLE2);

    let count = count_matching_messages(&monster_messages);

    assert_eq!(count, 3);
}

#[test]
fn count_matching_messages_patched_rules_in_example2() {
    let monster_messages = parse_monster_messages(EXAMPLE2);

    let count = count_matching_messages_patched_rules(&monster_messages);

    assert_eq!(count, 12);
}

#[test]
fn count_matching_messages_patched_rules_in_puzzle_input() {
    let monster_messages = parse_monster_messages(INPUT);

    let count = count_matching_messages_patched_rules(&monster_messages);

    assert_ne!(count, 14);
    assert_eq!(count, 198);
}
