use super::token_type::TokenType;

pub type LineNo = u16;

#[derive(Debug, Copy, Clone)]
pub struct Token<'a> {
    pub line: LineNo,
    pub content: &'a str,
    pub token_type: TokenType,
}
