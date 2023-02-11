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
        let c: u8 = self.advance();
        match c {
            b'(' => return self.get_token(token::TokenTypes::LParen),
            b')' => return self.get_token(token::TokenTypes::RParen),
            b'{' => return self.get_token(token::TokenTypes::LBrace),
            b'}' => return self.get_token(token::TokenTypes::RBrace),
            b',' => return self.get_token(token::TokenTypes::Comma),
            b'-' => return self.get_token(token::TokenTypes::Sub),
            b'+' => return self.get_token(token::TokenTypes::Add),
            b';' => return self.get_token(token::TokenTypes::Semicolon),
            b'*' => return self.get_token(token::TokenTypes::Mul),
            b'!' => return self.eq_after(token::TokenTypes::Not, token::TokenTypes::Neq),
            b'=' => return self.eq_after(token::TokenTypes::Assign, token::TokenTypes::Eql),
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
            b' ' | b'\r' | b'\t' => return self.scan_token(),
            b'\n' => {
                self.line += 1;
                return self.scan_token();
            }
            b'"' => return self.string(),
            _ => {
                self.error_handler.error(self.line, "Unexpected character.");
                return self.scan_token();
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

    fn get_token(&self, token_type: token::TokenTypes) -> token::Token {
        return token::Token::new(
            token_type,
            self.src[self.start..=self.current].to_vec(),
            self.line,
        );
    }
}

impl Iterator for Lexer<'_> {
    type Item = token::Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            return None;
        }

        self.start = self.current + 1;
        return Some(self.scan_token());
    }
}
