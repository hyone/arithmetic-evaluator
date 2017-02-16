extern crate combine;

use combine::*;
use combine::char::{ Spaces, digit, spaces };
use combine::combinator::{ Skip };

pub fn lex<'a, P>(p: P) -> Skip<P, Spaces<P::Input>>
  where P: Parser,
        P::Input: Stream<Item=char>
{
    p.skip(spaces())    
}

pub fn parens<I>(input: I) -> ParseResult<f64, I>
  where I: Stream<Item=char>
{
    between(lex(token('(')), token(')'), lex(parser(expr)))
        .parse_stream(input)
}

pub fn number<I>(input: I) -> ParseResult<f64, I>
  where I: Stream<Item=char>
{
    let sign = token('+').or(token('-'));

    let number = many1::<String, _>(digit().or(token('.')))
        .and_then(|digits| digits.parse::<f64>());

    (optional(sign), number)
        .map(|(prefix, num)| {
            match prefix {
                None | Some('+') => num,
                Some('-')        => -num,
                _                => unreachable!(),
            }
        })
        .parse_stream(input)
}

pub fn factor<I>(input: I) -> ParseResult<f64, I>
  where I: Stream<Item=char>
{
    parser(number).or(parser(parens)).parse_stream(input)
}

pub fn term<I>(input: I) -> ParseResult<f64, I>
  where I: Stream<Item=char>
{
    let operator = lex(one_of("*/".chars()))
        .map(|op| move |lhs, rhs| {
            match op {
                '*' => lhs * rhs,
                '/' => lhs / rhs,
                _   => unreachable!()
            }
        });

    chainl1(lex(parser(factor)), operator).parse_stream(input)
}

pub fn expr<I>(input: I) -> ParseResult<f64, I>
  where I: Stream<Item=char>
{
    let operator = lex(one_of("+-".chars()))
        .map(|op| move |lhs, rhs| {
            match op {
                '+' => lhs + rhs,
                '-' => lhs - rhs,
                _   => unreachable!()
            }
        });

    chainl1(lex(parser(term)), operator).parse_stream(input)
}

pub fn expr_parser<I>(input: I) -> ParseResult<f64, I>
  where I: Stream<Item=char>
{
    optional(spaces()).with(parser(expr)).parse_stream(input)
}

fn main() {
    println!("Hello, world!");
}

mod tests {
    use combine::*;
    use combine::primitives::{ Error, Info, ParseError, SourcePosition };
    use super::*;

    #[test]
    fn lex_test() {
        assert_eq!(digit().parse("4   "), Ok(('4', "   ")));
        assert_eq!(lex(digit()).parse("4   "), Ok(('4', "")));
    }

    #[test]
    fn parens_test() {
        assert_eq!(parser(parens).parse("(3)"), Ok((3., "")));
        assert_eq!(parser(parens).parse("(234.5)"), Ok((234.5, "")));
        assert_eq!(parser(parens).parse("( +3)"), Ok((3., "")));
        assert_eq!(parser(parens).parse("(  234.5 + 2.0    )"), Ok((236.5, "")));
        assert_eq!(parser(parens).parse("((234.5 + 2.0) * 2.0)"), Ok((473., "")));
    }

    #[test]
    fn number_success_test() {
        assert_eq!(parser(number).parse("234"),  Ok((234., "")));
        assert_eq!(parser(number).parse("-234.567"), Ok((-234.567, "")));
        assert_eq!(parser(number).parse(".567"), Ok((0.567, "")));
        assert_eq!(parser(number).parse("-234."), Ok((-234., "")));
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
        assert_eq!(parser(term).parse("5*3"),  Ok((15., "")));
        assert_eq!(parser(term).parse("6 / 3"),  Ok((2., "")));
        assert_eq!(parser(term).parse("6  * 3/  2"),  Ok((9., "")));
        assert_eq!(parser(term).parse("9"),  Ok((9., "")));
    }

    #[test]
    fn expr_test() {
        assert_eq!(parser(expr).parse("5+3"),  Ok((8., "")));
        assert_eq!(parser(expr).parse("5 - 3 - 2"),  Ok((0., "")));
        assert_eq!(parser(expr).parse("5"),  Ok((5., "")));
    }

    #[test]
    fn expr_test2() {
        assert_eq!(parser(expr).parse("5+3 * 2"),  Ok((11., "")));
        assert_eq!(parser(expr).parse("(5+3) * 2"),  Ok((16., "")));
        assert_eq!(parser(expr).parse("+5 * (-33 - 7) + 3"),  Ok((-197., "")));
    }
}
