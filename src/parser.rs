
use crate::{
    ast::Node,
    lexer::Lexer,
    token::{Token, TokenType},
};

pub struct Parser {
    text: String,
    tokens: Vec<Token>,
    current_token: usize,
}

impl Parser {
    pub fn new(text: String) -> anyhow::Result<Self> {
        let mut lexer = Lexer::new(text.clone());
        let tokens = lexer.get_all_tokens()?;
        Ok(Self {
            text,
            tokens,
            current_token: 0,
        })
    }

    fn get(&self, i: usize) -> anyhow::Result<&Token> {
        if let Some(token) = self.tokens.get(i) {
            Ok(token)
        } else {
            anyhow::bail!("token index out of bounds");
        }
    }

    fn current_token(&self) -> anyhow::Result<&Token> {
        self.get(self.current_token)
    }

    fn next_token(&mut self) {
        self.current_token += 1;
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
        if self.current_token()?.token == token_type {
            self.next_token();
        } else {
            Err(self.error(self.current_token()?, token_type))?;
        }
        Ok(())
    }

    fn factor(&mut self) -> anyhow::Result<Node> {
        let token = self.current_token()?.clone();

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
                if self.current_token()?.token == TokenType::LParen {
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

        while self.current_token()?.token == TokenType::Exp {
            let token = self.current_token()?.clone();
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

        while self.current_token()?.token == TokenType::Mult
            || self.current_token()?.token == TokenType::Div
        {
            let token = self.current_token()?.clone();
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
        let result = self.term()?;
        self.expr_cont(result)
    }

    fn expr_cont(&mut self, mut result: Node) -> anyhow::Result<Node> {
        while self.current_token()?.token == TokenType::Add
            || self.current_token()?.token == TokenType::Sub
        {
            let token = self.current_token()?.clone();
            if token.token == TokenType::Add {
                self.eat(TokenType::Add)?;
                result = Node::Expr(
                    Box::new(result),
                    crate::ast::Operator::Plus,
                    Box::new(self.term()?),
                )
            } else if token.token == TokenType::Sub {
                self.eat(TokenType::Sub)?;
                result = Node::Expr(
                    Box::new(result),
                    crate::ast::Operator::Minus,
                    Box::new(self.term()?),
                )
            }
        }
        Ok(result)
    }

    // look ahead parser
    fn assign(&mut self) -> anyhow::Result<Node> {
        if self
            .tokens
            .get(self.current_token)
            .map(|token| &token.token)
            == Some(&TokenType::Ident)
            && self
                .tokens
                .get(self.current_token + 1)
                .map(|token| &token.token)
                == Some(&TokenType::Assign)
        {
            let const_name = self.current_token()?.clone().value;

            self.eat(TokenType::Ident)?;
            self.eat(TokenType::Assign)?;

            return Ok(Node::AssignConst(const_name, Box::new(self.expr()?)));
        } else if self
            .tokens
            .get(self.current_token)
            .map(|token| &token.token)
            == Some(&TokenType::Ident)
            && self
                .tokens
                .get(self.current_token + 1)
                .map(|token| &token.token)
                == Some(&TokenType::LParen)
            && self
                .tokens
                .get(self.current_token + 2)
                .map(|token| &token.token)
                == Some(&TokenType::Ident)
            && self
                .tokens
                .get(self.current_token + 3)
                .map(|token| &token.token)
                == Some(&TokenType::RParen)
            && self
                .tokens
                .get(self.current_token + 4)
                .map(|token| &token.token)
                == Some(&TokenType::Assign)
        {
            let func_name = self.current_token()?.clone().value;

            self.eat(TokenType::Ident)?;
            self.eat(TokenType::LParen)?;

            let var_name = self.current_token()?.clone().value;
            self.eat(TokenType::Ident)?;

            self.eat(TokenType::RParen)?;
            self.eat(TokenType::Assign)?;

            return Ok(Node::AssignFunc(func_name, var_name, Box::new(self.expr()?)));
        } else {
            self.expr()
        }

    }
    pub fn calc(&mut self) -> anyhow::Result<Node> {
        let res = self.assign()?;

        self.eat(TokenType::EOI)?;

        Ok(res)
    }
}
