use combine::*;
use combine::char::{ Spaces, digit, spaces };
use combine::combinator::{ FnParser, Skip };
use types::*;

pub fn lex<'a, P>(p: P) -> Skip<P, Spaces<P::Input>>
  where P: Parser,
        P::Input: Stream<Item=char>
{
    p.skip(spaces())
}

pub fn parens<I>(input: I) -> ParseResult<Expr, I>
  where I: Stream<Item=char>
{
    between(lex(token('(')), token(')'), lex(parser(expr)))
        .map(|e| Expr::Paren(Box::new(e)))
        .parse_stream(input)
}

pub fn number<I>(input: I) -> ParseResult<Expr, I>
  where I: Stream<Item=char>
{
    let sign = token('+').or(token('-'));

    let number = many1::<String, _>(digit().or(token('.')))
        .and_then(|digits| digits.parse::<f64>());

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

pub fn factor<I>(input: I) -> ParseResult<Expr, I>
  where I: Stream<Item=char>
{
    parser(number).or(parser(parens)).parse_stream(input)
}

pub fn term<I>(input: I) -> ParseResult<Expr, I>
  where I: Stream<Item=char>
{
    let operator = lex(one_of("*/".chars()))
        .map(|op| move |lhs, rhs| {
            match op {
                '*' => Expr::Mul(Box::new(lhs), Box::new(rhs)),
                '/' => Expr::Div(Box::new(lhs), Box::new(rhs)),
                _   => unreachable!()
            }
        });

    chainl1(lex(parser(factor)), operator).parse_stream(input)
}

pub fn expr<I>(input: I) -> ParseResult<Expr, I>
  where I: Stream<Item=char>
{
    let operator = lex(one_of("+-".chars()))
        .map(|op| move |lhs, rhs| {
            match op {
                '+' => Expr::Add(Box::new(lhs), Box::new(rhs)),
                '-' => Expr::Sub(Box::new(lhs), Box::new(rhs)),
                _   => unreachable!()
            }
        });

    chainl1(lex(parser(term)), operator).parse_stream(input)
}


pub fn expr_parser<I>() -> FnParser<I, fn(I) -> ParseResult<Expr, I>>
  where I: Stream<Item=char>
{
    fn expr_parser_<I>(input: I) -> ParseResult<Expr, I>
      where I: Stream<Item=char>
    {
        optional(spaces()).with(parser(expr)).parse_stream(input)
    }
    parser(expr_parser_)
}


#[cfg(test)]
mod tests {
    use combine::*;
    use combine::primitives::{ Error, Info, ParseError, SourcePosition };
    use types::Expr::*;
    use super::*;

    #[test]
    fn lex_test() {
        assert_eq!(digit().parse("4   "), Ok(('4', "   ")));
        assert_eq!(lex(digit()).parse("4   "), Ok(('4', "")));
    }

    #[test]
    fn parens_test() {
        assert_eq!(parser(parens).parse("(3)"),
                   Ok((Paren(Box::new(Num(3.))), "")));
        assert_eq!(parser(parens).parse("( +234.5  )"),
                   Ok((Paren(Box::new(Num(234.5))), "")));
    }

    #[test]
    fn number_test() {
        assert_eq!(parser(number).parse("234"),
                   Ok((Num(234.), "")));
        assert_eq!(parser(number).parse("-234.567"),
                   Ok((Num(-234.567), "")));
        assert_eq!(parser(number).parse(".567"),
                   Ok((Num(0.567), "")));
        assert_eq!(parser(number).parse("-234."),
                   Ok((Num(-234.), "")));
    }

    #[test]
    fn number_error_test() {
        assert_eq!(
            parser(number).parse(State::new("")),
            Err(ParseError {
                position: SourcePosition { line: 1, column: 1 },
                errors: vec![
                    Error::Unexpected(Info::Borrowed("end of input")),
                    Error::Expected(Info::Borrowed("digit")),
                    Error::Expected(Info::Token('.'))
                ] })
        )
    }

    #[test]
    fn term_test() {
        assert_eq!(parser(term).parse("5*3"),
                   Ok((Mul(Box::new(Num(5.)), Box::new(Num(3.))), "")));
        assert_eq!(parser(term).parse("6 / 3"), 
                   Ok((Div(Box::new(Num(6.)), Box::new(Num(3.))), "")));
        assert_eq!(parser(term).parse("6  * 3/  2"),
                   Ok((Div(Box::new(Mul(Box::new(Num(6.)), Box::new(Num(3.)))),
                           Box::new(Num(2.))), "")));
        assert_eq!(parser(term).parse("9"),  Ok((Num(9.), "")));
    }

    #[test]
    fn expr_test() {
        assert_eq!(parser(expr).parse("5+3"),
                   Ok((Add(Box::new(Num(5.)), Box::new(Num(3.))), "")));
        assert_eq!(parser(expr).parse("5 - 3 - 2"),
                   Ok((Sub(Box::new(Sub(Box::new(Num(5.)), Box::new(Num(3.)))),
                           Box::new(Num(2.))), "")));
        assert_eq!(parser(expr).parse("5"),  Ok((Num(5.), "")));
    }

    #[test]
    fn expr_complex_test() {
        assert_eq!(parser(expr).parse("5+3 * 2"),
                   Ok((Add(Box::new(Num(5.)),
                           Box::new(Mul(Box::new(Num(3.)), Box::new(Num(2.))))), "")));
        assert_eq!(parser(expr).parse("(5+3) * 2"),
                   Ok((Mul(Box::new(Paren(
                               Box::new(Add(Box::new(Num(5.)), Box::new(Num(3.))))
                           )),
                           Box::new(Num(2.))), "")));
        assert_eq!(parser(expr).parse("+5 * (-33 - 7) + 3"),
                   Ok((Add(Box::new(Mul(
                              Box::new(Num(5.)),
                              Box::new(Paren(
                                  Box::new(Sub(Box::new(Num(-33.)), Box::new(Num(7.))))
                              ))
                           )),
                           Box::new(Num(3.))), "")));
        assert_eq!(parser(parens).parse("((234.5 + 2.0) * 2.0)"),
                   Ok((Paren(Box::new(Mul(
                       Box::new(Paren(Box::new(Add(
                           Box::new(Num(234.5)),
                           Box::new(Num(2.0))
                       )))),
                       Box::new(Num(2.0))
                   ))), "")));
    }
}
