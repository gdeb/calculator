//! # The Tokenizer
//!
//! This file contains a basic tokenizer.  It takes a &str as input and returns
//! an iterator.  Nothing complicated, this is the most naive implementation I
//! could imagine.

use std::str::Chars;
use std::iter::Peekable;

#[derive(PartialEq, Debug, Copy, Clone)]
pub enum Operator {
    Add,
    Substract,
    Multiply,
}

#[derive(PartialEq, Debug)]
pub enum Token {
    Number(usize),
    LeftParen,
    RightParen,
    Operator(Operator),
    InvalidToken,
}

pub struct Expression<'a> {
    iter: Peekable<Chars<'a>>,
}

impl <'a> Iterator for Expression<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        loop {
            match self.iter.next() {
                Some(' ') => continue,
                Some('+') => return Some(Token::Operator(Operator::Add)),
                Some('-') => return Some(Token::Operator(Operator::Substract)),
                Some('*') => return Some(Token::Operator(Operator::Multiply)),
                Some('(') => return Some(Token::LeftParen),
                Some(')') => return Some(Token::RightParen),
                Some(n @ '0' ... '9') => {
                    let mut number_str = n.to_string();
                    while let Some(&n @ '0' ... '9') = self.iter.peek() {
                        self.iter.next();
                        number_str.push(n);
                    }
                    return Some(Token::Number(number_str.parse().expect("")))
                },
                Some(_) => return Some(Token::InvalidToken),
                None => return None,
            }
        }
    }
}

pub type Tokens<'a> = Peekable<Expression<'a>>;

pub fn tokenize(s: &str) -> Tokens {
    let basic_tokens = Expression { iter: s.chars().peekable() };
    basic_tokens.peekable()
}


#[cfg(test)]
mod tests {
    use super::tokenize;
    use super::Token;
    use super::Operator;

    #[test]
    fn tokenizer_test() {
        let mut tokens = tokenize(" 123 +  (12 * 54 - 2)");

        assert_eq!(Some(Token::Number(123)), tokens.next());
        assert_eq!(Some(Token::Operator(Operator::Add)), tokens.next());
        assert_eq!(Some(Token::LeftParen), tokens.next());
        assert_eq!(Some(Token::Number(12)), tokens.next());
        assert_eq!(Some(Token::Operator(Operator::Multiply)), tokens.next());
        assert_eq!(Some(Token::Number(54)), tokens.next());
        assert_eq!(Some(Token::Operator(Operator::Substract)), tokens.next());
        assert_eq!(Some(Token::Number(2)), tokens.next());
        assert_eq!(Some(Token::RightParen), tokens.next());
        assert_eq!(None, tokens.next());
    }
}

