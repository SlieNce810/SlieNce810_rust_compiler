//! 语义分析：在生成 IR 之前做静态检查。
//!
//! 本 Lab 只需要一条规则：除法的右操作数不能是字面量 0。

use crate::parser::Node;

/// 语义检查入口。
///
/// 契约：
/// - 递归遍历整棵 AST
/// - 遇到 `BinaryOp { operator: '/', right: Number(0) }` 返回 `Err("Division by zero")`
/// - 其余情况返回 `Ok(())`
///
/// 注意：讲义里只检查字面量 0；若右操作数本身是表达式（如 `1/(2-2)`），
///       本 Lab 不要求检测常量折叠后的除零。
pub fn check_semantics(node: &Node) -> Result<(), String> {
    // TODO(lab0): 递归 + 除零判断
    let _ = node;
    todo!("实现 check_semantics")
}
