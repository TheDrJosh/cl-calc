use std::f64::consts;

use crate::{
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Interpreter {
    text: String,
    lexer: Lexer,
    current_token: Token,
}

impl Interpreter {
    pub fn new(text: String) -> anyhow::Result<Self> {
        let mut lexer = Lexer::new(text.clone());
        let current_token = lexer.get_next_token()?;
        Ok(Self {
            text,
            lexer,
            current_token,
        })
    }

    pub fn error(&self, token: &Token) -> anyhow::Error {
        anyhow::anyhow!(
            "Invalid syntax: at {} near {}",
            token.start,
            self.text[self.current_token.start - 2..self.current_token.start + 2].to_owned()
        )
    }

    pub fn eat(&mut self, token_type: TokenType) -> anyhow::Result<()> {
        if self.current_token.token == token_type {
            self.current_token = self.lexer.get_next_token()?;
        } else {
            Err(self.error(&self.current_token))?;
        }
        Ok(())
    }

    pub fn functions(x: f64, func: String) -> anyhow::Result<f64> {
        match func.as_str() {
            "sqrt" => Ok(x.sqrt()),
            "ln" => Ok(x.ln()),
            "abs" => Ok(x.abs()),
            "cos" => Ok(x.cos()),
            "sin" => Ok(x.sin()),
            "tan" => Ok(x.tan()),
            "log" => Ok(x.log10()),
            _ => anyhow::bail!("invalid function name: {}", func),
        }
    }

    pub fn constants(con: String) -> anyhow::Result<f64> {
        match con.as_str() {
            "pi" => Ok(consts::PI),
            "e" => Ok(consts::E),
            _ => anyhow::bail!("invalid constant name: {}", con),
        }
    }

    pub fn factor(&mut self) -> anyhow::Result<f64> {
        let token = self.current_token.clone();

        match token.token {
            TokenType::Number => {
                self.eat(TokenType::Number)?;
                Ok(token.value.parse()?)
            }
            TokenType::LParen => {
                self.eat(TokenType::LParen)?;
                let result = self.expr()?;
                self.eat(TokenType::RParen)?;
                Ok(result)
            }
            TokenType::Ident => {
                self.eat(TokenType::Ident)?;
                if self.current_token.token == TokenType::LParen {
                    self.eat(TokenType::LParen)?;
                    let result = Self::functions(self.expr()?, token.value)?;
                    self.eat(TokenType::RParen)?;
                    Ok(result)
                } else {
                    Self::constants(token.value)
                }
                
            }
            _ => Err(self.error(&token)),
        }
    }

    pub fn exp(&mut self)  -> anyhow::Result<f64> {
        let mut result = self.factor()?;

        while self.current_token.token == TokenType::Exp
        {
            let token = self.current_token.clone();
            if token.token == TokenType::Exp {
                self.eat(TokenType::Exp)?;
                result = result.powf(self.factor()?);
            }
        }
        Ok(result)
    }

    pub fn term(&mut self) -> anyhow::Result<f64> {
        let mut result = self.exp()?;

        while self.current_token.token == TokenType::Mult
            || self.current_token.token == TokenType::Div
        {
            let token = self.current_token.clone();
            if token.token == TokenType::Mult {
                self.eat(TokenType::Mult)?;
                result = result * self.exp()?;
            } else if token.token == TokenType::Div {
                self.eat(TokenType::Div)?;
                result = result / self.exp()?;
            }
        }
        Ok(result)
    }

    pub fn expr(&mut self) -> anyhow::Result<f64> {
        let mut result = self.term()?;

        while self.current_token.token == TokenType::Add
            || self.current_token.token == TokenType::Sub
        {
            let token = self.current_token.clone();
            if token.token == TokenType::Add {
                self.eat(TokenType::Add)?;
                let term = self.term()?;
                result = result + term;
            } else if token.token == TokenType::Sub {
                self.eat(TokenType::Sub)?;
                result = result - self.term()?;
            }
        }
        Ok(result)
    }

    pub fn calc(&mut self) -> anyhow::Result<f64> {
        let res = self.expr()?;

        self.eat(TokenType::EOI)?;

        Ok(res)
    }
}
