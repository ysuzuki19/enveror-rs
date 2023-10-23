use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum Stage {
    Dev,
    Prod,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct EnverorConfig {
    stage: Stage,
    cloud: Cloud,
    cors_origins: Vec<String>,
    worker_count: u8,
    timeout_seconds: f32,
    empty_string: String,
    sample: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct Cloud {
    api_key_id: String,
    api_secret_key: String,
    storage: CloudStorage,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct CloudStorage {
    images: String,
}

extern crate enveror;

#[test]
fn parse_deserialize() -> Result<(), Box<dyn std::error::Error>> {
    enveror::Enveror::new()
        .ignore_default_config()
        .path(PathBuf::from("./tests/case_enveror"))
        .load()?
        .construct::<EnverorConfig>()?;
    Ok(())
}
