mod node;
mod node_branch;
mod route;
mod string_into_json;

use crate::{error::Result, into_json::IntoJson};

use self::{node::Node, node_branch::Branch, route::Route};

#[derive(Debug)]
pub struct Tree {
    root: Branch,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            root: Branch::new(),
        }
    }

    pub fn insert(&mut self, key: String, content: String) -> Result<()> {
        let route = key.parse::<Route>()?;
        let leaf = Node::Leaf(content);
        self.root.insert(route, leaf)?;
        Ok(())
    }
}

impl IntoJson for Tree {
    fn into_json(self) -> String {
        self.root.into_json()
    }
}
