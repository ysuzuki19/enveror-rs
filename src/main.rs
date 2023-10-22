use std::path::PathBuf;

use error::EnverorResult;
use into_json::IntoJson;
use loader::Loader;

mod error;
mod into_json;
mod loader;
mod tree;
mod value;

fn main() -> EnverorResult<()> {
    let mut loader = Loader::builder()
        .paths(vec![PathBuf::from("./tests/case_enveror")])
        .build();
    loader.load()?;
    let values = loader.values();

    let mut tree = tree::Tree::new();
    for (key, value) in values {
        tree.insert(key.to_owned(), value.to_owned())?
    }

    println!("{}", tree.into_json());

    Ok(())
}
