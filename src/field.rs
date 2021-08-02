use serde_json::Value;
use std::hash::{Hash, Hasher};
use std::{collections::HashMap, fmt::Debug};

fn integer_decode(val: f64) -> (u64, i16, i8) {
    let bits: u64 = unsafe { std::mem::transmute(val) };
    let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
    let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
    let mantissa = if exponent == 0 {
        (bits & 0xfffffffffffff) << 1
    } else {
        (bits & 0xfffffffffffff) | 0x10000000000000
    };

    exponent -= 1023 + 52;
    (mantissa, exponent, sign)
}

#[derive(Hash, Eq, PartialEq)]
struct Distance((u64, i16, i8));

impl Distance {
    fn new(val: f64) -> Distance {
        Distance(integer_decode(val))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    Value(Value),
}

impl Hash for Field {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Field::Value(this) = self {
            hash_value(this, state);
            return;
        }

        panic!("Hashing this field type is not supported")
    }
}

pub fn hash_value<H: Hasher>(value: &Value, state: &mut H) {
    // JSON value
    // Bool
    if let Value::Bool(this) = value {
        this.hash(state);
        return;
    }

    // Number
    if let Value::Number(this) = value {
        // Integer
        if this.is_i64() {
            let this = this.as_i64().unwrap();
            this.hash(state);
            return;
        }
        // Unsigned
        if this.is_u64() {
            let this = this.as_u64().unwrap();
            this.hash(state);
            return;
        }
        // Double
        if this.is_f64() {
            let this = this.as_f64().unwrap();
            let this = Distance::new(this);
            this.hash(state);
            return;
        }
    }

    panic!("Hashing this type is not supported")
}
