use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Stage {
    Dev,
    Prod,
}

impl FromStr for Stage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "dev" => Ok(Self::Dev),
            "prod" => Ok(Self::Prod),
            _ => Err(()),
        }
    }
}

struct EnverorConfig {
    stage: Stage,
    cloud: Cloud,
    cors_origins: Vec<String>,
    worler_count: u8,
    timeout_seconds: f32,
    empty_string: String,
    sample: bool,
}

struct Cloud {
    api_key_id: String,
    api_secret_key: String,
    storage: CloudStorage,
}

struct CloudStorage {
    images: String,
}
