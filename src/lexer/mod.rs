use crate::error;
use crate::token::{Token, TokenTypes};
use std::str;

#[cfg(test)]
mod test;

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

    fn scan_token(&mut self) -> Token {
        if self.is_at_end() {
            return self.get_token(TokenTypes::Eof);
        }
        self.start = self.current;
        let c: u8 = self.advance();
        match c {
            b'(' => return self.get_token(TokenTypes::LParen),
            b')' => return self.get_token(TokenTypes::RParen),
            b'{' => return self.get_token(TokenTypes::LBrace),
            b'}' => return self.get_token(TokenTypes::RBrace),
            b'[' => return self.get_token(TokenTypes::LBrack),
            b']' => return self.get_token(TokenTypes::RBrack),
            b',' => return self.get_token(TokenTypes::Comma),
            b'-' => {
                if self.matches(b'-') {
                    return self.get_token(TokenTypes::Decrement);
                } else if self.matches(b'=') {
                    return self.get_token(TokenTypes::SubAssign);
                }
                return self.get_token(TokenTypes::Sub);
            }
            b'+' => {
                if self.matches(b'+') {
                    return self.get_token(TokenTypes::Increment);
                } else if self.matches(b'=') {
                    return self.get_token(TokenTypes::AddAssign);
                }
                return self.get_token(TokenTypes::Add);
            }
            b';' => return self.get_token(TokenTypes::Semicolon),
            b'*' => return self.eq_after(TokenTypes::Mul, TokenTypes::MulAssign),
            b'!' => return self.eq_after(TokenTypes::Not, TokenTypes::Neq),
            b'=' => return self.eq_after(TokenTypes::Assign, TokenTypes::Eql),
            b':' => return self.eq_after(TokenTypes::Colon, TokenTypes::Define),
            b'<' => {
                if self.matches(b'-') {
                    return self.get_token(TokenTypes::Arrow);
                } else if self.matches(b'<') {
                    return self.eq_after(TokenTypes::Lshift, TokenTypes::LshiftAssign);
                }
                return self.eq_after(TokenTypes::Lss, TokenTypes::Leq);
            }
            b'>' => {
                if self.matches(b'>') {
                    return self.eq_after(TokenTypes::Rshift, TokenTypes::RshiftAssign);
                }
                return self.eq_after(TokenTypes::Gtr, TokenTypes::Geq);
            }
            b'^' => return self.eq_after(TokenTypes::Caret, TokenTypes::CaretAssign),
            b'%' => return self.eq_after(TokenTypes::Percent, TokenTypes::PercentAssign),
            b'&' => {
                if self.matches(b'&') {
                    return self.get_token(TokenTypes::And);
                } else if self.matches(b'=') {
                    return self.get_token(TokenTypes::AndAssign);
                } else if self.matches(b'^') {
                    if self.peek() == b'=' {
                        self.advance();
                        return self.get_token(TokenTypes::BitClearAssign);
                    } else {
                        return self.get_token(TokenTypes::BitClear);
                    }
                } else {
                    return self.get_token(TokenTypes::Amp);
                }
            }
            b'|' => {
                if self.matches(b'=') {
                    return self.get_token(TokenTypes::OrAssign);
                } else if self.matches(b'|') {
                    return self.get_token(TokenTypes::OrOr);
                } else {
                    return self.get_token(TokenTypes::Or);
                }
            }
            b'/' => {
                if self.matches(b'/') {
                    while self.peek() != b'\n' && !self.is_at_end() {
                        self.advance();
                    }
                    return self.scan_token();
                } else if self.matches(b'*') {
                    while !self.is_at_end() {
                        if self.peek() == b'*' && self.peek_next() == b'/' {
                            break;
                        } else if self.peek() == b'\n' {
                            self.line += 1;
                        }
                        self.advance();
                    }
                    if self.is_at_end() {
                        self.error_handler.error(self.line, "Unterminated comment.");
                        return self.scan_token();
                    }
                    self.advance();
                    self.advance();
                    return self.scan_token();
                } else {
                    return self.eq_after(TokenTypes::Quo, TokenTypes::QuoAssign);
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
                if c == b'.' {
                    if self.peek() == b'.' && self.peek_next() == b'.' {
                        self.advance();
                        self.advance();
                        return self.get_token(TokenTypes::Ellipsis);
                    } else {
                        return self.get_token(TokenTypes::Dot);
                    }
                }
                if (c >= b'0' && c <= b'9') || c == b'.' {
                    return self.number(c);
                } else if (c >= b'a' && c <= b'z') || (c >= b'A' && c <= b'Z') || c == b'_' {
                    return self.identifier();
                } else {
                    self.error_handler.error(
                        self.line,
                        &format!("Unexpected character {}.", str::from_utf8(&[c]).unwrap()),
                    );
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

    fn eq_after(&mut self, t1: TokenTypes, t2: TokenTypes) -> Token {
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

    fn string(&mut self) -> Token {
        while self.peek() != b'"' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_handler.error(self.line, "Unterminated string.");
            return self.get_token(TokenTypes::Eof);
        }

        self.advance();
        return self.get_token(TokenTypes::String);
    }

    fn raw_string(&mut self) -> Token {
        while self.peek() != b'`' && !self.is_at_end() {
            if self.peek() == b'\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error_handler.error(self.line, "Unterminated string.");
            return self.get_token(TokenTypes::Eof);
        }

        self.advance();
        return self.get_token(TokenTypes::RawString);
    }

    fn get_token(&self, token_type: TokenTypes) -> Token {
        return Token::new(
            token_type,
            self.src[self.start..self.current].to_vec(),
            self.line,
        );
    }

    fn number(&mut self, c: u8) -> Token {
        if c == b'0' && (self.peek() == b'x' || self.peek() == b'X') {
            self.advance();

            while self.peek().is_ascii_hexdigit() {
                self.advance();
            }

            return self.get_token(TokenTypes::Hex);
        }

        if c == b'0' && (self.peek() == b'b' || self.peek() == b'B') {
            self.advance();

            while self.peek() == b'0' || self.peek() == b'1' {
                self.advance();
            }

            return self.get_token(TokenTypes::Binary);
        }

        let is_lead_zero = c == b'0';
        let mut has_e = false;
        let mut has_dot = false;
        let mut is_octal = true;

        loop {
            if self.peek() >= b'0' && self.peek() <= b'7' {
                self.advance();
            } else if self.peek().is_ascii_digit() {
                self.advance();
                is_octal = false;
            } else if !has_e && (self.peek() == b'e' || self.peek() == b'E') {
                self.advance();
                has_e = true;
                if self.peek() == b'+' || self.peek() == b'-' {
                    self.advance();
                }
            } else if !has_e && !has_dot && self.peek() == b'.' {
                self.advance();
                has_dot = true;
            } else if self.peek() == b'i' {
                self.advance();
                return self.get_token(TokenTypes::Imag);
            } else {
                break;
            }
        }

        let token_type = if has_e || has_dot {
            TokenTypes::Float
        } else if is_lead_zero && is_octal {
            TokenTypes::Octal
        } else {
            TokenTypes::Int
        };

        return self.get_token(token_type);
    }

    fn peek_next(&self) -> u8 {
        if self.current + 1 >= self.src_len {
            return b'\0';
        }
        return self.src[self.current + 1];
    }

    fn identifier(&mut self) -> Token {
        while (self.peek() >= b'a' && self.peek() <= b'z')
            || (self.peek() >= b'A' && self.peek() <= b'Z')
            || (self.peek() >= b'0' && self.peek() <= b'9')
            || self.peek() == b'_'
        {
            self.advance();
        }

        let text: Vec<u8> = self.src[self.start..self.current].to_vec();
        let token_type: TokenTypes = match text.as_slice() {
            b"break" => TokenTypes::Break,
            b"case" => TokenTypes::Case,
            b"chan" => TokenTypes::Chan,
            b"const" => TokenTypes::Const,
            b"continue" => TokenTypes::Continue,
            b"default" => TokenTypes::Default,
            b"defer" => TokenTypes::Defer,
            b"else" => TokenTypes::Else,
            b"fallthrough" => TokenTypes::Fallthrough,
            b"for" => TokenTypes::For,
            b"func" => TokenTypes::Func,
            b"go" => TokenTypes::Go,
            b"goto" => TokenTypes::Goto,
            b"if" => TokenTypes::If,
            b"import" => TokenTypes::Import,
            b"interface" => TokenTypes::Interface,
            b"map" => TokenTypes::Map,
            b"package" => TokenTypes::Package,
            b"range" => TokenTypes::Range,
            b"return" => TokenTypes::Return,
            b"select" => TokenTypes::Select,
            b"struct" => TokenTypes::Struct,
            b"switch" => TokenTypes::Switch,
            b"type" => TokenTypes::Type,
            b"var" => TokenTypes::Var,
            b"true" => TokenTypes::True,
            b"false" => TokenTypes::False,
            b"nil" => TokenTypes::Nil,
            _ => TokenTypes::Identifier,
        };

        return self.get_token(token_type);
    }
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end() {
            return None;
        }

        self.start = self.current;
        return Some(self.scan_token());
    }
}
