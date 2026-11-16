//! compiler —— Rust 编译器训练营 Lab1：SysY 词法分析器
//!
//! 用法：
//!   cargo run --release -- tests/cases/sample1.sy
//!
//! 输出全部走 stderr（讲义要求），stdout 保持干净，方便重定向比对。

mod lexer;

use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("usage: compiler <input.sy>");
        return ExitCode::from(2);
    };

    let src = match std::fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("failed to read {path}: {e}");
            return ExitCode::from(1);
        }
    };

    let out = lexer::lex(&src);

    // 讲义规定：有错就只打印错误，没错才打印 token 列表 —— 两者互斥。
    if !out.errors.is_empty() {
        for err in &out.errors {
            eprintln!("{}", err.render());
        }
        return ExitCode::SUCCESS;
    }

    for token in &out.tokens {
        eprintln!("{}", token.render());
    }

    ExitCode::SUCCESS
}
