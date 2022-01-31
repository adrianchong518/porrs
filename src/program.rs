use std::fmt;
use std::path::{Path, PathBuf};

use crate::lex::Lexer;
use crate::parse::{parse_token_block, OpBlock};
use crate::Error;

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
    pub(crate) root_block: OpBlock,
}

impl Program {
    pub fn from_path(path: &Path) -> Result<Self, Error> {
        let root_block = parse_token_block(&Lexer::from_path(path).token_block()?);

        log::info!("Parsed program at file: {}", path.display());
        log::trace!("Root Block: {:#?}", root_block);

        Ok(Program { root_block })
    }
}
