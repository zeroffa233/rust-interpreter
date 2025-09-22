#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    Integer,
    Plus,
    Minus,
    Eof,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>) -> Self {
        Token { token_type, value }
    }
}
