use std::{fmt::Display, rc::Rc};

use lama_bc::bytecode::{Location, StringPtr};

use crate::error::InterpreterError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[repr(C)]
pub enum Value {
    Int(i32),
    Sexp(StringPtr, String, Rc<Vec<Value>>),
    String(String),
    Array(Rc<Vec<Value>>),
    Ref(Location),
    ReturnAddress(usize),
}

impl Value {
    pub fn unwrap_int(self) -> Result<i32, InterpreterError> {
        match self {
            Value::Int(x) => Ok(x),
            _ => Err(InterpreterError::UnexpectedValue {
                expected: "int".to_string(),
                found: self.to_string(),
            }),
        }
    }

    pub fn unwrap_return_addr(self) -> Result<usize, InterpreterError> {
        match self {
            Value::ReturnAddress(addr) => Ok(addr),
            _ => Err(InterpreterError::UnexpectedValue {
                expected: "return address".to_string(),
                found: self.to_string(),
            }),
        }
    }

    pub fn unwrap_ref(self) -> Result<Location, InterpreterError> {
        match self {
            Value::Ref(loc) => Ok(loc),
            _ => Err(InterpreterError::UnexpectedValue {
                expected: "return address".to_string(),
                found: self.to_string(),
            }),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(x) => write!(f, "{}", x),
            Value::ReturnAddress(addr) => write!(f, "return({})", addr),
            Value::Ref(l) => write!(f, "ref({:?})", l),
            Value::Array(vals) => write!(
                f,
                "[{}]",
                vals.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::Sexp(_, tag_label, vals) => write!(
                f,
                "{}({})",
                tag_label,
                vals.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            Value::String(s) => write!(f, "\"{}\"", s),
        }
    }
}

#[repr(i32)]
enum ValueTag {
    String = 0x00000001,
    Array = 0x00000003,
    Sexp = 0x00000005,
    Closure = 0x00000007,
    ReturnAddress = 0x00000009,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct NativeValue(i32);

pub struct Ref(pub *mut NativeValue);

impl Ref {
    fn set(&self, val: NativeValue) {

    }
}

impl NativeValue {
    pub fn box_i32(num: i32) -> Self {
        NativeValue((num << 1) | 0x0001)
    }

    pub fn wrap(num: i32) -> Self {
        NativeValue(num)
    }

    pub fn wrap_ref(r: Ref) -> Self {
        Self(r.0 as i32)
    }

    pub fn unwrap_ref(self) -> Ref {
        Ref(self.0 as *mut NativeValue)
    }

    pub fn boxed(&self) -> bool {
        (self.0 & 0x0001) == 0
    }

    pub fn unboxed(&self) -> bool {
        !self.boxed()
    }

    pub fn unwrap_int(self) -> Result<i32, InterpreterError> {
        if self.boxed() {
            Err(InterpreterError::UnexpectedValue {
                expected: "int".to_string(),
                found: "boxed value".to_string(),
            })
        } else {
            Ok(self.0 >> 1)
        }
    }
}
