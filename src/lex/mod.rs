pub mod command;
mod cursor;
pub mod token;

use cursor::Cursor;

use thiserror::Error;
use token::{Token, TokenStream};

#[non_exhaustive]
#[derive(Clone, PartialEq, Eq, Debug, Hash, Error)]
pub enum LexError {
    #[error("unknown chart content at line {line}, col {col}")]
    UnknownCommand {
        /// Line number inside the chart file.
        line: usize,
        /// Column number inside the chart file.
        col: usize,
    },
    #[error("expected token {message} not found at line {line}, col {col}")]
    ExpectedToken {
        /// Line number inside the chart file.
        line: usize,
        /// Column number inside the chart file.
        col: usize,
        /// Message containing expected token details.
        message: &'static str,
    },
}

/// Lexical analysis result type, giving [`LexError`] when lexing fails.
pub type Result<T> = std::result::Result<T, LexError>;

/// Tokenizes chart content.
pub fn tokenize(source: &str) -> Result<TokenStream> {
    let mut cursor = Cursor::new(source);

    let mut tokens = vec![];
    while !cursor.is_end() {
        tokens.push(Token::from_cursor(&mut cursor)?);
    }

    Ok(TokenStream::from_tokens(tokens))
}
