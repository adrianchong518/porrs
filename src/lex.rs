use std::io::BufReader;
use std::path::Path;
use std::process::exit;
use std::{fs::File, io::BufRead};

use crate::program::{FileLocation, FilePosition};

#[derive(Debug)]
enum TokenType {
    Word,
    Int(u64),
}

#[derive(Debug)]
pub(crate) struct Token {
    typ: TokenType,
    text: String,
    loc: FileLocation,
}

pub(crate) struct Lexer {
    file_reader: BufReader<File>,
    current_location: FileLocation,
    lexing_line: String,
}

impl Lexer {
    pub(crate) fn from_path(path: &Path) -> Self {
        let file_reader = BufReader::new(File::open(path).unwrap_or_else(|err| {
            log::error!("Unable to read provided file: {:?} ({})", path, err);
            exit(1);
        }));
        log::debug!("Opened file: {:?}", path);

        Lexer {
            file_reader,
            current_location: FileLocation::from_path(path),
            lexing_line: "".to_owned(),
        }
    }

    pub(crate) fn next_token(&mut self) -> Option<Token> {
        if self.lexing_line.is_empty() {
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

                    log::trace!(
                        "Read {} bytes from {}: \"{}\"",
                        bytes,
                        self.current_location,
                        self.lexing_line,
                    );
                }
                Err(_) => todo!(),
            }
        }

        let loc = self.current_location.clone();

        let token_end = self
            .lexing_line
            .find(char::is_whitespace)
            .unwrap_or(self.lexing_line.len());

        debug_assert!(self.current_location.pos.is_some());
        self.current_location.pos.as_mut().unwrap().col += token_end;

        let text = self.lexing_line.drain(..token_end).collect::<String>();
        self.lexing_line = self.lexing_line.trim_start().to_owned();

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
