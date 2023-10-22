use std::{collections::VecDeque, str::FromStr};

use crate::error::{EnverorError, EnverorResult};

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

    pub fn pop(&mut self) -> EnverorResult<String> {
        if let Some(next) = self.subroute.pop_front() {
            let front = self.front.clone();
            self.front = next;
            Ok(front)
        } else {
            Err(EnverorError::Custom("Route has only front value".into()))
        }
    }
}

impl FromStr for Route {
    type Err = EnverorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(EnverorError::Custom(format!("Invalid key route: {}", s)));
        }
        let mut subroute = s.split('.').map(|s| s.to_owned()).collect::<VecDeque<_>>();
        let front = subroute.pop_front().expect("Failed to get front");
        Ok(Self { front, subroute })
    }
}
