use crate::token::{Token, TokenType};

pub struct Lexer {
    text: Vec<char>,
    pos: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(text: String) -> Self {
        let current_char = text.clone().chars().next();
        Self {
            text: text.chars().collect(),
            pos: 0,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.pos += 1;

        if self.pos > self.text.len() - 1 {
            self.current_char = None;
        } else {
            self.current_char = Some(self.text[self.pos]);
        }
    }

    fn skip_whitespace(&mut self) {
        while self.current_char.map_or(false, |char| char.is_whitespace()) {
            self.advance();
        }
    }

    fn identifier(&mut self) -> Token {
        let mut result = String::default();
        let pos = self.pos;

        while self.current_char.map_or(false, |char| char.is_alphabetic()) {
            result.push(self.current_char.unwrap());
            self.advance();
        }

        Token::new(result, TokenType::Ident, pos)
    }

    fn number(&mut self) -> Token {
        let mut result = String::default();
        let pos = self.pos;

        if self.current_char == Some('-') {
            result.push('-');
            self.advance();
        }

        while self.current_char.map_or(false, |char| char.is_ascii_digit()) {
            result.push(self.current_char.unwrap());
            self.advance();
        }

        if self.current_char == Some('.') {
            result.push('.');
            self.advance();
        }

        while self.current_char.map_or(false, |char| char.is_ascii_digit()) {
            result.push(self.current_char.unwrap());
            self.advance();
        }

        Token::new(result, TokenType::Number, pos)
    }

    pub fn get_next_token(&mut self) -> anyhow::Result<Token> {
        loop {
            if let Some(current_char) = self.current_char {
                match current_char {
                    '+' => {
                        self.advance();
                        return Ok(Token::new(String::from("+"), TokenType::Add, self.pos));
                    }
                    '-' => {
                        self.advance();
                        return Ok(Token::new(String::from("-"), TokenType::Sub, self.pos));
                    }
                    '*' => {
                        self.advance();
                        return Ok(Token::new(String::from("*"), TokenType::Mult, self.pos));
                    }
                    '/' => {
                        self.advance();
                        return Ok(Token::new(String::from("/"), TokenType::Div, self.pos));
                    }
                    '^' => {
                        self.advance();
                        return Ok(Token::new(String::from("^"), TokenType::Exp, self.pos));
                    }
                    '(' => {
                        self.advance();
                        return Ok(Token::new(String::from("("), TokenType::LParen, self.pos));
                    }
                    ')' => {
                        self.advance();
                        return Ok(Token::new(String::from(")"), TokenType::RParen, self.pos));
                    }
                    '=' => {
                        self.advance();
                        return Ok(Token::new(String::from("="), TokenType::Assign, self.pos));
                    }
                    _ => {}
                };

                if current_char.is_whitespace() {
                    self.skip_whitespace();
                    continue;
                }

                if current_char.is_ascii_digit() || current_char == '-' {
                    return Ok(self.number());
                }

                if current_char.is_alphabetic() {
                    return Ok(self.identifier());
                }

                anyhow::bail!("Invalid character: {}", current_char);
            } else {
                break;
            }
        }
        Ok(Token::new(
            "".to_owned(),
            TokenType::EOI,
            self.text.len() - 1,
        ))
    }

    pub fn get_all_tokens(&mut self) -> anyhow::Result<Vec<Token>> {
        let mut res = Vec::new();
        loop {
            let token = self.get_next_token()?;
            res.push(token.clone());
            if token.token == TokenType::EOI {
                break;
            }
        }
        Ok(res)
    }

}
