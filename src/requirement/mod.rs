use crate::field::Field;

pub use compare::*;

pub mod compare;

pub type BoxedRequirement = Box<dyn Requirement>;

pub trait Requirement {
    fn description(&self) -> String;
    fn distance_from(&self, field: &Field) -> u64;
}

impl std::fmt::Debug for dyn Requirement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.description().fmt(f)
    }
}
