use std::{error, fmt};

use crate::lex::LexingError;
use crate::program::FileLocation;
use crate::simulate::SimulationError;

#[derive(Debug)]
enum ErrorKind {
    Lexing(LexingError),
    Simulation(SimulationError),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match self {
            Lexing(err) => write!(f, "[Lexing] {}", err),
            Simulation(err) => write!(f, "[Simulation] {}", err),
        }
    }
}

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    loc_stack: Vec<FileLocation>,
}

impl Error {
    pub(crate) fn push_loc(mut self, loc: &FileLocation) -> Self {
        self.loc_stack.push(loc.clone());
        self
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {
            kind,
            loc_stack: Vec::new(),
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

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "--- {} --- {}",
            if let Some(loc) = self.loc_stack.get(0) {
                loc.to_string()
            } else {
                "Unknown Location".to_string()
            },
            self.kind
        )
    }
}

impl error::Error for Error {}
