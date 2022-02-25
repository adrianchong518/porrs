use std::fmt;

use crate::program::FileLocation;

#[derive(Debug)]
pub(crate) struct OpBlock(Vec<Op>);

impl OpBlock {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, op: Op) {
        self.0.push(op)
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Op> {
        self.0.iter()
    }
}

#[derive(Debug)]
pub struct Op {
    pub(crate) typ: OpType,
    pub(crate) loc: FileLocation,
}

#[derive(Debug)]
pub(crate) enum OpType {
    PushInt(u64),
    Intrinsic(Intrinsic),
    If(If),
    While(While),
}

#[derive(Debug)]
pub(crate) enum Intrinsic {
    Dup,
    Swap,
    Drop,
    Print,
    Over,
    Rot,
    Plus,
    Subtract,
    Multiply,
    DivMod,
}

impl Intrinsic {
    const DUP_TEXT: &'static str = "dup";
    const SWAP_TEXT: &'static str = "swap";
    const DROP_TEXT: &'static str = "drop";
    const PRINT_TEXT: &'static str = "print";
    const OVER_TEXT: &'static str = "over";
    const ROT_TEXT: &'static str = "rot";
    const PLUS_TEXT: &'static str = "+";
    const SUBTRACT_TEXT: &'static str = "-";
    const MULTIPLY_TEXT: &'static str = "*";
    const DIV_MOD_TEXT: &'static str = "divmod";

    pub(crate) fn as_str(&self) -> &str {
        match self {
            Self::Dup => Self::DUP_TEXT,
            Self::Swap => Self::SWAP_TEXT,
            Self::Drop => Self::DROP_TEXT,
            Self::Print => Self::PRINT_TEXT,
            Self::Over => Self::OVER_TEXT,
            Self::Rot => Self::ROT_TEXT,
            Self::Plus => Self::PLUS_TEXT,
            Self::Subtract => Self::SUBTRACT_TEXT,
            Self::Multiply => Self::MULTIPLY_TEXT,
            Self::DivMod => Self::DIV_MOD_TEXT,
        }
    }
}

pub(crate) struct InvalidIntrinsicError;

impl TryFrom<&str> for Intrinsic {
    type Error = InvalidIntrinsicError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        Ok(match text {
            Self::DUP_TEXT => Self::Dup,
            Self::SWAP_TEXT => Self::Swap,
            Self::DROP_TEXT => Self::Drop,
            Self::PRINT_TEXT => Self::Print,
            Self::OVER_TEXT => Self::Over,
            Self::ROT_TEXT => Self::Rot,
            Self::PLUS_TEXT => Self::Plus,
            Self::SUBTRACT_TEXT => Self::Subtract,
            Self::MULTIPLY_TEXT => Self::Multiply,
            Self::DIV_MOD_TEXT => Self::DivMod,
            _ => return Err(InvalidIntrinsicError),
        })
    }
}

impl fmt::Display for Intrinsic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug)]
pub(crate) struct IfStarBlock {
    pub(crate) loc: FileLocation,
    pub(crate) cond: OpBlock,
    pub(crate) inner: OpBlock,
}

#[derive(Debug)]
pub(crate) struct If {
    pub(crate) if_block: OpBlock,
    pub(crate) if_star_blocks: Vec<IfStarBlock>,
    pub(crate) else_block: Option<OpBlock>,
}

impl If {
    pub(crate) fn new() -> Self {
        Self {
            if_block: OpBlock::new(),
            if_star_blocks: Vec::new(),
            else_block: None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct While {
    pub(crate) cond_block: OpBlock,
    pub(crate) do_loc: Option<FileLocation>,
    pub(crate) do_block: OpBlock,
}

impl While {
    pub(crate) fn new() -> Self {
        Self {
            cond_block: OpBlock::new(),
            do_loc: None,
            do_block: OpBlock::new(),
        }
    }
}
