//! 词法分析：把输入字符串转成 Token 流。
//!
//! 本文件由 pest 直接从 `expression.pest` 生成语法解析器，
//! 因此"词法"这一步实际上是对 pest 生成的 Pair 流做一次形状检查 + 类型化。

use pest::Parser;
use pest_derive::Parser;


/// pest 根据 src/expression.pest 生成的解析器。
/// `#[grammar]` 的路径相对于 `src/` 目录解析。
#[derive(Parser)]
#[grammar = "expression.pest"]
pub struct ExpressionParser;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    /// 整型常量
    Integer(i64),
    /// 二元运算符：'+' | '-' | '*' | '/'
    Operator(char),
}

/// 词法分析入口。
///
/// 契约：
/// - 输入形如 `1+1` / `12 * 3`（整数 + 单个运算符 + 整数）
/// - 输出按出现顺序排列的 `[Integer, Operator, Integer]`
/// - 输入不合法时 panic（讲义实现即为 `unwrap_or_else(|e| panic!(...))`）
///
/// 提示：
/// - `ExpressionParser::parse(Rule::expr, input)` 返回 `Result<Pairs<Rule>, Error>`
/// - `pairs.next()` 拿到最外层 `expr` 这个 Pair
/// - `pair.into_inner()` 遍历它的子节点 `int` / `op`
/// - `p.as_rule()` 判类型，`p.as_str()` 取原始文本
pub fn tokenize(input: &str) -> Vec<Token> {
    // TODO(lab0): 按上面的提示实现
    let pair = ExpressionParser::parse(Rule::expr, input)
        .unwrap_or_else(|e| panic!("Parse error: {}", e))
        .next().unwrap(); // 取第一个匹配

    pair.into_inner().map(|p| match p.as_rule() {
        Rule::int => Token::Integer(p.as_str().parse().unwrap()),
        Rule::op => Token::Operator(p.as_str().chars().next().unwrap()),
        _ => unreachable!(),
    }).collect()
}

