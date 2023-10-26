mod line_parser;

use std::{collections::HashMap, path::PathBuf};

use crate::error::EnverorResult;

use self::line_parser::LineParser;

#[derive(typed_builder::TypedBuilder)]
pub struct Loader {
    #[builder(default = vec![PathBuf::from(".enveror")])]
    paths: Vec<PathBuf>,

    #[builder(setter(skip), default)]
    kvmap: HashMap<String, String>,
}

impl Loader {
    pub fn load(&mut self) -> EnverorResult<()> {
        for (k, v) in std::env::vars() {
            self.kvmap.insert(k, v);
        }

        for path in &self.paths {
            let content = std::fs::read_to_string(path)?;
            for line in content.lines() {
                let (k, v) = LineParser::new(line).parse()?;
                self.kvmap.insert(k, v);
            }
        }

        Ok(())
    }

    pub fn values(self) -> HashMap<String, String> {
        self.kvmap
    }
}
