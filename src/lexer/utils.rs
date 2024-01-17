use super::tokens::*;

pub fn new_token(tok_type: TokenType, literal: String) -> Token {
    Token {
        kind: tok_type,
        literal,
    }
}

pub fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
