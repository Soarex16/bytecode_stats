use std::{
    io::{self, Write},
    rc::Rc,
};

use lama_bc::bytecode::BuiltIn;

use crate::{error::InterpreterError, stack::Stack, value::{Value, NativeValue, Ref}};

use super::Environment;

pub struct RustEnvironment {
    arrays: Vec<Vec<NativeValue>>
}

impl RustEnvironment {
    pub fn new() -> Self {
        Self { arrays: Vec::new() }
    }
}

impl Environment for RustEnvironment {
    fn built_in(&mut self, b: BuiltIn, stack: &mut Stack) -> Result<NativeValue, InterpreterError> {
        match b {
            BuiltIn::Array(size) => {
                let mut vals = stack.take(size as usize)?;
                vals.reverse();
                self.arrays.push(vals);
                Ok(NativeValue::wrap_ref(Ref(self.arrays.last_mut().unwrap().as_mut_ptr() as *mut NativeValue)))
            }
            b => Err(InterpreterError::Failure(format!(
                "unsupported bultin: {:?}",
                b
            ))),
        }
    }

    fn library(
        &mut self,
        func: &str,
        _nargs: usize,
        stack: &mut Stack,
    ) -> Result<NativeValue, InterpreterError> {
        match func {
            "Lread" => {
                print!("> ");
                io::stdout()
                    .flush()
                    .map_err(|_| InterpreterError::Unknown("IO error".to_string()))?;
                let mut input = String::new();
                io::stdin().read_line(&mut input).map_err(|_| {
                    InterpreterError::Failure("Error evaluating builtin Read".to_string())
                })?;
                let num: i32 = input.trim().parse().map_err(|_| {
                    InterpreterError::Failure("Error parsing number in builtin Read".to_string())
                })?;
                Ok(NativeValue::box_i32(num))
            }
            "Lwrite" => {
                let val = stack.pop()?;
                println!("{}", val.unwrap_int()?);
                Ok(NativeValue::box_i32(0))
            }
            "Llength" => {
                // let val = stack.pop()?;
                // match val {
                //     Value::Sexp(_, _, vals) => Ok(Value::Int(vals.len() as i32)),
                //     Value::String(str) => Ok(Value::Int(str.len() as i32)),
                //     Value::Array(vals) => Ok(Value::Int(vals.len() as i32)),
                //     _ => Err(InterpreterError::UnexpectedValue {
                //         expected: "sexp, array or string".to_string(),
                //         found: val.to_string(),
                //     }),
                // }
                todo!()
            }
            _ => Err(InterpreterError::UnknownFunction(func.to_string())),
        }
    }
}
