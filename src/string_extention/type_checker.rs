pub(super) trait TypeChecker {
    fn is_vec(&self) -> bool;
    fn is_bool(&self) -> bool;
    fn is_number(&self) -> bool;
}

impl<S: AsRef<str>> TypeChecker for S {
    fn is_vec(&self) -> bool {
        self.as_ref().starts_with('[') && self.as_ref().ends_with(']')
    }

    fn is_bool(&self) -> bool {
        self.as_ref() == "true" || self.as_ref() == "false"
    }

    fn is_number(&self) -> bool {
        self.as_ref().parse::<f64>().is_ok()
            || self.as_ref().parse::<i128>().is_ok()
            || self.as_ref().parse::<u128>().is_ok()
    }
}
