pub type TokenType = String;

pub struct Token {
    pub kind: TokenType,
    pub literal: String,
}

// Defining the actual token names

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";
pub const IDENT: &str = "IDENT";

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";
pub const LT: &str = "<";
pub const GT: &str = ">";
pub const LESS_EQ: &str = "<=";
pub const GREATER_EQ: &str = ">=";
pub const ARROW: &str = "->";
pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";
pub const PLUS_EQ: &str = "+=";
pub const MINUS_EQ: &str = "-=";
pub const PERCENT: &str = "%";
pub const PERCENT_EQ: &str = "%=";
pub const SLASH_EQ: &str = "/=";
pub const ASTERISK_EQ: &str = "*=";
pub const POW: &str = "**";
pub const POW_EQ: &str = "**=";
pub const INC: &str = "++";
pub const DEC: &str = "--";
pub const AND: &str = "&&";
pub const OR: &str = "||";
pub const BIT_AND: &str = "&";
pub const BIT_OR: &str = "|";
pub const BIT_XOR: &str = "^";
pub const BIT_SR: &str = ">>";
pub const BIT_SL: &str = "<<";
pub const BIT_AND_EQ: &str = "&=";
pub const BIT_OR_EQ: &str = "|=";
pub const BIT_XOR_EQ: &str = "^=";
pub const BIT_SR_EQ: &str = ">>=";
pub const BIT_SL_EQ: &str = "<<=";

// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const COLON: &str = ":";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
pub const LBRACKET: &str = "[";
pub const RBRACKET: &str = "]";
pub const DOT: &str = ".";

// Keywords

// If Else statements
pub const IF: &str = "if";
pub const ELSE: &str = "else";

// Loops
pub const FOR: &str = "for";
pub const WHILE: &str = "while";
pub const DO: &str = "do";

// Loop control
pub const BREAK: &str = "break";
pub const CONTINUE: &str = "continue";

// Switch statements
pub const SWITCH: &str = "switch";
pub const CASE: &str = "case";
pub const DEFAULT: &str = "default";

// Function
pub const RETURN: &str = "return";

// Types
pub const CHAR: &str = "char";
pub const SHORT: &str = "short";
pub const INT: &str = "int";
pub const LONG: &str = "long";
pub const FLOAT: &str = "float";
pub const DOUBLE: &str = "double";
pub const VOID: &str = "void";
pub const LONG_LONG: &str = "long long";
pub const UNSIGNED: &str = "unsigned";

// User defined types
pub const STRUCT: &str = "struct";
pub const UNION: &str = "union";
pub const ENUM: &str = "enum";

// Storage classes
pub const AUTO: &str = "auto";
pub const REGISTER: &str = "register";
pub const STATIC: &str = "static";
pub const EXTERN: &str = "extern";

// Type qualifiers
pub const CONST: &str = "const";
pub const VOLATILE: &str = "volatile";

// For reading numbers
pub const HEX: &str = "0x";
pub const OCT: &str = "0";

// For reading strings
pub const STRING: &str = "\"";
pub const CHAR_LITERAL: &str = "'";
pub const ESCAPE: &str = "\\";
pub const NEWLINE: &str = "\n";
pub const TAB: &str = "\t";
pub const BACKSPACE: &str = r"\b";
pub const FORMFEED: &str = r"\f";
pub const CARRIAGE_RETURN: &str = r"\r";
pub const VERTICAL_TAB: &str = r"\v";
pub const SINGLE_QUOTE: &str = r"\'";
pub const DOUBLE_QUOTE: &str = r#"\""#;
pub const QUESTION_MARK: &str = r"\?";
pub const BACKSLASH: &str = r"\\";
pub const OCTAL: &str = r"\0";
pub const HEXADECIMAL: &str = r"\x";

// Misc
pub const SIZEOF: &str = "sizeof";
pub const TYPEDEF: &str = "typedef";
pub const GOTO: &str = "goto";
pub const INLINE: &str = "inline";
