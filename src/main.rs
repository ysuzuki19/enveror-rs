use std::path::PathBuf;

use error::EnverorResult;
use loader::Loader;

mod error;
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
        println!("{}: {}", key, value);
        tree.insert(key.to_owned(), value.to_owned())?
    }

    println!("{:#?}", tree);

    Ok(())
}
