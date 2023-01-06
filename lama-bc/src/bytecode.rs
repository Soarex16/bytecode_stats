use std::fmt::Display;
use std::str;

pub struct ByteFile<'a> {
    string_table: &'a [u8],
    _public_symbols_table: &'a [u8],
    pub global_area_size: usize,
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
            _public_symbols_table: public_symbols_table,
            global_area_size,
            code,
        }
    }

    pub fn string<'a>(&'a self, ptr: &StringPtr) -> Result<&'a str, std::str::Utf8Error> {
        let nul_range_end = self.string_table[ptr.0..]
            .iter()
            .position(|&c| c == b'\0')
            .unwrap_or(self.string_table.len()); // default to length if no `\0` present
        str::from_utf8(&self.string_table[ptr.0..ptr.0 + nul_range_end])
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
    // Barray_patt not defined here
    StrCmp,  // =str, Bstring_patt
    String,  // #string, Bstring_tag_patt
    Array,   // #array, Barray_tag_patt
    Sexp,    // #sexp, Bsexp_tag_patt
    Boxed,   // #ref, Bboxed_patt
    UnBoxed, // #val, Bunboxed_patt
    Closure, // #fun, Bclosure_tag_patt
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum FunctionCall {
    Function { ptr: InstructionPtr, nargs: u32 },
    Library { func: String, nargs: u32 },
    BuiltIn(BuiltIn),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum BuiltIn {
    Elem,
    Sta,

    Closure,
    String,
    Array(u32),
    Sexp,

    Tag,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct StringPtr(pub usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InstructionPtr(pub usize); // instruction offset in opcode vec

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
        nargs: u32,
        nlocals: u32,
    },
    END,
    RET,
    CALL(FunctionCall),

    DROP,
    DUP,
    SWAP,
    TAG {
        tag: StringPtr,
        size: u32,
    },
    ARRAY(u32),
    PATT(Pattern),

    FAIL(u32, u32), // line number, leave a value
    LINE(u32),

    // Closures are not supported
    CBEGIN {
        nargs: u32,
        nlocals: u32,
    },
    CLOSURE {
        ptr: InstructionPtr,
        refs: Vec<Location>,
    },
    CALLC {
        nargs: u32,
    },
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
