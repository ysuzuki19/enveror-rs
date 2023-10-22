use crate::error::{EnverorError, EnverorResult};

pub struct LineParser {
    line: String,
}

impl LineParser {
    pub fn new(line: String) -> Self {
        Self { line }
    }

    pub fn parse(self) -> EnverorResult<(String, String)> {
        let parts = self.line.splitn(2, '=').collect::<Vec<_>>();

        if parts.len() != 2 {
            return Err(EnverorError::Custom(format!("Invalid line: {}", self.line)));
        }

        let lfs = parts[0].trim();
        let rfs = parts[1].trim();

        Ok((lfs.to_owned(), rfs.to_owned()))
    }
}
