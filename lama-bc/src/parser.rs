use crate::bytecode::{
    BinOp, BuiltIn, ByteFile, InstructionPtr, JumpCondition, Location, OpCode, Pattern, StringPtr,
};
use std::{mem::size_of, str, usize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BytecodeParseError {
    #[error("invalid file header")]
    InvalidHeader,
    #[error("malformed file")]
    MalformedFile,
    #[error("invalid opcode `{0:#04X?}`")]
    InvalidOpcode(u8),
    #[error("unknown builtin opcode `{0:#04X?}`")]
    UnknownBuiltin(u8),
    #[error("invalid location `{0:#04X?}`")]
    InvalidLoc(u8),
    #[error("unexpected end of file")]
    UnexpectedEof,
    #[error("unsupported instruction {insn:?}")]
    UnsupportedInstruction { insn: &'static str },
    #[error("unknown data store error")]
    Unknown,
}

pub fn parse(bytes: &Vec<u8>) -> Result<ByteFile, BytecodeParseError> {
    const HEADER_SIZE: usize = 3 * size_of::<u32>();

    if bytes.len() < HEADER_SIZE {
        return Err(BytecodeParseError::InvalidHeader);
    }

    let (header, bytes) = bytes.split_at(HEADER_SIZE);

    let string_table_size: usize = read_u32(&header[..4]) as usize;
    let global_area_size: usize = read_u32(&header[4..8]) as usize;
    let public_symbols_number: usize = read_u32(&header[8..12]) as usize;

    if bytes.len() < HEADER_SIZE + string_table_size + public_symbols_number {
        return Err(BytecodeParseError::MalformedFile);
    }

    // Still a mystery why public_symbols_number * 2
    let (public_symbols_table, bytes) =
        bytes.split_at(size_of::<u32>() * public_symbols_number * 2);
    let (string_table, code) = bytes.split_at(string_table_size);

    let code = read_code(code)?;
    Ok(ByteFile::new(
        string_table,
        public_symbols_table,
        global_area_size,
        code,
    ))
}

fn read_code(input: &[u8]) -> Result<Vec<OpCode>, BytecodeParseError> {
    const OPS: [BinOp; 13] = [
        BinOp::Plus,
        BinOp::Minus,
        BinOp::Mul,
        BinOp::Div,
        BinOp::Mod,
        BinOp::Lt,
        BinOp::LtEq,
        BinOp::Gt,
        BinOp::GtEq,
        BinOp::Eq,
        BinOp::Neq,
        BinOp::And,
        BinOp::Or,
    ];

    const PATTERNS: [Pattern; 7] = [
        Pattern::StrCmp,
        Pattern::String,
        Pattern::Array,
        Pattern::Sexp,
        Pattern::Boxed,
        Pattern::UnBoxed,
        Pattern::Closure,
    ];

    let mut code = vec![];

    let mut position = 0;

    let read_bytes = |pos: &mut usize, cnt: usize| {
        if *pos + cnt <= input.len() {
            let x = &input[*pos..*pos + cnt];
            *pos += cnt;
            Ok(x)
        } else {
            Err(BytecodeParseError::UnexpectedEof)
        }
    };

    let parse_u32 = |pos: &mut usize| read_bytes(pos, 4).map(read_u32);

    let parse_i32 = |pos: &mut usize| read_bytes(pos, 4).map(read_i32);

    let parse_byte = |pos: &mut usize| read_bytes(pos, 1).map(|b| b[0]);

    let parse_loc = |pos: &mut usize, loc: u8| {
        parse_u32(pos).and_then(|x| {
            let x = x;
            match loc {
                0 => Ok(Location::Global(x)),
                1 => Ok(Location::Local(x)),
                2 => Ok(Location::Arg(x)),
                3 => Ok(Location::Closure(x)),
                _ => Err(BytecodeParseError::InvalidLoc(loc)),
            }
        })
    };

    let parse_string = |pos: &mut usize| parse_u32(pos).map(|x| StringPtr(x as usize));

    loop {
        let x = parse_byte(&mut position)?;

        let high = (x & 0xF0) >> 4;
        let low = x & 0x0F;

        let opcode = match high {
            0 => OpCode::BINOP(OPS[low as usize - 1]),
            1 => match low {
                0 => OpCode::CONST(parse_i32(&mut position)?),
                1 => OpCode::STRING(parse_string(&mut position)?),
                2 => OpCode::SEXP {
                    tag: parse_string(&mut position)?,
                    size: parse_u32(&mut position)?,
                },
                3 => OpCode::STI,
                4 => OpCode::STA,
                5 => OpCode::JMP(InstructionPtr(parse_u32(&mut position)?)),
                6 => OpCode::END,
                7 => OpCode::RET,
                8 => OpCode::DROP,
                9 => OpCode::DUP,
                10 => OpCode::SWAP,
                11 => OpCode::ELEM,

                _ => Err(BytecodeParseError::InvalidOpcode(x))?,
            },
            2 => OpCode::LD(parse_loc(&mut position, low)?),
            3 => OpCode::LDA(parse_loc(&mut position, low)?),
            4 => OpCode::ST(parse_loc(&mut position, low)?),
            5 => {
                match low {
                    0 => OpCode::CJMP(
                        JumpCondition::Zero,
                        InstructionPtr(parse_u32(&mut position)?),
                    ),
                    1 => OpCode::CJMP(
                        JumpCondition::NotZero,
                        InstructionPtr(parse_u32(&mut position)?),
                    ),
                    2 => OpCode::BEGIN {
                        args_count: parse_u32(&mut position)?,
                        locals_count: parse_u32(&mut position)?,
                    },
                    3 => OpCode::CBEGIN {
                        args_count: parse_u32(&mut position)?,
                        locals_count: parse_u32(&mut position)?,
                    },
                    4 => {
                        let ptr = InstructionPtr(parse_u32(&mut position)?);
                        let size = parse_u32(&mut position)?;
                        let mut closure = vec![];
                        for _ in 0..size {
                            let loc = parse_byte(&mut position)?;
                            closure.push(parse_loc(&mut position, loc)?)
                        }
                        OpCode::CLOSURE {
                            ptr,
                            refs: closure,
                        }
                    }
                    5 => OpCode::CALLC {
                        args_count: parse_u32(&mut position)?,
                    },
                    // 3 => Err(BytecodeParseError::UnsupportedInstruction { insn: "CBEGIN" })?,
                    // 4 => Err(BytecodeParseError::UnsupportedInstruction { insn: "CLOSURE" })?,
                    // 5 => Err(BytecodeParseError::UnsupportedInstruction { insn: "CALLC" })?,
                    6 => OpCode::CALL {
                        ptr: InstructionPtr(parse_u32(&mut position)?),
                        args_count: parse_u32(&mut position)?,
                    },
                    7 => OpCode::TAG {
                        tag: parse_string(&mut position)?,
                        size: parse_u32(&mut position)?,
                    },
                    8 => OpCode::ARRAY(parse_u32(&mut position)?),
                    9 => {
                        let pos = parse_u32(&mut position)?;
                        parse_u32(&mut position)?; // skip "leave a value" param
                        OpCode::FAIL(pos)
                    }
                    10 => OpCode::LINE(parse_u32(&mut position)?),

                    _ => Err(BytecodeParseError::InvalidOpcode(x))?,
                }
            }
            6 => OpCode::PATT(PATTERNS[low as usize]),
            7 => {
                let builtin = match low {
                    0 => BuiltIn::Read,
                    1 => BuiltIn::Write,
                    2 => BuiltIn::Length,
                    3 => BuiltIn::String,
                    4 => BuiltIn::Array(parse_u32(&mut position)?),
                    _ => Err(BytecodeParseError::UnknownBuiltin(low))?,
                };
                OpCode::BUILTIN(builtin)
            }
            15 => break,
            _ => Err(BytecodeParseError::InvalidOpcode(x))?,
        };

        code.push(opcode);
    }

    Ok(code)
}

fn read_u32(input: &[u8]) -> u32 {
    u32::from_le_bytes(input.try_into().unwrap())
}

fn read_i32(input: &[u8]) -> i32 {
    i32::from_ne_bytes(input.try_into().unwrap())
}
