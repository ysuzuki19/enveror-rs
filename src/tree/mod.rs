mod node;
mod node_branch;
mod route;

use crate::{error::EnverorResult, into_json::IntoJson};

use self::{node::Node, node_branch::Branch, route::Route};

#[derive(Debug)]
pub struct Tree {
    root: Node,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            root: Branch::new().into(),
        }
    }

    pub fn insert(&mut self, key: String, content: String) -> EnverorResult<()> {
        let route = key.parse::<Route>()?;
        let leaf = Node::Leaf(content);
        if let Node::Branch(ref mut branch) = self.root {
            branch.insert(route, leaf)?;
        }
        Ok(())
    }
}

impl IntoJson for Tree {
    fn into_json(self) -> String {
        self.root.into_json()
    }
}
