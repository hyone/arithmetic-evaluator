extern crate combine;
extern crate docopt;
extern crate num;
extern crate rustc_serialize;

mod eval;
mod parser;
mod types;
mod utils;

use combine::Parser;
use docopt::Docopt;
use std::io;
use std::io::prelude::*;
use std::process;
use std::str;
use eval::eval;
use parser::expr_parser;

const USAGE: &'static str = "
Usage:
  arithmetic-evaluator [options]
 
Options:
  -e TEXT      Specify expression to be evaludated
  -h, --help   Print this message
 
Parse and evaluate simple arithmetic text.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_e: Option<String>,
    arg_file: Vec<String>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let input: String =
        if let Some(text) = args.flag_e {
            text
        } else {
            let mut text = String::new();
            io::stdin().read_to_string(&mut text).unwrap();
            text
        };

    match expr_parser().parse(input.as_str()) {
        Ok((expr, _)) => {
            println!("{} = {}", format!("{}", expr), eval(expr));
            process::exit(0);
        },
        Err(err) => {
            println!("Error: {}", err);
            process::exit(1)
        }
    }
}
