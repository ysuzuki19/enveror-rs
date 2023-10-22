use crate::error::{EnverorError, EnverorResult};

use super::{value_parser::ValueParser, Number};

pub(super) struct VecParser(String);

impl VecParser {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn is_vec(&self) -> bool {
        self.0.starts_with('[') && self.0.ends_with(']')
    }

    fn strip_bracket(&self) -> &str {
        self.0.trim_start_matches('[').trim_end_matches(']')
    }

    pub fn vec_type(&self) -> &str {
        if self.0.contains('"') {
            "str"
        } else {
            let elements = self.0.split(',').map(|s| s.trim()).collect::<Vec<_>>();
            let is_number = elements
                .iter()
                .all(|&s| ValueParser::new(s.to_owned()).is_number());
            let is_bool = elements
                .iter()
                .all(|&s| ValueParser::new(s.to_owned()).is_bool());
            if is_number {
                return "number";
            }
            if is_bool {
                return "bool";
            }
            "invalid"
        }
    }

    pub fn into_vec_str(self) -> EnverorResult<Vec<String>> {
        //TODO
        Ok(vec![self.0])
    }

    pub fn into_vec_number(self) -> EnverorResult<Vec<Number>> {
        let elements = self.0.split(',').map(|s| s.trim()).collect::<Vec<_>>();
        let is_number = elements
            .iter()
            .all(|&s| ValueParser::new(s.to_owned()).is_number());
        if is_number {
            return Ok(elements
                .iter()
                .map(|&s| ValueParser::new(s.to_owned()).into_number().unwrap())
                .collect::<Vec<_>>());
        }
        Err(EnverorError::InvalidConfig(
            "Vec must to be same type".into(),
        ))
    }

    pub fn into_vec_bool(self) -> EnverorResult<Vec<bool>> {
        let elements = self.0.split(',').map(|s| s.trim()).collect::<Vec<_>>();
        let is_bool = elements
            .iter()
            .all(|&s| ValueParser::new(s.to_owned()).is_bool());
        if is_bool {
            return Ok(elements
                .iter()
                .map(|&s| ValueParser::new(s.to_owned()).into_bool().unwrap())
                .collect::<Vec<_>>());
        }
        Err(EnverorError::InvalidConfig(
            "Vec must to be same type".into(),
        ))
    }
}
