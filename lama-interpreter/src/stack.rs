use crate::{error::InterpreterError, value::NativeValue};

const MAX_STACK_SIZE : usize = 100;

#[derive(Debug)]
pub struct Stack {
    stack: [NativeValue; MAX_STACK_SIZE],
    ptr: usize,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            stack: [NativeValue::box_i32(0); MAX_STACK_SIZE],
            ptr: 0,
        }
    }

    pub fn push(&mut self, v: NativeValue) -> Result<(), InterpreterError> {
        if self.ptr + 1 < MAX_STACK_SIZE {
            self.stack[self.ptr] = v;
            self.ptr += 1;
            Ok(())
        } else {
            Err(InterpreterError::ValueStackUnderflow)
        }
    }

    pub fn pop(&mut self) -> Result<NativeValue, InterpreterError> {
        if self.ptr == 0 {
            Err(InterpreterError::ValueStackUnderflow)
        } else {
            self.ptr -= 1;
            Ok(self.stack[self.ptr])
        }
    }

    pub fn take(&mut self, count: usize) -> Result<Vec<NativeValue>, InterpreterError> {
        if self.ptr < count {
            Err(InterpreterError::ValueStackUnderflow)?;
        }

        let mut values = Vec::new();
        for _ in 0..count {
            values.push(self.stack[self.ptr]);
            self.ptr -= 1
        }

        Ok(values)
    }

    pub fn drop(&mut self) -> Result<(), InterpreterError> {
        if self.ptr == 0 {
            Err(InterpreterError::ValueStackUnderflow)
        } else {
            self.ptr -= 1;
            Ok(())
        }
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
