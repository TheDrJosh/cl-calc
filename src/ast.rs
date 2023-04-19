use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Number(f64),
    Expr(Box<Node>, Operator, Box<Node>),
    Function(String, Box<Node>),
    Const(String),
    AssignConst(String, Box<Node>),
    AssignFunc(String, String, Box<Node>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
    Pow,
}
