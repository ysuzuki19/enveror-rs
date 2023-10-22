use crate::error::{EnverorError, EnverorResult};

use super::Number;

pub(super) struct ValueParser(String);

impl ValueParser {
    pub fn new(s: String) -> Self {
        Self(s)
    }

    pub fn is_vec(&self) -> bool {
        self.0.starts_with('[') && self.0.ends_with(']')
    }

    pub fn is_bool(&self) -> bool {
        self.0 == "true" || self.0 == "false"
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

    pub fn is_number(&self) -> bool {
        self.0.parse::<Number>().is_ok()
    }

    pub fn into_number(self) -> EnverorResult<Number> {
        self.0
            .parse::<Number>()
            .map_err(|_| EnverorError::Custom(format!("Invalid number value: {}", self.0)))
    }
}
