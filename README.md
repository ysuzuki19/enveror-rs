# enveror

[<img alt="github" src="https://img.shields.io/badge/github-ysuzuki19/enveror-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/ysuzuki19/enveror)
[<img alt="crates.io" src="https://img.shields.io/crates/v/enveror.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/enveror)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-enveror-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/enveror)

Library for Structured Environment Variables

# Feature

- represent the structure of env-vals using `.`
- define the structure of environment variables using `serde` in `enveror`
- each variable (without object) is fully defined within a single line

# Sample

## config

If you want to define following structure,

```json
{
  "STAGE": "dev",
  "CLOUD": {
    "API_KEY_ID": "hogehoge=hog",
    "API_SECRET_KEY": "fug+;l[l;uw:er-0-63-096z,nxvcafuga",
    "STORAGE": {
      "IMAGES": "myimages"
    }
  },
  "CORS_ORIGINS": ["http://localhost:3000", "", "https://enveror.example.com"],
  "WORKER_COUNT": 4,
  "TIMEOUT_SECONDS": 2.3,
  "EMPTY_STRING": " ",
  "SAMPLE": true,
  "CONFIG": {
    "FLAGS": [true, false, false, true],
    "NUMBERS": [1, 2, 3, 4, 5, 6, 7, 8, 9]
  }
}
```

you can set variable as following environment variables.

```
STAGE = "dev"
CLOUD.API_KEY_ID = "hogehoge=hog"
CLOUD.API_SECRET_KEY = "fug+;l[l;uw:er\-0-63-096z,nxvcafuga"
CLOUD.STORAGE.IMAGES = "myimages"
CORS_ORIGINS =["http://localhost:3000", "", "https://enveror.example.com"]
WORKER_COUNT =4
TIMEOUT_SECONDS= 2.3
EMPTY_STRING=" "
SAMPLE = true
CONFIG.FLAGS = [true, false, false, true]
CONFIG.NUMBERS = [1, 2, 3, 4, 5, 6, 7, 8, 9]
```

## code

The following is testing code for parsing and deserialization.

```rust
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
```
