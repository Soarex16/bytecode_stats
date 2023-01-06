use lama_bc::bytecode::BuiltIn;

use crate::{error::InterpreterError, stack::Stack, value::Value};

pub trait Environment {
    fn built_in(&mut self, b: BuiltIn, stack: &mut Stack) -> Result<Value, InterpreterError>;
    fn library(
        &mut self,
        func: &str,
        nargs: usize,
        stack: &mut Stack,
    ) -> Result<Value, InterpreterError>;
}
