use lama_bc::bytecode::{InstructionPtr, Location};

use crate::{error::InterpreterError, scope::Scope, value::{NativeValue, Ref}};

pub struct ActivationRecord {
    args: Scope,
    locals: Scope,
    return_address: InstructionPtr,
}

impl ActivationRecord {
    pub fn lookup(&self, loc: &Location) -> Result<NativeValue, InterpreterError> {
        match loc {
            Location::Arg(l) => self.args.lookup(*l as usize),
            Location::Local(l) => self.locals.lookup(*l as usize),
            l => Err(InterpreterError::UnexpectedLocation(*l)),
        }
    }

    pub fn set(&mut self, loc: &Location, val: NativeValue) -> Result<(), InterpreterError> {
        match loc {
            Location::Arg(l) => self.args.set(*l as usize, val),
            Location::Local(l) => self.locals.set(*l as usize, val),
            l => Err(InterpreterError::UnexpectedLocation(*l)),
        }
    }

    pub(crate) fn lookup_ref(&mut self, loc: &Location) -> Result<Ref, InterpreterError> {
        match loc {
            Location::Arg(l) => self.args.lookup_ref(*l as usize),
            Location::Local(l) => self.locals.lookup_ref(*l as usize),
            l => Err(InterpreterError::UnexpectedLocation(*l)),
        }
    }
}

pub struct CallStack {
    records: Vec<ActivationRecord>,
}

impl CallStack {
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    pub fn begin(&mut self, args: Vec<NativeValue>, nlocals: usize, return_address: InstructionPtr) {
        let record = ActivationRecord {
            args: Scope::from(args),
            locals: Scope::new(nlocals),
            return_address,
        };
        self.records.push(record)
    }

    pub fn end(&mut self) -> Result<InstructionPtr, InterpreterError> {
        self.records
            .pop()
            .map(|rec| rec.return_address)
            .ok_or(InterpreterError::CallStackUnderflow)
    }

    pub fn top(&self) -> Result<&ActivationRecord, InterpreterError> {
        self.records
            .last()
            .ok_or(InterpreterError::CallStackUnderflow)
    }

    pub fn top_mut(&mut self) -> Result<&mut ActivationRecord, InterpreterError> {
        self.records
            .last_mut()
            .ok_or(InterpreterError::CallStackUnderflow)
    }
}
