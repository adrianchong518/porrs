use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;

use crate::program::{FileLocation, FilePosition};
use crate::token::{Token, TokenType};
use crate::Error;

#[derive(Debug)]
pub(crate) enum LexingError {}

impl fmt::Display for LexingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[Lexing] Error")
    }
}

pub(crate) struct Lexer {
    file_reader: BufReader<File>,
    current_location: FileLocation,
    lexing_line: String,
}

impl Lexer {
    pub(crate) fn from_path(path: &Path) -> Self {
        let file_reader = BufReader::new(File::open(path).unwrap_or_else(|err| {
            log::error!("Unable to read provided file: {} ({})", path.display(), err);
            exit(1);
        }));
        log::debug!("Opened file: {}", path.display());

        Lexer {
            file_reader,
            current_location: FileLocation::from_path(path),
            lexing_line: "".to_owned(),
        }
    }

    pub(crate) fn next_token(&mut self) -> Result<Option<Token>, Error> {
        let (text, loc) = {
            let mut loc = self.current_location.clone();
            let mut text = "".to_owned();

            while text.is_empty() {
                if self.lexing_line.is_empty() {
                    if let None = self.next_line() {
                        return Ok(None);
                    }
                }

                if self.lexing_line.starts_with("//") {
                    self.lexing_line.clear();
                    continue;
                }

                loc = self.current_location.clone();
                let initial_len = self.lexing_line.len();

                let token_end = self
                    .lexing_line
                    .find(char::is_whitespace)
                    .unwrap_or(self.lexing_line.len());

                text = self.lexing_line.drain(..token_end).collect::<String>();
                self.lexing_line = self.lexing_line.trim_start().to_owned();

                debug_assert!(self.current_location.pos.is_some());
                self.current_location.pos.as_mut().unwrap().col +=
                    initial_len - self.lexing_line.len();
            }

            (text, loc)
        };

        let typ = TokenType::from_str(&text);

        let token = Token { typ, loc };

        log::trace!("Lexed token: {:#?}", token);

        Ok(Some(token))
    }

    pub(crate) fn current_location(&self) -> FileLocation {
        self.current_location.clone()
    }

    fn next_line(&mut self) -> Option<()> {
        match self.file_reader.read_line(&mut self.lexing_line) {
            Ok(0) => return None,
            Ok(bytes) => {
                match self.current_location.pos.as_mut() {
                    Some(pos) => {
                        pos.line += 1;
                        pos.col = 1;
                    }
                    None => self.current_location.pos = Some(FilePosition { line: 1, col: 1 }),
                }

                log::debug!(
                    "Read {} bytes from {}: \"{}\"",
                    bytes,
                    self.current_location,
                    self.lexing_line,
                );
            }
            Err(_) => {
                log::error!(
                    "Failed to read the contents of file: {}",
                    self.current_location.path.display()
                );
                exit(1);
            }
        }

        Some(())
    }
}
