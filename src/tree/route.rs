use std::{collections::VecDeque, str::FromStr};

use crate::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Route {
    front: String,
    subroute: VecDeque<String>,
}

impl Route {
    pub fn has_subroute(&self) -> bool {
        !self.subroute.is_empty()
    }

    pub fn front(&self) -> String {
        self.front.clone()
    }

    pub fn pop(&mut self) -> Result<String> {
        if let Some(next) = self.subroute.pop_front() {
            let front = self.front.clone();
            self.front = next;
            Ok(front)
        } else {
            Err(Error::Custom("Route has only front value".into()))
        }
    }
}

impl FromStr for Route {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            return Err(Error::Custom(format!("Invalid key route: {s}")));
        }
        let mut subroute = s.split('.').map(|s| s.to_owned()).collect::<VecDeque<_>>();
        let front = subroute.pop_front().expect("Failed to get front");
        Ok(Self { front, subroute })
    }
}
