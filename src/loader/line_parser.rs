use crate::error::{Error, Result};

pub(super) struct LineParser<'a> {
    line: &'a str,
}

impl<'a> LineParser<'a> {
    pub(super) fn new(line: &'a str) -> Self {
        Self { line }
    }

    pub(super) fn parse(self) -> Result<(String, String)> {
        let parts = self.line.splitn(2, '=').collect::<Vec<_>>();

        if parts.len() != 2 {
            Err(Error::InvalidConfig("must to be include '='".into()))
        } else {
            let lfs = parts[0].trim().to_owned();
            let rfs = parts[1].trim().to_owned();
            Ok((lfs, rfs))
        }
    }
}
