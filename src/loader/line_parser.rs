use crate::error::{Error, Result};

pub(super) struct LineParser<'a> {
    line: &'a str,
}

impl<'a> LineParser<'a> {
    pub(super) fn new(line: &'a str) -> Self {
        Self { line }
    }

    fn is_empty(&self) -> bool {
        self.line.is_empty()
    }

    fn is_comment(&self) -> bool {
        self.line.starts_with('#')
    }

    pub(super) fn parse(self) -> Result<Option<(String, String)>> {
        if self.is_empty() || self.is_comment() {
            return Ok(None);
        }

        let parts = self.line.splitn(2, '=').collect::<Vec<_>>();

        if parts.len() != 2 {
            Err(Error::InvalidConfig("must to be include '='".into()))
        } else {
            let lfs = parts[0].trim().to_owned();
            let rfs = parts[1].trim().to_owned();
            Ok(Some((lfs, rfs)))
        }
    }
}
