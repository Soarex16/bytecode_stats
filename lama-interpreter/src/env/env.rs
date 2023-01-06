use lama_bc::bytecode::BuiltIn;

use crate::{error::InterpreterError, stack::Stack, value::{NativeValue}};

pub trait Environment {
    fn built_in(&mut self, b: BuiltIn, stack: &mut Stack) -> Result<NativeValue, InterpreterError>;

    fn library(
        &mut self,
        func: &str,
        nargs: usize,
        stack: &mut Stack,
    ) -> Result<NativeValue, InterpreterError>;
}
