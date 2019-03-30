use combine::{
    combinator::Skip,
    error::{ ParseError, StreamError },
    parser::char::{ Spaces, digit, spaces },
    stream::StreamErrorFor,
    Parser, Stream,
    between, chainl1, many1, one_of, optional, token
};
use num::rational::Ratio;
use std::str::FromStr;
use types::*;

fn lex<I, P>(p: P) -> Skip<P, Spaces<P::Input>>
    where P: Parser<Input = I>,
          I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    p.skip(spaces())
}

fn parens<I>() -> impl Parser<Input = I, Output = Expr>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    between(lex(token('(')), token(')'), lex(expr()))
        .map(|e| Expr::Paren(Box::new(e)))
}

fn number<I>() -> impl Parser<Input = I, Output = Expr>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    let sign = token('+').or(token('-'));

    let number = many1::<String, _>(digit().or(token('/')))
        .and_then(|digits| {
            Ratio::from_str(digits.as_str())
                .map_err(StreamErrorFor::<I>::other)
        });

    (optional(sign), number)
        .map(|(prefix, num)| {
            match prefix {
                None | Some('+') => Expr::Num(num),
                Some('-')        => Expr::Num(-num),
                _                => unreachable!(),
            }
        })
}

fn factor<I>() -> impl Parser<Input = I, Output = Expr>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    number().or(parens())
}

fn term<I>() -> impl Parser<Input = I, Output = Expr>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    let operator = one_of("*/".chars())
        .map(|op| move |lhs, rhs| {
            match op {
                '*' => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                '/' => Expr::Div(Box::new(lhs), Box::new(rhs)),
                _   => unreachable!()
            }
        });

    chainl1(lex(factor()), lex(operator))
}

fn expr<I>() -> impl Parser<Input = I, Output = Expr>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    expr_()
}

// need to use `parser!` to break the recursive use of `expr`
// (avoid infinitely recursive impl Trait type)
// see: https://github.com/rust-lang/rust/issues/47659
parser!{
    fn expr_[I]()(I) -> Expr
        where [ I: Stream<Item = char> ]
    {
        let operator = one_of("+-".chars())
            .map(|op| move |lhs, rhs| {
                match op {
                    '+' => Expr::Add(Box::new(lhs), Box::new(rhs)),
                    '-' => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                    _   => unreachable!()
                }
            });

        chainl1(lex(term()), lex(operator))
    }
}

pub fn parser<I>() -> impl Parser<Input = I, Output = Expr>
    where I: Stream<Item = char>,
          I::Error: ParseError<I::Item, I::Range, I::Position>
{
    optional(spaces()).with(expr())
}

#[cfg(test)]
mod tests {
    use combine::{
        error::StringStreamError,
        stream::state::State,
    };
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
        assert_eq!(parens().parse("(3)"),
                   Ok((Paren(Box::new(Num(to_r("3")))), "")));
        assert_eq!(parens().parse("( +234/5  )"),
                   Ok((Paren(Box::new(Num(to_r("234/5")))), "")));
    }

    #[test]
    fn number_test() {
        assert_eq!(number().parse("234"),
                   Ok((Num(to_r("234")), "")));
        assert_eq!(number().parse("-234/567"),
                   Ok((Num(to_r("-234/567")), "")));
        assert_eq!(number().parse("1/567"),
                   Ok((Num(to_r("1/567")), "")));
        assert_eq!(number().parse("-234"),
                   Ok((Num(to_r("-234")), "")));
    }

    #[test]
    fn number_error_test() {
        assert_eq!(number().parse(State::new("")),
                   Err(StringStreamError::Eoi))
    }

    #[test]
    fn term_test() {
        assert_eq!(term().parse("5*3"),
                   Ok((Mul(Box::new(Num(to_r("5"))), Box::new(Num(to_r("3")))), "")));
        assert_eq!(term().parse("6 / 3"), 
                   Ok((Div(Box::new(Num(to_r("6"))), Box::new(Num(to_r("3")))), "")));
        assert_eq!(term().parse("6  *3 /  2"),
                   Ok((Div(Box::new(Mul(Box::new(Num(to_r("6"))), Box::new(Num(to_r("3"))))),
                           Box::new(Num(to_r("2")))), "")));
        assert_eq!(term().parse("9"),  Ok((Num(to_r("9")), "")));
    }

    #[test]
    #[should_panic]
    fn term_error_test() {
        assert_eq!(term().parse("3/ 2"),
                   Ok((Div(Box::new(Num(to_r("3"))), Box::new(Num(to_r("2")))), "")));
    }

    #[test]
    fn expr_test() {
        assert_eq!(expr().parse("5+3"),
                   Ok((Add(Box::new(Num(to_r("5"))), Box::new(Num(to_r("3")))), "")));
        assert_eq!(expr().parse("5 - 3 - 2"),
                   Ok((Sub(Box::new(Sub(Box::new(Num(to_r("5"))), Box::new(Num(to_r("3"))))),
                           Box::new(Num(to_r("2")))), "")));
        assert_eq!(expr().parse("5"),  Ok((Num(to_r("5")), "")));
    }

    #[test]
    fn expr_complex_test() {
        assert_eq!(expr().parse("5+3 * 2"),
                   Ok((Add(Box::new(Num(to_r("5"))),
                           Box::new(Mul(Box::new(Num(to_r("3"))), Box::new(Num(to_r("2")))))), "")));
        assert_eq!(expr().parse("(5+3) * 2"),
                   Ok((Mul(Box::new(Paren(
                               Box::new(Add(Box::new(Num(to_r("5"))), Box::new(Num(to_r("3")))))
                           )),
                           Box::new(Num(to_r("2")))), "")));
        assert_eq!(expr().parse("+5 * (-33 - 7) + 3"),
                   Ok((Add(Box::new(Mul(
                              Box::new(Num(to_r("5"))),
                              Box::new(Paren(
                                  Box::new(Sub(Box::new(Num(to_r("-33"))), Box::new(Num(to_r("7")))))
                              ))
                           )),
                           Box::new(Num(to_r("3")))), "")));
        assert_eq!(parens().parse("((234/5 + 2) * -2/3)"),
                   Ok((Paren(Box::new(Mul(
                       Box::new(Paren(Box::new(Add(
                           Box::new(Num(to_r("234/5"))),
                           Box::new(Num(to_r("2")))
                       )))),
                       Box::new(Num(to_r("-2/3")))
                   ))), "")));
    }
}
