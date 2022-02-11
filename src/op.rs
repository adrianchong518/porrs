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
    If(IfOp),
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
    pub(crate) fn from_str(text: &str) -> Option<Self> {
        match text {
            "dup" => Some(Self::Dup),
            "swap" => Some(Self::Swap),
            "drop" => Some(Self::Drop),
            "print" => Some(Self::Print),
            "over" => Some(Self::Over),
            "rot" => Some(Self::Rot),
            "+" => Some(Self::Plus),
            "-" => Some(Self::Subtract),
            "*" => Some(Self::Multiply),
            "divmod" => Some(Self::DivMod),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct IfStarBlock {
    pub(crate) loc: FileLocation,
    pub(crate) cond: OpBlock,
    pub(crate) inner: OpBlock,
}

#[derive(Debug)]
pub(crate) struct IfOp {
    pub(crate) if_block: OpBlock,
    pub(crate) if_star_blocks: Vec<IfStarBlock>,
    pub(crate) else_block: Option<OpBlock>,
}

impl IfOp {
    pub(crate) fn new() -> Self {
        Self {
            if_block: OpBlock::new(),
            if_star_blocks: Vec::new(),
            else_block: None,
        }
    }
}
