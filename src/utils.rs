#![cfg(test)]

use num::rational::{ Ratio, BigRational };
use std::str::FromStr;

pub fn to_r(s: &str) -> BigRational {
    Ratio::from_str(s).unwrap()
}
