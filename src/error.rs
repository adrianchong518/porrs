use std::{error, fmt};

use crate::lex::LexingError;
use crate::parse::ParsingError;
use crate::program::FileLocation;
use crate::simulate::SimulationError;
use crate::token::Marker;

#[derive(Debug)]
enum ErrorKind {
    Lexing(LexingError),
    Parsing(ParsingError),
    Simulation(SimulationError),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match self {
            Lexing(err) => write!(f, "[Lexing] {}", err),
            Parsing(err) => write!(f, "[Parsing] {}", err),
            Simulation(err) => write!(f, "[Simulation] {}", err),
        }
    }
}

// TODO Add an info stack
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    loc: Option<FileLocation>,
    info_stack: Vec<Info>,
}

impl Error {
    pub fn info_stack(&self) -> &[Info] {
        &self.info_stack
    }

    pub(crate) fn add_loc(mut self, loc: FileLocation) -> Self {
        self.loc = Some(loc);
        self
    }

    pub(crate) fn has_loc(&self) -> bool {
        self.loc.is_some()
    }

    pub(crate) fn push_info(mut self, kind: InfoKind, loc: FileLocation) -> Self {
        self.info_stack.push(Info { kind, loc });
        self
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            kind,
            loc: None,
            info_stack: Vec::new(),
        }
    }
}

impl From<SimulationError> for Error {
    fn from(err: SimulationError) -> Self {
        Self::from(ErrorKind::Simulation(err))
    }
}

impl From<LexingError> for Error {
    fn from(err: LexingError) -> Self {
        Self::from(ErrorKind::Lexing(err))
    }
}

impl From<ParsingError> for Error {
    fn from(err: ParsingError) -> Self {
        Self::from(ErrorKind::Parsing(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "<-- {} --> {}",
            if let Some(loc) = &self.loc {
                loc.to_string()
            } else {
                "Unknown Location".to_string()
            },
            self.kind
        )
    }
}

impl error::Error for Error {}

#[derive(Debug)]
pub(crate) enum InfoKind {
    BlockStart(Marker),
}

impl fmt::Display for InfoKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BlockStart(marker) => write!(f, "`{}` block starts here", marker),
        }
    }
}

#[derive(Debug)]
pub struct Info {
    kind: InfoKind,
    loc: FileLocation,
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<-- {} --> {}", self.loc, self.kind)
    }
}
