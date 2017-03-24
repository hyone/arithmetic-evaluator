use combine::*;
use combine::char::{ Spaces, digit, spaces };
use combine::combinator::{ FnParser, Skip };
use num::rational::Ratio;
use std::str::FromStr;
use std::marker::PhantomData;
use types::*;

pub struct Arithmetic<I>(PhantomData<Fn(I) -> I>);

type ArithmeticParseResult<I> = ParseResult<Expr, I>;
type ArithmeticParser<I> = FnParser<I, fn(I) -> ArithmeticParseResult<I>>;

fn lex<'a, P>(p: P) -> Skip<P, Spaces<P::Input>>
    where P: Parser,
          P::Input: Stream<Item=char>
{
    p.skip(spaces())
}

impl <I> Arithmetic<I>
    where I: Stream<Item=char>
{
    fn parens() -> ArithmeticParser<I> {
        parser(Arithmetic::parens_)
    }
    fn parens_(input: I) -> ArithmeticParseResult<I> {
        between(lex(token('(')), token(')'), lex(Arithmetic::expr()))
            .map(|e| Expr::Paren(Box::new(e)))
            .parse_stream(input)
    }

    fn number() -> ArithmeticParser<I> {
        parser(Arithmetic::number_)
    }
    fn number_(input: I) -> ArithmeticParseResult<I> {
        let sign = token('+').or(token('-'));

        let number = many1::<String, _>(digit().or(token('/')))
            .and_then(|digits| Ratio::from_str(digits.as_str()));

        (optional(sign), number)
            .map(|(prefix, num)| {
                match prefix {
                    None | Some('+') => Expr::Num(num),
                    Some('-')        => Expr::Num(-num),
                    _                => unreachable!(),
                }
            })
        .parse_stream(input)
    }

    fn factor() -> ArithmeticParser<I> {
        parser(Arithmetic::factor_)
    }
    fn factor_(input: I) -> ArithmeticParseResult<I> {
        Arithmetic::number()
            .or(Arithmetic::parens())
            .parse_stream(input)
    }

    fn term() -> ArithmeticParser<I> {
        parser(Arithmetic::term_)
    }
    fn term_(input: I) -> ArithmeticParseResult<I> {
        let operator = one_of("*/".chars())
            .map(|op| move |lhs, rhs| {
                match op {
                    '*' => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                    '/' => Expr::Div(Box::new(lhs), Box::new(rhs)),
                    _   => unreachable!()
                }
            });

        chainl1(lex(Arithmetic::factor()), lex(operator)).parse_stream(input)
    }

    fn expr() -> ArithmeticParser<I> {
        parser(Arithmetic::expr_)
    }
    fn expr_(input: I) -> ArithmeticParseResult<I> {
        let operator = one_of("+-".chars())
            .map(|op| move |lhs, rhs| {
                match op {
                    '+' => Expr::Add(Box::new(lhs), Box::new(rhs)),
                    '-' => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                    _   => unreachable!()
                }
            });

        chainl1(lex(Arithmetic::term()), lex(operator)).parse_stream(input)
    }

    pub fn parser() -> ArithmeticParser<I> {
        parser(Arithmetic::parser_)
    }
    fn parser_(input: I) -> ArithmeticParseResult<I> {
        optional(spaces()).with(Arithmetic::expr()).parse_stream(input)
    }
}

#[cfg(test)]
mod tests {
    use combine::*;
    use combine::primitives::{ Error, Info, ParseError, SourcePosition };
    use types::Expr::*;
    use utils::*;
    use super::*;

    #[test]
    fn lex_test() {
        assert_eq!(digit().parse("4   "), Ok(('4', "   ")));
        assert_eq!(lex(digit()).parse("4   "), Ok(('4', "")));
    }

    #[test]
    fn parens_test() {
        assert_eq!(Arithmetic::parens().parse("(3)"),
                   Ok((Paren(Box::new(Num(to_r("3")))), "")));
        assert_eq!(Arithmetic::parens().parse("( +234/5  )"),
                   Ok((Paren(Box::new(Num(to_r("234/5")))), "")));
    }

