mod line_parser;

use std::{collections::HashMap, path::PathBuf};

use crate::error::Result;

use self::line_parser::LineParser;

#[derive(typed_builder::TypedBuilder)]
pub(super) struct Loader {
    paths: Vec<PathBuf>,

    #[builder(default = false)]
    ignore_file_notfound: bool,
}

impl Loader {
    pub fn load(self) -> Result<HashMap<String, String>> {
        let mut kvmap = HashMap::new();

        for (k, v) in std::env::vars() {
            kvmap.insert(k, v);
        }

        for path in &self.paths {
            match std::fs::read_to_string(path) {
                Ok(content) => {
                    for line in content.lines() {
                        let (k, v) = LineParser::new(line).parse()?;
                        kvmap.insert(k, v);
                    }
                }
                Err(e) => {
                    if !self.ignore_file_notfound || e.kind() != std::io::ErrorKind::NotFound {
                        return Err(e.into());
                    }
                }
            }
        }

        Ok(kvmap)
    }
}
