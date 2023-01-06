use lama_bc::bytecode::{Location, StringPtr};
use thiserror::Error;

#[derive(Error, Clone, Debug)]
pub enum InterpreterError {
    #[error("unknown error ({0})")]
    Unknown(String),
    #[error("invalid string reference ({0:?})")]
    InvalidString(StringPtr),
    #[error("unexpected value (expected {expected:?}, found {found:?})")]
    UnexpectedValue { expected: String, found: String },
    #[error("unexpected location ({0:?})")]
    UnexpectedLocation(Location),
    #[error("invalid instruction offset {0})")]
    InvalidInstructionPtr(usize),
    #[error("index out of range {0})")]
    IndexOutOfRange(usize),
    #[error("invalid value access {0})")]
    InvalidValueAccess(usize),
    #[error("value stack is empty")]
    ValueStackUnderflow,
    #[error("value stack overflow")]
    ValueStackOverflow,
    #[error("call stack is empty")]
    CallStackUnderflow,
    #[error("Failure: {0}")]
    Failure(String),
    #[error("unsupported instruction {0}")]
    UnsupportedInstruction(String),
    #[error("unknown function {0}")]
    UnknownFunction(String),
}
