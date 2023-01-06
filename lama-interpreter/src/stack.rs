use crate::{error::InterpreterError, value::Value};

#[derive(Debug)]
pub struct Stack {
    stack: Vec<Value>,
}

impl Stack {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn push(&mut self, v: Value) {
        self.stack.push(v);
    }

    pub fn pop(&mut self) -> Result<Value, InterpreterError> {
        self.stack
            .pop()
            .ok_or(InterpreterError::ValueStackUnderflow)
    }

    pub fn take(&mut self, count: usize) -> Result<Vec<Value>, InterpreterError> {
        if self.stack.len() < count {
            Err(InterpreterError::ValueStackUnderflow)?;
        }

        let mut values = Vec::new();
        for _ in 0..count {
            values.push(self.stack.pop().unwrap())
        }

        Ok(values)
    }

    pub fn drop(&mut self) -> Result<(), InterpreterError> {
        self.stack
            .pop()
            .map(|_| ())
            .ok_or(InterpreterError::ValueStackUnderflow)
    }

    pub fn dup(&mut self) -> Result<(), InterpreterError> {
        let x = self.pop()?;
        let y = x.clone();
        self.push(x);
        self.push(y);
        Ok(())
    }

    pub(crate) fn swap(&mut self) -> Result<(), InterpreterError> {
        let x = self.pop()?;
        let y = self.pop()?;
        self.push(y);
        self.push(x);
        Ok(())
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}
