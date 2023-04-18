use std::fmt::Debug;

pub enum Node {
    Number(f64),
    Expr(Box<Node>, Operator, Box<Node>),
    Function(String, Box<dyn Fn(f64) -> f64>, Box<Node>),
}
impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::Expr(arg0, arg1, arg2) => f.debug_tuple("Expr").field(arg0).field(arg1).field(arg2).finish(),
            Self::Function(arg0, _, arg2) => f.debug_tuple("Function").field(arg0).field(arg2).finish(),
        }
    }
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Mult,
    Div,
    Pow,
    Assign,
}