use num::rational::BigRational;
use std::fmt;
use self::Expr::*;

#[derive(Debug, PartialEq)]
pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Num(BigRational),
    Paren(Box<Expr>)
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Num(ref n)        => write!(f, "{}", n),
            Add(ref m, ref n) => write!(f, "{} + {}", m, n),
            Sub(ref m, ref n) => write!(f, "{} - {}", m, n),
            Mul(ref m, ref n) => write!(f, "{} * {}", m, n),
            Div(ref m, ref n) => write!(f, "{} / {}", m, n),
            Paren(ref e)      => write!(f, "({})", e),
        }
    }
}
