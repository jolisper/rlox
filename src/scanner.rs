pub struct Scanner<'a> {
    // In the C version the scanner use pointers to source code characters,
    // in this Rust version the scanner has a source code reference and
    // indexes to characters, avoiding to use unsafe raw pointers.
    pub source: &'a str,
    start: usize,
    current: usize,
    line: i32,
}

pub struct Token {
    pub ttype: TokenType,
    pub start: usize,
    pub length: usize,
    pub line: i32,
    // C version force the meaning of start pointer using it as a pointer to
    // error message, in this Rust version without raw pointers, an optional
    // error message string is used instead.
    pub message: Option<String>,
}

impl Token {
    fn new(ttype: TokenType, scanner: &Scanner) -> Self {
        Token {
            ttype,
            start: scanner.start,
            length: scanner.current - scanner.start,
            line: scanner.line,
            message: Option::None,
        }
    }

    fn new_error(message: &str, scanner: &Scanner) -> Self {
        Token {
            ttype: TokenType::TokenError,
            start: scanner.start,
            length: scanner.current - scanner.start,
            line: scanner.line,
            message: Option::Some(message.to_owned()),
        }
    }
}

#[derive(Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    TokenLeftParen,
    TokenRightParen,
    TokenLeftBrace,
    TokenRightBrace,
    TokenComma,
    TokenDot,
    TokenMinus,
    TokenPlus,
    TokenSemicolon,
    TokenSlash,
    TokenStar,

    // One or tow character tokens.
    TokenBang,
    TokenBangEqual,
    TokenEqual,
    TokenEqualEqual,
    TokenGreater,
    TokenGreaterEqual,
    TokenLess,
    TokenLessEqual,

    // Literals.
    TokenIdentifier,
    TokenString,
    TokenNumber,

    // Keywords.
    TokenAnd,
    TokenClass,
    TokenElse,
    TokenFalse,
    TokenFor,
    TokenFun,
    TokenIf,
    TokenNil,
    TokenOr,
    TokenPrint,
    TokenReturn,
    TokenSuper,
    TokenThis,
    TokenTrue,
    TokenVar,
    TokenWhile,

    // Signals.
    TokenError,
    TokenEOF,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_token(&mut self) -> Token {
        self.skip_whitespace();

        self.start = self.current;

        if self.is_at_end() {
            return Token::new(TokenType::TokenEOF, self);
        }

        let c: char = self.advance();

        if self.is_alpha(c) {
            return self.identifier();
        }

        if self.is_digit(c) {
            return self.number();
        }

        match c {
            '(' => return Token::new(TokenType::TokenLeftParen, self),
            ')' => return Token::new(TokenType::TokenRightParen, self),
            '{' => return Token::new(TokenType::TokenLeftBrace, self),
            '}' => return Token::new(TokenType::TokenRightBrace, self),
            ';' => return Token::new(TokenType::TokenSemicolon, self),
            ',' => return Token::new(TokenType::TokenComma, self),
            '.' => return Token::new(TokenType::TokenDot, self),
            '-' => return Token::new(TokenType::TokenMinus, self),
            '+' => return Token::new(TokenType::TokenPlus, self),
            '/' => return Token::new(TokenType::TokenSlash, self),
            '*' => return Token::new(TokenType::TokenStar, self),
            '!' => {
                return Token::new(
                    if self.match_current('=') {
                        TokenType::TokenBangEqual
                    } else {
                        TokenType::TokenBang
                    },
                    self,
                );
            }
            '=' => {
                return Token::new(
                    if self.match_current('=') {
                        TokenType::TokenEqualEqual
                    } else {
                        TokenType::TokenEqual
                    },
                    self,
                );
            }
            '<' => {
                return Token::new(
                    if self.match_current('=') {
                        TokenType::TokenLessEqual
                    } else {
                        TokenType::TokenLess
                    },
                    self,
                );
            }
            '>' => {
                return Token::new(
                    if self.match_current('=') {
                        TokenType::TokenGreaterEqual
                    } else {
                        TokenType::TokenGreater
                    },
                    self,
                );
            }
            '"' => self.string(),
            _ => Token::new_error("Unexpected character.", self),
        }
    }

    fn is_at_end(&self) -> bool {
        self.source.chars().nth(self.current).is_none()
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.source.chars().nth(self.current - 1).expect(&format!(
            "No source char at scanner.current - 1 position: {}",
            self.current - 1,
        ));
    }

    fn match_current(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).expect(&format!(
            "No char at scanner.current position: {}",
            self.current
        )) != expected
        {
            return false;
        }

        self.current += 1;
        true
    }

    fn skip_whitespace(&mut self) {
        loop {
            let c = self.peek();
            match c {
                ' ' | '\r' | '\t' => {
                    self.advance();
                    break;
                }
                '\n' => {
                    self.line += 1;
                    self.advance();
                    break;
                }
                '/' => {
                    if self.peek_next() == '/' {
                        while self.peek() != '\n' && !self.is_at_end() {
                            self.advance();
                        }
                    } else {
                        return;
                    }
                    break;
                }
                _ => return,
            }
        }
    }

    fn peek(&mut self) -> char {
        let peeked = self.source.chars().nth(self.current);
        match peeked {
            Some(p) => p,
            None => '\0',
        }
    }

    fn peek_next(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source
            .chars()
            .nth(self.current + 1)
            .expect("No char at scanner.current + 1")
    }

    fn string(&mut self) -> Token {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Token::new_error("Unterminated string.", self);
        }

        self.advance();

        return Token::new(TokenType::TokenString, self);
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
    }

    fn identifier(&mut self) -> Token {
        loop {
            let p = self.peek();
            if self.is_alpha(p) || self.is_digit(p) {
                self.advance();
            } else {
                break;
            }
        }
        Token::new(self.identifier_type(), self)
    }

    fn identifier_type(&self) -> TokenType {
        match self
            .source
            .chars()
            .nth(self.start)
            .expect(&format!("No char at scanner.start: {}", self.start))
        {
            'a' => self.check_keyword(1, 2, "nd", TokenType::TokenAnd),
            'c' => self.check_keyword(1, 4, "lass", TokenType::TokenClass),
            'e' => self.check_keyword(1, 3, "lse", TokenType::TokenElse),
            'f' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).expect(&format!(
                        "No char ar scanner.start + 1 position: {}",
                        self.start + 1
                    )) {
                        'a' => self.check_keyword(2, 3, "lse", TokenType::TokenFalse),
                        'o' => self.check_keyword(2, 1, "r", TokenType::TokenFor),
                        'u' => self.check_keyword(2, 1, "n", TokenType::TokenFun),
                        _ => TokenType::TokenIdentifier,
                    }
                } else {
                    TokenType::TokenIdentifier
                }
            }
            'i' => self.check_keyword(1, 1, "f", TokenType::TokenIf),
            'n' => self.check_keyword(1, 2, "il", TokenType::TokenNil),
            'o' => self.check_keyword(1, 1, "r", TokenType::TokenOr),
            'p' => self.check_keyword(1, 4, "rint", TokenType::TokenPrint),
            'r' => self.check_keyword(1, 5, "eturn", TokenType::TokenReturn),
            's' => self.check_keyword(1, 4, "uper", TokenType::TokenSuper),
            't' => {
                if self.current - self.start > 1 {
                    match self.source.chars().nth(self.start + 1).expect(&format!(
                        "No char at scanner.start + 1 position: {}",
                        self.start + 1
                    )) {
                        'h' => self.check_keyword(2, 2, "is", TokenType::TokenThis),
                        'r' => self.check_keyword(2, 1, "ue", TokenType::TokenTrue),
                        _ => TokenType::TokenIdentifier,
                    }
                } else {
                    TokenType::TokenIdentifier
                }
            }
            'v' => self.check_keyword(1, 2, "ar", TokenType::TokenVar),
            'w' => self.check_keyword(1, 4, "hile", TokenType::TokenWhile),
            _ => TokenType::TokenIdentifier,
        }
    }

    fn check_keyword(
        &self,
        start: usize,
        length: usize,
        rest: &str,
        ttype: TokenType,
    ) -> TokenType {
        let literal: String = self
            .source
            .chars()
            .skip(self.current + start)
            .take(length)
            .collect();

        if literal.len() == start + length && literal.eq(&rest) {
            return ttype;
        }

        TokenType::TokenIdentifier
    }

    fn number(&mut self) -> Token {
        loop {
            let p = self.peek();
            if self.is_digit(p) {
                self.advance();
            } else {
                break;
            }
        }

        // Look for a fraction part.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            loop {
                let p = self.peek();
                if self.is_digit(p) {
                    self.advance();
                } else {
                    break;
                }
            }
        }
        return Token::new(TokenType::TokenNumber, self);
    }
}
