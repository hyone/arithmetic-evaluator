use types::*;

pub fn eval(expr: &Expr) -> f64 {
    match *expr {
        Expr::Num(n) => n,
        Expr::Add(ref m, ref n) => eval(m) + eval(n),
        Expr::Sub(ref m, ref n) => eval(m) - eval(n),
        Expr::Mul(ref m, ref n) => eval(m) * eval(n),
        Expr::Div(ref m, ref n) => eval(m) / eval(n),
        Expr::Paren(ref e)      => eval(e),
    }
}

#[cfg(test)]
mod tests {
    use types::Expr::*;
    use super::eval;

    #[test]
    fn eval_basic_test() {
        assert_eq!(eval(&Num(3.)), 3.);
        assert_eq!(eval(&Add(Box::new(Num(5.)), Box::new(Num(3.)))), 8.);
        assert_eq!(eval(&Sub(Box::new(Num(5.)), Box::new(Num(3.)))), 2.);
        assert_eq!(eval(&Mul(Box::new(Num(5.)), Box::new(Num(3.)))), 15.);
        assert_eq!(eval(&Div(Box::new(Num(6.)), Box::new(Num(3.)))), 2.);
    }

    #[test]
    fn eval_complex_test() {
        assert_eq!(eval(&Add(Box::new(Sub(
                                 Box::new(Add(Box::new(Num(12.)), Box::new(Num(6.)))),
                                 Box::new(Num(4.)))),
                             Box::new(Num(3.)))),
                   17.);

        assert_eq!(eval(&Add(Box::new(Add(Box::new(Num(1.)),
                                          Box::new(Mul(Box::new(Num(2.)), Box::new(Num(3.)))))),
                             Box::new(Num(4.)))),
                   11.);

        assert_eq!(eval(&Mul(Box::new(Num(2.)),
                             Box::new(Paren(Box::new(Add(Box::new(Num(3.)),
                                                         Box::new(Num(4.)))))))),
                   14.);

        assert_eq!(eval(&Add(Box::new(Div(Box::new(Mul(Box::new(Num(2.)),
                                                       Box::new(Num(2.)))),
                                          Box::new(Paren(Box::new(Sub(Box::new(Num(5.)),
                                                                      Box::new(Num(1.)))))))),
                             Box::new(Num(3.)))),
                   4.);
    }
}
