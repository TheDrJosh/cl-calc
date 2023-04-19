use std::{collections::HashMap, f64::consts};

use crate::{ast::Node, parser::Parser};

#[derive(Default)]
pub struct Interpreter {
    pub consts: HashMap<String, f64>,
    pub funcs: HashMap<String, (String, Node)>,
    pub ans: f64,
}

impl Interpreter {
    fn functions(&mut self, func: String, x: f64) -> anyhow::Result<f64> {
        Ok(match func.as_str() {
            "sqrt" => x.sqrt(),
            "ln" => x.ln(),
            "abs" => x.abs(),
            "cos" => x.cos(),
            "sin" => x.sin(),
            "tan" => x.tan(),
            "log" => x.log10(),
            "ans" => self.ans,
            _ => {
                if let Some((var, body)) = self
                    .funcs
                    .get(&func)
                    .map(|(var, body)| (var.clone(), body.clone()))
                {
                    let temp = self.consts.insert(var.clone(), x);

                    let res = self.step(body.clone())?;

                    if let Some(temp) = temp {
                        self.consts.insert(var.clone(), temp);
                    }

                    res
                } else {
                    anyhow::bail!("invalid function name: {}", func)
                }
            }
        })
    }

    fn constants(&self, con: String) -> anyhow::Result<f64> {
        match con.as_str() {
            "pi" => Ok(consts::PI),
            "e" => Ok(consts::E),
            _ => {
                if let Some(val) = self.consts.get(&con) {
                    Ok(*val)
                } else {
                    anyhow::bail!("invalid constant name: {}", con)
                }
            }
        }
    }

    pub fn run(&mut self, text: String) -> anyhow::Result<f64> {
        let mut parser = Parser::new(text)?;
        let node = parser.calc()?;
        let res = self.step(node)?;
        self.ans = res;
        Ok(res)
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
            },
            Node::Function(func, node) => {
                let val = self.step(*node)?;
                self.functions(func, val)?
            }
            Node::AssignConst(name, expr) => {
                let val = self.step(*expr)?;
                self.consts.insert(name, val);
                val
            }
            Node::AssignFunc(name, var, body) => {
                self.funcs.insert(name, (var, *body));
                0.
            }
            Node::Const(const_name) => self.constants(const_name)?,
        })
    }
}
