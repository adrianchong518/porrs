pub(crate) enum OpType {
    PushInt(u64),
    Intrinsic(IntrinsicType),
}

pub(crate) enum IntrinsicType {
    Plus,
    Print,
}

pub struct Op {
    pub(crate) typ: OpType,
}
