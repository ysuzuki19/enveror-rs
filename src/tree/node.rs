use crate::{into_json::IntoJson, value::ValueValidator};

use super::node_branch::Branch;

#[derive(Debug)]
pub enum Node {
    Leaf(String),
    Branch(Branch),
}

impl From<Branch> for Node {
    fn from(branch: Branch) -> Self {
        Node::Branch(branch)
    }
}

impl IntoJson for Node {
    fn into_json(self) -> String {
        match self {
            Node::Leaf(value) => ValueValidator::new(value).into_json(),
            Node::Branch(branch) => branch.into_json(),
        }
    }
}
