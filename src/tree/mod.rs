mod route;

use std::collections::HashMap;

use crate::{
    error::{EnverorError, EnverorResult},
    value::Value,
};

use self::route::Route;

#[derive(Debug)]
pub struct Branch {
    children: HashMap<String, Node>,
}

impl Branch {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, mut route: Route, leaf: Node) -> EnverorResult<()> {
        if route.has_subroute() {
            let key = route.pop()?;
            let child = self.children.entry(key).or_insert(Branch::new().into());
            match child {
                Node::Leaf(_) => {
                    return Err(EnverorError::InvalidConfig(
                        "cannot act as both a category and a value.".into(),
                    ))
                }
                Node::Branch(branch) => {
                    branch.insert(route, leaf)?;
                }
            };
        } else {
            let key = route.front();
            match self.children.get(&key) {
                Some(_) => {
                    return Err(EnverorError::InvalidConfig(
                        "cannot act as both a category and a value.".into(),
                    ))
                }
                None => self.children.insert(key, leaf),
            };
        }
        Ok(())
    }
}

impl From<Branch> for Node {
    fn from(branch: Branch) -> Self {
        Node::Branch(branch)
    }
}

#[derive(Debug)]
pub enum Node {
    Leaf(Value),
    Branch(Branch),
}

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
        let leaf = Node::Leaf(content.parse()?);
        if let Node::Branch(ref mut branch) = self.root {
            branch.insert(route, leaf)?;
        }
        Ok(())
    }
}
