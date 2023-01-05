use std::iter;

use crate::{error::InterpreterError, value::Value};

pub struct Scope {
    values: Vec<Value>,
}

impl Scope {
    pub fn new(size: usize) -> Self {
        Self {
            values: iter::repeat(Value::Int(0)).take(size).collect(),
        }
    }

    pub fn lookup(&self, index: usize) -> Result<Value, InterpreterError> {
        self.values
            .get(index)
            .map(|x| x.clone())
            .ok_or_else(|| InterpreterError::InvalidValueAccess(index))
    }

    pub fn set(&mut self, index: usize, val: Value) -> Result<(), InterpreterError> {
        self.values
            .get_mut(index)
            .map(|r| {
                *r = val;
                ()
            })
            .ok_or_else(|| InterpreterError::InvalidValueAccess(index))
    }
}

impl From<Vec<Value>> for Scope {
    fn from(values: Vec<Value>) -> Self {
        Self { values }
    }
}
