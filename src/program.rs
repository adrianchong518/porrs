use std::fmt;
use std::path::{Path, PathBuf};

use crate::lex::Lexer;
use crate::parse::{parse_token, Block};

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
    pub(crate) root_block: Block,
}

impl Program {
    pub fn from_path(path: &Path) -> Self {
        let ops = Lexer::from_path(path).map(parse_token).collect();

        log::info!("Parsed file {}", path.display());

        Program {
            file_path: path.to_path_buf(),
            root_block: Block::from_vec(ops),
        }
    }
}
