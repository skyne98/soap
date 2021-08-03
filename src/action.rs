use serde_json::Value;
use std::hash::{Hash, Hasher};

use crate::{field::hash_value, state::State};

#[derive(Debug, Clone)]
pub struct Consequence {
    pub action: String,
    pub argument: Option<Value>,
    pub result: State,
}

impl PartialOrd for Consequence {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.result.partial_cmp(&other.result)
    }
}
impl PartialEq for Consequence {
    fn eq(&self, other: &Self) -> bool {
        self.result.eq(&other.result)
    }
}
impl Eq for Consequence {}
impl Hash for Consequence {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.action.hash(state);
        if let Some(argument) = &self.argument {
            hash_value(&argument, state);
        }
        self.result.hash(state);
    }
}

pub trait Action {
    fn key(&self) -> String;
    fn prepare(&self, state: &State) -> State;
    fn options(&self, state: &State) -> Vec<(Consequence, u64)>;
}
