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
        let vp = ValueParser::new(s.to_string());

        if vp.is_vec() {
            let vp = VecParser::new(s.to_owned());
            match vp.vec_type() {
                "str" => return Ok(Self::VecStr(vp.into_vec_str()?)),
                "number" => return Ok(Self::VecNumber(vp.into_vec_number()?)),
                "bool" => return Ok(Self::VecBool(vp.into_vec_bool()?)),
                _ => return Err(EnverorError::InvalidConfig("vec".into())),
            }
        }
        if vp.is_bool() {
            return Ok(Self::Bool(vp.into_bool()?));
        }
        if vp.is_number() {
            return Ok(Self::Number(vp.into_number()?));
        }

        Ok(Value::Str(s.to_string()))
    }
}
