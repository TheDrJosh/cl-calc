use crate::{ast::Node, parser::Parser};

pub fn run(text: String) -> anyhow::Result<f64> {
    let mut parser = Parser::new(text)?;
    let node = parser.calc()?;

    step(node)
}

fn step(node: Node) -> anyhow::Result<f64> {
    Ok(match node {
        Node::Number(num) => num,
        Node::Expr(node1, op, node2) => match op {
            crate::ast::Operator::Plus => step(*node1)? + step(*node2)?,
            crate::ast::Operator::Minus => step(*node1)? - step(*node2)?,
            crate::ast::Operator::Mult => step(*node1)? * step(*node2)?,
            crate::ast::Operator::Div => step(*node1)? / step(*node2)?,
            crate::ast::Operator::Pow => step(*node1)?.powf(step(*node2)?),
        },
        Node::Function(_, func, node) => func(step(*node)?),
    })
}
