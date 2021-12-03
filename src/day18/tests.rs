use super::*;

const INPUT: &str = include_str!("../../input/2020/day18.txt");

#[test]
fn parse_math_homework_in_puzzle_input() {
    let homework = parse_math_homework(INPUT);

    assert_eq!(homework.len(), 373);
}

#[test]
fn parse_tokens_example1() {
    let tokens = ArithmeticLexer::new("1 + 2 * 3 + 4 * 5 + 6".chars())
        .collect::<Result<Vec<Token>, _>>()
        .unwrap();

    assert_eq!(
        tokens,
        vec![
            Token::Integer(1),
            Token::Plus,
            Token::Integer(2),
            Token::Asterisk,
            Token::Integer(3),
            Token::Plus,
            Token::Integer(4),
            Token::Asterisk,
            Token::Integer(5),
            Token::Plus,
            Token::Integer(6),
        ]
    );
}

#[test]
fn parse_tokens_example2() {
    let tokens = ArithmeticLexer::new("1 + (2 * 3) + (4 * (5 + 6))".chars())
        .collect::<Result<Vec<Token>, _>>()
        .unwrap();

    assert_eq!(
        tokens,
        vec![
            Token::Integer(1),
            Token::Plus,
            Token::LParen,
            Token::Integer(2),
            Token::Asterisk,
            Token::Integer(3),
            Token::RParen,
            Token::Plus,
            Token::LParen,
            Token::Integer(4),
            Token::Asterisk,
            Token::LParen,
            Token::Integer(5),
            Token::Plus,
            Token::Integer(6),
            Token::RParen,
            Token::RParen,
        ]
    );
}

#[test]
fn parse_expression_example1() {
    let tokens = ArithmeticLexer::new("1 + 2 * 3 + 4 * 5 + 6".chars())
        .collect::<Result<Vec<Token>, _>>()
        .unwrap();
    let expression = EqualPrecedence::parse(tokens).unwrap();

    assert_eq!(
        expression,
        Expression {
            elements: vec![
                Term::Integer(1),
                Term::Integer(2),
                Term::Add(0, 1),
                Term::Integer(3),
                Term::Multiply(2, 3),
                Term::Integer(4),
                Term::Add(4, 5),
                Term::Integer(5),
                Term::Multiply(6, 7),
                Term::Integer(6),
                Term::Add(8, 9),
            ],
            root: Some(Term::Add(8, 9))
        }
    );
}

#[test]
fn parse_expression_example2() {
    let tokens = ArithmeticLexer::new("1 + (2 * 3) + (4 * (5 + 6))".chars())
        .collect::<Result<Vec<Token>, _>>()
        .unwrap();
    let expression = EqualPrecedence::parse(tokens).unwrap();

    assert_eq!(
        expression,
        Expression {
            elements: vec![
                Term::Integer(1),
                Term::Integer(2),
                Term::Integer(3),
                Term::Multiply(1, 2),
                Term::Add(0, 3),
                Term::Integer(4),
                Term::Integer(5),
                Term::Integer(6),
                Term::Add(6, 7),
                Term::Multiply(5, 8),
                Term::Add(4, 9),
            ],
            root: Some(Term::Add(4, 9))
        }
    );
}

#[test]
fn parse_expression_example3() {
    let tokens = ArithmeticLexer::new("2 * 3 + (4 * 5)".chars())
        .collect::<Result<Vec<Token>, _>>()
        .unwrap();
    let expression = EqualPrecedence::parse(tokens).unwrap();

    assert_eq!(
        expression,
        Expression {
            elements: vec![
                Term::Integer(2),
                Term::Integer(3),
                Term::Multiply(0, 1),
                Term::Integer(4),
                Term::Integer(5),
                Term::Multiply(3, 4),
                Term::Add(2, 5),
            ],
            root: Some(Term::Add(2, 5))
        }
    );
}

