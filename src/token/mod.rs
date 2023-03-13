use std::fmt;
use std::str;

pub struct Token {
    pub token_type: TokenTypes,
    pub lexeme: Vec<u8>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenTypes, lexeme: Vec<u8>, line: usize) -> Token {
        Token {
            token_type,
            lexeme,
            line,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "Lexeme: {:?}, Line: {}, Type: {:?}",str::from_utf8(&self.lexeme).unwrap(), self.line, self.token_type);
    }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenTypes {
    // Keywords
    Break,
    Default,
    Func,
    Interface,
    Select,
    Case,
    Defer,
    Go,
    Map,
    Struct,
    Chan,
    Else,
    Goto,
    Package,
    Switch,
    Const,
    Fallthrough,
    If,
    Range,
    Type,
    Continue,
    For,
    Import,
    Return,
    Var,
    Dot,

    // Operators and delimiters
    Add,
    Sub,
    Mul,
    Quo,
    And,
    Amp,
    Or,
    OrOr,
    AddAssign,
    SubAssign,
    MulAssign,
    QuoAssign,
    AndAssign,
    OrAssign,
    Arrow,
    Eql,
    Lss,
    Gtr,
    Assign,
    Not,
    Neq,
    Leq,
    Geq,
    Define,
    Ellipsis,
    LParen,
    LBrack,
    LBrace,
    Comma,
    RParen,
    RBrack,
    RBrace,
    Semicolon,
    Colon,
    Caret,
    Percent,
    CaretAssign,
    PercentAssign,
    BitClear,
    BitClearAssign,
    Increment,
    Decrement,
    Lshift,
    LshiftAssign,
    Rshift,
    RshiftAssign,
    

    // Literals
    Int,
    Octal,
    Hex,
    Binary,
    Float,
    Imag,
    String,
    RawString,
    False,
    True,
    Nil,

    // Misc
    Identifier,
    Eof,
}
