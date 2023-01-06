use std::ffi::c_int;

use lama_bc::bytecode::BuiltIn;

use crate::{builtin::Environment, stack::Stack, value::Value, error::InterpreterError};

#[repr(i32)]
enum Tag {
    String = 0x00000001,
    Array = 0x00000003,
    Sexp = 0x00000005,
    Closure = 0x00000007
}

#[link(name = "runtime")]
#[cfg(all(target_os = "linux", target_arch = "x86"))]
extern "stdcall" {
    fn read() -> c_int;
    fn write(val: c_int) -> c_int;
}

fn box_int(num: i32) -> i32 {
    (num << 1) | 0x0001
}

fn unbox_int(num: i32) -> i32 {
    num >> 1
}

pub struct UnsafeEnvironment;

impl Environment for UnsafeEnvironment {
    fn eval(&mut self, b: BuiltIn, stack: &mut Stack) -> Result<Value, InterpreterError> {
        match b {
            BuiltIn::Read => {
                let num = unsafe { read() }; // boxed num
                Ok(Value::Int(unbox_int(num)))
            },
            BuiltIn::Write => todo!(),
            BuiltIn::Length => todo!(),
            BuiltIn::String => todo!(),
            BuiltIn::Array(_) => todo!(),
        }
    }
}