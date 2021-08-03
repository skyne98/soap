use serde_json::Value;

use crate::field::Field;

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

// Standard requirements
pub enum CompareRequirement {
    Equals(Field),
    MoreThan(Field),
    MoreThanEquals(Field),
    LessThan(Field),
    LessThanEquals(Field),
}

impl Requirement for CompareRequirement {
    fn description(&self) -> String {
        match self {
            CompareRequirement::Equals(field) => format!("== {:?}", field),
            CompareRequirement::MoreThan(field) => format!("> {:?}", field),
            CompareRequirement::MoreThanEquals(field) => format!(">= {:?}", field),
            CompareRequirement::LessThan(field) => format!("< {:?}", field),
            CompareRequirement::LessThanEquals(field) => format!("<= {:?}", field),
        }
    }

    fn distance_from(&self, other: &Field) -> u64 {
        let this = match self {
            CompareRequirement::Equals(field) => field,
            CompareRequirement::MoreThan(field) => field,
            CompareRequirement::MoreThanEquals(field) => field,
            CompareRequirement::LessThan(field) => field,
            CompareRequirement::LessThanEquals(field) => field,
        };

        if let Field::Value(this) = this {
            if let Field::Value(other) = other {
                // Both are JSON values
                // Bool
                if let Value::Bool(this) = this {
                    if let Value::Bool(other) = other {
                        return match self {
                            CompareRequirement::Equals(_) => {
                                if this == other {
                                    0
                                } else {
                                    1
                                }
                            }
                            _ => panic!("Cannot use this operation on two booleans"),
                        };
                    }
                }

                // Number
                if let Value::Number(this) = this {
                    if let Value::Number(other) = other {
                        // Integer
                        if this.is_i64() {
                            if other.is_i64() {
                                let this = this.as_i64().unwrap() as u64;
                                let other = other.as_i64().unwrap() as u64;
                                return match self {
                                    CompareRequirement::Equals(_) => {
                                        this.max(other) - this.min(other)
                                    }
                                    CompareRequirement::MoreThan(_) => {
                                        if other > this {
                                            0
                                        } else if other == this {
                                            1
                                        } else {
                                            this.max(other) - this.min(other)
                                        }
                                    }
                                    CompareRequirement::MoreThanEquals(_) => {
                                        if other >= this {
                                            0
                                        } else {
                                            this.max(other) - this.min(other)
                                        }
                                    }
                                    CompareRequirement::LessThan(_) => {
                                        if other < this {
                                            0
                                        } else if other == this {
                                            1
                                        } else {
                                            this.max(other) - this.min(other)
                                        }
                                    }
                                    CompareRequirement::LessThanEquals(_) => {
                                        if other <= this {
                                            0
                                        } else {
                                            this.max(other) - this.min(other)
                                        }
                                    }
                                };
                            }
                        }
                        // Unsigned
                        if this.is_u64() {
                            if other.is_u64() {
                                let this = this.as_u64().unwrap();
                                let other = other.as_u64().unwrap();
                                return match self {
                                    CompareRequirement::Equals(_) => {
                                        this.max(other) - this.min(other)
                                    }
                                    CompareRequirement::MoreThan(_) => {
                                        if other > this {
                                            0
                                        } else if other == this {
                                            1
                                        } else {
                                            this.max(other) - this.min(other)
                                        }
                                    }
                                    CompareRequirement::MoreThanEquals(_) => {
                                        if other >= this {
                                            0
                                        } else {
                                            this.max(other) - this.min(other)
                                        }
                                    }
                                    CompareRequirement::LessThan(_) => {
                                        if other < this {
                                            0
                                        } else if other == this {
                                            1
                                        } else {
                                            this.max(other) - this.min(other)
                                        }
                                    }
                                    CompareRequirement::LessThanEquals(_) => {
                                        if other <= this {
                                            0
                                        } else {
                                            this.max(other) - this.min(other)
                                        }
                                    }
                                };
                            }
                        }
                        // Double
                        if this.is_f64() {
                            if other.is_f64() {
                                let this = (this.as_f64().unwrap() * 100.0).round() as u64;
                                let other = (other.as_f64().unwrap() * 100.0).round() as u64;
                                return match self {
                                    CompareRequirement::Equals(_) => {
                                        this.max(other) - this.min(other)
                                    }
                                    CompareRequirement::MoreThan(_) => {
                                        if other > this {
                                            0
                                        } else if other == this {
                                            1
                                        } else {
                                            this.max(other) - this.min(other)
                                        }
                                    }
                                    CompareRequirement::MoreThanEquals(_) => {
                                        if other >= this {
                                            0
                                        } else {
                                            this.max(other) - this.min(other)
                                        }
                                    }
                                    CompareRequirement::LessThan(_) => {
                                        if other < this {
                                            0
                                        } else if other == this {
                                            1
                                        } else {
                                            this.max(other) - this.min(other)
                                        }
                                    }
                                    CompareRequirement::LessThanEquals(_) => {
                                        if other <= this {
                                            0
                                        } else {
                                            this.max(other) - this.min(other)
                                        }
                                    }
                                };
                            }
                        }
                    }
                }
            }
        }

        panic!("Unsupported distance")
    }
}
