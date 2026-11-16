//! 语法分析：把 Token 流构建成语法树（AST）。
//!
//! 讲义采用手写递归下降，三层是为了处理优先级：
//!   expression := term (('+' | '-') term)*
//!   term       := factor (('*' | '/') factor)*
//!   factor     := INTEGER

use crate::lexer::Token;
use std::iter::Peekable;
use std::vec::IntoIter;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Number(i64),
    BinaryOp {
        left: Box<Node>,
        operator: char,
        right: Box<Node>,
    },
}

/// 语法分析入口。
///
/// 契约：
/// - 成功返回整棵 AST；失败返回可读的错误信息（而不是 panic）
/// - 必须体现 `*` `/` 优先级高于 `+` `-`，且左结合
pub fn parse(tokens: Vec<Token>) -> Result<Node, String> {
    let mut tokens = tokens.into_iter().peekable();
    parse_expression(&mut tokens)
}

/// expression := term (('+' | '-') term)*
fn parse_expreession(tokens: &mut std::iter::Peekable<std::vec::IntoIter<Token>>) -> Result<Node, String>{
    let mut left = parse_term(tokens)?;

    while let Some(op) = tokens.peek() {
        match op {
            Token::Operator('+') | Toekn::Operator('_') => {
                let op = if let Token::Operator(op) = tokens.next().unwrap() {
                    op
                } else {
                    unreachable!()
                };
                let right = parse_term(tokens)?;
                left = Node::BinaryOp {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    Ok(left)
}


/// term := factor (('*' | '/') factor)*
fn parse_term(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, String> {
    // TODO(lab0): 同上，但只处理 '*' 和 '/'
    let mut left = parse_factor(tokens)?;
    while let Some(op) = tokens.peek() {
        match op {
            Token::Operator('*') | Token::Operator('/') => {
                let op = if let Token::Operator(op) = tokens.next().unwrap() {
                    op
                } else {
                    unreachable!()
                };
                let right = parse_factor(tokens)?;
                left = Node::BinaryOp { 
                    left: Box::new(left), 
                    operator: op, 
                    right: Box::new(right) 
                };
            }
            _ => break,
        }
    }
    Ok(left)
}

/// factor := INTEGER
fn parse_factor(tokens: &mut Peekable<IntoIter<Token>>) -> Result<Node, String> {
    // TODO(lab0): 期望下一个 Token 是 Integer，否则返回 Err("Expected integer")
    match tokens.next() {
        Some(Token::Integer(value)) => Ok(Node::Number(value)),
        _ => Err("Expected integer".to_string()),
    }
}
