//! SysY 词法分析器
//!
//! 分工：
//!   - `sysy.pest`：只负责"在哪里切开"（哪些字符序列构成一个词素）
//!   - 本文件：负责"切开之后它是什么"（词素 → TokenKind）+ 记账（行号、错误）
//!
//! 这里刻意**不**用 pest 生成 `Token`，而是把 Pair 树再翻译一遍。原因有三：
//!   1. 关键字识别要做成"查表"，比在文法里给 9 个关键字各加负向前瞻干净；
//!   2. 进制转换（0x1 → 1、077 → 63）只能在 Rust 侧做；
//!   3. 出错时还要继续扫，pest 的 Err 做不到，得自己收集。

use pest::Parser;
use pest_derive::Parser;

/// pest 根据 `src/sysy.pest` 生成的解析器。
/// `#[grammar]` 的路径是相对 `src/` 解析的（写 `sysy.pest`，不是 `src/sysy.pest`）。
#[derive(Parser)]
#[grammar = "sysy.pest"]
pub struct SysYLexer;

// ============================================================================
// Token 类型
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind {
    // ---- 关键字 ----
    Const,
    Int,
    Void,
    If,
    Else,
    While,
    Break,
    Continue,
    Return,

    // ---- 标识符与常量 ----
    Ident,
    IntegerConst(i64),

    // ---- 运算符 ----
    Plus,
    Minus,
    Mul,
    Div,
    Mod,
    Assign,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    Not,
    And,
    Or,

    // ---- 分隔符 ----
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBrackt,
    RBrackt,
    Comma,
    Semicolon,
}

impl TokenKind {
    /// 输出里那个大写名字，必须和讲义逐个字母对上。
    pub fn name(&self) -> &'static str {
        match self {
            TokenKind::Const => "CONST",
            TokenKind::Int => "INT",
            TokenKind::Void => "VOID",
            TokenKind::If => "IF",
            TokenKind::Else => "ELSE",
            TokenKind::While => "WHILE",
            TokenKind::Break => "BREAK",
            TokenKind::Continue => "CONTINUE",
            TokenKind::Return => "RETURN",
            TokenKind::Ident => "IDENT",
            TokenKind::IntegerConst(_) => "INTEGER_CONST",
            TokenKind::Plus => "PLUS",
            TokenKind::Minus => "MINUS",
            TokenKind::Mul => "MUL",
            TokenKind::Div => "DIV",
            TokenKind::Mod => "MOD",
            TokenKind::Assign => "ASSIGN",
            TokenKind::Eq => "EQ",
            TokenKind::Neq => "NEQ",
            TokenKind::Lt => "LT",
            TokenKind::Gt => "GT",
            TokenKind::Le => "LE",
            TokenKind::Ge => "GE",
            TokenKind::Not => "NOT",
            TokenKind::And => "AND",
            TokenKind::Or => "OR",
            TokenKind::LParen => "L_PAREN",
            TokenKind::RParen => "R_PAREN",
            TokenKind::LBrace => "L_BRACE",
            TokenKind::RBrace => "R_BRACE",
            TokenKind::LBrackt => "L_BRACKT",
            TokenKind::RBrackt => "R_BRACKT",
            TokenKind::Comma => "COMMA",
            TokenKind::Semicolon => "SEMICOLON",
        }
    }
}

/// 一个 token 的完整信息。
///
/// `lexeme` 保留源码原文（出错好定位、以后做错误提示也能用），
/// 但打印时 INTEGER_CONST 走的是 `kind` 里的十进制值 —— 见 `text()`。
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    /// 输出里 token 后面跟的那一段文本。
    ///
    /// 讲义要求"对十六进制和八进制数字常量输出 token 时需输出其十进制的值"，
    /// 所以 INTEGER_CONST 不能用 `lexeme`，要用已经转换好的 `kind` 里的值。
    fn text(&self) -> String {
        match &self.kind {
            TokenKind::IntegerConst(v) => v.to_string(),
            _ => self.lexeme.clone(),
        }
    }

    /// 一行完整输出：`INT int at Line 1.`
    pub fn render(&self) -> String {
        format!("{} {} at Line {}.", self.kind.name(), self.text(), self.line)
    }
}

