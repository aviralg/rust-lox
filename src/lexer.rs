use std::iter::Peekable;
use std::str::Chars;
use std::vec::Vec;

use crate::location::*;

#[derive(PartialEq, Clone, Debug)]
pub enum TokenType {
    // Group
    LParen,
    RParen,
    LBrace,
    RBrace,

    Semicolon,
    Comma,
    Dot,

    Minus,
    Plus,
    Slash,
    Star,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,

    Error(String),
}

#[derive(PartialEq, Clone, Debug)]
pub struct Token(Location, TokenType);

pub struct Lexer<'a> {
    row: usize,
    col: usize,
    pos: usize,
    size: usize,
    content: &'a str,
    iter: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(content: &'a str) -> Self {
        Lexer {
            row: 0,
            col: 0,
            pos: 0,
            size: content.chars().count(),
            content,
            iter: content.chars().peekable(),
        }
    }

    fn position(&self) -> Position {
        Position::new(self.row, self.col, self.pos)
    }

    fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }

    fn is_next(&mut self, ch: char) -> bool {
        match self.peek() {
            None => return false,
            Some(&ch2) => return ch == ch2,
        }
    }

    fn is_ascii_digit(&mut self) -> bool {
        match self.peek() {
            None => return false,
            Some(ch) => return ch.is_ascii_digit(),
        }
    }

    fn is_identifier_character(&mut self) -> bool {
        match self.peek() {
            None => false,
            Some(&ch) => ch == '_' || ch.is_ascii_alphanumeric(),
        }
    }

    fn get_token_type(&self, value: &str) -> TokenType {
        match value {
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
            _ => TokenType::Identifier(String::from(value)),
        }
    }

    fn next(&mut self) -> Option<char> {
        let res = self.iter.next();
        match &res {
            None => {}
            Some('\n') => {
                self.pos += 1;
                self.col = 0;
                self.row += 1;
            }
            Some(_) => {
                self.pos += 1;
                self.col += 1;
            }
        }
        res
    }

    fn next_up_to(&mut self, ch: char) -> bool {
        loop {
            let c1 = self.next();
            self.pos += 1;
            match c1 {
                None => return false,
                Some(c) => {
                    if c == '\n' {
                        self.row += 1;
                        self.col = 0;
                    }
                    if c == ch {
                        break;
                    } else {
                        continue;
                    }
                }
            }
        }
        true
    }

    fn extract(&self, beg: &Position) -> &str {
        let end = self.position();
        &self.content[beg.pos..end.pos]
    }

    fn make_token(&self, beg: Position, token_type: TokenType) -> Token {
        let end = self.position();
        Token(Location::new(beg, end), token_type)
    }

    fn make_error(&self, beg: Position, msg: &str) -> Token {
        let end = self.position();
        Token(Location::new(beg, end), TokenType::Error(String::from(msg)))
    }

    fn make_number(&self, beg: Position) -> Token {
        let value: f64 = self.extract(&beg).parse().unwrap();
        let end = self.position();
        Token(Location::new(beg, end), TokenType::Number(value))
    }

    // fn make_token_1(&self, beg: Position, token_type: TokenType) -> Token {
    //     self.make_token_n(beg, token_type, 1)
    // }
    //
    // fn make_token_2(&self, beg: Position, token_type: TokenType) -> Token {
    //     self.make_token_n(beg, token_type, 2)
    // }

    fn match_next(&mut self, beg: Position, ch: &char, pass: TokenType, fail: TokenType) -> Token {
        match self.iter.next_if_eq(&ch) {
            None => self.make_token(beg, fail),
            _ => {
                self.next();
                self.make_token(beg, pass)
            }
        }
    }

    fn eat_junk(&mut self) {
        let mut comment = false;
        loop {
            match self.peek() {
                None => break,
                Some(ch) => match &ch {
                    ' ' | '\r' | '\t' => {
                        self.next();
                    }
                    '#' => {
                        self.next();
                        comment = true;
                    }
                    '\n' => {
                        self.next();
                        comment = false;
                    }
                    // read random characters if comment is true
                    _ => {
                        if comment {
                            self.next();
                        } else {
                            break;
                        }
                    }
                },
            }
        }
    }

    // ""
    fn parse_string(&mut self, beg: Position) -> Token {
        let mut slash = false;
        loop {
            match self.peek() {
                // TODO : fix this
                None => {
                    let msg = "unterminated string literal starting at position {beg.pos}";
                    return self.make_error(beg, msg);
                }
                Some('\\') => {
                    slash = !slash;
                    self.next();
                }
                Some('"') => {
                    self.next();
                    if !slash {
                        let msg = &self.content[beg.pos..self.pos];
                        return self.make_token(beg, TokenType::String(String::from(msg)));
                    }
                }
                Some(_) => {
                    self.next();
                }
            }
        }
    }

    // <digit>+(.<digit>+)?
    fn parse_number(&mut self, beg: Position) -> Token {
        let mut has_trailing_dot = false;

        while self.is_ascii_digit() {
            self.next();
        }

        if self.is_next('.') {
            self.next();

            while self.is_ascii_digit() {
                has_trailing_dot = false;
                self.next();
            }
        }

        if has_trailing_dot {
            return self.make_error(beg, "trailing '.' in number literal");
        } else {
            return self.make_number(beg);
        }
    }

    fn parse_identifier(&mut self, beg: Position) -> Token {
        while self.is_identifier_character() {
            self.next();
        }

        let content: &str = self.extract(&beg);
        let token_type: TokenType = self.get_token_type(content);
        return self.make_token(beg, token_type);
    }

    fn scan_token(&mut self) -> Token {
        self.eat_junk();

        let beg = self.position();

        match self.next() {
            None => self.make_token(beg, TokenType::Eof),
            Some(ch) => match ch {
                '"' => self.parse_string(beg),
                '/' => self.make_token(beg, TokenType::Slash),
                '(' => self.make_token(beg, TokenType::LParen),
                ')' => self.make_token(beg, TokenType::RParen),
                '{' => self.make_token(beg, TokenType::LBrace),
                '}' => self.make_token(beg, TokenType::RBrace),
                ',' => self.make_token(beg, TokenType::Comma),
                '.' => self.make_token(beg, TokenType::Dot),
                '-' => self.make_token(beg, TokenType::Minus),
                '+' => self.make_token(beg, TokenType::Plus),
                ';' => self.make_token(beg, TokenType::Semicolon),
                '*' => self.make_token(beg, TokenType::Star),
                '!' => self.match_next(beg, &'=', TokenType::BangEqual, TokenType::Bang),
                '=' => self.match_next(beg, &'=', TokenType::EqualEqual, TokenType::Equal),
                '<' => self.match_next(beg, &'=', TokenType::LessEqual, TokenType::Less),
                '>' => self.match_next(beg, &'=', TokenType::GreaterEqual, TokenType::Greater),
                _ => {
                    if ch.is_ascii_digit() {
                        self.parse_number(beg)
                    } else if ch.is_ascii_alphabetic() || ch == '_' {
                        self.parse_identifier(beg)
                    } else {
                        self.make_token(
                            beg,
                            TokenType::Error(String::from("Invalid character {ch}")),
                        )
                    }
                }
            },
        }
    }
}

