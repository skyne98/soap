use crate::field::Field;

#[inline]
pub fn distance_eq<T: std::cmp::PartialEq>(this: T, other: T) -> u64 {
    if this == other {
        0
    } else {
        1
    }
}

#[inline]
pub fn distance_i64(this: i64, other: i64) -> u64 {
    (this.max(other) - this.min(other)) as u64
}

#[inline]
pub fn distance_u64(this: u64, other: u64) -> u64 {
    this.max(other) - this.min(other)
}

#[inline]
pub fn distance_f64(this: f64, other: f64) -> u64 {
    (this.max(other) - this.min(other) * 100.0).round() as u64
}

impl Field {
    pub fn distance_to(&self, other: &Field) -> u64 {
        if let Field::Value(_this) = self {
            if let Field::Value(_other) = other {
                // Both are JSON values
                unimplemented!()
            }
        }

        // Bool
        if let Field::Bool(this) = self {
            if let Field::Bool(other) = other {
                return distance_eq(this, other);
            }
        }

        // String
        if let Field::String(this) = self {
            if let Field::String(other) = other {
                return distance_eq(this, other);
            }
        }

        // Integer
        if let Field::I64(this) = *self {
            if let Field::I64(other) = *other {
                return distance_i64(this, other);
            }
        }
        // Unsigned
        if let Field::U64(this) = *self {
            if let Field::U64(other) = *other {
                return distance_u64(this, other);
            }
        }
        // Double
        if let Field::F64(this) = *self {
            if let Field::F64(other) = *other {
                return distance_f64(this, other);
            }
        }

        panic!("Comparing values of incompatible types")
    }
}
