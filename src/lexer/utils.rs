use super::tokens::*;

pub fn new_token(tok_type: TokenType, ch: char) -> Token {
    Token {
        kind: tok_type,
        literal: ch.to_string(),
    }
}

pub fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
