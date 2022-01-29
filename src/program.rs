use std::fmt;
use std::path::{Path, PathBuf};

use crate::lex::Lexer;
use crate::parse;
use crate::parse::Op;

#[derive(Clone, Debug)]
pub(crate) struct FilePosition {
    pub(crate) line: usize,
    pub(crate) col: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct FileLocation {
    pub(crate) path: PathBuf,
    pub(crate) pos: Option<FilePosition>,
}

impl FileLocation {
    pub(crate) fn from_path(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            pos: None,
        }
    }
}

impl fmt::Display for FileLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path.display())?;

        if let Some(pos) = self.pos.as_ref() {
            write!(f, ":{}:{}", pos.line, pos.col)?;
        }

        Ok(())
    }
}

pub struct Program {
    file_path: PathBuf,
    pub(crate) lexer: Lexer,
}

impl Program {
    pub fn from_path(path: &Path) -> Self {
        let lexer = Lexer::from_path(path);

        Program {
            file_path: path.to_path_buf(),
            lexer,
        }
    }

    pub(crate) fn next_op(&mut self) -> Option<Op> {
        let token = self.lexer.next_token()?;
        Some(parse::parse_token(token))
    }
}