#[test]
fn parse_expression_example5() {
    let tokens = ArithmeticLexer::new("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))".chars())
        .collect::<Result<Vec<Token>, _>>()
        .unwrap();
    let expression = EqualPrecedence::parse(tokens).unwrap();

    assert_eq!(
        expression,
        Expression {
            elements: vec![
                Term::Integer(5),
                Term::Integer(9),
                Term::Multiply(0, 1),
                Term::Integer(7),
                Term::Integer(3),
                Term::Multiply(3, 4),
                Term::Integer(3),
                Term::Multiply(5, 6),
                Term::Integer(9),
                Term::Add(7, 8),
                Term::Integer(3),
                Term::Multiply(9, 10),
                Term::Integer(8),
                Term::Integer(6),
                Term::Add(12, 13),
                Term::Integer(4),
                Term::Multiply(14, 15),
                Term::Add(11, 16),
                Term::Multiply(2, 17),
            ],
            root: Some(Term::Multiply(2, 17))
        }
    );
}

#[test]
fn evaluate_expression_with_equal_precedence_example1() {
    assert_eq!(
        Math::<EqualPrecedence>::evaluate("1 + 2 * 3 + 4 * 5 + 6"),
        Ok(71)
    );
}

#[test]
fn evaluate_expression_with_equal_precedence_example2() {
    assert_eq!(
        Math::<EqualPrecedence>::evaluate("1 + (2 * 3) + (4 * (5 + 6))"),
        Ok(51)
    );
}

#[test]
fn evaluate_expression_with_equal_precedence_example3() {
    assert_eq!(Math::<EqualPrecedence>::evaluate("2 * 3 + (4 * 5)"), Ok(26));
}

#[test]
fn evaluate_expression_with_equal_precedence_example4() {
    assert_eq!(
        Math::<EqualPrecedence>::evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"),
        Ok(437)
    );
}

#[test]
fn evaluate_expression_with_equal_precedence_example5() {
    assert_eq!(
        Math::<EqualPrecedence>::evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        Ok(12240)
    );
}

#[test]
fn evaluate_expression_with_equal_precedence_example6() {
    assert_eq!(
        Math::<EqualPrecedence>::evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        Ok(13632)
    );
}

#[test]
fn sum_of_math_results_with_equal_precedence_in_puzzle_input() {
    let homework = parse_math_homework(INPUT);

    let sum = sum_of_math_results_with_equal_precedence(&homework);

    assert_eq!(sum, 5374004645253);
}

#[test]
fn evaluate_expression_with_custom_precedence_example1() {
    assert_eq!(
        Math::<CustomPrecedence>::evaluate("1 + 2 * 3 + 4 * 5 + 6"),
        Ok(231)
    );
}

#[test]
fn evaluate_expression_with_custom_precedence_example2() {
    assert_eq!(
        Math::<CustomPrecedence>::evaluate("1 + (2 * 3) + (4 * (5 + 6))"),
        Ok(51)
    );
}

#[test]
fn evaluate_expression_with_custom_precedence_example3() {
    assert_eq!(
        Math::<CustomPrecedence>::evaluate("2 * 3 + (4 * 5)"),
        Ok(46)
    );
}

#[test]
fn evaluate_expression_with_custom_precedence_example4() {
    assert_eq!(
        Math::<CustomPrecedence>::evaluate("5 + (8 * 3 + 9 + 3 * 4 * 3)"),
        Ok(1445)
    );
}

#[test]
fn evaluate_expression_with_custom_precedence_example5() {
    assert_eq!(
        Math::<CustomPrecedence>::evaluate("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"),
        Ok(669060)
    );
}

#[test]
fn evaluate_expression_with_custom_precedence_example6() {
    assert_eq!(
        Math::<CustomPrecedence>::evaluate("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
        Ok(23340)
    );
}

#[test]
fn sum_of_math_results_with_custom_precedence_in_puzzle_input() {
    let homework = parse_math_homework(INPUT);

    let sum = sum_of_math_results_with_custom_precedence(&homework);

    assert_eq!(sum, 88782789402798);
}