pub fn tokens(content: &str) -> Vec<Token> {
    let mut lex = Lexer::new(content);
    let mut vec = std::vec::Vec::new();
    loop {
        let token = lex.scan_token();
        let done = token.1 == TokenType::Eof;
        vec.push(token);
        if done {
            break;
        }
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn it_works() {
        assert_eq!(
            tokens("/"),
            vec![
                Token(Location::new(Position::new(0, 0, 0), Position::new(0, 1, 1)), TokenType::Slash),
                Token(Location::new(Position::new(0, 1, 1), Position::new(0, 1, 1)), TokenType::Eof)
            ]
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_string() {
        assert_eq!(
            tokens("\"I am a string\""),
            vec![
                Token(Location::new2((0, 0, 0), (0, 15, 15)), TokenType::String(String::from("\"I am a string\""))),
                Token(Location::new2((0, 15, 15), (0, 15, 15)), TokenType::Eof)
            ]
        );
    }

    #[test]
    #[rustfmt::skip]
    fn test_number() {
        assert_eq!(
            tokens("123.45"),
            vec![
                Token(Location::new2((0, 0, 0), (0, 6, 6)), TokenType::Number(123.45)),
                Token(Location::new2((0, 6, 6), (0, 6, 6)), TokenType::Eof)
            ]
        );
        assert_eq!(
            tokens("123"),
            vec![
                Token(Location::new2((0, 0, 0), (0, 3, 3)), TokenType::Number(123.0)),
                Token(Location::new2((0, 3, 3), (0, 3, 3)), TokenType::Eof)
            ]
        );
    }
}
