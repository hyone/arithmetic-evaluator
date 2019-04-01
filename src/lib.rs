#[macro_use]
extern crate combine;
extern crate num;

mod eval;
mod parser;
mod types;
mod utils;

pub use eval::eval;
pub use parser::parser;
pub use types::Expr;
