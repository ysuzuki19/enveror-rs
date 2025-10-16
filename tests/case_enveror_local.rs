use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct EnverorConfig {
    developer: Developer,
    default: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct Developer {
    name: String,
}

#[test]
fn parse_deserialize() -> Result<(), Box<dyn std::error::Error>> {
    enveror::Enveror::new()
        .ignore_default_config(true)
        .ignore_file_notfound(true)
        .path(PathBuf::from("./tests/case_enveror_local"))
        .load()?
        .construct::<EnverorConfig>()?;
    Ok(())
}
