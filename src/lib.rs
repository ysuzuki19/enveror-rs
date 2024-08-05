mod error;
mod into_json;
mod loader;
mod tree;

use std::path::PathBuf;

pub use error::Error;
use error::Result;
use into_json::IntoJson;
use loader::Loader;

fn default_config_path() -> PathBuf {
    PathBuf::from(".enveror")
}

#[derive(Debug, Default)]
pub struct Enveror {
    ignore_default_config: bool,
    config_paths: Option<Vec<PathBuf>>,
    ignore_file_notfound: bool,
    loaded_json: Option<String>,
}

impl Enveror {
    pub fn new() -> Self {
        Self {
            ignore_default_config: false,
            config_paths: None,
            ignore_file_notfound: false,
            loaded_json: None,
        }
    }

    pub fn ignore_default_config(mut self, flag: bool) -> Self {
        self.ignore_default_config = flag;
        self
    }

    pub fn ignore_file_notfound(mut self, flag: bool) -> Self {
        self.ignore_file_notfound = flag;
        self
    }

    pub fn path(mut self, config_path: PathBuf) -> Self {
        match self.config_paths {
            Some(ref mut config_paths) => {
                config_paths.push(config_path);
            }
            None => {
                self.config_paths = Some(vec![config_path]);
            }
        }
        self
    }

    pub fn paths(mut self, config_paths: Vec<PathBuf>) -> Self {
        for config_path in config_paths {
            self = self.path(config_path);
        }
        self
    }

    pub fn load(mut self) -> Result<Self> {
        let mut paths = Vec::new();
        if !self.ignore_default_config {
            paths.push(default_config_path());
        }
        if let Some(config_paths) = self.config_paths.take() {
            paths.extend(config_paths);
        }

        let entries = Loader::builder()
            .paths(paths)
            .ignore_file_notfound(self.ignore_file_notfound)
            .build()
            .load()?;

        let mut tree = tree::Tree::new();
        for (k, v) in entries {
            tree.insert(k, v)?
        }

        self.loaded_json = Some(tree.into_json());

        Ok(self)
    }

    pub fn construct<T>(self) -> Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let json = self
            .loaded_json
            .ok_or(error::Error::Custom("configs not loaded".into()))?;
        let deserialized = serde_json::from_str(&json)?;
        Ok(deserialized)
    }
}
