use std::{iter::Peekable, slice::Iter};

use crate::token::Token;

#[derive(Debug)]
pub enum IntegerExpression {
    Literal(String),
}

#[derive(Debug)]
pub enum Node {
    Exit(IntegerExpression),
}

fn split_tokens(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut statements = Vec::new();

    let mut tokens = tokens.iter().peekable();
    let mut buffer = Vec::new();

    while let Some(token) = tokens.next() {
        if let Token::NewLine = token {
            if !buffer.is_empty() {
                statements.push(buffer.clone());
                buffer.clear();
            }
        } else {
            buffer.push(token.clone());
        }
    }

    if !buffer.is_empty() {
        statements.push(buffer);
    }

    return statements;
}

fn parse_int_expression(statement: &[Token]) -> IntegerExpression {
    // TODO: parse more properly
    match statement.get(0).unwrap() {
        Token::IntLiteral(value) => IntegerExpression::Literal(value.clone()),
        token => panic!("Invalid expression token '{:?}'", token),
    }
}

fn parse_statement(statement: Vec<Token>) -> Node {
    let first_token = statement.get(0).expect("cannot be empty");
    match first_token {
        Token::Exit => {
            let expression = parse_int_expression(&statement[1..]);
            Node::Exit(expression)
        }
        token => panic!("Invalid statement token '{:?}'", token),
    }
}

pub fn construct_ast(tokens: Vec<Token>) -> Vec<Node> {
    let mut ast = Vec::new();

    let statements = split_tokens(tokens);
    for statement in statements {
        let node = parse_statement(statement);
        ast.push(node);
    }

    return ast;
}
