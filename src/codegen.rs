//! 中间代码生成：AST -> LLVM IR。
//!
//! 讲义做法：先建立模块 `main: float ()`，再用 Builder 在 entry 基本块里
//! 逐条折叠出加减乘除指令，最后在主函数里 `ret` 该值。

use crate::parser::Node;
use llvm_sys::prelude::{LLVMContextRef, LLVMModuleRef, LLVMValueRef};

/// 递归生成表达式对应的 LLVM 值。
///
/// 契约：
/// - `Number(n)`：返回 float 常量（讲义用 `LLVMConstReal(float_ty, n as f64)`，
///   即整型常量在 IR 里按 float 处理）
/// - `BinaryOp`：先递归取左右值，再按运算符发射 `fadd` / `fsub` / `fmul` / `fdiv`
/// - 发射指令前需要把 builder 定位到当前基本块末尾
///
/// 提示：
/// - 取第一个函数 / 第一个基本块：`LLVMGetFirstFunction` / `LLVMGetFirstBasicBlock`
/// - 建 builder：`LLVMCreateBuilderInContext(context)`
/// - 定位插入点：`LLVMPositionBuilderAtEnd(builder, bb)`
/// - 用完全部指令后记得 `LLVMDisposeBuilder(builder)`，否则泄漏
/// - `LLVMBuildFAdd` 等最后一个参数是名字（`CString` 的 `as_ptr()`）
pub fn generate_llvm_ir(
    node: &Node,
    module: LLVMModuleRef,
    context: LLVMContextRef,
) -> LLVMValueRef {
    // TODO(lab0): match node { Number / BinaryOp }
    let _ = (node, module, context);
    todo!("实现 generate_llvm_ir")
}
