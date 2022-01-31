use std::process::exit;
use std::rc::Rc;

use crate::lex::{Token, TokenBlock, TokenType};

#[derive(Debug)]
pub(crate) enum OpType {
    PushInt(u64),
    Intrinsic(IntrinsicType),
    If(OpBlock),
    End,
}

#[derive(Debug)]
pub(crate) enum IntrinsicType {
    Plus,
    Subtract,
    Multiply,
    DivMod,
    Print,
}

impl IntrinsicType {
    fn from_word(word: &str) -> Option<Self> {
        match word {
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
pub struct Op {
    pub(crate) typ: OpType,
    pub(crate) token: Rc<Token>,
}

#[derive(Debug)]
pub(crate) struct OpBlock {
    pub(crate) ops: Vec<Op>,
}

impl From<Vec<Op>> for OpBlock {
    fn from(ops: Vec<Op>) -> Self {
        Self { ops }
    }
}

pub(crate) fn parse_token_block(block: &TokenBlock) -> OpBlock {
    block
        .tokens
        .iter()
        .map(parse_token)
        .collect::<Vec<Op>>()
        .into()
}

fn parse_token(token: &Rc<Token>) -> Op {
    let op = match &token.typ {
        TokenType::Word => parse_word(token),
        TokenType::Int(val) => Op {
            typ: OpType::PushInt(*val),
            token: Rc::clone(token),
        },
        TokenType::If(tok_block) => Op {
            typ: OpType::If(parse_token_block(&tok_block)),
            token: Rc::clone(token),
        },
        TokenType::End => Op {
            typ: OpType::End,
            token: Rc::clone(token),
        },
    };

    log::trace!("Parsed token as operation: {:#?}", op);

    op
}

fn parse_word(token: &Rc<Token>) -> Op {
    let intrinsic_type = IntrinsicType::from_word(&token.text).unwrap_or_else(|| {
        log::error!("--- {} --- Unknown word `{}`", token.loc, token.text);
        exit(1)
    });

    Op {
        typ: OpType::Intrinsic(intrinsic_type),
        token: Rc::clone(token),
    }
}
