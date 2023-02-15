use std::fmt;
use std::str;

pub struct Token {
    token_type: TokenTypes,
    lexeme: Vec<u8>,
    line: usize,
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
    While,
    Dot,

    // Operators and delimiters
    Add,
    Sub,
    Mul,
    Quo,
    Rem,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    AndNot,
    AddAssign,
    SubAssign,
    MulAssign,
    QuoAssign,
    RemAssign,
    AndAssign,
    OrAssign,
    XorAssign,
    ShlAssign,
    ShrAssign,
    AndNotAssign,
    LAnd,
    LOr,
    Arrow,
    Inc,
    Dec,
    Eql,
    Lss,
    Gtr,
    Assign,
    Not,
    Neg,
    Neq,
    Leq,
    Geq,
    Define,
    Ellipsis,
    LParen,
    LBrack,
    LBrace,
    Comma,
    Period,
    RParen,
    RBrack,
    RBrace,
    Semicolon,
    Colon,
    DBlColon,

    // Literals
    Ident,
    Int,
    Float,
    Imag,
    Char,
    String,
    RawString,
    False,
    True,
    Nil,

    // Misc
    Illegal,
    Identifier,
    Eof,
}