// ============================================================================
// 词法错误
// ============================================================================

/// Lab1 只有一种错误：type A（出现了不属于任何 token 的字符）。
#[derive(Debug, Clone)]
pub struct LexError {
    pub line: usize,
    pub ch: char,
}

impl LexError {
    /// `Error type A at Line 3: Mysterious character "~".`
    pub fn render(&self) -> String {
        format!(
            "Error type A at Line {}: Mysterious character \"{}\".",
            self.line, self.ch
        )
    }
}

// ============================================================================
// 对外入口
// ============================================================================

#[derive(Debug, Default)]
pub struct LexOutput {
    /// 按出现顺序排列的 token。有错误时这个 vec 不完整，调用方应当忽略它。
    pub tokens: Vec<Token>,
    /// 按出现顺序排列的词法错误。
    pub errors: Vec<LexError>,
}

/// 词法分析主流程。
///
/// # 为什么 `parse` 可以直接 `expect`？
/// 因为 `sysy.pest` 里 `program = { SOI ~ (token | unknown_char)* ~ EOI }`，
/// 而 `unknown_char = @{ ANY }` 能吃掉任意字符 —— 这条文法对任何合法 UTF-8
/// 输入都能匹配成功。（能不 panic 就不 panic，是走这条兜底规则换来的。）
pub fn lex(src: &str) -> LexOutput {
    let mut out = LexOutput::default();

    let pairs = SysYLexer::parse(Rule::program, src)
        .expect("sysy.pest 的 program 规则应当匹配任意输入");

    for pair in pairs {
        // pairs 只有一个元素：最外层的 program
        for inner in pair.into_inner() {
            // 行号：pest 的 line_col() 返回 (行, 列)，都是 1 起算。
            // 只以 '\n' 分行，所以 CRLF 文本也算得对。
            let line = inner.line_col().0;

            match inner.as_rule() {
                // pest 的 EOI 也会作为子节点出现（零宽），跳过即可
                Rule::EOI => {}

                // 兜底规则吃到的字符 = 词法错误
                Rule::unknown_char => {
                    let ch = inner.as_str().chars().next().unwrap();
                    out.errors.push(LexError { line, ch });
                }

                // 标识符 or 关键字
                Rule::ident_or_kw => {
                    let s = inner.as_str();
                    let kind = keyword(s).unwrap_or(TokenKind::Ident);
                    out.tokens.push(Token { kind, lexeme: s.to_string(), line });
                }

                // 三种整数字面量（文法保证已经把进制前缀切好了）
                Rule::hex_int | Rule::oct_int | Rule::dec_int => {
                    out.tokens.push(Token {
                        kind: TokenKind::IntegerConst(parse_integer(inner.as_str())),
                        lexeme: inner.as_str().to_string(),
                        line,
                    });
                }

                // 剩下的全是运算符 / 分隔符，规则名和 TokenKind 一一对应
                other => {
                    let kind = symbol_kind(other)
                        .unwrap_or_else(|| unreachable!("未处理的规则: {:?}", other));
                    out.tokens.push(Token { kind, lexeme: inner.as_str().to_string(), line });
                }
            }
        }
    }

    out
}

// ============================================================================
// TODO：这两处是实验的核心考点，自己补
// ============================================================================

/// 关键字表：把标识符形状的字符串映射到关键字 TokenKind。
///
/// 【为什么放在这里而不是写进 .pest 文件】
/// 见 `sysy.pest` 第 2 节的注释：PEG 不做最长匹配，`intx` 会被 `"int"` 前缀吃成
/// INT + IDENT。先整段读成 identifier 再查表，是真实词法器的标准做法。
///
/// 【要填的 9 个关键字】从讲义第 6 页抄，一个不多一个不少：
///   const int void if else while break continue return
///
/// 【提示】
///   写成 `match word { "const" => Some(TokenKind::Const), ... _ => None }`
///   注意最后要有 `_ => None` 兜底 —— 查不到就是普通标识符。
fn keyword(word: &str) -> Option<TokenKind> {
    Some(match word {
        "const" => TokenKind::Const,
        "int" => TokenKind::Int,
        "void" => TokenKind::Void,
        "if" => TokenKind::If,
        "else" => TokenKind::Else,
        "while" => TokenKind::While,
        "break" => TokenKind::Break,
        "continue" => TokenKind::Continue,
        "return" => TokenKind::Return,
        // 查不到 = 普通标识符。这一条是这个函数存在的全部意义：
        // `intx` / `iffy` / `returns` 都走到这里，而不是被当成关键字前缀。
        _ => return None,
    })
}

