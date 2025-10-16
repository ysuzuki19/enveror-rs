use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct EnverorConfig {
    name: String,
}

#[test]
fn parse_deserialize() -> Result<(), Box<dyn std::error::Error>> {
    enveror::Enveror::new()
        .ignore_default_config(true)
        .path(PathBuf::from("./tests/case_enveror_empty_line"))
        .load()?
        .construct::<EnverorConfig>()?;
    Ok(())
}
