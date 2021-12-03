//! # Day 18: Operation Order
//!
//! As you look out the window and notice a heavily-forested continent slowly
//! appear over the horizon, you are interrupted by the child sitting next to
//! you. They're curious if you could help them with their math homework.
//!
//! Unfortunately, it seems like this "math" follows different rules than you
//! remember.
//!
//! The homework (your puzzle input) consists of a series of expressions that
//! consist of addition (+), multiplication (*), and parentheses ((...)). Just
//! like normal math, parentheses indicate that the expression inside must be
//! evaluated before it can be used by the surrounding expression. Addition
//! still finds the sum of the numbers on both sides of the operator, and
//! multiplication still finds the product.
//!
//! However, the rules of operator precedence have changed. Rather than
//! evaluating multiplication before addition, the operators have the same
//! precedence, and are evaluated left-to-right regardless of the order in
//! which they appear.
//!
//! For example, the steps to evaluate the expression 1 + 2 * 3 + 4 * 5 + 6 are
//! as follows:
//!
//! ```text
//! 1 + 2 * 3 + 4 * 5 + 6
//!   3   * 3 + 4 * 5 + 6
//!       9   + 4 * 5 + 6
//!          13   * 5 + 6
//!              65   + 6
//!                  71
//! ```
//!
//! Parentheses can override this order; for example, here is what happens if
//! parentheses are added to form 1 + (2 * 3) + (4 * (5 + 6)):
//!
//! ```text
//! 1 + (2 * 3) + (4 * (5 + 6))
//! 1 +    6    + (4 * (5 + 6))
//!      7      + (4 * (5 + 6))
//!      7      + (4 *   11   )
//!      7      +     44
//!             51
//! ```
//!
//! Here are a few more examples:
//!
//! ```text
//! 2 * 3 + (4 * 5) becomes 26.
//! 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 437.
//! 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 12240.
//! ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 13632.
//! ```
//!
//! Before you can help with the homework, you need to understand it yourself.
//! Evaluate the expression on each line of the homework; what is the sum of the
//! resulting values?
//!
//! ## Part 2
//!
//! You manage to answer the child's questions and they finish part 1 of their
//! homework, but get stuck when they reach the next section: advanced math.
//!
//! Now, addition and multiplication have different precedence levels, but
//! they're not the ones you're familiar with. Instead, addition is evaluated
//! before multiplication.
//!
//! For example, the steps to evaluate the expression `1 + 2 * 3 + 4 * 5 + 6`
//! are now as follows:
//!
//! ```text
//! 1 + 2 * 3 + 4 * 5 + 6
//!   3   * 3 + 4 * 5 + 6
//!   3   *   7   * 5 + 6
//!   3   *   7   *  11
//!      21       *  11
//!          231
//! ```
//!
//! Here are the other examples from above:
//!
//! ```text
//! 1 + (2 * 3) + (4 * (5 + 6)) still becomes 51.
//! 2 * 3 + (4 * 5) becomes 46.
//! 5 + (8 * 3 + 9 + 3 * 4 * 3) becomes 1445.
//! 5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4)) becomes 669060.
//! ((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2 becomes 23340.
//! ```
//!
//! What do you get if you add up the results of evaluating the homework
//! problems using these new rules?
//!
//! [Advent of Code 2020 - Day 18](https://adventofcode.com/2020/day/18)

use std::fmt::Debug;
use std::mem;
use std::str::FromStr;

pub trait Evaluate {
    type Output;
    type Error;

    fn evaluate(expression: &str) -> Result<Self::Output, Self::Error>;
}

trait Lexer {
    type Token;
    type Error;

    fn next_token(&mut self) -> Result<Self::Token, Self::Error>;
}

pub trait Parser {
    type Output;
    type Error;

    fn parse(tokens: impl IntoIterator<Item = Token>) -> Result<Self::Output, Self::Error>;
}

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq)]
pub enum ParseMathExpressionError {
    InvalidCharacter(char),
    MissingNumber,
    MissingLeftOperand,
    MissingRightOperand,
    MissingOperator,
    UnbalancedParens,
}

