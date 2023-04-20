use std::{collections::HashMap, f64::consts, path::PathBuf, fs};

use crate::{ast::Node, parser::Parser};

#[derive(Default, Clone)]
pub struct Interpreter {
    pub consts: HashMap<String, f64>,
    pub funcs: HashMap<String, (String, Node)>,
    pub ans: f64,
    pub executed_lines: Vec<String>
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
            "ans" => Ok(self.ans),
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
        let mut parser = Parser::new(text.clone())?;
        let node = parser.calc()?;
        let res = self.step(node)?;
        self.ans = res;
        self.executed_lines.push(text.trim().to_owned());
        Ok(res)
    }

    pub fn run_file(&mut self, path: PathBuf) -> anyhow::Result<(Vec<f64>, f64)> {
        let contents = fs::read_to_string(path)?;

        let mut interpreter = self.clone();

        let mut debug_out = Vec::new();

        for line in contents.split('\n') {
            let mut line = line.to_owned();
            let do_out = line.chars().next() == Some('!');
            if do_out {
                line.remove(0);
            }
            let out = interpreter.run(line)?;
            if do_out {
                debug_out.push(out);
            }
        }

        *self = interpreter;
        
        Ok((debug_out, self.ans))
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
