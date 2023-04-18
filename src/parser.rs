
use crate::{
    ast::Node,
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    text: String,
    lexer: Lexer,
    current_token: Token,
}

impl Parser {
    pub fn new(text: String) -> anyhow::Result<Self> {
        let mut lexer = Lexer::new(text.clone());
        let current_token = lexer.get_next_token()?;
        Ok(Self {
            text,
            lexer,
            current_token,
        })
    }

    fn error(&self, token: &Token, expected: TokenType) -> anyhow::Error {
        let mut arrow = " ".repeat(token.start - 1);
        arrow.push('^');
        anyhow::anyhow!(
            "Invalid syntax, expected {:?}\n{}\n{}",
            expected,
            self.text.trim_end(),
            arrow
        )
    }

    fn eat(&mut self, token_type: TokenType) -> anyhow::Result<()> {
        if self.current_token.token == token_type {
            self.current_token = self.lexer.get_next_token()?;
        } else {
            Err(self.error(&self.current_token, token_type))?;
        }
        Ok(())
    }

    

    fn factor(&mut self) -> anyhow::Result<Node> {
        let token = self.current_token.clone();

        match token.token {
            TokenType::Number => {
                self.eat(TokenType::Number)?;
                let num = token.value.parse()?;
                Ok(Node::Number(num))
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
                    let inner = self.expr()?;
                    self.eat(TokenType::RParen)?;
                    Ok(Node::Function(token.value, Box::new(inner)))
                } else {
                    Ok(Node::Const(token.value))
                }
            }
            _ => Err(self.error(&token, TokenType::Number)),
        }
    }

    fn exp(&mut self) -> anyhow::Result<Node> {
        let mut result = self.factor()?;

        while self.current_token.token == TokenType::Exp {
            let token = self.current_token.clone();
            if token.token == TokenType::Exp {
                self.eat(TokenType::Exp)?;
                result = Node::Expr(
                    Box::new(result),
                    crate::ast::Operator::Pow,
                    Box::new(self.factor()?),
                )
            }
        }
        Ok(result)
    }

    fn term(&mut self) -> anyhow::Result<Node> {
        let mut result = self.exp()?;

        while self.current_token.token == TokenType::Mult
            || self.current_token.token == TokenType::Div
        {
            let token = self.current_token.clone();
            if token.token == TokenType::Mult {
                self.eat(TokenType::Mult)?;
                result = Node::Expr(
                    Box::new(result),
                    crate::ast::Operator::Mult,
                    Box::new(self.exp()?),
                )
            } else if token.token == TokenType::Div {
                self.eat(TokenType::Div)?;
                result = Node::Expr(
                    Box::new(result),
                    crate::ast::Operator::Div,
                    Box::new(self.exp()?),
                )
            }
        }
        Ok(result)
    }

    fn expr(&mut self) -> anyhow::Result<Node> {
        let mut result = self.term()?;

        while self.current_token.token == TokenType::Add
            || self.current_token.token == TokenType::Sub
        {
            let token = self.current_token.clone();
            if token.token == TokenType::Add {
                self.eat(TokenType::Add)?;
                result = Node::Expr(Box::new(result), crate::ast::Operator::Plus, Box::new(self.term()?))

            } else if token.token == TokenType::Sub {
                self.eat(TokenType::Sub)?;
                result = Node::Expr(Box::new(result), crate::ast::Operator::Minus, Box::new(self.term()?))
            }
        }
        Ok(result)
    }

    fn assign(&mut self) -> anyhow::Result<Node> {

        let token = self.current_token.clone();
        if token.token == TokenType::Ident {
            self.eat(TokenType::Ident)?;
            let mut func_param = None;
            if self.current_token.token == TokenType::LParen {
                self.eat(TokenType::LParen)?;
                func_param = Some(self.current_token.clone());
                self.eat(TokenType::Ident)?;
                self.eat(TokenType::RParen)?;
            }

            if self.current_token.token == TokenType::Assign {
                if let Some(func_param) = func_param {
                    Ok(Node::AssignFunc(token.value, func_param.value, Box::new(self.expr()?)))
                } else {
                    Ok(Node::AssignConst(token.value, Box::new(self.expr()?)))
                }
            } else {
                anyhow::bail!("test");
            }
            
        } else {
            self.expr()
        }

        

    }


    pub fn calc(&mut self) -> anyhow::Result<Node> {
        let res = self.expr()?;



        self.eat(TokenType::EOI)?;

        Ok(res)
    }
}
