use std::path::PathBuf;

use serde::{Deserialize, Serialize};

fn default_timeout() -> u32 {
    3
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct EnverorConfig {
    programatic_bool: bool,
    programatic_int: i32,
    programatic_str: String,
    programatic_arr: Vec<i32>,
    #[serde(default = "default_timeout")]
    timeout: u32,
}

extern crate enveror;

#[test]
fn parse_deserialize() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("PROGRAMATIC_BOOL", "true");
    std::env::set_var("PROGRAMATIC_INT", "1");
    std::env::set_var("PROGRAMATIC_STR", "SETTED");
    std::env::set_var("PROGRAMATIC_ARR", "[1,2,3]");

    enveror::Enveror::new()
        .ignore_default_config()
        .path(PathBuf::from("./tests/case_enveror_local"))
        .load()?
        .construct::<EnverorConfig>()?;
    Ok(())
}
