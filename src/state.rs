use std::hash::{Hash, Hasher};
use std::{collections::HashMap, fmt::Debug};

use crate::distance::*;
use crate::field::Field;

#[derive(Debug, Clone)]
pub struct State {
    fields: HashMap<String, Field>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.distance_to(other).partial_cmp(&0)
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.distance_to(other) == 0
    }
}
impl Eq for State {}
impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for (key, value) in &self.fields {
            key.hash(state);
            value.hash(state)
        }
    }
}

impl State {
    pub fn new() -> Self {
        State {
            fields: HashMap::new(),
        }
    }

    pub fn insert<S: AsRef<str>>(&mut self, key: S, value: Field) -> Option<Field> {
        self.fields.insert(key.as_ref().to_owned(), value)
    }

    pub fn get<S: AsRef<str>>(&self, key: S) -> Option<Field> {
        self.fields.get(key.as_ref()).map(|value| value.clone())
    }

    pub fn contains_key<S: AsRef<str>>(&self, key: S) -> bool {
        self.fields.contains_key(key.as_ref())
    }

    pub fn distance_to(&self, other: &State) -> u64 {
        println!("----- Looking for distance -----");
        println!("From: {:#?}", self);
        println!("To: {:#?}", other);

        let mut distance: u64 = 0;
        for (key, value) in &other.fields {
            let self_value = self.get(key);
            if let Some(self_value) = self_value {
                distance += self_value.distance_to(value);
            } else {
                distance += 1;
            }
        }

        println!("= {}", distance);

        distance
    }

    pub fn with_field<S: AsRef<str>>(&self, key: S, value: Field) -> Self {
        let mut clone = self.clone();
        clone.insert(key, value);
        clone
    }
}
