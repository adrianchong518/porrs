use crate::program::FileLocation;

#[derive(Debug)]
pub(crate) struct OpBlock(pub(crate) Vec<Op>);

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
    Plus,
    Subtract,
    Multiply,
    DivMod,
    Print,
}

impl Intrinsic {
    pub(crate) fn from_str(text: &str) -> Option<Self> {
        match text {
            "+" => Some(Self::Plus),
            "-" => Some(Self::Subtract),
            "*" => Some(Self::Multiply),
            "divmod" => Some(Self::DivMod),
            "print" => Some(Self::Print),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct IfOp {
    pub(crate) if_block: OpBlock,
    pub(crate) else_block: Option<OpBlock>,
}
