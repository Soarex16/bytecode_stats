use std::fmt::Display;

pub struct ByteFile<'a> {
    string_table: &'a [u8],
    public_symbols_table: &'a [u8],
    global_area_size: usize,
    pub code: Vec<OpCode>,
}

impl ByteFile<'_> {
    pub fn new<'a>(
        string_table: &'a [u8],
        public_symbols_table: &'a [u8],
        global_area_size: usize,
        code: Vec<OpCode>,
    ) -> ByteFile<'a> {
        ByteFile {
            string_table,
            public_symbols_table,
            global_area_size,
            code,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BinOp {
    Plus,  // +
    Minus, // -
    Mul,   // *
    Div,   // /
    Mod,   // %
    Lt,    // <
    LtEq,  // <=
    Gt,    // >
    GtEq,  // >=
    Eq,    // ==
    Neq,   // !=
    And,   // &&
    Or,    // !!
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Pattern {
    StrCmp,  // =str
    String,  // #string
    Array,   // #array
    Sexp,    // #sexp
    Boxed,   // #ref
    UnBoxed, // #val
    Closure, // #fun
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum JumpCondition {
    Zero,
    NotZero,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Location {
    Arg(u32),
    Local(u32),
    Global(u32),
    Closure(u32),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuiltIn {
    Read,
    Write,
    Length,
    String,
    Array(u32),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StringPtr(pub usize);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InstructionPtr(pub u32);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum OpCode {
    BINOP(BinOp),
    CONST(i32),
    STRING(StringPtr),
    SEXP {
        tag: StringPtr,
        size: u32,
    },

    LD(Location),
    LDA(Location),
    ST(Location),
    STI,
    STA,
    ELEM,

    JMP(InstructionPtr),
    CJMP(JumpCondition, InstructionPtr),

    BEGIN {
        args_count: u32,
        locals_count: u32,
    },
    END,
    RET,
    CALL {
        ptr: InstructionPtr,
        args_count: u32,
    },

    DROP,
    DUP,
    SWAP,
    TAG {
        tag: StringPtr,
        size: u32,
    },
    ARRAY(u32),
    PATT(Pattern),

    FAIL(u32),
    LINE(u32),

    BUILTIN(BuiltIn),

    // Closures are not supported
    CBEGIN {
        args_count: u32,
        locals_count: u32,
    },
    CLOSURE {
        ptr: InstructionPtr,
        refs: Vec<Location>,
    },
    CALLC {
        args_count: u32,
    },
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
