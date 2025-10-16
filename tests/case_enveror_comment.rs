use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct EnverorConfig {
    name: String,
    sentence: String,
}

#[test]
fn parse_deserialize() -> Result<(), Box<dyn std::error::Error>> {
    let c = enveror::Enveror::new()
        .ignore_default_config(true)
        .path(PathBuf::from("./tests/case_enveror_comment"))
        .load()?
        .construct::<EnverorConfig>()?;
    assert_eq!(c.sentence, "# HOGE\\nhoge desu");
    Ok(())
}
