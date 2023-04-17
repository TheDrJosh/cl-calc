#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token: TokenType,
    pub start: usize,
}

impl Token {
    pub fn new(value: String, token: TokenType, start: usize) -> Self {
        Self {
            value,
            token,
            start,
        }
    }
}

impl Default for Token {
    fn default() -> Self {
        Self { value: Default::default(), token: TokenType::EOI, start: Default::default() }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenType {
    Number,
    Add,
    Sub,
    Mult,
    Div,
    Exp,
    LParen,
    RParen,
    Ident,
    EOI,
}
