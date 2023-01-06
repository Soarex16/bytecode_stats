use std::rc::Rc;

use lama_bc::bytecode::*;

use crate::{
    call_stack::CallStack, env::Environment, error::InterpreterError, scope::Scope, stack::Stack,
    value::Value,
};

pub struct Interpreter<'a> {
    ip: usize,
    bf: &'a ByteFile<'a>,
    stack: Stack,
    call_stack: CallStack,
    globals: Scope,
    env: Box<dyn Environment>,
}

impl Interpreter<'_> {
    pub fn new<'a>(bf: &'a ByteFile, env: Box<dyn Environment>) -> Interpreter<'a> {
        Interpreter {
            globals: Scope::new(bf.global_area_size),
            call_stack: CallStack::new(),
            stack: Stack::new(),
            ip: 0,
            env,
            bf,
        }
    }

    fn begin(&mut self, nargs: usize, nlocals: usize) -> Result<(), InterpreterError> {
        let return_address = InstructionPtr(self.stack.pop()?.unwrap_return_addr()?);
        self.call_stack
            .begin(self.stack.take(nargs)?, nlocals, return_address);
        Ok(())
    }

    fn end(&mut self) -> Result<(), InterpreterError> {
        let ret = self.call_stack.end()?;
        self.jump(&ret)
    }

    fn call(&mut self, ptr: &InstructionPtr) -> Result<(), InterpreterError> {
        self.stack.push(Value::ReturnAddress(self.ip));
        self.jump(ptr)
    }

    fn lookup(&self, loc: &Location) -> Result<Value, InterpreterError> {
        match loc {
            Location::Global(l) => self.globals.lookup(*l as usize),
            Location::Closure(_) => Err(InterpreterError::Unknown(
                "Closures are not supported".to_string(),
            )),
            l => self.call_stack.top()?.lookup(l),
        }
    }

    fn set(&mut self, loc: &Location, val: Value) -> Result<(), InterpreterError> {
        match loc {
            Location::Closure(_) => Err(InterpreterError::Unknown(
                "Closures are not supported".to_string(),
            )),
            Location::Global(l) => self.globals.set(*l as usize, val),
            l => self.call_stack.top_mut()?.set(l, val),
        }
    }

    pub fn run(&mut self, args: Vec<String>) -> Result<(), InterpreterError> {
        let nargs = args.len() as i32;
        self.stack.push(Value::Array(Rc::new(
            args.into_iter().map(Value::String).collect(),
        ))); // args
        self.stack.push(Value::Int(nargs)); // argc
        self.stack.push(Value::ReturnAddress(self.bf.code.len())); // main

        while self.ip < self.bf.code.len() {
            let opcode = &self.bf.code[self.ip];

            // println!("ip: {}", self.ip);
            // println!("stack: {:?}", self.stack);
            // println!("opcode: {:?}", opcode);

            match opcode {
                OpCode::CONST(x) => self.stack.push(Value::Int(*x)),
                OpCode::BINOP(op) => {
                    let rhs = self.stack.pop()?;
                    let lhs = self.stack.pop()?;
                    self.stack.push(self.eval_bin_op(op, lhs, rhs)?)
                }

                OpCode::JMP(ptr) => {
                    self.jump(ptr)?;
                    continue;
                }
                OpCode::CJMP(cond, ptr) => {
                    let v = self.stack.pop()?.unwrap_int()?;
                    match cond {
                        JumpCondition::Zero => {
                            if v == 0 {
                                self.jump(ptr)?;
                                continue;
                            }
                        }
                        JumpCondition::NotZero => {
                            if v != 0 {
                                self.jump(ptr)?;
                                continue;
                            }
                        }
                    }
                }

                OpCode::BEGIN { nargs, nlocals } => {
                    self.begin(*nargs as usize, *nlocals as usize)?
                }
                OpCode::END => {
                    self.end()?;
                }
                OpCode::CALL(FunctionCall::Function { ptr, nargs: _ }) => {
                    self.call(ptr)?;
                    continue;
                }
                OpCode::CALL(FunctionCall::Library { func, nargs }) => {
                    let res = self.env.library(func, *nargs as usize, &mut self.stack)?;
                    self.stack.push(res);
                }
                OpCode::CALL(FunctionCall::BuiltIn(b)) => {
                    let res = self.env.built_in(*b, &mut self.stack)?;
                    self.stack.push(res);
                }
                OpCode::FAIL(line, _) => {
                    let x = self.stack.pop()?;
                    return Err(InterpreterError::Failure(format!(
                        "matching value {} failure at {}",
                        x, line
                    )));
                }

                OpCode::DROP => self.stack.drop()?,
                OpCode::DUP => self.stack.dup()?,
                OpCode::SWAP => self.stack.swap()?,

                OpCode::LD(loc) => self.stack.push(self.lookup(loc)?),
                OpCode::ST(loc) => {
                    let val = self.stack.pop()?;
                    self.stack.push(val.clone());
                    self.set(loc, val)?
                }

                OpCode::STI => {
                    let val = self.stack.pop()?;
                    let loc = self.stack.pop()?.unwrap_ref()?;
                    self.set(&loc, val)?
                }
                OpCode::SEXP { tag, size } => {
                    let mut vals = self.stack.take(*size as usize)?;
                    vals.reverse();
                    let tag_label = self
                        .bf
                        .string(tag)
                        .map_err(|_| InterpreterError::InvalidString(*tag))?
                        .to_owned();
                    self.stack.push(Value::Sexp(*tag, tag_label, Rc::new(vals)))
                }
                OpCode::STRING(s) => {
                    let str = self
                        .bf
                        .string(s)
                        .map_err(|_| InterpreterError::InvalidString(*s))?
                        .to_owned();
                    self.stack.push(Value::String(str))
                }
                OpCode::LDA(loc) => self.stack.push(Value::Ref(*loc)),
                OpCode::STA => todo!(),
                OpCode::ELEM => {
                    let idx = self.stack.pop()?.unwrap_int()? as usize;
                    let val = self.stack.pop()?;
                    let item = match val {
                        Value::Sexp(_, _, ref items) => {
                            items.get(idx).ok_or(InterpreterError::IndexOutOfRange(idx))
                        }
                        Value::Array(ref items) => {
                            items.get(idx).ok_or(InterpreterError::IndexOutOfRange(idx))
                        }
                        _ => Err(InterpreterError::UnexpectedValue {
                            expected: "array or sexp".to_string(),
                            found: val.to_string(),
                        }),
                    }?;
                    self.stack.push(item.clone());
                }

                OpCode::TAG { tag, size } => {
                    let matches = match self.stack.pop()? {
                        Value::Sexp(t, _, items) if t == *tag && items.len() == *size as usize => 1,
                        _ => 0,
                    };
                    self.stack.push(Value::Int(matches))
                }
                OpCode::ARRAY(size) => {
                    let matches = match self.stack.pop()? {
                        Value::Array(items) if items.len() == *size as usize => 1,
                        _ => 0,
                    };
                    self.stack.push(Value::Int(matches))
                }
                OpCode::PATT(p) => {
                    let val = self.stack.pop()?;
                    let matches = self.check_pattern(p, val)?;
                    self.stack.push(matches);
                }

                OpCode::CBEGIN { .. } => {
                    return Err(InterpreterError::UnsupportedInstruction(
                        "CBEGIN".to_string(),
                    ))
                }
                OpCode::CLOSURE { .. } => {
                    return Err(InterpreterError::UnsupportedInstruction(
                        "CLOSURE".to_string(),
                    ))
                }
                OpCode::CALLC { .. } => {
                    return Err(InterpreterError::UnsupportedInstruction(
                        "CALLC".to_string(),
                    ))
                }

                OpCode::LINE(_) => (),
                OpCode::RET => (), // unused
            }
            self.ip += 1;
        }
        Ok(())
    }

    fn jump(&mut self, ptr: &InstructionPtr) -> Result<(), InterpreterError> {
        let new_ip = ptr.0;
        if new_ip > self.bf.code.len() {
            Err(InterpreterError::InvalidInstructionPtr(new_ip))?;
        }

        self.ip = new_ip;
        Ok(())
    }

    fn eval_bin_op(&self, op: &BinOp, lhs: Value, rhs: Value) -> Result<Value, InterpreterError> {
        let l = lhs.unwrap_int()?;
        let r = rhs.unwrap_int()?;

        let res = match op {
            BinOp::Plus => l + r,
            BinOp::Minus => l - r,
            BinOp::Mul => l * r,
            BinOp::Div => l / r,
            BinOp::Mod => l % r,
            BinOp::Lt => i32::from(l < r),
            BinOp::LtEq => i32::from(l <= r),
            BinOp::Gt => i32::from(l > r),
            BinOp::GtEq => i32::from(l >= r),
            BinOp::Eq => i32::from(l == r),
            BinOp::Neq => i32::from(l != r),
            BinOp::And => i32::from(l & r != 0),
            BinOp::Or => i32::from(l | r != 0),
        };

        Ok(Value::Int(res))
    }

    fn check_pattern(&mut self, p: &Pattern, val: Value) -> Result<Value, InterpreterError> {
        let result = match p {
            Pattern::String => match val {
                Value::String(_) => Ok(1),
                _ => Ok(0),
            },
            Pattern::Array => match val {
                Value::Array(_) => Ok(1),
                _ => Ok(0),
            },
            Pattern::Sexp => match val {
                Value::Sexp(_, _, _) => Ok(1),
                _ => Ok(0),
            },
            Pattern::Boxed => match val {
                Value::Int(_) => Ok(0),
                _ => Ok(1),
            },
            Pattern::UnBoxed => match val {
                Value::Int(_) => Ok(1),
                _ => Ok(0),
            },
            Pattern::Closure => Err(InterpreterError::UnsupportedInstruction(
                "PATT(Closure)".to_string(),
            )),
            Pattern::StrCmp => {
                let str = self.stack.pop()?;
                match (val, str) {
                    (Value::String(x), Value::String(y)) if x == y => Ok(1),
                    _ => Ok(0),
                }
            }
        }?;
        Ok(Value::Int(result))
    }
}
