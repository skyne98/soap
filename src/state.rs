use std::hash::{Hash, Hasher};
use std::{collections::HashMap, fmt::Debug};

use crate::field::Field;
use crate::goal::Goal;

#[derive(Clone)]
pub struct State {
    fields: HashMap<String, Field>,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        debug!("----- Partial compare -----");
        self.distance_to(other).partial_cmp(&0)
    }
}
impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        debug!("----- Partial equality -----");
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

    pub fn get_as_bool<S: AsRef<str>>(&self, key: S) -> Option<bool> {
        self.get(key).map(|f| f.as_bool()).flatten()
    }

    pub fn get_as_u64<S: AsRef<str>>(&self, key: S) -> Option<u64> {
        self.get(key).map(|f| f.as_u64()).flatten()
    }

    pub fn get_as_i64<S: AsRef<str>>(&self, key: S) -> Option<i64> {
        self.get(key).map(|f| f.as_i64()).flatten()
    }

    pub fn get_as_f64<S: AsRef<str>>(&self, key: S) -> Option<f64> {
        self.get(key).map(|f| f.as_f64()).flatten()
    }

    pub fn contains_key<S: AsRef<str>>(&self, key: S) -> bool {
        self.fields.contains_key(key.as_ref())
    }

    pub fn distance_to(&self, other: &State) -> u64 {
        debug!("----- Looking for distance -----");
        debug!("From: {:?}", self);
        debug!("To: {:?}", other);

        let mut distance: u64 = 0;
        for (key, value) in &other.fields {
            let self_value = self.get(key);
            if let Some(self_value) = self_value {
                distance += self_value.distance_to(value);
            } else {
                distance += 1;
            }
        }

        debug!("= {}", distance);

        distance
    }

    pub fn distance_to_goal(&self, goal: &Goal) -> u64 {
        debug!("----- Looking for distance to goal -----");
        debug!("From: {:?}", self);
        debug!("To: {:?}", goal);

        let mut distance: u64 = 0;
        for (key, value) in goal.requirements() {
            let self_value = self.get(key);
            if let Some(self_value) = self_value {
                distance += value.distance_from(&self_value);
            } else {
                distance += 1;
            }
        }

        debug!("= {}", distance);

        distance
    }

    pub fn with_field<S: AsRef<str>>(&self, key: S, value: Field) -> Self {
        let mut clone = self.clone();
        clone.insert(key, value);
        clone
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fields.fmt(f)
    }
}
