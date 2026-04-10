#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Let, Fn, Return,
    If, Elif, Else,
    Loop, Break, Continue,
    Export,
    Struct,

    Ident(String),
    Number(i64),

    LParen, RParen,
    LBrace, RBrace,
    LBracket, RBracket,

    Comma, Semicolon,
    Colon, Dot,

    Plus, Minus, Star, Slash,

    EqEq, NotEq,
    Lt, Gt, LtEq, GtEq,

    Assign,

    EOF,
}

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self { input: input.chars().collect(), pos: 0 }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.pos).copied()
    }

    fn next(&mut self) -> Option<char> {
        let c = self.peek();
        self.pos += 1;
        c
    }

    fn skip_ws(&mut self) {
        while matches!(self.peek(), Some(c) if c.is_whitespace()) {
            self.next();
        }
    }

    fn ident(&mut self) -> Token {
        let mut s = String::new();
        while matches!(self.peek(), Some(c) if c.is_alphanumeric() || c == '_') {
            s.push(self.next().unwrap());
        }

        match s.as_str() {
            "let" => Token::Let,
            "fn" => Token::Fn,
            "return" => Token::Return,
            "if" => Token::If,
            "elif" => Token::Elif,
            "else" => Token::Else,
            "loop" => Token::Loop,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "export" => Token::Export,
            "struct" => Token::Struct,
            _ => Token::Ident(s),
        }
    }

    fn number(&mut self) -> Token {
        let mut s = String::new();

        if self.peek() == Some('0') {
            s.push(self.next().unwrap());

            match self.peek() {
                Some('x') => {
                    self.next();
                    let mut hex = String::new();
                    while matches!(self.peek(), Some(c) if c.is_ascii_hexdigit()) {
                        hex.push(self.next().unwrap());
                    }
                    return Token::Number(i64::from_str_radix(&hex, 16).unwrap());
                }
                Some('b') => {
                    self.next();
                    let mut bin = String::new();
                    while matches!(self.peek(), Some(c) if c == '0' || c == '1') {
                        bin.push(self.next().unwrap());
                    }
                    return Token::Number(i64::from_str_radix(&bin, 2).unwrap());
                }
                _ => {}
            }
        }

        while matches!(self.peek(), Some(c) if c.is_numeric()) {
            s.push(self.next().unwrap());
        }

        Token::Number(s.parse().unwrap())
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_ws();

        match self.next() {
            Some('/') => match self.peek() {
                Some('/') => {
                    while let Some(c) = self.peek() {
                        if c == '\n' { break; }
                        self.next();
                    }
                    self.next_token()
                }
                Some('*') => {
                    self.next();
                    while let Some(c) = self.next() {
                        if c == '*' && self.peek() == Some('/') {
                            self.next();
                            break;
                        }
                    }
                    self.next_token()
                }
                _ => Token::Slash,
            },

            Some('(') => Token::LParen,
            Some(')') => Token::RParen,
            Some('{') => Token::LBrace,
            Some('}') => Token::RBrace,
            Some('[') => Token::LBracket,
            Some(']') => Token::RBracket,

            Some(',') => Token::Comma,
            Some(';') => Token::Semicolon,
            Some(':') => Token::Colon,
            Some('.') => Token::Dot,

            Some('+') => Token::Plus,
            Some('-') => Token::Minus,
            Some('*') => Token::Star,

            Some('=') => {
                if self.peek() == Some('=') {
                    self.next();
                    Token::EqEq
                } else {
                    Token::Assign
                }
            }

            Some('!') => {
                if self.peek() == Some('=') {
                    self.next();
                    Token::NotEq
                } else {
                    panic!("Unexpected !");
                }
            }

            Some('<') => {
                if self.peek() == Some('=') {
                    self.next();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }

            Some('>') => {
                if self.peek() == Some('=') {
                    self.next();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }

            Some(c) if c.is_alphabetic() || c == '_' => {
                self.pos -= 1;
                self.ident()
            }

            Some(c) if c.is_numeric() => {
                self.pos -= 1;
                self.number()
            }

            None => Token::EOF,

            Some(c) => panic!("Unexpected char {}", c),
        }
    }
}