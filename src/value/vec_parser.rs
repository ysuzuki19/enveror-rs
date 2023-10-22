use crate::error::{EnverorError, EnverorResult};

use super::{validator::Validator, value_parser::ValueParser, Number};

trait StripBracket {
    fn strip_bracket(&self) -> String;
}

impl StripBracket for String {
    fn strip_bracket(&self) -> String {
        self.trim_start_matches('[')
            .trim_end_matches(']')
            .to_owned()
    }
}

trait CommaSeparate {
    fn comma_separate(&self) -> Vec<String>;
}

impl CommaSeparate for String {
    fn comma_separate(&self) -> Vec<String> {
        self.split(',').map(|s| s.trim().to_owned()).collect()
    }
}

pub(super) struct VecParser(String);

impl VecParser {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn into_value(self) -> EnverorResult<super::Value> {
        match self.estimate_vec_type() {
            "str" => Ok(super::Value::VecStr(self.into_vec_str()?)),
            "bool" => Ok(super::Value::VecBool(self.into_vec_bool()?)),
            "number" => Ok(super::Value::VecNumber(self.into_vec_number()?)),
            _ => Err(EnverorError::InvalidConfig(
                "Vec must to be same type".into(),
            )),
        }
    }

    fn estimate_vec_type(&self) -> &str {
        if self.0.contains('"') {
            "str"
        } else {
            let elements = self.0.strip_bracket().comma_separate();
            if elements.iter().all(|s| s.is_bool()) {
                return "bool";
            }
            if elements.iter().all(|s| s.is_number()) {
                return "number";
            }
            "invalid"
        }
    }

    fn into_vec_str(self) -> EnverorResult<Vec<String>> {
        //TODO
        Ok(vec![self.0])
    }

    fn into_vec_number(self) -> EnverorResult<Vec<Number>> {
        self.0
            .strip_bracket()
            .comma_separate()
            .into_iter()
            .map(|s| ValueParser::new(s).into_number())
            .collect()
    }

    fn into_vec_bool(self) -> EnverorResult<Vec<bool>> {
        self.0
            .strip_bracket()
            .comma_separate()
            .into_iter()
            .map(|s| ValueParser::new(s).into_bool())
            .collect()
    }
}
