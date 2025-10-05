mod lexer;
pub mod tokens;

pub use lexer::Lexer;
pub use tokens::Token;

use crate::error::LexerError;

pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut lexer = Lexer::new(input);
    lexer.tokenize()
}
