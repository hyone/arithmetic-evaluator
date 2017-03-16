use num::rational::BigRational;
use types::*;

pub fn eval(expr: Expr) -> BigRational {
    match expr {
        Expr::Num(n)    => n,
        Expr::Add(m, n) => eval(*m) + eval(*n),
        Expr::Sub(m, n) => eval(*m) - eval(*n),
        Expr::Mul(m, n) => eval(*m) * eval(*n),
        Expr::Div(m, n) => eval(*m) / eval(*n),
        Expr::Paren(e)  => eval(*e),
    }
}

#[cfg(test)]
mod tests {
    use types::Expr::*;
    use utils::*;
    use super::eval;

    #[test]
    fn eval_basic_test() {
        assert_eq!(eval(Num(to_r("3"))), to_r("3"));
        assert_eq!(eval(Add(Box::new(Num(to_r("5"))), Box::new(Num(to_r("3"))))), to_r("8"));
        assert_eq!(eval(Sub(Box::new(Num(to_r("5"))), Box::new(Num(to_r("3"))))), to_r("2"));
        assert_eq!(eval(Mul(Box::new(Num(to_r("5"))), Box::new(Num(to_r("3"))))), to_r("15"));
        assert_eq!(eval(Div(Box::new(Num(to_r("6"))), Box::new(Num(to_r("3"))))), to_r("2"));
    }

    #[test]
    fn eval_complex_test() {
        assert_eq!(eval(Add(Box::new(Sub(
                                Box::new(Add(Box::new(Num(to_r("-12"))), Box::new(Num(to_r("6"))))),
                                Box::new(Num(to_r("-4")))
                            )),
                            Box::new(Num(to_r("3"))))),
                   to_r("1"));

        assert_eq!(eval(Mul(Box::new(Add(
                                Box::new(Num(to_r("1"))),
                                Box::new(Mul(Box::new(Num(to_r("2/5"))), Box::new(Num(to_r("3")))))
                            )),
                            Box::new(Num(to_r("4"))))),
                   to_r("44/5"));

        assert_eq!(eval(Mul(Box::new(Num(to_r("-2/3"))),
                            Box::new(Paren(Box::new(Add(Box::new(Num(to_r("3"))),
                                                        Box::new(Num(to_r("4"))))))))),
                   to_r("-14/3"));

        assert_eq!(eval(Add(Box::new(Div(Box::new(Mul(Box::new(Num(to_r("2"))),
                                                      Box::new(Num(to_r("-2"))))),
                                         Box::new(Paren(Box::new(Sub(Box::new(Num(to_r("-5"))),
                                                                     Box::new(Num(to_r("1"))))))))),
                            Box::new(Num(to_r("3"))))),
                   to_r("11/3"));
    }
}
