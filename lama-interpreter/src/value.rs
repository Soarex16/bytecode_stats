use std::{fmt::Display, rc::Rc};

use lama_bc::bytecode::{Location, StringPtr};

use crate::error::InterpreterError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Value {
    Empty,
    Int(i32),
    Sexp(StringPtr, String, Rc<Vec<Value>>),
    String(String),
    Array(Rc<Vec<Value>>),
    Ref(Location),
    ReturnAddress(usize),
}

impl Value {
    pub fn unwrap_int(self) -> Result<i32, InterpreterError> {
        match self {
            Value::Int(x) => Ok(x),
            _ => Err(InterpreterError::UnexpectedValue {
                expected: "int".to_string(),
                found: self.to_string(),
            }),
        }
    }

    pub fn unwrap_return_addr(self) -> Result<usize, InterpreterError> {
        match self {
            Value::ReturnAddress(addr) => Ok(addr),
            _ => Err(InterpreterError::UnexpectedValue {
                expected: "return address".to_string(),
                found: self.to_string(),
            }),
        }
    }

    pub fn unwrap_ref(self) -> Result<Location, InterpreterError> {
        match self {
            Value::Ref(loc) => Ok(loc),
            _ => Err(InterpreterError::UnexpectedValue {
                expected: "return address".to_string(),
                found: self.to_string(),
            }),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(x) => write!(f, "{}", x),
            Value::Empty => write!(f, "empty"),
            Value::ReturnAddress(addr) => write!(f, "return({})", addr),
            Value::Ref(l) => write!(f, "ref({:?})", l),
            Value::Array(vals) => write!(
                f,
                "[{}]",
                vals.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Sexp(tag, tag_label, vals) => write!(
                f,
                "{}({})",
                tag_label,
                vals.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::String(s) => write!(f, "\"{}\"", s),
        }
    }
}