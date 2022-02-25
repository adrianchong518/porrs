use std::fmt;

use crate::error::InfoKind;
use crate::lex::Lexer;
use crate::op::{If, Intrinsic, Op, OpBlock, OpType, While};
use crate::program::FileLocation;
use crate::token::{Marker, Token, TokenType};
use crate::{Error, Result};

#[derive(Debug)]
pub(crate) enum ParsingError {
    UnexpectedMarker(Marker, UnexpectedMarker),
    MissingMarker(Marker, MissingMarker),
    UnknownWord(String),
}

impl ParsingError {
    fn into_error(self) -> Error {
        Error::from(self)
    }
}

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedMarker(marker, err) => {
                write!(f, "Unexpected token `{}`: {}", marker, err)
            }

            Self::MissingMarker(marker, err) => {
                write!(f, "Missing expected token `{}`: {}", marker, err)
            }

            Self::UnknownWord(text) => write!(f, "Unknown word: `{}`", text),
        }
    }
}

#[derive(Debug)]
pub(crate) enum MissingMarker {
    BlockNotClosed,
    RequiredByBlock(Marker, Marker),
}

impl MissingMarker {
    fn into_error(self, marker: Marker) -> Error {
        ParsingError::MissingMarker(marker, self).into_error()
    }
}

impl fmt::Display for MissingMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlockNotClosed => {
                write!(f, "Expected an `{}` token to end the block", Marker::End)
            }

            Self::RequiredByBlock(marker, block) => {
                write!(
                    f,
                    "`{}` block is required to contain a `{}` block",
                    block, marker
                )
            }
        }
    }
}

#[derive(Debug)]
pub(crate) enum UnexpectedMarker {
    General(String),
    FreeFloating(Marker),
    Repeated(Marker, Marker),
    NotApplicable(Marker, Marker),
}

impl UnexpectedMarker {
    fn into_error(self, marker: Marker) -> Error {
        ParsingError::UnexpectedMarker(marker, self).into_error()
    }
}

