# demo_compiler

Rust 编译器训练营 **Lab0 - 环境配置** 的演示工程骨架。
目标：读入 `input.txt` 里的双目运算表达式（如 `1+1`），做词法 / 语法 / 语义分析，
最后用 `llvm-sys` 生成一个只含 `ret` 的 LLVM IR `main` 函数。

> 本仓库只提供**文件骨架与函数契约**，所有 `todo!()` 由你补完。

## 目录结构

```
demo_compiler/
├── .cargo/config.toml    # 项目级 LLVM_SYS_140_PREFIX（不依赖 shell）
├── Cargo.toml            # 依赖：pest / pest_derive / llvm-sys 140
├── input.txt             # 待编译的表达式（默认 1+1）
├── README.md
├── scripts/
│   └── setup_env.sh      # 一键补齐 LLVM 14 环境并自检
└── src/
    ├── main.rs           # 流程编排：读文件 -> 各阶段 -> 打印 IR
    ├── expression.pest   # pest 文法：expr = int ~ op ~ int
    ├── lexer.rs          # 词法：字符串 -> Vec<Token>
    ├── parser.rs         # 语法：Vec<Token> -> AST（递归下降，处理优先级）
    ├── semantic.rs       # 语义：除零检查
    └── codegen.rs        # 中间代码生成：AST -> LLVM IR
```

## 环境要求

| 组件 | 版本 / 说明 |
|---|---|
| Rust | stable ≥ 1.85（`edition = "2024"` 要求） |
| LLVM | 14，`llvm-config` 在 `/usr/lib/llvm-14/bin` |
| 环境变量 | `LLVM_SYS_140_PREFIX=/usr/lib/llvm-14` |
| 系统包 | `llvm-14 llvm-runtime clang-14 zlib1g-dev libtinfo-dev` |

一键补齐 + 自检（只自检不动系统加 `--check`）：

```bash
bash scripts/setup_env.sh
bash scripts/setup_env.sh --check   # 装完之后单独复核
```

`LLVM_SYS_140_PREFIX` 已经写进 `.cargo/config.toml` 的 `[env]` 段，
所以**不 source `~/.bashrc` 也能构建**——非交互 shell（`bash -lc`、VSCode 任务、CI）
里 `~/.bashrc` 会在开头 `case $- in *i*) ;; *) return;; esac` 提前返回，`export` 其实不生效。

## 构建与运行

```bash
cd demo_compiler
cargo build             # 首次会拉 pest / llvm-sys
cargo run               # 读 ./input.txt，打印 LLVM IR
```

验证 IR 可执行（`main` 返回 float，`lli` 不报错即通过）：

```bash
cargo run > out.ll && lli out.ll; echo "exit=$?"
```

预期 IR 形如：

```llvm
define float @main() {
entry:
  %add = fadd float 1.000000e+00, 1.000000e+00
  ret float %add
}
```

## 待实现清单

- [ ] `lexer::tokenize`：pest 解析 → `[Integer, Operator, Integer]`
- [ ] `parser::parse_expression / parse_term / parse_factor`：递归下降 + 优先级 + 左结合
- [ ] `semantic::check_semantics`：`/` 右操作数为字面量 0 时报错
- [ ] `codegen::generate_llvm_ir`：`fadd/fsub/fmul/fdiv`
- [ ] `main`：上下文 / 模块 / `main` 函数 / `entry` 基本块 / `ret` / 打印与释放

## 两个容易踩的坑

1. `#[grammar = "..."]` 的路径按 **`src/` 目录**解析，所以 `expression.pest` 必须放在 `src/` 下，
   且写 `#[grammar = "expression.pest"]`（不是 `"src/expression.pest"`）。
2. `CString::new("main").unwrap().as_ptr()` 这种写法会让临时值在语句结束时被释放，
   指针变悬垂。先把 `CString` 绑到 `let` 变量上，再取 `as_ptr()`。
