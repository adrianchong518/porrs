use std::process::exit;

use crate::lex::{Token, TokenType};

#[derive(Debug)]
pub(crate) enum OpType {
    PushInt(u64),
    Intrinsic(IntrinsicType),
}

#[derive(Clone, Copy, Debug)]
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
    pub(crate) token: Token,
}

pub(crate) fn parse_token(token: Token) -> Op {
    let op = match token.typ {
        TokenType::Word => parse_word(token),
        TokenType::Int(val) => Op {
            typ: OpType::PushInt(val),
            token,
        },
    };

    log::trace!("Parsed token as operation: {:?}", op);

    op
}

fn parse_word(token: Token) -> Op {
    let intrinsic_type = IntrinsicType::from_word(&token.text).unwrap_or_else(|| {
        log::error!("--- {} --- Unknown word `{}`", token.loc, token.text);
        exit(1)
    });

    Op {
        typ: OpType::Intrinsic(intrinsic_type),
        token,
    }
}