impl fmt::Display for UnexpectedMarker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::General(msg) => write!(f, "{}", msg),

            Self::FreeFloating(marker) => {
                write!(f, "`{}` must be associated with a block", marker)
            }

            Self::Repeated(marker, block) => write!(
                f,
                "`{}` cannot appear twice in succession within the same `{}` block",
                marker, block
            ),

            Self::NotApplicable(marker, block) => {
                write!(f, "`{}` cannot appear with in an `{}` block", marker, block)
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

    pub(crate) fn into_root_block(mut self) -> Result<OpBlock> {
        let mut op_block = OpBlock::new();

        while let Some(parsed) = self.parse_next_token()? {
            match parsed {
                Parsed::Op(op) => {
                    log::trace!("Parsed token as operation: {:#?}", op);
                    op_block.push(op);
                }

                Parsed::Marker { marker, loc } => {
                    return Err(UnexpectedMarker::FreeFloating(marker)
                        .into_error(marker)
                        .add_loc(loc));
                }
            }
        }

        Ok(op_block)
    }

    fn parse_next_token(&mut self) -> Result<Option<Parsed>> {
        self.lexer
            .consume_token()?
            .map(|token| self.parse_token(token))
            .transpose()
    }

    fn parse_token(&mut self, token: Token) -> Result<Parsed> {
        let parsed = match token.typ {
            TokenType::Word(word) => Parsed::Op(self.parse_word(word.as_str(), token.loc)?),

            TokenType::Int(val) => Parsed::Op(Op {
                typ: OpType::PushInt(val),
                loc: token.loc,
            }),

            TokenType::Marker(marker) => self.parse_marker(marker, token.loc)?,
        };

        log::trace!("Parsed token: {:#?}", parsed);

        Ok(parsed)
    }

    fn parse_word(&mut self, text: &str, loc: FileLocation) -> Result<Op> {
        match Intrinsic::try_from(text) {
            Ok(intr) => Ok(Op {
                typ: OpType::Intrinsic(intr),
                loc,
            }),
            Err(_) => Err(ParsingError::UnknownWord(text.to_owned())
                .into_error()
                .add_loc(loc)),
        }
    }

    fn parse_marker(&mut self, marker: Marker, loc: FileLocation) -> Result<Parsed> {
        match marker {
            Marker::If => self.parse_if(loc).map(|op| Parsed::Op(op)),
            Marker::While => self.parse_while(loc).map(|op| Parsed::Op(op)),
            _ => Ok(Parsed::Marker { marker, loc }),
        }
    }

    fn parse_if(&mut self, if_loc: FileLocation) -> Result<Op> {
        enum ParseState {
            If,
            IfStar,
            Else,
        }

        let mut if_op = If::new();
        let mut parse_state = ParseState::If;

        while let Some(parsed) = self.parse_next_token()? {
            match parsed {
                Parsed::Op(op) => match parse_state {
                    ParseState::If => if_op.if_block.push(op),
                    ParseState::IfStar => if_op.if_star_blocks.last_mut().unwrap().inner.push(op),
                    ParseState::Else => if_op.else_block.as_mut().unwrap().push(op),
                },

                Parsed::Marker { marker, loc } => match marker {
                    Marker::Else => match parse_state {
                        ParseState::If | ParseState::IfStar => {
                            parse_state = ParseState::Else;
                            if_op.else_block = Some(OpBlock::new());
                        }

                        ParseState::Else => {
                            return Err(UnexpectedMarker::Repeated(Marker::Else, Marker::If)
                                .into_error(Marker::Else)
                                .add_loc(loc)
                                .push_info(InfoKind::BlockStart(Marker::If), if_loc));
                        }
                    },

                    Marker::IfStar => match parse_state {
                        ParseState::Else => {
                            let cond = if_op.else_block.take().unwrap();
                            if_op.if_star_blocks.push(crate::op::IfStarBlock {
                                loc,
                                cond,
                                inner: OpBlock::new(),
                            });

                            parse_state = ParseState::IfStar;
                        }

                        _ => {
                            return Err(UnexpectedMarker::General(format!(
                                "`{}` must follow a `{}` with a condition block",
                                Marker::IfStar,
                                Marker::Else
                            ))
                            .into_error(Marker::IfStar)
                            .add_loc(loc)
                            .push_info(InfoKind::BlockStart(Marker::If), if_loc));
                        }
                    },

                    Marker::End => {
                        return Ok(Op {
                            typ: OpType::If(if_op),
                            loc: if_loc,
                        })
                    }

                    _ => {
                        return Err(UnexpectedMarker::NotApplicable(marker, Marker::If)
                            .into_error(marker)
                            .add_loc(loc)
                            .push_info(InfoKind::BlockStart(Marker::If), if_loc));
                    }
                },
            }
        }

        Err(MissingMarker::BlockNotClosed
            .into_error(Marker::End)
            .add_loc(self.lexer.current_location())
            .push_info(InfoKind::BlockStart(Marker::If), if_loc))
    }

    fn parse_while(&mut self, while_loc: FileLocation) -> Result<Op> {
        enum ParseState {
            Cond,
            Do,
        }

        let mut while_op = While::new();
        let mut parse_state = ParseState::Cond;

        while let Some(parsed) = self.parse_next_token()? {
            match parsed {
                Parsed::Op(op) => match parse_state {
                    ParseState::Cond => while_op.cond_block.push(op),
                    ParseState::Do => while_op.do_block.push(op),
                },

                Parsed::Marker { marker, loc } => match marker {
                    Marker::Do => match parse_state {
                        ParseState::Cond => {
                            parse_state = ParseState::Do;
                            while_op.do_loc = Some(loc);
                        }

                        ParseState::Do => {
                            return Err(UnexpectedMarker::Repeated(Marker::Do, Marker::While)
                                .into_error(Marker::Do)
                                .add_loc(loc)
                                .push_info(InfoKind::BlockStart(Marker::While), while_loc));
                        }
                    },

                    Marker::End => match parse_state {
                        ParseState::Cond => {
                            return Err(MissingMarker::RequiredByBlock(Marker::Do, Marker::While)
                                .into_error(Marker::Do)
                                .add_loc(loc)
                                .push_info(InfoKind::BlockStart(Marker::While), while_loc));
                        }
                        ParseState::Do => {
                            return Ok(Op {
                                typ: OpType::While(while_op),
                                loc: while_loc,
                            })
                        }
                    },

                    _ => {
                        return Err(UnexpectedMarker::NotApplicable(marker, Marker::If)
                            .into_error(marker)
                            .add_loc(loc)
                            .push_info(InfoKind::BlockStart(Marker::While), while_loc));
                    }
                },
            }
        }

        Err(MissingMarker::BlockNotClosed
            .into_error(Marker::End)
            .add_loc(self.lexer.current_location())
            .push_info(InfoKind::BlockStart(Marker::While), while_loc))
    }
}