    #[test]
    fn number_test() {
        assert_eq!(Arithmetic::number().parse("234"),
                   Ok((Num(to_r("234")), "")));
        assert_eq!(Arithmetic::number().parse("-234/567"),
                   Ok((Num(to_r("-234/567")), "")));
        assert_eq!(Arithmetic::number().parse("1/567"),
                   Ok((Num(to_r("1/567")), "")));
        assert_eq!(Arithmetic::number().parse("-234"),
                   Ok((Num(to_r("-234")), "")));
    }

    #[test]
    fn number_error_test() {
        assert_eq!(
            Arithmetic::number().parse(State::new("")),
            Err(ParseError {
                    position: SourcePosition { line: 1, column: 1 },
                    errors: vec![
                        Error::Unexpected(Info::Borrowed("end of input")),
                        Error::Expected(Info::Borrowed("digit")),
                        Error::Expected(Info::Token('/'))
                    ]
            })
        )
    }

    #[test]
    fn term_test() {
        assert_eq!(Arithmetic::term().parse("5*3"),
                   Ok((Mul(Box::new(Num(to_r("5"))), Box::new(Num(to_r("3")))), "")));
        assert_eq!(Arithmetic::term().parse("6 / 3"), 
                   Ok((Div(Box::new(Num(to_r("6"))), Box::new(Num(to_r("3")))), "")));
        assert_eq!(Arithmetic::term().parse("6  *3 /  2"),
                   Ok((Div(Box::new(Mul(Box::new(Num(to_r("6"))), Box::new(Num(to_r("3"))))),
                           Box::new(Num(to_r("2")))), "")));
        assert_eq!(Arithmetic::term().parse("9"),  Ok((Num(to_r("9")), "")));
    }

    #[test]
    #[should_panic]
    fn term_error_test() {
        assert_eq!(Arithmetic::term().parse("3/ 2"),
                   Ok((Div(Box::new(Num(to_r("3"))), Box::new(Num(to_r("2")))), "")));
    }

    #[test]
    fn expr_test() {
        assert_eq!(Arithmetic::expr().parse("5+3"),
                   Ok((Add(Box::new(Num(to_r("5"))), Box::new(Num(to_r("3")))), "")));
        assert_eq!(Arithmetic::expr().parse("5 - 3 - 2"),
                   Ok((Sub(Box::new(Sub(Box::new(Num(to_r("5"))), Box::new(Num(to_r("3"))))),
                           Box::new(Num(to_r("2")))), "")));
        assert_eq!(Arithmetic::expr().parse("5"),  Ok((Num(to_r("5")), "")));
    }

    #[test]
    fn expr_complex_test() {
        assert_eq!(Arithmetic::expr().parse("5+3 * 2"),
                   Ok((Add(Box::new(Num(to_r("5"))),
                           Box::new(Mul(Box::new(Num(to_r("3"))), Box::new(Num(to_r("2")))))), "")));
        assert_eq!(Arithmetic::expr().parse("(5+3) * 2"),
                   Ok((Mul(Box::new(Paren(
                               Box::new(Add(Box::new(Num(to_r("5"))), Box::new(Num(to_r("3")))))
                           )),
                           Box::new(Num(to_r("2")))), "")));
        assert_eq!(Arithmetic::expr().parse("+5 * (-33 - 7) + 3"),
                   Ok((Add(Box::new(Mul(
                              Box::new(Num(to_r("5"))),
                              Box::new(Paren(
                                  Box::new(Sub(Box::new(Num(to_r("-33"))), Box::new(Num(to_r("7")))))
                              ))
                           )),
                           Box::new(Num(to_r("3")))), "")));
        assert_eq!(Arithmetic::parens().parse("((234/5 + 2) * -2/3)"),
                   Ok((Paren(Box::new(Mul(
                       Box::new(Paren(Box::new(Add(
                           Box::new(Num(to_r("234/5"))),
                           Box::new(Num(to_r("2")))
                       )))),
                       Box::new(Num(to_r("-2/3")))
                   ))), "")));
    }
}
