use std::fmt;
use std::path::{Path, PathBuf};

use crate::lex::Lexer;
use crate::op::OpBlock;
use crate::parse::Parser;
use crate::Result;

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
    pub(crate) fn from_path(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
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
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let lexer = Lexer::from_path(&path)?;
        let parser = Parser::from_lexer(lexer);
        let root_block = parser.into_root_block()?;

        log::info!("Parsed program at file: {}", path.as_ref().display());
        log::trace!("Root Block: {:#?}", root_block);

        Ok(Program { root_block })
    }
}
