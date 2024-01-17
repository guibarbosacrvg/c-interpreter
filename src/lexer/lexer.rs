use super::tokens::*;
use super::utils::*;

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: &str,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lex: Lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: "\0",
        };
        lex.read_char();
        lex
    }

    pub fn skip_whitespace(&mut self) {
        while self.position < self.input.len() {
            match self.ch {
                " " | "\t" | "\n" | "\r" => self.read_char(),
                _ => break,
            }
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = "\0"; /* ASCII code for "NUL" character, indicates EOF or that nothing was read yet */
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap().to_string().as_str();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn read_identifier(&mut self) -> String {
        let start_position = self.position;
        while is_letter(self.ch.chars().nth(0).unwrap()) {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    pub fn read_number(&mut self) -> String {
        let start_position = self.position;
        while self.ch.chars().nth(0).unwrap().is_digit(10) {
            self.read_char();
        }
        self.input[start_position..self.position].to_string()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let tok: Token = match self.ch {
            "=" => new_token(ASSIGN.to_string(), self.ch.to_string()),
            ";" => new_token(SEMICOLON.to_string(), self.ch.to_string()),
            "," => new_token(COMMA.to_string(), self.ch.to_string()),
            "(" => new_token(LPAREN.to_string(), self.ch.to_string()),
            ")" => new_token(RPAREN.to_string(), self.ch.to_string()),
            "+" => new_token(PLUS.to_string(), self.ch.to_string()),
            "{" => new_token(LBRACE.to_string(), self.ch.to_string()),
            "}" => new_token(RBRACE.to_string(), self.ch.to_string()),
            "[" => new_token(LBRACKET.to_string(), self.ch.to_string()),
            "]" => new_token(RBRACKET.to_string(), self.ch.to_string()),
            "-" => new_token(MINUS.to_string(), self.ch.to_string()),
            "!" => new_token(BANG.to_string(), self.ch.to_string()),
            "/" => new_token(SLASH.to_string(), self.ch.to_string()),
            "*" => new_token(ASTERISK.to_string(), self.ch.to_string()),
            "<" => new_token(LT.to_string(), self.ch.to_string()),
            ">" => new_token(GT.to_string(), self.ch.to_string()),
            // "<=" => new_token(LESS_EQ.to_string(), self.ch.to_string()),
            // ">=" => new_token(GREATER_EQ.to_string(), self.ch.to_string()),
            "\0" => new_token(EOF.to_string(), self.ch.to_string()),
            _ => {
                if is_letter(self.ch.chars().nth(0).unwrap()) {
                    let literal: String = self.read_identifier();
                    self.read_char();
                    return new_token(IDENT.to_string(), literal);
                } else if self.ch.chars().nth(0).unwrap().is_digit(10) {
                    let literal: String = self.read_number();
                    return new_token(INT.to_string(), literal);
                } else {
                    new_token(ILLEGAL.to_string(), self.ch.to_string())
                }
            }
        };

        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_lexer() {
        let input: String = String::from("=+(){},;");
        let lex: Lexer = Lexer::new(input);
        assert_eq!(lex.input, String::from("=+(){},;"));
        assert_eq!(lex.position, 0);
        assert_eq!(lex.read_position, 1);
        assert_eq!(lex.ch, "=");
    }

    #[test]
    fn test_next_token() {
        let input: String = "int add(int a, int b){\n   return a + b;\n}\n\nint main(void){\n  int x = 5;\n    int y = 4;\n    int result = add(x,y);\n    return 0;\n}".to_string();
        let mut lex: Lexer = Lexer::new(input);
        let expected_tokens: Vec<Token> = vec![
            new_token(INT.to_string(), "int".to_string()),
            new_token(IDENT.to_string(), "add".to_string()),
            new_token(LPAREN.to_string(), "(".to_string()),
            new_token(INT.to_string(), "int".to_string()),
            new_token(IDENT.to_string(), "a".to_string()),
            new_token(COMMA.to_string(), ",".to_string()),
            new_token(INT.to_string(), "int".to_string()),
            new_token(IDENT.to_string(), "b".to_string()),
            new_token(RPAREN.to_string(), ")".to_string()),
            new_token(LBRACE.to_string(), "{".to_string()),
            new_token(RETURN.to_string(), "return".to_string()),
            new_token(IDENT.to_string(), "a".to_string()),
            new_token(PLUS.to_string(), "+".to_string()),
            new_token(IDENT.to_string(), "b".to_string()),
            new_token(SEMICOLON.to_string(), ";".to_string()),
            new_token(RBRACE.to_string(), "}".to_string()),
            new_token(INT.to_string(), "int".to_string()),
            new_token(IDENT.to_string(), "main".to_string()),
            new_token(LPAREN.to_string(), "(".to_string()),
            new_token(VOID.to_string(), "void".to_string()),
            new_token(RPAREN.to_string(), ")".to_string()),
            new_token(LBRACE.to_string(), "{".to_string()),
            new_token(INT.to_string(), "int".to_string()),
            new_token(IDENT.to_string(), "x".to_string()),
            new_token(ASSIGN.to_string(), "=".to_string()),
            new_token(INT.to_string(), "5".to_string()),
            new_token(SEMICOLON.to_string(), ";".to_string()),
            new_token(INT.to_string(), "int".to_string()),
            new_token(IDENT.to_string(), "y".to_string()),
            new_token(ASSIGN.to_string(), "=".to_string()),
            new_token(INT.to_string(), "4".to_string()),
            new_token(SEMICOLON.to_string(), ";".to_string()),
            new_token(INT.to_string(), "int".to_string()),
            new_token(IDENT.to_string(), "result".to_string()),
            new_token(ASSIGN.to_string(), "=".to_string()),
            new_token(IDENT.to_string(), "add".to_string()),
            new_token(LPAREN.to_string(), "(".to_string()),
            new_token(IDENT.to_string(), "x".to_string()),
            new_token(COMMA.to_string(), ",".to_string()),
            new_token(IDENT.to_string(), "y".to_string()),
            new_token(RPAREN.to_string(), ")".to_string()),
            new_token(SEMICOLON.to_string(), ";".to_string()),
            new_token(RETURN.to_string(), "return".to_string()),
            new_token(INT.to_string(), "0".to_string()),
            new_token(SEMICOLON.to_string(), ";".to_string()),
            new_token(RBRACE.to_string(), "}".to_string()),
            new_token(EOF.to_string(), "\0".to_string()),
        ];
        for expected_token in expected_tokens {
            let tok: Token = lex.next_token();
            assert_eq!(tok.kind, expected_token.kind);
            assert_eq!(tok.literal, expected_token.literal);
        }
    }
}
