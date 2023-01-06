use std::ffi::{c_int, c_void};

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
