use std::path::PathBuf;

use error::EnverorResult;
use into_json::IntoJson;
use loader::Loader;

mod error;
mod into_json;
mod loader;
mod tree;
mod value;

#[derive(Debug, Default)]
pub struct Enveror {
    default_config_path: PathBuf,
    ignore_default_config: bool,
    config_paths: Option<Vec<PathBuf>>,
    // must_load_config: bool,
    loaded_json: Option<String>,
}

impl Enveror {
    pub fn new() -> Self {
        Self {
            default_config_path: PathBuf::from(".enveror"),
            ignore_default_config: false,
            config_paths: None,
            // must_load_config: false,
            loaded_json: None,
        }
    }

    pub fn ignore_default_config(mut self) -> Self {
        self.ignore_default_config = true;
        self
    }

    // pub fn must_load_config(mut self) -> Self {
    //     self.must_load_config = true;
    //     self
    // }

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

    pub fn load(mut self) -> EnverorResult<Self> {
        let mut paths = Vec::new();
        if !self.ignore_default_config {
            paths.push(self.default_config_path.clone());
        }
        if let Some(config_paths) = self.config_paths.take() {
            paths.extend(config_paths);
        }

        let mut loader = Loader::builder().paths(paths).build();
        loader.load()?;
        let values = loader.values();

        let mut tree = tree::Tree::new();
        for (key, value) in values {
            tree.insert(key.to_owned(), value.to_owned())?
        }

        self.loaded_json = Some(tree.into_json());

        Ok(self)
    }

    pub fn construct<T>(&self) -> EnverorResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        let json = self
            .loaded_json
            .clone()
            .ok_or(error::EnverorError::Custom("configs not loaded".into()))?;
        let deserialized = serde_json::from_str(&json)?;
        Ok(deserialized)
    }
}