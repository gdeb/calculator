//! # The Parser
//!
//! This file contains a basic parser.  It is inspired by Pratt, even though
//! I am not sure it qualifies as a Pratt parser (it is not written in OOP style)
//!
//! Nevertheless, the parsing logic is mostly contained in the 3 functions
//! 'binding_power', 'parse_infix' and 'parse_prefix'.

use super::tokenizer::Operator;
use super::tokenizer::Tokens;
use super::tokenizer::Token;

/// This is a ultra simple algebraic expression AST.  It only supports values
/// and binary operators.  To be able to support the unary minus operator, we
/// use a binary operator minus with 0 as left operand.  However, it could
/// easily be modified to have 2 kinds of operators, or even more.
#[derive(PartialEq, Debug)]
pub enum AST {
    Value(usize),
    Operator(Operator, Box<AST>, Box<AST>)
}

#[derive(PartialEq, Debug)]
pub enum ParseError {
    UnmatchedLeftParenthesis,
    UnmatchedRightParenthesis,
    NothingToParse,
    InvalidPrefix,
    InvalidInfix,
    InvalidToken,
    InvalidExpression,
}

/// The 'binding power' function returns a value that determines how strongly
/// a token is binded to its left (and right).  This is kind of equivalent to
/// the priority of operators, only for any token.  The actual values returned
/// by this function are irrelevant, what matters is their ordering.
fn binding_power(token: &Token) -> usize {
    match token {
        &Token::Number(_) => 0,
        &Token::LeftParen => 5,
        &Token::RightParen => 5,
        &Token::Operator(Operator::Add) => 15,
        &Token::Operator(Operator::Substract) => 15,
        &Token::Operator(Operator::Multiply) => 20,
        &Token::InvalidToken => 0,
    }
}

/// The parse_prefix function is what happens when a token is considered as a
/// prefix.  For example, in  '1 + (2 * 3)', the tokens 1, (, 2 and 3 will be
/// treated as prefix.
fn parse_prefix(current: Token, tokens: &mut Tokens) -> Result<AST, ParseError> {
    match current {
        Token::Number(n) => {
            Ok(AST::Value(n))
        },
        Token::LeftParen => {
            let expr = try!(parse_expression(tokens, 5));
            match tokens.next() {
                Some(Token::RightParen) => Ok(expr),
                _ => Err(ParseError::UnmatchedLeftParenthesis),
            }
        },
        Token::Operator(Operator::Substract) => {
            let left = Box::new(AST::Value(0));
            let right = Box::new(try!(parse_expression(tokens, 15)));
            Ok(AST::Operator(Operator::Substract, left, right))
        }
        _ => Err(ParseError::InvalidPrefix)
    }
}

/// Here, this method determines what happens when a token is parsed as an infix
/// operator.  Basically, all arithmetic operators are infix.
fn parse_infix(left: AST, current: Token, tokens: &mut Tokens) -> Result<AST, ParseError> {
    match current {
        Token::Operator(op) => {
            let bp = binding_power(&current);
            let right = try!(parse_expression(tokens, bp));
            Ok(AST::Operator(op, Box::new(left), Box::new(right)))
        },
        Token::RightParen => Err(ParseError::UnmatchedRightParenthesis),
        _ => Err(ParseError::InvalidInfix)
    }
}

fn next_token_bp(tokens: &mut Tokens) -> usize {
    match tokens.peek() {
        Some(ref token) => binding_power(&token),
        None => 0,
    }
}

fn parse_expression(tokens: &mut Tokens, bp: usize) -> Result<AST, ParseError> {
    match tokens.next() {
        Some(Token::InvalidToken) => Err(ParseError::InvalidToken),
        Some(token) => {
            let mut expr = try!(parse_prefix(token, tokens));
            while next_token_bp(tokens) > bp {
                let next_token = tokens.next().unwrap();
                expr = try!(parse_infix(expr, next_token, tokens));
            }
            Ok(expr)
        },
        None => Err(ParseError::NothingToParse)
    }
}

/// The 'parse' function takes an iterator of tokens and returns (possibly) a
/// valid AST.
pub fn parse(tokens: &mut Tokens) -> Result<AST, ParseError> {
    let result = try!(parse_expression(tokens, 0));
    match tokens.next() {
        Some(_) => Err(ParseError::InvalidExpression),
        None => Ok(result),
    }
}


#[cfg(test)]
mod tests {
    use super::parse;
    use super::AST;
    use super::ParseError;
    use super::super::tokenizer::tokenize;
    use super::super::tokenizer::Operator;

    fn parse_expr(expr: &str) -> Result<AST, ParseError> {
        let mut tokens = tokenize(expr);
        parse(&mut tokens)
    }

    #[test]
    fn basic_value() {
        assert_eq!(parse_expr("42"), Ok(AST::Value(42)));
    }

    #[test]
    fn basic_binary_operation() {
        let left = Box::new(AST::Value(1));
        let right = Box::new(AST::Value(2));
        let result = AST::Operator(Operator::Add, left, right);
        assert_eq!(parse_expr("1 + 2"), Ok(result));
    }

    #[test]
    fn invalid_expression() {
        assert_eq!(parse_expr("1 1"), Err(ParseError::InvalidExpression));
    }

    #[test]
    fn missing_right_parenthesis() {
        assert_eq!(parse_expr("(1"), Err(ParseError::UnmatchedLeftParenthesis));
    }

    #[test]
    fn missing_left_parenthesis() {
        assert_eq!(parse_expr("1 )"), Err(ParseError::UnmatchedRightParenthesis))
    }

    #[test]
    fn invalid_prefix_expression() {
        assert_eq!(parse_expr("+ 1"), Err(ParseError::InvalidPrefix));
    }

    #[test]
    fn invalid_token_expression() {
        assert_eq!(parse_expr("= 1"), Err(ParseError::InvalidToken));
    }

}