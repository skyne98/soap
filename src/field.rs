use serde_json::Value;
use std::hash::{Hash, Hasher};

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

#[derive(Clone)]
pub enum Field {
    Value(Value),
    Bool(bool),
    String(String),
    U64(u64),
    I64(i64),
    F64(f64),
}

impl Field {
    pub fn as_value(&self) -> Option<Value> {
        match self {
            Field::Value(val) => Some(val.clone()),
            _ => None,
        }
    }
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Field::Value(val) => match val {
                Value::Bool(val) => Some(val.clone()),
                _ => None,
            },
            Field::Bool(val) => Some(*val),
            _ => None,
        }
    }
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            Field::Value(val) => match val {
                Value::Number(val) => val.as_i64(),
                _ => None,
            },
            Field::I64(val) => Some(*val),
            _ => None,
        }
    }
    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Field::Value(val) => match val {
                Value::Number(val) => val.as_u64(),
                _ => None,
            },
            Field::U64(val) => Some(*val),
            _ => None,
        }
    }
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Field::Value(val) => match val {
                Value::Number(val) => val.as_f64(),
                _ => None,
            },
            Field::F64(val) => Some(*val),
            _ => None,
        }
    }
}

impl From<bool> for Field {
    fn from(val: bool) -> Self {
        Field::Bool(val)
    }
}
impl From<i64> for Field {
    fn from(val: i64) -> Self {
        Field::I64(val)
    }
}
impl From<u64> for Field {
    fn from(val: u64) -> Self {
        Field::U64(val)
    }
}
impl From<f64> for Field {
    fn from(val: f64) -> Self {
        Field::F64(val)
    }
}
impl Hash for Field {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Field::Value(val) => hash_value(val, state),
            Field::Bool(val) => val.hash(state),
            Field::String(val) => val.hash(state),
            Field::U64(val) => val.hash(state),
            Field::I64(val) => val.hash(state),
            Field::F64(val) => Distance::new(*val).hash(state),
            _ => panic!("Hashing this field type is not supported"),
        }
    }
}
impl std::fmt::Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Field::Value(value) => value.fmt(f),
            Field::Bool(value) => value.fmt(f),
            Field::String(value) => value.fmt(f),
            Field::U64(value) => value.fmt(f),
            Field::I64(value) => value.fmt(f),
            Field::F64(value) => value.fmt(f),
        }
    }
}
impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Field::Value(val) => match other {
                Field::Value(other) => val.eq(other),
                _ => false,
            },
            Field::Bool(val) => match other {
                Field::Bool(other) => val.eq(other),
                _ => false,
            },
            Field::String(val) => match other {
                Field::String(other) => val.eq(other),
                _ => false,
            },
            Field::U64(val) => match other {
                Field::U64(other) => val.eq(other),
                _ => false,
            },
            Field::I64(val) => match other {
                Field::I64(other) => val.eq(other),
                _ => false,
            },
            Field::F64(val) => match other {
                Field::F64(other) => val.eq(other),
                _ => false,
            },
            _ => panic!("Checking equality of this type of field is not yet supported"),
        }
    }
}
impl Eq for Field {}

pub fn hash_value<H: Hasher>(value: &Value, state: &mut H) {
    // JSON value
    // Bool
    if let Value::Bool(this) = value {
        this.hash(state);
        return;
    }

    // String
    if let Value::String(this) = value {
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

    panic!("Hashing this type ({:?}) is not supported", value)
}
