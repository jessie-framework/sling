use putbackpeekmore::PutBackPeekMore;
use sling_cache::{Decode, Encode};
use std::str::Chars;
mod reader;

pub struct Tokenizer<'chars> {
    stream: PutBackPeekMore<Chars<'chars>, 4>,
    position: usize,
}

impl<'chars> Tokenizer<'chars> {
    pub fn new(input: &'chars str) -> Self {
        Self {
            stream: PutBackPeekMore::new(input.chars()),
            position: 0,
        }
    }

    pub(crate) fn start_record(&self) -> Position {
        Position {
            start: self.position,
            end: 0,
        }
    }

    pub(crate) fn end_record(&self, mut input: Position) -> Position {
        input.end = self.position;
        input
    }

    pub(crate) fn peek_stream(&mut self) -> &Option<char> {
        self.stream.peek()
    }

    pub(crate) fn advance_stream(&mut self) -> Option<char> {
        self.position += 1;
        self.stream.next()
    }

    pub(crate) fn peek_stream_value(&mut self, input: usize) -> &[Option<char>] {
        self.stream.peek_value(input)
    }

    pub fn consume_next_token(&mut self) -> Token {
        self.consume_comment();
        let peek = self.peek_stream();

        match peek {
            Some(next_code_point) => match next_code_point {
                '(' => self.consume_left_paranthesis(),
                ')' => self.consume_right_paranthesis(),
                '{' => self.consume_left_curly(),
                '}' => self.consume_right_curly(),
                '[' => self.consume_left_square(),
                ']' => self.consume_right_square(),
                '<' => self.consume_left_angle(),
                '>' => self.consume_right_angle(),
                '&' => self.consume_ampersand(),
                '+' => self.consume_plus(),
                '-' => self.consume_minus(),
                '/' => self.consume_slash(),
                '*' => self.consume_asterisk(),
                '!' => self.consume_exclamation_mark(),
                '?' => self.consume_question_mark(),
                '~' => self.consume_tilde(),
                '^' => self.consume_caret(),
                '@' => self.consume_at_sign(),
                '%' => self.consume_modulo(),
                '|' => self.consume_pipe(),
                ';' => self.consume_semicolon(),
                ':' => self.consume_colon(),
                '.' => self.consume_dot(),
                ',' => self.consume_comma(),
                '=' => self.consume_equal_sign(),
                '"' => self.consume_string(),
                v if v.is_ascii_whitespace() => self.consume_whitespace(),
                v if v.is_ascii_alphabetic() => self.consume_ident_token(),
                v if v.is_ascii_digit() => self.consume_digit_token(),
                _ => {
                    //TODO : compiler error
                    unreachable!()
                }
            },
            None => Token::EOF,
        }
    }

    #[inline]
    pub(crate) fn consume_comment(&mut self) {
        match self.peek_stream_value(2) {
            [Some('/'), Some('/')] => {
                self.consume_single_line_comment();
            }
            [Some('/'), Some('*')] => {
                self.consume_multi_line_comment();
            }
            _ => {}
        }
    }

    #[inline]
    pub(crate) fn consume_left_paranthesis(&mut self) -> Token {
        self.advance_stream();
        Token::LeftParenthesis
    }
    #[inline]
    pub(crate) fn consume_right_paranthesis(&mut self) -> Token {
        self.advance_stream();
        Token::RightParenthesis
    }
    #[inline]
    pub(crate) fn consume_left_curly(&mut self) -> Token {
        self.advance_stream();
        Token::LeftCurly
    }

    #[inline]
    pub(crate) fn consume_right_curly(&mut self) -> Token {
        self.advance_stream();
        Token::RightCurly
    }
    #[inline]
    pub(crate) fn consume_left_square(&mut self) -> Token {
        self.advance_stream();
        Token::LeftSquare
    }
    #[inline]
    pub(crate) fn consume_right_square(&mut self) -> Token {
        self.advance_stream();
        Token::RightSquare
    }
    #[inline]
    pub(crate) fn consume_plus(&mut self) -> Token {
        self.advance_stream();
        Token::Plus
    }
    #[inline]
    pub(crate) fn consume_minus(&mut self) -> Token {
        self.advance_stream();
        Token::Minus
    }
    #[inline]
    pub(crate) fn consume_asterisk(&mut self) -> Token {
        self.advance_stream();
        Token::Asterisk
    }
    #[inline]
    pub(crate) fn consume_question_mark(&mut self) -> Token {
        self.advance_stream();
        Token::QuestionMark
    }
    #[inline]
    pub(crate) fn consume_tilde(&mut self) -> Token {
        self.advance_stream();
        Token::Tilde
    }
    #[inline]
    pub(crate) fn consume_caret(&mut self) -> Token {
        self.advance_stream();
        Token::Caret
    }

    #[inline]
    pub(crate) fn consume_at_sign(&mut self) -> Token {
        self.advance_stream();
        Token::AtSign
    }
    #[inline]
    pub(crate) fn consume_modulo(&mut self) -> Token {
        self.advance_stream();
        Token::Modulo
    }
    #[inline]
    pub(crate) fn consume_semicolon(&mut self) -> Token {
        self.advance_stream();
        Token::Semicolon
    }
    #[inline]
    pub(crate) fn consume_colon(&mut self) -> Token {
        self.advance_stream();
        Token::Colon
    }
    #[inline]
    pub(crate) fn consume_dot(&mut self) -> Token {
        self.advance_stream();
        Token::Dot
    }
    #[inline]
    pub(crate) fn consume_comma(&mut self) -> Token {
        self.advance_stream();
        Token::Comma
    }

