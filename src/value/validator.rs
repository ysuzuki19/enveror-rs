use super::Number;

pub(super) trait Validator {
    fn is_vec(&self) -> bool;
    fn is_bool(&self) -> bool;
    fn is_number(&self) -> bool;
}

impl Validator for String {
    fn is_vec(&self) -> bool {
        self.starts_with('[') && self.ends_with(']')
    }

    fn is_bool(&self) -> bool {
        self == "true" || self == "false"
    }

    fn is_number(&self) -> bool {
        self.parse::<Number>().is_ok()
    }
}