/// 把整数字面量的**源码文本**转换成十进制数值。
///
/// 【三路判断，顺序有讲究】
///   1. 以 `0x` 或 `0X` 开头  → 十六进制，剥掉前缀后按 16 进制解析
///      `0x1` → 1     `0xA` → 10     `0xcD` → 205
///   2. 以 `0` 开头且长度 > 1 → 八进制，剥掉开头的 0 后按 8 进制解析
///      `02` → 2      `077` → 63     `00` → 0
///   3. 其余                   → 十进制
///      `0` → 0       `2147483647` → 2147483647
///
/// 【坑】第 2 条必须判 `len() > 1`，否则单个 `0` 会被当成八进制去解析空串。
/// 【坑】"剥掉开头的 0" 只剥一个：`077` 剩下 `"77"`，不是 `"077"` 也不是 `""`。
///
/// 【可用 API】
///   `str::strip_prefix("0x")`  → Option<&str>，不匹配返回 None，
///                                正好可以和 `"0X"` 组合：`raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X"))`
///   `i64::from_str_radix(digits, radix)` → `Result<i64, ParseIntError>`
///   `str::parse::<i64>()`       → 十进制专用，等价于 from_str_radix(s, 10)
fn parse_integer(raw: &str) -> i64 {
    // 先定"剥掉什么前缀、按几进制读"，再统一解析一次 —— 三条路各自调一次
    // from_str_radix 会让错误处理重复三遍，先归一化更清爽。
    let (digits, radix) = if let Some(hex) = raw.strip_prefix("0x").or_else(|| raw.strip_prefix("0X")) {
        // 0x1 / 0Xab：剥掉 2 个字符
        (hex, 16)
    } else if raw.len() > 1 && raw.starts_with('0') {
        // 02 / 077 / 00：只剥掉 1 个 '0'（`077` → "77"）
        // 必须判 len > 1，否则单个 "0" 会剥成空串，八进制解析直接失败
        (&raw[1..], 8)
    } else {
        (raw, 10)
    };

    i64::from_str_radix(digits, radix)
        .unwrap_or_else(|_| panic!("整数字面量超出 i64 范围: {raw}"))
}

/// 运算符 / 分隔符的 Rule → TokenKind 映射。
/// 这部分纯体力活，而且和 `sysy.pest` 里的规则名一一对应，直接写完了。
fn symbol_kind(rule: Rule) -> Option<TokenKind> {
    Some(match rule {
        Rule::eq => TokenKind::Eq,
        Rule::neq => TokenKind::Neq,
        Rule::le => TokenKind::Le,
        Rule::ge => TokenKind::Ge,
        Rule::and => TokenKind::And,
        Rule::or => TokenKind::Or,
        Rule::plus => TokenKind::Plus,
        Rule::minus => TokenKind::Minus,
        Rule::star => TokenKind::Mul,
        Rule::slash => TokenKind::Div,
        Rule::percent => TokenKind::Mod,
        Rule::assign => TokenKind::Assign,
        Rule::lt => TokenKind::Lt,
        Rule::gt => TokenKind::Gt,
        Rule::not => TokenKind::Not,
        Rule::l_paren => TokenKind::LParen,
        Rule::r_paren => TokenKind::RParen,
        Rule::l_brace => TokenKind::LBrace,
        Rule::r_brace => TokenKind::RBrace,
        Rule::l_brackt => TokenKind::LBrackt,
        Rule::r_brackt => TokenKind::RBrackt,
        Rule::comma => TokenKind::Comma,
        Rule::semicolon => TokenKind::Semicolon,
        _ => return None,
    })
}