#[allow(missing_copy_implementations)]
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LParen,
    RParen,
    Integer(i64),
    Plus,
    Asterisk,
    EndOfExpr,
    EndOfInput,
}

#[derive(Debug)]
struct ArithmeticLexer<I> {
    input: I,
    current: Option<char>,
}

impl<I> ArithmeticLexer<I>
where
    I: Iterator<Item = char>,
{
    pub fn new(input: impl IntoIterator<Item = char, IntoIter = I>) -> Self {
        Self {
            input: input.into_iter(),
            current: None,
        }
    }
}

impl<I> Lexer for ArithmeticLexer<I>
where
    I: Iterator<Item = char>,
{
    type Token = Token;
    type Error = ParseMathExpressionError;

    fn next_token(&mut self) -> Result<Self::Token, Self::Error> {
        while let Some(c) = self.current.or_else(|| self.input.next()) {
            self.current = None;
            match c {
                '(' => return Ok(Token::LParen),
                ')' => return Ok(Token::RParen),
                '+' => return Ok(Token::Plus),
                '*' => return Ok(Token::Asterisk),
                '\n' => return Ok(Token::EndOfExpr),
                '0'..='9' => {
                    let mut digits = c.to_string();
                    while let Some(c) = self.input.next() {
                        if c.is_ascii_digit() {
                            digits.push(c);
                        } else {
                            self.current = Some(c);
                            break;
                        }
                    }
                    let num = i64::from_str(&digits).unwrap();
                    return Ok(Token::Integer(num));
                }
                _ if c.is_whitespace() => {}
                _ => return Err(ParseMathExpressionError::InvalidCharacter(c)),
            }
        }
        Ok(Token::EndOfInput)
    }
}

