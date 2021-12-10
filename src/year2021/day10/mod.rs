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
//! ## Part Two
//!
//! Now, discard the corrupted lines. The remaining lines are incomplete.
//!
//! Incomplete lines don't have any incorrect characters - instead, they're
//! missing some closing characters at the end of the line. To repair the
//! navigation subsystem, you just need to figure out the sequence of closing
//! characters that complete all open chunks in the line.
//!
//! You can only use closing characters (`)`, `]`, `}`, or `>`), and you must
//! add them in the correct order so that only legal pairs are formed and all
//! chunks end up closed.
//!
//! In the example above, there are five incomplete lines:
//!
//! `[({(<(())[]>[[{[]{<()<>>` - Complete by adding }}]])})].
//! `[(()[<>])]({[<{<<[]>>(` - Complete by adding )}>]}).
//! `(((({<>}<{<{<>}{[]{[]{}` - Complete by adding }}>}>)))).
//! `{<[[]]>}<{[{[{[]{()[[[]` - Complete by adding ]]}}]}]}>.
//! `<{([{{}}[<[[[<>{}]]]>[]]` - Complete by adding ])}>.
//!
//! Did you know that autocomplete tools also have contests? It's true! The
//! score is determined by considering the completion string
//! character-by-character. Start with a total score of 0. Then, for each
//! character, multiply the total score by 5 and then increase the total score
//! by the point value given for the character in the following table:
//!
//! - `)`: 1 point.
//! - `]`: 2 points.
//! - `}`: 3 points.
//! - `>`: 4 points.
//!
//! So, the last completion string above - `])}>` - would be scored as follows:
//!
//! - Start with a total score of 0.
//! - Multiply the total score by 5 to get 0, then add the value of ] (2) to get
//!   a new total score of 2.
//! - Multiply the total score by 5 to get 10, then add the value of ) (1) to
//!   get a new total score of 11.
//! - Multiply the total score by 5 to get 55, then add the value of } (3) to
//!   get a new total score of 58.
//! - Multiply the total score by 5 to get 290, then add the value of > (4) to
//!   get a new total score of 294.
//!
//! The five lines' completion strings have total scores as follows:
//!
//! - `}}]])})]` - 288957 total points.
//! - `)}>]})` - 5566 total points.
//! - `}}>}>))))` - 1480781 total points.
//! - `]]}}]}]}>` - 995444 total points.
//! - `])}>` - 294 total points.
//!
//! Autocomplete tools are an odd bunch: the winner is found by sorting all of
//! the scores and then taking the middle score. (There will always be an odd
//! number of scores to consider.) In this example, the middle score is 288957
//! because there are the same number of scores smaller and larger than it.
//!
//! Find the completion string for each incomplete line, score the completion
//! strings, and sort the scores. What is the middle score?
//!
//! [Advent of Code 2021 - Day 10](https://adventofcode.com/2021/day/10)

#[aoc_generator(day10)]
pub fn parse(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|s| s.to_string())
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SyntaxError {
    ClosingCharacterMismatch(char, char),
    MissingOpeningCharacter(char),
    MissingClosingCharacter(Vec<char>),
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
                        return Err(SyntaxError::ClosingCharacterMismatch(opening, c));
                    }
                } else {
                    return Err(SyntaxError::MissingOpeningCharacter(
                        matching_opening_character(c),
                    ));
                }
            }
            _ => {}
        }
    }
    if opened.is_empty() {
        Ok(())
    } else {
        let missing_closing = opened
            .into_iter()
            .rev()
            .map(matching_closing_character)
            .collect();
        Err(SyntaxError::MissingClosingCharacter(missing_closing))
    }
}

fn score_syntax_error(error: &SyntaxError) -> u64 {
    match error {
        SyntaxError::ClosingCharacterMismatch(_, c) => match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => 0,
        },
        SyntaxError::MissingOpeningCharacter(_) => 0,
        SyntaxError::MissingClosingCharacter(_) => 0,
    }
}

fn score_auto_complete(error: &SyntaxError) -> u64 {
    match error {
        SyntaxError::ClosingCharacterMismatch(_, _) => 0,
        SyntaxError::MissingOpeningCharacter(_) => 0,
        SyntaxError::MissingClosingCharacter(chars) => chars.iter().fold(0, |acc, c| {
            acc * 5
                + match c {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => 0,
                }
        }),
    }
}

#[aoc(day10, part1)]
pub fn total_syntax_error_score(code_lines: &[String]) -> u64 {
    code_lines
        .iter()
        .filter_map(|line| parse_code_line(line).err())
        .filter(|err| match err {
            SyntaxError::ClosingCharacterMismatch(_, _) => true,
            _ => false,
        })
        .map(|err| score_syntax_error(&err))
        .sum()
}

#[aoc(day10, part2)]
pub fn total_auto_complete_score(code_lines: &[String]) -> u64 {
    let mut scores = code_lines
        .iter()
        .filter_map(|line| parse_code_line(line).err())
        .filter(|err| match err {
            SyntaxError::MissingClosingCharacter(_) => true,
            _ => false,
        })
        .map(|err| score_auto_complete(&err))
        .collect::<Vec<_>>();
    scores.sort();
    let middle = scores.len() / 2;
    scores[middle]
}

#[cfg(test)]
mod tests;
