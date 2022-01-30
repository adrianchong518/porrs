use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::exit;

use crate::program::{FileLocation, FilePosition};

#[derive(Debug)]
pub(crate) enum TokenType {
    Word,
    Int(u64),
}

#[derive(Debug)]
pub(crate) struct Token {
    pub(crate) typ: TokenType,
    pub(crate) text: String,
    pub(crate) loc: FileLocation,
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

    pub(crate) fn next_token(&mut self) -> Option<Token> {
        while self.lexing_line.is_empty() {
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

                    let trimmed = self.lexing_line.trim();

                    log::trace!(
                        "Read {} bytes from {}: \"{}\" (trimmed: \"{}\")",
                        bytes,
                        self.current_location,
                        self.lexing_line,
                        trimmed
                    );

                    self.lexing_line = trimmed.to_owned();
                }
                Err(_) => {
                    log::error!(
                        "Failed to read the contents of file: {}",
                        self.current_location.path.display()
                    );
                    exit(1);
                }
            }
        }

        let loc = self.current_location.clone();
        let initial_len = self.lexing_line.len();

        let token_end = self
            .lexing_line
            .find(char::is_whitespace)
            .unwrap_or(self.lexing_line.len());

        let text = self.lexing_line.drain(..token_end).collect::<String>();
        self.lexing_line = self.lexing_line.trim_start().to_owned();

        debug_assert!(self.current_location.pos.is_some());
        self.current_location.pos.as_mut().unwrap().col += initial_len - self.lexing_line.len();

        let typ = if let Ok(val) = text.parse::<u64>() {
            TokenType::Int(val)
        } else {
            TokenType::Word
        };

        let token = Token { typ, text, loc };

        log::trace!("Lexed token: {:?}", token);

        Some(token)
    }
}

impl Iterator for Lexer {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}
