use std::collections::HashMap;

use crate::{ast::Node, parser::Parser};

#[derive(Default)]
pub struct Interpreter {
    consts: HashMap<String, f64>,
    funcs: HashMap<String, (String, Node)>,
}

impl Interpreter {
    pub fn run(&mut self, text: String) -> anyhow::Result<f64> {
        let mut parser = Parser::new(text)?;
        let node = parser.calc()?;
        self.step(node)
    }
    
    fn step(&mut self, node: Node) -> anyhow::Result<f64> {
        Ok(match node {
            Node::Number(num) => num,
            Node::Expr(node1, op, node2) => match op {
                crate::ast::Operator::Plus => self.step(*node1)? + self.step(*node2)?,
                crate::ast::Operator::Minus => self.step(*node1)? - self.step(*node2)?,
                crate::ast::Operator::Mult => self.step(*node1)? * self.step(*node2)?,
                crate::ast::Operator::Div => self.step(*node1)? / self.step(*node2)?,
                crate::ast::Operator::Pow => self.step(*node1)?.powf(self.step(*node2)?),
                crate::ast::Operator::Assign => {
                    todo!()
                },
            },
            Node::Function(_, func, node) => func(self.step(*node)?),
        })
    }
    
}


