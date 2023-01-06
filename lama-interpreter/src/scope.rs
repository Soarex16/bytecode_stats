use std::iter;

use crate::{error::InterpreterError, value::{NativeValue, Ref}};

pub struct Scope {
    values: Vec<NativeValue>,
}

impl Scope {
    pub fn new(size: usize) -> Self {
        Self {
            values: iter::repeat(NativeValue::box_i32(0)).take(size).collect(),
        }
    }

    pub fn lookup(&self, index: usize) -> Result<NativeValue, InterpreterError> {
        self.values
            .get(index)
            .cloned()
            .ok_or(InterpreterError::InvalidValueAccess(index))
    }

    pub fn lookup_ref(&mut self, index: usize) -> Result<Ref, InterpreterError> {
        self.values
            .get_mut(index)
            .map(|r| Ref(r as *mut NativeValue))
            .ok_or(InterpreterError::InvalidValueAccess(index))
    }

    pub fn set(&mut self, index: usize, val: NativeValue) -> Result<(), InterpreterError> {
        self.values
            .get_mut(index)
            .map(|r| {
                *r = val;
            })
            .ok_or(InterpreterError::InvalidValueAccess(index))
    }
}

impl From<Vec<NativeValue>> for Scope {
    fn from(values: Vec<NativeValue>) -> Self {
        Self { values }
    }
}
