use std::ffi::{c_int, c_void};
use std::rc::Rc;

use lama_bc::bytecode::BuiltIn;

use super::Environment;
use crate::{error::InterpreterError, stack::Stack, value::NativeValue};

#[link(name = "runtime")]
#[cfg(all(target_os = "linux", target_arch = "x86"))]
extern "C" {
    fn Lread() -> c_int;
    fn Lwrite(val: c_int) -> c_int;

    fn Barray(bn: c_int, ...) -> *mut c_void;

    fn __gc_init();
}

fn box_int(num: i32) -> i32 {
    (num << 1) | 0x0001
}

fn unbox_int(num: i32) -> i32 {
    num >> 1
}

pub struct NativeEnvironment;

impl NativeEnvironment {
    #[cfg(all(target_os = "linux", target_arch = "x86"))]
    pub fn new() -> Self {
        unsafe {
            __gc_init();
        }
        NativeEnvironment
    }

    #[cfg(not(all(target_os = "linux", target_arch = "x86")))]
    pub fn new() -> Self {
        NativeEnvironment
    }
}

impl Environment for NativeEnvironment {
    #[cfg(all(target_os = "linux", target_arch = "x86"))]
    fn built_in(&mut self, b: BuiltIn, stack: &mut Stack) -> Result<NativeValue, InterpreterError> {
        // match b {
        //     BuiltIn::Array(size) => {
        //         let mut vals = stack.take(size as usize)?;
        //         vals.reverse();
        //         Ok(Value::Array(Rc::new(vals)))
        //     }
        //     b => Err(InterpreterError::Failure(format!(
        //         "unsupported bultin: {:?}",
        //         b
        //     ))),
        // }
        todo!()
    }

    #[cfg(all(target_os = "linux", target_arch = "x86"))]
    fn library(
        &mut self,
        func: &str,
        _nargs: usize,
        stack: &mut Stack,
    ) -> Result<NativeValue, InterpreterError> {
        // match func {
        //     "Lread" => {
        //         let num = unsafe { Lread() };
        //         Ok(Value::Int(unbox_int(num)))
        //     }
        //     "Lwrite" => {
        //         let num = stack.pop()?.unwrap_int()?;
        //         unsafe { Lwrite(box_int(num)) };
        //         Ok(Value::Int(0))
        //     }
        //     "Llength" => {
        //         let val = stack.pop()?;
        //         match val {
        //             Value::Sexp(_, _, vals) => Ok(Value::Int(vals.len() as i32)),
        //             Value::String(str) => Ok(Value::Int(str.len() as i32)),
        //             Value::Array(vals) => Ok(Value::Int(vals.len() as i32)),
        //             _ => Err(InterpreterError::UnexpectedValue {
        //                 expected: "sexp, array or string".to_string(),
        //                 found: val.to_string(),
        //             }),
        //         }
        //     }
        //     _ => Err(InterpreterError::UnknownFunction(func.to_string())),
        // }
        todo!()
    }

    #[cfg(not(all(target_os = "linux", target_arch = "x86")))]
    fn built_in(&mut self, b: BuiltIn, stack: &mut Stack) -> Result<NativeValue, InterpreterError> {
        panic!("Available only on i686")
    }

    #[cfg(not(all(target_os = "linux", target_arch = "x86")))]
    fn library(
        &mut self,
        func: &str,
        _nargs: usize,
        stack: &mut Stack,
    ) -> Result<NativeValue, InterpreterError> {
        panic!("Available only on i686")
    }
}
