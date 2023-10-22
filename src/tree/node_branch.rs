use std::collections::HashMap;

use crate::{
    error::{EnverorError, EnverorResult},
    into_json::IntoJson,
};

use super::{node::Node, route::Route};

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

impl IntoJson for Branch {
    fn into_json(self) -> String {
        let entries: Vec<String> = self
            .children
            .into_iter()
            .map(|(key, value)| format!("\"{}\":{}", key, value.into_json()))
            .collect();
        format!("{{{}}}", entries.join(","))
    }
}
