use std::fmt;

use crate::program::FileLocation;

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) typ: TokenType,
    pub(crate) loc: FileLocation,
}

#[derive(Debug)]
pub(crate) enum TokenType {
    Word(String),
    Int(u64),
    Marker(Marker),
}

impl<'token> TokenType {
    pub(crate) fn from_str(text: &str) -> Self {
        if let Some(marker) = Marker::from_str(text) {
            Self::Marker(marker)
        } else if let Ok(val) = text.parse::<u64>() {
            Self::Int(val)
        } else {
            Self::Word(text.to_owned())
        }
    }
}

#[derive(Debug)]
pub(crate) enum Marker {
    If,
    Else,
    End,
}

impl Marker {
    const IF_TEXT: &'static str = "if";
    const ELSE_TEXT: &'static str = "else";
    const END_TEXT: &'static str = "end";

    fn from_str(text: &str) -> Option<Self> {
        match text {
            Self::IF_TEXT => Some(Self::If),
            Self::ELSE_TEXT => Some(Self::Else),
            Self::END_TEXT => Some(Self::End),
            _ => None,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::If => Self::IF_TEXT,
            Self::Else => Self::ELSE_TEXT,
            Self::End => Self::END_TEXT,
        }
    }
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
