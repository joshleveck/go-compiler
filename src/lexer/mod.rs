use crate::error;
use crate::token;

pub struct Lexer<'src> {
    current: usize,
    src: &'src Vec<u8>,
    src_len: usize,
    start: usize,
    line: usize,
}

impl Lexer<'_> {
    pub fn new(src: &Vec<u8>) -> Lexer {
        Lexer {
            current: 0,
            src,
            src_len: src.len(),
            start: 0,
            line: 1,
        }
    }

    // pub fn scanTokens(&mut self) {
    //     while !self.isAtEnd() {
    //         self.start = self.current;
    //         self.scanToken();
    //     }
    //     self.tokens.push(token::Token::new(
    //         token::TokenTypes::EOF,
    //         &Vec::<u8>::new(),
    //         self.line,
    //     ));
    // }

    fn scan_token(&mut self) -> token::Token {
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
            b'!' => {
                if self.matches(b'=') {
                    return self.get_token(token::TokenTypes::Neq);
                }
                return self.get_token(token::TokenTypes::Not);
            }
            _ => {
                error::ErrorHandler().error(self.line, "Unexpected character.");
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
