# compiler —— Lab1：SysY 词法分析器

Rust 编译器训练营（2025 秋）实验一。用 `pest` 写一个 SysY 词法分析器。

## 实验契约（从讲义 P8 抄下来的硬要求）

| 项 | 要求 |
|---|---|
| 输入 | 命令行第一个参数是一个 `.sy` 文件路径 |
| 输出通道 | **全部走 stderr**（`eprintln!`），stdout 保持干净 |
| token 输出格式 | `KIND lexeme at Line N.`，例如 `INT int at Line 1.` |
| 错误输出格式 | `Error type A at Line N: Mysterious character "X".` |
| 有错时 | **只**打印所有错误，**不**打印 token（两种情况互斥） |
| 注释 | 全部忽略，不产生 token |
| 数字 | 十六进制 / 八进制要输出**十进制值**（`0x1` → `1`，`077` → `63`） |
| package 名 | 必须是 `compiler`，OJ 按这个名字跑二进制 |

## 目录结构

```
compiler/
├── Cargo.toml
├── src/
│   ├── main.rs      # 读文件 → 调 lexer → 按上面两种格式打印
│   ├── lexer.rs     # Pair 树 → Token / LexError        ★ 有两处 TODO
│   └── sysy.pest    # PEG 文法（切分规则）              ★ 建议逐行读注释
└── tests/
    ├── cases/       # *.sy 源码 + 同名 *.out 金标准
    └── run_cases.sh # 批量比对
```

## 快速开始

```bash
cargo build --release

# 跑单个用例（输出在 stderr，所以用 2>&1 才看得到）
./target/release/compiler tests/cases/sample1.sy

# 跑全部用例并比对金标准
./tests/run_cases.sh
```

现在的状态：`cargo build` 能过，但一跑就会在 `src/lexer.rs` 的两处 `todo!()` 上 panic。
填完那两处，`./tests/run_cases.sh` 应该 **13 个全 PASS**。

## 要你自己写的两处

### 1. `keyword(&str) -> Option<TokenKind>`

9 个关键字：`const int void if else while break continue return`。

写成 `match word { "const" => Some(TokenKind::Const), ..., _ => None }`。

> 为什么不把关键字写进 `.pest` 文件？见下面「坑 2」。

### 2. `parse_integer(&str) -> i64`

三路判断，**顺序不能换**：

| 条件 | 进制 | 例子 |
|---|---|---|
| 以 `0x` / `0X` 开头 | 16，剥前缀 | `0x1`→1，`0xA`→10，`0xcD`→205 |
| 以 `0` 开头且 `len() > 1` | 8，剥**一个**前导 0 | `02`→2，`077`→63，`00`→0 |
| 其余 | 10 | `0`→0，`2147483647`→2147483647 |

可用 API：`str::strip_prefix`、`i64::from_str_radix(digits, radix)`、`str::parse`。

两个容易写崩的点：单个 `0` 不能走八进制（剥完是空串）；`077` 只剥一个 0。

## 五个坑（都写在对应源码的注释里）

1. **`program` 必须能容忍非法字符** —— 否则第一个错误就让 `parse` 返回 `Err`，
   拿不到"所有错误"。靠 `unknown_char = @{ ANY }` 兜底，见 `sysy.pest` 第 1 节。

2. **PEG 是有序选择，不是最长匹配** —— 关键字如果写成 `int = { "int" }` 排在
   IDENT 前面，`intx` 会被切成 `INT` + `IDENT("x")`。所以先整段读成 identifier
   再查表（真实词法器也是这么干的）。见 `sysy.pest` 第 2 节。

3. **进制前缀的顺序** —— `hex_int | oct_int | dec_int`，这个顺序动了就全错。
   如果 `dec` 在前，`0x1` 会被切成 `0` + `x1`。见 `sysy.pest` 第 3 节。

4. **运算符的长符号要排在短符号前** —— `==` 在 `=` 前、`<=` 在 `<` 前。
   否则 `a == b` 变成两个 `ASSIGN`。见 `sysy.pest` 第 4 节。

5. **`WHITESPACE` / `COMMENT` 是 pest 的特殊规则名** —— 定义成 `_{ ... }` 之后，
   空白和注释会被自动插到每条规则的序列元素之间，**根本不会进 Pair 树**，
   不用手写跳过逻辑。见 `sysy.pest` 第 5 节。

## 已知取舍（讲义没规定，按最宽松的方式处理了）

| 情况 | 当前行为 | 严格 SysY 规范 |
|---|---|---|
| `08` | 落回十进制读成 8 | 八进制里没有 8，应报错 |
| `0x` 后面没数字 | 切成 `INTEGER_CONST 0` + `IDENT x` | 应报错 |
| 未闭合的 `/*` | 当 `DIV` + `MUL`，后面内容继续按代码扫 | 应报错 |
| `float` 关键字 / 浮点字面量 | 讲义 P6/P7 的 token 表里没有，一律当 IDENT | 真实 SysY 有 `FLOAT_CONST` |
| `123abc` | 切成 `INTEGER_CONST 123` + `IDENT abc` | 应报错 |

如果 OJ 有 hard 用例踩到上面某条，再来收紧 —— 优先动 `sysy.pest`，
把对应规则从"能匹配"改成"匹配失败落到 `unknown_char`"即可。

## 提交清单（讲义 P25-P27）

- [ ] `package.name == "compiler"`
- [ ] 在 `lab1` 分支上提交，不要交到 `main`
- [ ] 推到 gitcode，提交仓库链接
- [ ] 用 `./tests/run_cases.sh` 全绿再推

> 注：训练营 OJ（`hz-t3.matpool.com:26885`）在闭营后大概率不可达，
> 所以本仓库用 `tests/cases/` 的本地金标准做验收。金标准里的
> `sample1~4` 是从讲义 P13-P21 逐行誊下来的，和讲义输出完全一致。
