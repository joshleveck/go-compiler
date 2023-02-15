use crate::error;
use crate::token;

pub struct Lexer<'src> {
    current: usize,
    src: &'src Vec<u8>,
    src_len: usize,
    start: usize,
    line: usize,
    error_handler: error::ErrorHandler,
}

impl Lexer<'_> {
    pub fn new(src: &Vec<u8>) -> Lexer {
        Lexer {
            current: 0,
            src,
            src_len: src.len(),
            start: 0,
            line: 1,
            error_handler: error::ErrorHandler::new(),
        }
    }

    fn scan_token(&mut self) -> token::Token {
        if self.is_at_end() {
            return self.get_token(token::TokenTypes::Eof);
        }
        self.start = self.current;
        let c: u8 = self.advance();
        match c {
            b'(' => return self.get_token(token::TokenTypes::LParen),
            b')' => return self.get_token(token::TokenTypes::RParen),
            b'{' => return self.get_token(token::TokenTypes::LBrace),
            b'}' => return self.get_token(token::TokenTypes::RBrace),
            b',' => return self.get_token(token::TokenTypes::Comma),
            b'.' => return self.get_token(token::TokenTypes::Dot),
            b'-' => return self.get_token(token::TokenTypes::Sub),
            b'+' => return self.get_token(token::TokenTypes::Add),
            b';' => return self.get_token(token::TokenTypes::Semicolon),
            b'*' => return self.get_token(token::TokenTypes::Mul),
            b'!' => return self.eq_after(token::TokenTypes::Not, token::TokenTypes::Neq),
            b'=' => return self.eq_after(token::TokenTypes::Assign, token::TokenTypes::Eql),
            b':' => return self.eq_after(token::TokenTypes::Colon, token::TokenTypes::Define),
            b'<' => return self.eq_after(token::TokenTypes::Lss, token::TokenTypes::Leq),
            b'>' => return self.eq_after(token::TokenTypes::Gtr, token::TokenTypes::Geq),
            b'/' => {
                if self.matches(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                    return self.scan_token();
                } else {
                    return self.get_token(token::TokenTypes::Quo);
                }
            }
            b' ' | b'\r' | b'\t' | b'\0' => return self.scan_token(),
            b'\n' => {
                self.line += 1;
                return self.scan_token();
            }
            b'"' => return self.string(),
            b'`' => return self.raw_string(),
            _ => {
                if c >= b'0' && c <= b'9' {
                    return self.number();
                } else if (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_' {
                    return self.identifier();
                } else {
                    self.error_handler.error(self.line, "Unexpected character.");
                    return self.scan_token();
                }
            }
        }
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.src_len;
    }

    fn advance(&mut self) -> u8 {
        let c: u8 = self.src[self.current];
        self.current += 1;
        return c;
    }

    fn matches(&mut self, expected: u8) -> bool {
        if self.is_at_end() || self.src[self.current] != expected {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn eq_after(&mut self, t1: token::TokenTypes, t2: token::TokenTypes) -> token::Token {
        if self.matches(b'=') {
            return self.get_token(t2);
        }
        return self.get_token(t1);
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            return b'\0';
        }
        return self.src[self.current];
    }

    fn string(&mut self) -> token::Token {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_handler.error(self.line, "Unterminated string.");
            return self.get_token(token::TokenTypes::Eof);
        }

        self.advance();
        return self.get_token(token::TokenTypes::String);
    }

    fn raw_string(&mut self) -> token::Token {
        while self.peek() != b'`' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_handler.error(self.line, "Unterminated string.");
            return self.get_token(token::TokenTypes::Eof);
        }

        self.advance();
        return self.get_token(token::TokenTypes::RawString);
    }

    fn get_token(&self, token_type: token::TokenTypes) -> token::Token {
        return token::Token::new(
            token_type,
            self.src[self.start..self.current].to_vec(),
            self.line,
        );
    }

    fn number(&mut self) -> token::Token {
        while self.peek() >= b'0' && self.peek() <= b'9' {
            self.advance();
        }

        if self.peek() == b'.' && self.peek_next() >= b'0' && self.peek_next() <= b'9' {
            self.advance();
            while self.peek() >= b'0' && self.peek() <= b'9' {
                self.advance();
            }
        }

        return self.get_token(token::TokenTypes::Int);
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.src_len {
            return b'\0';
        }
        return self.src[self.current + 1];
    }

    fn identifier(&mut self) -> token::Token {
        while (self.peek() >= b'a' && self.peek() <= b'z')
            || (self.peek() >= b'A' && self.peek() <= b'Z')
            || (self.peek() >= b'0' && self.peek() <= b'9')
            || self.peek() == b'_'
        {
            self.advance();
        }

        let text: Vec<u8> = self.src[self.start..self.current].to_vec();
        let token_type: token::TokenTypes = match text.as_slice() {
            b"and" => token::TokenTypes::And,
            b"struct" => token::TokenTypes::Struct,
            b"else" => token::TokenTypes::Else,
            b"false" => token::TokenTypes::False,
            b"for" => token::TokenTypes::For,
            b"func" => token::TokenTypes::Func,
            b"if" => token::TokenTypes::If,
            b"nil" => token::TokenTypes::Nil,
            b"return" => token::TokenTypes::Return,
            b"true" => token::TokenTypes::True,
            b"var" => token::TokenTypes::Var,
            b"while" => token::TokenTypes::While,
            b"Const" => token::TokenTypes::Const,
            _ => token::TokenTypes::Identifier,
        };

        return self.get_token(token_type);
    }
}

impl Iterator for Lexer<'_> {
    type Item = token::Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            return None;
        }

        self.start = self.current;
        return Some(self.scan_token());
    }
}
