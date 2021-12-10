//! # Day 10: Syntax Scoring
//!
//! You ask the submarine to determine the best route out of the deep-sea cave,
//! but it only replies:
//!
//! ```text
//! Syntax error in navigation subsystem on line: all of them
//! ```
//!
//! All of them?! The damage is worse than you thought. You bring up a copy of
//! the navigation subsystem (your puzzle input).
//!
//! The navigation subsystem syntax is made of several lines containing chunks.
//! There are one or more chunks on each line, and chunks contain zero or more
//! other chunks. Adjacent chunks are not separated by any delimiter; if one
//! chunk stops, the next chunk (if any) can immediately start. Every chunk must
//! open and close with one of four legal pairs of matching characters:
//!
//! - If a chunk opens with `(`, it must close with `)`.
//! - If a chunk opens with `[`, it must close with `]`.
//! - If a chunk opens with `{`, it must close with `}`.
//! - If a chunk opens with `<`, it must close with `>`.
//!
//! So, `()` is a legal chunk that contains no other chunks, as is `[]`. More
//! complex but valid chunks include `([])`, `{()()()}`, `<([{}])>`,
//! `[<>({}){}[([])<>]]`, and even `(((((((((())))))))))`.
//!
//! Some lines are incomplete, but others are corrupted. Find and discard the
//! corrupted lines first.
//!
//! A corrupted line is one where a chunk closes with the wrong character - that
//! is, where the characters it opens and closes with do not form one of the
//! four legal pairs listed above.
//!
//! Examples of corrupted chunks include `(]`, `{()()()>`, `(((()))}`, and
//! `<([]){()}[{}])`. Such a chunk can appear anywhere within a line, and its
//! presence causes the whole line to be considered corrupted.
//!
//! For example, consider the following navigation subsystem:
//!
//! ```text
//! [({(<(())[]>[[{[]{<()<>>
//! [(()[<>])]({[<{<<[]>>(
//! {([(<{}[<>[]}>{[]{[(<()>
//! (((({<>}<{<{<>}{[]{[]{}
//! [[<[([]))<([[{}[[()]]]
//! [{[{({}]{}}([{[{{{}}([]
//! {<[[]]>}<{[{[{[]{()[[[]
//! [<(<(<(<{}))><([]([]()
//! <{([([[(<>()){}]>(<<{{
//! <{([{{}}[<[[[<>{}]]]>[]]
//! ```
//!
//! Some of the lines aren't corrupted, just incomplete; you can ignore these
//! lines for now. The remaining five lines are corrupted:
//!
//! - `{([(<{}[<>[]}>{[]{[(<()>` - Expected ], but found } instead.
//! - `[[<[([]))<([[{}[[()]]]` - Expected ], but found ) instead.
//! - `[{[{({}]{}}([{[{{{}}([]` - Expected ), but found ] instead.
//! - `[<(<(<(<{}))><([]([]()` - Expected >, but found ) instead.
//! - `<{([([[(<>()){}]>(<<{{` - Expected ], but found > instead.
//!
//! Stop at the first incorrect closing character on each corrupted line.
//!
//! Did you know that syntax checkers actually have contests to see who can get
//! the high score for syntax errors in a file? It's true! To calculate the
//! syntax error score for a line, take the first illegal character on the line
//! and look it up in the following table:
//!
//! - `)`: 3 points.
//! - `]`: 57 points.
//! - `}`: 1197 points.
//! - `>`: 25137 points.
//!
//! In the above example, an illegal ) was found twice (2*3 = 6 points),
//! an illegal ] was found once (57 points), an illegal } was found once
//! (1197 points), and an illegal > was found once (25137 points). So, the total
//! syntax error score for this file is 6+57+1197+25137 = 26397 points!
//!
//! Find the first illegal character in each corrupted line of the navigation
//! subsystem. What is the total syntax error score for those errors?
//!
//! [Advent of Code 2021 - Day 10](https://adventofcode.com/2021/day/10)

const SCORE_LUT: [u32; 4] = [3, 57, 1197, 25137];

#[aoc_generator(day10)]
pub fn parse(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyntaxError {
    ClosingCharacterMismatch(char, char),
    MissingClosingCharacter(char),
    MissingOpeningCharacter(char),
}

fn matching_opening_character(closing: char) -> char {
    match closing {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        _ => panic!("invalid closing character"),
    }
}

fn matching_closing_character(opening: char) -> char {
    match opening {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => panic!("invalid opening character"),
    }
}

fn parse_code_line(code_line: &str) -> Result<(), SyntaxError> {
    let mut opened = Vec::new();
    for c in code_line.chars() {
        match c {
            '(' | '[' | '{' | '<' => opened.push(c),
            ')' | ']' | '}' | '>' => {
                if let Some(opening) = opened.pop() {
                    if c != matching_closing_character(opening) {
                        return Err(SyntaxError::ClosingCharacterMismatch(c, opening));
                    }
                } else {
                    return Err(SyntaxError::MissingOpeningCharacter(c));
                }
            }
            _ => {}
        }
    }
    if let Some(opening) = opened.pop() {
        return Err(SyntaxError::MissingClosingCharacter(opening));
    }
    Ok(())
}

fn score_syntax_error(error: SyntaxError) -> u32 {
    match error {
        SyntaxError::ClosingCharacterMismatch(c, _) => match c {
            ')' => SCORE_LUT[0],
            ']' => SCORE_LUT[1],
            '}' => SCORE_LUT[2],
            '>' => SCORE_LUT[3],
            _ => 0,
        },
        SyntaxError::MissingClosingCharacter(_) => 0,
        SyntaxError::MissingOpeningCharacter(_) => 0,
    }
}

#[aoc(day10, part1)]
pub fn total_syntax_error_score(code_lines: &[String]) -> u32 {
    code_lines
        .iter()
        .filter_map(|line| parse_code_line(line).err())
        .map(|err| score_syntax_error(err))
        .sum()
}

#[cfg(test)]
mod tests;
