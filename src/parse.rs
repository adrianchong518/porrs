use std::fmt;

use crate::lex::Lexer;
use crate::op::{IfOp, Intrinsic, Op, OpBlock, OpType};
use crate::program::FileLocation;
use crate::token::{Marker, TokenType};
use crate::Error;

#[derive(Debug)]
pub(crate) enum ParsingError {
    UnexpectedMarker(Marker, String),
    UnknownWord(String),
    BlockNotClosed,
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParsingError::UnexpectedMarker(marker, msg) => {
                write!(f, "Unexpected block marker `{}`: {}", marker, msg)
            }
            ParsingError::UnknownWord(text) => write!(f, "Unknown word: `{}`", text),
            ParsingError::BlockNotClosed => {
                write!(f, "Expected an `{}` token to end a block", Marker::End)
            }
        }
    }
}

#[derive(Debug)]
enum Parsed {
    Op(Op),
    Marker { marker: Marker, loc: FileLocation },
}

pub(crate) struct Parser {
    lexer: Lexer,
}

impl Parser {
    pub(crate) fn from_lexer(lexer: Lexer) -> Self {
        Self { lexer }
    }

    pub(crate) fn into_root_block(mut self) -> Result<OpBlock, Error> {
        let mut ops = Vec::new();

        while let Some(parsed) = self.parse_next_token()? {
            match parsed {
                Parsed::Op(op) => {
                    log::trace!("Parsed token as operation: {:#?}", op);
                    ops.push(op);
                }

                Parsed::Marker { marker, loc } => {
                    let msg = format!("`{}` must be associated with a block", marker);
                    return Err(
                        Error::from(ParsingError::UnexpectedMarker(marker, msg)).push_loc(loc)
                    );
                }
            }
        }

        Ok(OpBlock(ops))
    }

    fn parse_next_token(&mut self) -> Result<Option<Parsed>, Error> {
        let token = if let Some(tok) = self.lexer.next_token()? {
            tok
        } else {
            return Ok(None);
        };

        let parsed = match token.typ {
            TokenType::Word(ref text) => Parsed::Op(self.parse_word(&text, token.loc)?),

            TokenType::Int(val) => Parsed::Op(Op {
                typ: OpType::PushInt(val),
                loc: token.loc,
            }),

            TokenType::Marker(marker) => self.parse_block_marker(marker, token.loc)?,
        };

        log::trace!("Parsed token: {:#?}", parsed);

        Ok(Some(parsed))
    }

    fn parse_word(&mut self, text: &str, loc: FileLocation) -> Result<Op, Error> {
        match Intrinsic::from_str(text) {
            Some(intr) => Ok(Op {
                typ: OpType::Intrinsic(intr),
                loc,
            }),
            None => Err(Error::from(ParsingError::UnknownWord(text.to_owned())).push_loc(loc)),
        }
    }

    fn parse_block_marker(&mut self, marker: Marker, loc: FileLocation) -> Result<Parsed, Error> {
        match marker {
            Marker::If => self.parse_if(loc).map(|op| Parsed::Op(op)),
            Marker::Else | Marker::End => Ok(Parsed::Marker { marker, loc }),
        }
    }

    fn parse_if(&mut self, if_loc: FileLocation) -> Result<Op, Error> {
        enum ParseState {
            If,
            Else,
        }

        let mut if_op = IfOp {
            if_block: OpBlock(Vec::new()),
            else_block: None,
        };

        let mut parse_state = ParseState::If;

        while let Some(parsed) = self.parse_next_token()? {
            match parsed {
                Parsed::Op(op) => match parse_state {
                    ParseState::If => if_op.if_block.0.push(op),
                    ParseState::Else => if_op.else_block.as_mut().unwrap().0.push(op),
                },

                Parsed::Marker { marker, loc } => match marker {
                    Marker::Else => match parse_state {
                        ParseState::If => {
                            parse_state = ParseState::Else;
                            if_op.else_block = Some(OpBlock(Vec::new()));
                        }
                        ParseState::Else => {
                            return Err(Error::from(ParsingError::UnexpectedMarker(
                                marker,
                                format!(
                                    "`{}` cannot appear twice for one `{}` block",
                                    Marker::Else,
                                    Marker::If
                                ),
                            ))
                            .push_loc(loc));
                        }
                    },

                    Marker::End => {
                        return Ok(Op {
                            typ: OpType::If(if_op),
                            loc: if_loc,
                        })
                    }

                    _ => {
                        let msg = format!(
                            "`{}` cannot appear with in an `{}` block",
                            marker,
                            Marker::If
                        );
                        return Err(
                            Error::from(ParsingError::UnexpectedMarker(marker, msg)).push_loc(loc)
                        );
                    }
                },
            }
        }

        Err(Error::from(ParsingError::BlockNotClosed).push_loc(self.lexer.current_location()))
    }
}
