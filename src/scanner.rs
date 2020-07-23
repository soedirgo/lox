use crate::error;
use crate::token::{Literal, Token};
use crate::token_type::TokenType;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme.
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::new(),
            Literal::None,
            self.line,
        ));

        self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token_helper(TokenType::LeftParen),
            ')' => self.add_token_helper(TokenType::RightParen),
            '{' => self.add_token_helper(TokenType::LeftBrace),
            '}' => self.add_token_helper(TokenType::RightBrace),
            ',' => self.add_token_helper(TokenType::Comma),
            '.' => self.add_token_helper(TokenType::Dot),
            '-' => self.add_token_helper(TokenType::Minus),
            '+' => self.add_token_helper(TokenType::Plus),
            ';' => self.add_token_helper(TokenType::Semicolon),
            '*' => self.add_token_helper(TokenType::Star),

            '!' => {
                if self.match_('=') {
                    self.add_token_helper(TokenType::BangEqual)
                } else {
                    self.add_token_helper(TokenType::Bang)
                }
            }
            '=' => {
                if self.match_('=') {
                    self.add_token_helper(TokenType::EqualEqual)
                } else {
                    self.add_token_helper(TokenType::Equal)
                }
            }
            '<' => {
                if self.match_('=') {
                    self.add_token_helper(TokenType::LessEqual)
                } else {
                    self.add_token_helper(TokenType::Less)
                }
            }
            '>' => {
                if self.match_('=') {
                    self.add_token_helper(TokenType::GreaterEqual)
                } else {
                    self.add_token_helper(TokenType::Greater)
                }
            }

            '/' => {
                if self.match_('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token_helper(TokenType::Slash)
                }
            }

            ' ' | '\r' | '\t' => (),
            '\n' => self.line += 1,

            '"' => self.string(),

            c => {
                if c.is_ascii_digit() {
                    self.number()
                } else if c.is_ascii_alphabetic() {
                    self.identifier()
                } else {
                    error(self.line, "Unexpected character.")
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.peek().is_ascii_alphanumeric() {
            self.advance();
        }

        // See if the identifier is a reserved word.
        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();

        let type_ = match text.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token_helper(type_)
    }

    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part.
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // Consume the "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let literal = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect::<String>()
            .parse()
            .unwrap();
        self.add_token(TokenType::Num, Literal::Num(literal))
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            }
            self.advance();
        }

        // Unterminated string.
        if self.is_at_end() {
            error(self.line, "Unterminated string.");
            return;
        }

        // The closing ".
        self.advance();

        // Trim the surrounding quotes.
        let value = self
            .source
            .chars()
            .skip(self.start + 1)
            .take(self.current - self.start - 2)
            .collect();
        self.add_token(TokenType::Str, Literal::Str(value))
    }

    fn match_(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;

        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current += 1;

        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token_helper(&mut self, type_: TokenType) {
        self.add_token(type_, Literal::None);
    }

    fn add_token(&mut self, type_: TokenType, literal: Literal) {
        let text: String = self
            .source
            .chars()
            .skip(self.start)
            .take(self.current - self.start)
            .collect();
        self.tokens
            .push(Token::new(type_, text, literal, self.line))
    }
}
