mod validator;
mod value_parser;
mod vec_parser;

use std::str::FromStr;

use crate::error::EnverorError;

use self::{value_parser::ValueParser, vec_parser::VecParser};

pub type Number = f64;

#[derive(Debug, Clone)]
pub enum Value {
    VecBool(Vec<bool>),
    VecNumber(Vec<Number>),
    VecStr(Vec<String>),
    Bool(bool),
    Number(Number),
    Str(String),
}

impl FromStr for Value {
    type Err = EnverorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ValueParser::new(s.to_string()).into_value()
    }
}
