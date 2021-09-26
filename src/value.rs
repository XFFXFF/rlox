use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    String(String),
    Number(f32),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => fmt::Display::fmt(b, f),
            Value::String(s) => fmt::Display::fmt(s, f),
            Value::Number(n) => fmt::Display::fmt(n, f),
            Value::Nil => fmt::Display::fmt("nil", f),
        }
    }
}
