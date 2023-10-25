pub(super) trait StringChecker {
    fn is_vec(&self) -> bool;
    fn is_bool(&self) -> bool;
    fn is_number(&self) -> bool;
}

impl StringChecker for String {
    fn is_vec(&self) -> bool {
        self.starts_with('[') && self.ends_with(']')
    }

    fn is_bool(&self) -> bool {
        self == "true" || self == "false"
    }

    fn is_number(&self) -> bool {
        self.parse::<f64>().is_ok() || self.parse::<i128>().is_ok() || self.parse::<u128>().is_ok()
    }
}
