use serde_json::Value;

use crate::field::Field;

impl Field {
    pub fn distance_to(&self, other: &Field) -> u64 {
        if let Field::Value(this) = self {
            if let Field::Value(other) = other {
                // Both are JSON values
                // Bool
                if let Value::Bool(this) = this {
                    if let Value::Bool(other) = other {
                        return if this == other { 0 } else { 1 };
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
                                return this.max(other) - this.min(other);
                            }
                        }
                        // Unsigned
                        if this.is_u64() {
                            if other.is_u64() {
                                let this = this.as_u64().unwrap();
                                let other = other.as_u64().unwrap();
                                return this.max(other) - this.min(other);
                            }
                        }
                        // Double
                        if this.is_f64() {
                            if other.is_f64() {
                                let this = (this.as_f64().unwrap() * 100.0).round() as u64;
                                let other = (other.as_f64().unwrap() * 100.0).round() as u64;
                                return this.max(other) - this.min(other);
                            }
                        }
                    }
                }
            }
        }

        panic!("Comparing values of incompatible types")
    }
}
