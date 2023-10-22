use crate::error::{EnverorError, EnverorResult};

use super::{validator::Validator, Number};

pub(super) struct ValueParser(String);

impl ValueParser {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn into_value(self) -> EnverorResult<super::Value> {
        if self.0.is_vec() {
            super::VecParser::new(self.0).into_value()
        } else if self.0.is_bool() {
            Ok(super::Value::Bool(self.into_bool()?))
        } else if self.0.is_number() {
            Ok(super::Value::Number(self.into_number()?))
        } else {
            Ok(super::Value::Str(self.0))
        }
    }

    pub fn into_bool(self) -> EnverorResult<bool> {
        match self.0.as_str() {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(EnverorError::Custom(format!(
                "Invalid bool value: {}",
                self.0
            ))),
        }
    }

    pub fn into_number(self) -> EnverorResult<Number> {
        self.0
            .parse::<Number>()
            .map_err(|_| EnverorError::Custom(format!("Invalid number value: {}", self.0)))
    }
}
