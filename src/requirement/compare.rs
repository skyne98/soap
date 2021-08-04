use crate::{
    distance::{distance_eq, distance_f64, distance_i64, distance_u64},
    field::Field,
};

use super::Requirement;

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

        // JSON value
        if let Field::Value(_this) = this {
            if let Field::Value(_other) = other {
                // Both are JSON values
                unimplemented!()
            }
        }

        // Bool
        if let Field::Bool(this) = this {
            if let Field::Bool(other) = other {
                return match self {
                    CompareRequirement::Equals(_) => distance_eq(this, other),
                    _ => panic!("Cannot use this operation on two booleans"),
                };
            }
        }

        // String
        if let Field::String(this) = this {
            if let Field::String(other) = other {
                return match self {
                    CompareRequirement::Equals(_) => distance_eq(this, other),
                    _ => panic!("Cannot use this operation on two strings"),
                };
            }
        }

        // Integer
        if let Field::I64(this) = *this {
            if let Field::I64(other) = *other {
                return match self {
                    CompareRequirement::Equals(_) => distance_i64(this, other),
                    CompareRequirement::MoreThan(_) => {
                        if other > this {
                            0
                        } else if other == this {
                            1
                        } else {
                            distance_i64(this, other)
                        }
                    }
                    CompareRequirement::MoreThanEquals(_) => {
                        if other >= this {
                            0
                        } else {
                            distance_i64(this, other)
                        }
                    }
                    CompareRequirement::LessThan(_) => {
                        if other < this {
                            0
                        } else if other == this {
                            1
                        } else {
                            distance_i64(this, other)
                        }
                    }
                    CompareRequirement::LessThanEquals(_) => {
                        if other <= this {
                            0
                        } else {
                            distance_i64(this, other)
                        }
                    }
                };
            }
        }

        // Unsigned
        if let Field::U64(this) = *this {
            if let Field::U64(other) = *other {
                return match self {
                    CompareRequirement::Equals(_) => distance_u64(this, other),
                    CompareRequirement::MoreThan(_) => {
                        if other > this {
                            0
                        } else if other == this {
                            1
                        } else {
                            distance_u64(this, other)
                        }
                    }
                    CompareRequirement::MoreThanEquals(_) => {
                        if other >= this {
                            0
                        } else {
                            distance_u64(this, other)
                        }
                    }
                    CompareRequirement::LessThan(_) => {
                        if other < this {
                            0
                        } else if other == this {
                            1
                        } else {
                            distance_u64(this, other)
                        }
                    }
                    CompareRequirement::LessThanEquals(_) => {
                        if other <= this {
                            0
                        } else {
                            distance_u64(this, other)
                        }
                    }
                };
            }
        }

        // Double
        if let Field::F64(this) = *this {
            if let Field::F64(other) = *other {
                return match self {
                    CompareRequirement::Equals(_) => distance_f64(this, other),
                    CompareRequirement::MoreThan(_) => {
                        if other > this {
                            0
                        } else if other == this {
                            1
                        } else {
                            distance_f64(this, other)
                        }
                    }
                    CompareRequirement::MoreThanEquals(_) => {
                        if other >= this {
                            0
                        } else {
                            distance_f64(this, other)
                        }
                    }
                    CompareRequirement::LessThan(_) => {
                        if other < this {
                            0
                        } else if other == this {
                            1
                        } else {
                            distance_f64(this, other)
                        }
                    }
                    CompareRequirement::LessThanEquals(_) => {
                        if other <= this {
                            0
                        } else {
                            distance_f64(this, other)
                        }
                    }
                };
            }
        }

        panic!("Unsupported distance between {:?} and {:?}", this, other)
    }
}
