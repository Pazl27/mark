#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Basic content
    Text(String),
    Newline,
    Whitespace,
    Eof,

    // Markdown markers
    Hash(u8),
    Asterisk(u8),
    Underscore(u8),
    Tilde(u8),
    Backtick(u8),
    LeftBracket,
    RightBracket,
    LeftParen,
    RightParen,
    Exclamation,
    GreaterThan,

    // Lists and rules
    Hyphen,
    Number(u32),
    Dot,
    Plus,

    // Tables
    Pipe,
    Colon,

    // Links and references
    Url(String),
}
