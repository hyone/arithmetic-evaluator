extern crate combine;

use combine::*;
use combine::char::{ digit };
use combine::primitives::{ Consumed };

pub fn parens<I>(input: I) -> ParseResult<f64, I>
  where I: Stream<Item=char> {
    between(token('('), token(')'), parser(factor))
        .parse_stream(input)
  }

pub fn factor<I>(input: I) -> ParseResult<f64, I>
  where I: Stream<Item=char> {
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

// pub fn term<I>(input: I) -> ParseResult<f64, I>
//   where I: Stream<Item=char> {
//     try(
//         (parser(factor), token('*').or(token('/')), parser(term))
//         .map(|(lhs, op, rhs)| {
//             match op {
//                 '*' => lhs * rhs,
//                 '/' => lhs / rhs,
//                 _   => unreachable!(),
//             }
//         })
//     ).or(parser(factor))
//     .parse_stream(input)
//   }

pub fn term<I>(input: I) -> ParseResult<f64, I>
  where I: Stream<Item=char> {
    parser(factor)
        .then(|lhs| {
            (one_of("*/".chars()), parser(term))
                .map(move |(op, rhs)| {
                    match op {
                        '*' => lhs * rhs,
                        '/' => lhs / rhs,
                        _   => unreachable!(),
                    }
                })
                .or(parser(move |i: I| {
                    Ok((lhs, Consumed::Empty(i)))
                }))
        })
        .parse_stream(input)
  }

fn main() {
    println!("Hello, world!");
}

mod tests {
    use combine::*;
    use combine::primitives::{ Error, Info, ParseError, SourcePosition };
    use super::*;

    #[test]
    fn parens_test() {
        assert_eq!(parser(parens).parse("(3)"), Ok((3., "")));
        assert_eq!(parser(parens).parse("(+234.5)"), Ok((234.5, "")));
    }

    #[test]
    fn factor_success_test() {
        assert_eq!(parser(factor).parse("234"),  Ok((234., "")));
        assert_eq!(parser(factor).parse("+234"), Ok((234., "")));
        assert_eq!(parser(factor).parse("-234.567"), Ok((-234.567, "")));
        assert_eq!(parser(factor).parse("+.567"), Ok((0.567, "")));
        assert_eq!(parser(factor).parse("-234."), Ok((-234., "")));
    }

    #[test]
    fn factor_error_test() {
        assert_eq!(
            parser(factor).parse(State::new("")), 
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
    fn term_success_test() {
        assert_eq!(parser(term).parse("5*3"),  Ok((15., "")));
        // assert_eq!(parser(term).parse("5 * 3"),  Ok((15., "")));
        assert_eq!(parser(term).parse("5"),  Ok((5., "")));
    }
}
