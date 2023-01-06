use std::{rc::Rc, io};

use lama_bc::bytecode::BuiltIn;

use crate::{stack::Stack, value::Value, error::InterpreterError};

use super::Environment;

pub struct NativeEnvironment;

impl Environment for NativeEnvironment {
    fn built_in(&mut self, b: BuiltIn, stack: &mut Stack) -> Result<Value, InterpreterError> {
        match b {
            BuiltIn::Array(size) => {
                let mut vals = stack.take(size as usize)?;
                vals.reverse();
                Ok(Value::Array(Rc::new(vals)))
            },
            b => Err(InterpreterError::Failure(format!("unsupported bultin: {:?}", b))),
        }
    }

    fn library(
        &mut self,
        func: &str,
        nargs: usize,
        stack: &mut Stack,
    ) -> Result<Value, InterpreterError> {
        match func {
            "Lread" => {
                print!(">");
                let mut input = String::new();
                io::stdin().read_line(&mut input).map_err(|_| {
                    InterpreterError::Failure("Error evaluating builtin Read".to_string())
                })?;
                let num: i32 = input.trim().parse().map_err(|_| {
                    InterpreterError::Failure("Error parsing number in builtin Read".to_string())
                })?;
                Ok(Value::Int(num))
            },
            "Lwrite" => {
                let val = stack.pop()?;
                println!("{}", val);
                Ok(Value::Int(0))
            },
            "Llength" => {
                let val = stack.pop()?;
                match val {
                    Value::Sexp(_, _, vals) => Ok(Value::Int(vals.len() as i32)),
                    Value::String(str) => Ok(Value::Int(str.len() as i32)),
                    Value::Array(vals) => Ok(Value::Int(vals.len() as i32)),
                    _ => Err(InterpreterError::UnexpectedValue {
                        expected: "sexp, array or string".to_string(),
                        found: val.to_string(),
                    }),
                }
            },
            _ => Err(InterpreterError::UnknownFunction(func.to_string())),
        }
    }
}

/* use std::{ffi::{c_int, c_void}, rc::Rc, io};

use lama_bc::bytecode::BuiltIn;

use crate::{builtin::Environment2, error::InterpreterError, stack::Stack, value::Value};

#[repr(i32)]
enum Tag {
    String = 0x00000001,
    Array = 0x00000003,
    Sexp = 0x00000005,
    Closure = 0x00000007,
}

#[link(name = "runtime")]
#[cfg(all(target_os = "linux", target_arch = "x86"))]
extern "C" {
    fn Lread() -> c_int;
    fn Lwrite(val: c_int) -> c_int;

    fn Barray(bn: c_int, ...) -> *mut c_void;
}

fn box_int(num: i32) -> i32 {
    (num << 1) | 0x0001
}

fn unbox_int(num: i32) -> i32 {
    num >> 1
}

pub struct UnsafeEnvironment;

impl Environment2 for UnsafeEnvironment {
    fn eval(&mut self, b: BuiltIn, stack: &mut Stack) -> Result<Value, InterpreterError> {
        unimplemented!()
        // match b {
        //     BuiltIn::Read => {
        //         let num = unsafe { Lread() };
        //         Ok(Value::Int(unbox_int(num)))
        //     }
        //     BuiltIn::Write => {
        //         let num = stack.pop()?.unwrap_int()?;
        //         unsafe { Lwrite(box_int(num)) };
        //         Ok(Value::Int(0))
        //     }
        //     BuiltIn::Length => todo!(),
        //     BuiltIn::String => todo!(),
        //     BuiltIn::Array(_) => todo!(),
        // }
    }
}
 */