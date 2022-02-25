use std::fmt;

use crate::program::FileLocation;

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) typ: TokenType,
    pub(crate) loc: FileLocation,
}

#[derive(Debug)]
pub(crate) enum TokenType {
    Word(Word),
    Int(u64),
    Marker(Marker),
}

impl<T: AsRef<str>> From<T> for TokenType {
    fn from(text: T) -> Self {
        let text = text.as_ref();

        if let Ok(marker) = Marker::try_from(text) {
            Self::Marker(marker)
        } else if let Ok(val) = text.parse::<u64>() {
            Self::Int(val)
        } else {
            Self::Word(Word(text.to_owned()))
        }
    }
}

#[derive(Debug)]
pub(crate) struct Word(String);

impl Word {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum Marker {
    If,
    IfStar,
    Else,
    While,
    Do,
    End,
}

impl Marker {
    const IF_TEXT: &'static str = "if";
    const IF_STAR_TEXT: &'static str = "if*";
    const ELSE_TEXT: &'static str = "else";
    const WHILE_TEXT: &'static str = "while";
    const DO_TEXT: &'static str = "do";
    const END_TEXT: &'static str = "end";

    const fn as_str(&self) -> &'static str {
        match self {
            Self::If => Self::IF_TEXT,
            Self::IfStar => Self::IF_STAR_TEXT,
            Self::Else => Self::ELSE_TEXT,
            Self::While => Self::WHILE_TEXT,
            Self::Do => Self::DO_TEXT,
            Self::End => Self::END_TEXT,
        }
    }
}

pub(crate) struct InvalidMarkerError;

impl TryFrom<&str> for Marker {
    type Error = InvalidMarkerError;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        Ok(match text {
            Self::IF_TEXT => Self::If,
            Self::IF_STAR_TEXT => Self::IfStar,
            Self::ELSE_TEXT => Self::Else,
            Self::WHILE_TEXT => Self::While,
            Self::DO_TEXT => Self::Do,
            Self::END_TEXT => Self::End,
            _ => return Err(InvalidMarkerError),
        })
    }
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
