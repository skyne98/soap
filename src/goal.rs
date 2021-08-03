use std::{collections::HashMap, fmt::Debug};

use crate::requirement::BoxedRequirement;

pub struct Goal {
    requirements: HashMap<String, BoxedRequirement>,
}

impl Goal {
    pub fn new() -> Self {
        Goal {
            requirements: HashMap::new(),
        }
    }

    pub fn requirements(&self) -> &HashMap<String, BoxedRequirement> {
        &self.requirements
    }

    pub fn insert<S: AsRef<str>>(
        &mut self,
        key: S,
        value: BoxedRequirement,
    ) -> Option<BoxedRequirement> {
        self.requirements.insert(key.as_ref().to_owned(), value)
    }

    pub fn get<S: AsRef<str>>(&self, key: S) -> Option<&BoxedRequirement> {
        self.requirements
            .get(key.as_ref())
            .map(|value| value.clone())
    }

    pub fn contains_key<S: AsRef<str>>(&self, key: S) -> bool {
        self.requirements.contains_key(key.as_ref())
    }

    pub fn with_req<S: AsRef<str>>(mut self, key: S, value: BoxedRequirement) -> Self {
        self.insert(key, value);
        self
    }
}

impl std::fmt::Debug for Goal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.requirements.fmt(f)
    }
}