    #[inline]
    pub(crate) fn consume_pipe(&mut self) -> Token {
        // generational function name
        self.advance_stream();
        if self.peek_stream() == &Some('|') {
            self.advance_stream();
            return Token::Or;
        }
        Token::Pipe
    }

    #[inline]
    pub(crate) fn consume_ampersand(&mut self) -> Token {
        self.advance_stream();
        if self.peek_stream() == &Some('&') {
            self.advance_stream();
            return Token::And;
        }
        Token::Ampersand
    }
    #[inline]
    pub(crate) fn consume_equal_sign(&mut self) -> Token {
        self.advance_stream();
        if self.peek_stream() == &Some('=') {
            self.advance_stream();
            return Token::EqualTo;
        }
        Token::EqualSign
    }
    #[inline]
    pub(crate) fn consume_exclamation_mark(&mut self) -> Token {
        self.advance_stream();
        if self.peek_stream() == &Some('=') {
            self.advance_stream();
            return Token::NotEqualTo;
        }
        Token::ExclamationMark
    }

    #[inline]
    pub(crate) fn consume_left_angle(&mut self) -> Token {
        self.advance_stream();
        if self.peek_stream() == &Some('=') {
            self.advance_stream();
            return Token::LessThanAndEqual;
        }
        Token::LeftAngle
    }
    #[inline]
    pub(crate) fn consume_right_angle(&mut self) -> Token {
        self.advance_stream();
        if self.peek_stream() == &Some('=') {
            self.advance_stream();
            return Token::GreaterThanAndEqual;
        }
        Token::RightAngle
    }

    #[inline]
    pub(crate) fn consume_string(&mut self) -> Token {
        let record = self.start_record();
        self.advance_stream();
        loop {
            let peek = self.peek_stream();
            match peek {
                Some('"') => {
                    self.advance_stream();
                    return Token::String(self.end_record(record));
                }
                Some(_v) => {
                    // if *v == '\\' {
                    //     out.push(self.consume_escaped_char())
                    // } else {
                    //     out.push(*v);
                    // } // TODO : we are going to analyze strings later
                    self.advance_stream();
                }
                None => {
                    //TODO: Compiler error
                    unreachable!();
                }
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn consume_escaped_char(&mut self) -> char {
        self.advance_stream();
        match self.peek_stream_value(2) {
            [Some('u'), Some('{')] => {
                self.advance_stream();
                self.advance_stream();
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }

    #[inline]
    pub(crate) fn consume_whitespace(&mut self) -> Token {
        while let Some(v) = self.peek_stream()
            && v.is_ascii_whitespace()
        {
            self.advance_stream();
        }

        Token::Whitespace
    }

    #[inline]
    pub(crate) fn consume_digit_token(&mut self) -> Token {
        let record = self.start_record();
        loop {
            let peek = self.peek_stream();
            if let Some(v) = peek
                && (v.is_ascii_digit())
            {
                self.advance_stream();
                continue;
            }
            break;
        }
        Token::Digit(self.end_record(record))
    }

    #[inline]
    pub(crate) fn consume_slash(&mut self) -> Token {
        self.advance_stream();
        Token::Slash
    }

    #[inline]
    pub(crate) fn consume_single_line_comment(&mut self) {
        loop {
            let next = self.advance_stream();
            match next {
                Some('\n') | None => return,
                _ => {}
            }
        }
    }

    #[inline]
    pub(crate) fn consume_multi_line_comment(&mut self) {
        self.advance_stream();
        loop {
            let peek = self.peek_stream_value(2);
            if peek == [Some('*'), Some('/')] {
                self.advance_stream();
                self.advance_stream();
                return;
            }
            if peek == [None] {
                //TODO : compiler error
                return;
            }
            self.advance_stream();
        }
    }
    #[inline]
    pub(crate) fn consume_ident_token(&mut self) -> Token {
        let record = self.start_record();
        loop {
            let peek = self.peek_stream();
            if let Some(v) = peek
                && (v.is_ascii_alphabetic() || v.is_ascii_digit() || *v == '_')
            {
                self.advance_stream();
                continue;
            }
            break;
        }
        Token::Ident(self.end_record(record))
    }
}

#[derive(PartialEq, Debug, Decode, Encode, Hash)]
pub enum Token {
    EqualTo,             // ==
    NotEqualTo,          // !=
    GreaterThanAndEqual, // >=
    LessThanAndEqual,    // <=
    And,                 // &&
    Or,                  // ||
    AtSign,              // @
    EqualSign,           // =
    Semicolon,           // ;
    Colon,               // :
    Comma,               // ,
    Dot,                 // .
    Plus,                // +
    Minus,               // -
    Slash,               // /
    Asterisk,            // *
    Ampersand,           // &
    Modulo,              // %
    ExclamationMark,     // !
    QuestionMark,        // ?
    Tilde,               // ~
    Caret,               // ^
    Pipe,                // |
    LeftParenthesis,     // (
    RightParenthesis,    // )
    LeftSquare,          // [
    RightSquare,         // ]
    LeftCurly,           // {
    RightCurly,          // }
    LeftAngle,           // <
    RightAngle,          // >
    Whitespace,          // lmao
    Digit(Position),     //21321
    Ident(Position),     //things_like_this OR this_2
    String(Position),    //"things like this"
    Char(char),          // 'h'
    Integer(u64),
    EOF, // End of file
}

#[derive(PartialEq, Debug, Decode, Encode, Hash)]
pub struct Position {
    start: usize,
    end: usize,
}