impl<I> Iterator for ArithmeticLexer<I>
where
    I: Iterator<Item = char>,
{
    type Item = Result<<Self as Lexer>::Token, <Self as Lexer>::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_token() {
            Ok(token) => {
                if token == Token::EndOfInput {
                    None
                } else {
                    Some(Ok(token))
                }
            }
            Err(err) => Some(Err(err)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Term {
    Integer(i64),
    Add(usize, usize),
    Multiply(usize, usize),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    elements: Vec<Term>,
    root: Option<Term>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct EqualPrecedence;

impl Parser for EqualPrecedence {
    type Output = Expression;
    type Error = ParseMathExpressionError;

    fn parse(tokens: impl IntoIterator<Item = Token>) -> Result<Self::Output, Self::Error> {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum State {
            Empty,
            Lhs(usize),
            Op(Operation, usize),
        }

        let mut elements = Vec::new();
        let mut open = Vec::new();
        let mut state = State::Empty;

        for token in vec![Token::LParen]
            .into_iter()
            .chain(tokens.into_iter())
            .chain(vec![Token::RParen].into_iter())
        {
            match token {
                Token::LParen => {
                    open.push(mem::replace(&mut state, State::Empty));
                }
                Token::RParen => {
                    match state {
                        State::Empty => {}
                        State::Lhs(_) => {}
                        State::Op(_, _) => {
                            return Err(ParseMathExpressionError::MissingRightOperand)
                        }
                    }
                    if let Some(prev_state) = open.pop() {
                        let index = elements.len();
                        match prev_state {
                            State::Empty => {
                                state = State::Lhs(index - 1);
                            }
                            State::Lhs(_) => return Err(ParseMathExpressionError::MissingOperator),
                            State::Op(op, lhs) => {
                                let index = elements.len();
                                match op {
                                    Operation::Add => elements.push(Term::Add(lhs, index - 1)),
                                    Operation::Multiply => {
                                        elements.push(Term::Multiply(lhs, index - 1));
                                    }
                                }
                                state = State::Lhs(index);
                            }
                        }
                    } else {
                        return Err(ParseMathExpressionError::UnbalancedParens);
                    }
                }
                Token::Integer(num) => {
                    let index = elements.len();
                    elements.push(Term::Integer(num));
                    match state {
                        State::Empty => state = State::Lhs(index),
                        State::Lhs(_) => return Err(ParseMathExpressionError::MissingOperator),
                        State::Op(op, lhs) => {
                            match op {
                                Operation::Add => elements.push(Term::Add(lhs, index)),
                                Operation::Multiply => {
                                    elements.push(Term::Multiply(lhs, index));
                                }
                            }
                            state = State::Lhs(index + 1);
                        }
                    }
                }
                Token::Plus => match state {
                    State::Empty => return Err(ParseMathExpressionError::MissingLeftOperand),
                    State::Lhs(lhs) => state = State::Op(Operation::Add, lhs),
                    State::Op(_, _) => return Err(ParseMathExpressionError::MissingRightOperand),
                },
                Token::Asterisk => match state {
                    State::Empty => return Err(ParseMathExpressionError::MissingLeftOperand),
                    State::Lhs(lhs) => state = State::Op(Operation::Multiply, lhs),
                    State::Op(_, _) => return Err(ParseMathExpressionError::MissingRightOperand),
                },
                Token::EndOfExpr => {}
                Token::EndOfInput => {}
            }
        }
        let root = elements.last().copied();
        Ok(Expression { elements, root })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CustomPrecedence;

impl Parser for CustomPrecedence {
    type Output = Expression;
    type Error = ParseMathExpressionError;

    #[allow(unused_assignments)]
    fn parse(tokens: impl IntoIterator<Item = Token>) -> Result<Self::Output, Self::Error> {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum State {
            Empty,
            Lhs(usize),
            Add(usize),
            Mul(usize),
            Mul2(usize, usize),
            MulAdd(usize, usize),
        }

        let mut elements = Vec::new();
        let mut open = Vec::new();
        let mut state = State::Empty;

        for token in vec![Token::LParen]
            .into_iter()
            .chain(tokens.into_iter())
            .chain(vec![Token::RParen].into_iter())
        {
            match token {
                Token::LParen => {
                    open.push(mem::replace(&mut state, State::Empty));
                }
                Token::RParen => {
                    match state {
                        State::Empty => {}
                        State::Lhs(_) => {}
                        State::Add(_) => return Err(ParseMathExpressionError::MissingRightOperand),
                        State::Mul(_) => return Err(ParseMathExpressionError::MissingRightOperand),
                        State::Mul2(lhs, rhs) => {
                            let index = elements.len();
                            elements.push(Term::Multiply(lhs, rhs));
                            state = State::Lhs(index);
                        }
                        State::MulAdd(_, _) => {
                            return Err(ParseMathExpressionError::MissingRightOperand)
                        }
                    }
                    if let Some(prev_state) = open.pop() {
                        let index = elements.len();
                        match prev_state {
                            State::Empty => {
                                state = State::Lhs(index - 1);
                            }
                            State::Lhs(_) => return Err(ParseMathExpressionError::MissingOperator),
                            State::Add(lhs) => {
                                elements.push(Term::Add(lhs, index - 1));
                                state = State::Lhs(index);
                            }
                            State::Mul(lhs) => {
                                state = State::Mul2(lhs, index - 1);
                            }
                            State::Mul2(_, _) => {
                                return Err(ParseMathExpressionError::MissingOperator)
                            }
                            State::MulAdd(lhs, rhs) => {
                                elements.push(Term::Add(rhs, index - 1));
                                state = State::Mul2(lhs, index);
                            }
                        }
                    } else {
                        return Err(ParseMathExpressionError::UnbalancedParens);
                    }
                }
                Token::Integer(num) => {
                    let index = elements.len();
                    elements.push(Term::Integer(num));
                    match state {
                        State::Empty => state = State::Lhs(index),
                        State::Lhs(_) => return Err(ParseMathExpressionError::MissingOperator),
                        State::Add(lhs) => {
                            elements.push(Term::Add(lhs, index));
                            state = State::Lhs(index + 1);
                        }
                        State::Mul(lhs) => state = State::Mul2(lhs, index),
                        State::Mul2(_, _) => return Err(ParseMathExpressionError::MissingOperator),
                        State::MulAdd(lhs, rhs) => {
                            elements.push(Term::Add(rhs, index));
                            state = State::Mul2(lhs, index + 1);
                        }
                    }
                }
                Token::Plus => match state {
                    State::Empty => return Err(ParseMathExpressionError::MissingLeftOperand),
                    State::Lhs(lhs) => state = State::Add(lhs),
                    State::Add(_) => return Err(ParseMathExpressionError::MissingRightOperand),
                    State::Mul(_) => return Err(ParseMathExpressionError::MissingRightOperand),
                    State::Mul2(lhs, rhs) => state = State::MulAdd(lhs, rhs),
                    State::MulAdd(_, _) => {
                        return Err(ParseMathExpressionError::MissingRightOperand)
                    }
                },
                Token::Asterisk => match state {
                    State::Empty => return Err(ParseMathExpressionError::MissingLeftOperand),
                    State::Lhs(lhs) => state = State::Mul(lhs),
                    State::Add(_) => return Err(ParseMathExpressionError::MissingRightOperand),
                    State::Mul(_) => return Err(ParseMathExpressionError::MissingRightOperand),
                    State::Mul2(lhs, rhs) => {
                        let index = elements.len();
                        elements.push(Term::Multiply(lhs, rhs));
                        state = State::Mul(index);
                    }
                    State::MulAdd(_, _) => {
                        return Err(ParseMathExpressionError::MissingRightOperand)
                    }
                },
                Token::EndOfExpr => {}
                Token::EndOfInput => {}
            }
        }
        let root = elements.last().copied();
        Ok(Expression { elements, root })
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Math<P> {
    parser: P,
}

impl<P> Evaluate for Math<P>
where
    P: Parser<Output = Expression>,
    <P as Parser>::Error: Debug,
{
    type Output = i64;
    type Error = <P as Parser>::Error;

    fn evaluate(expression: &str) -> Result<Self::Output, Self::Error> {
        let tokens = ArithmeticLexer::new(expression.chars())
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let ast = P::parse(tokens).unwrap();

        let mut open = vec![(ast.root.unwrap(), false)];
        let mut stack = vec![];

        while let Some((current, visited)) = open.pop() {
            match current {
                Term::Integer(num) => {
                    stack.push(num);
                }
                Term::Add(lhs, rhs) => {
                    if visited {
                        let b = stack.pop().unwrap();
                        let a = stack.pop().unwrap();
                        stack.push(a + b);
                    } else {
                        open.push((current, true));
                        open.push((ast.elements[rhs], false));
                        open.push((ast.elements[lhs], false));
                    }
                }
                Term::Multiply(lhs, rhs) => {
                    if visited {
                        let b = stack.pop().unwrap();
                        let a = stack.pop().unwrap();
                        stack.push(a * b);
                    } else {
                        open.push((current, true));
                        open.push((ast.elements[rhs], false));
                        open.push((ast.elements[lhs], false));
                    }
                }
            }
        }
        debug_assert_eq!(stack.len(), 1);
        Ok(stack.pop().unwrap())
    }
}

#[aoc_generator(day18)]
pub fn parse_math_homework(input: &str) -> Vec<String> {
    input.lines().map(|s| String::from(s)).collect()
}

#[aoc(day18, part1)]
pub fn sum_of_math_results_with_equal_precedence(homework: &[String]) -> i64 {
    homework
        .iter()
        .map(|expr| Math::<EqualPrecedence>::evaluate(expr).unwrap())
        .sum()
}

#[aoc(day18, part2)]
pub fn sum_of_math_results_with_custom_precedence(homework: &[String]) -> i64 {
    homework
        .iter()
        .map(|expr| Math::<CustomPrecedence>::evaluate(expr).unwrap())
        .sum()
}

#[cfg(test)]
mod tests;
