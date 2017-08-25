use std::ascii::escape_default;
use std::error;
use std::fmt;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct Error {
    file_name: String,
    line: u32,
    kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error ({}:{}): {}", self.file_name, self.line, self.kind)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self.kind {
            ErrorKind::InvalidCharacter(_) => "Lexer Error: Invalid Character",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

#[derive(Clone, Debug)]
pub enum ErrorKind {
    InvalidCharacter(u8),
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &ErrorKind::InvalidCharacter(c) =>
                write!(f, "Invalid character '{}'", (c as char).escape_default()),
        }
    }
}

macro_rules! syntax_error {
    ($file_name:expr, $lexer:ident, $($arg:tt)*) => {
        return Err(SyntaxError {
            file_name: $file_name,
            line: $lexer.line,
            message: format!($($arg)*),
        })
    }
}
