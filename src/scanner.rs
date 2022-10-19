use std::{
    iter::{Enumerate, Peekable},
    str::Chars,
};

use super::{
    token::{LineNo, Token},
    token_type::TokenType::{self, *},
};

trait Scannable {
    fn scanner(&self) -> Scanner;
}

impl Scannable for &str {
    fn scanner(&self) -> Scanner {
        Scanner {
            string: &self,
            iter: self.chars().enumerate().peekable(),
            line_count: 1,
            char_count: 0,
            done: false,
        }
    }
}

struct Scanner<'a> {
    string: &'a str,
    iter: Peekable<Enumerate<Chars<'a>>>,
    line_count: LineNo,
    char_count: usize,
    done: bool,
}

impl<'a> Scanner<'a> {
    fn peek(&mut self) -> char {
        self.iter.peek().map(|(.., b)| *b).unwrap_or('@')
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let next = self.iter.next();
        if next.is_none() {
            self.done = true;
            return Some(Token {
                content: "",
                line: self.line_count,
                token_type: EndOfFile,
            });
        }
        let (char_count, next_char) = next.unwrap();
        self.char_count = char_count;
        macro_rules! single {
            ($token_type:ident) => {
                Some(Token {
                    content: &self.string[char_count..char_count + 1],
                    line: self.line_count,
                    token_type: $token_type,
                })
            };
        }
        let mut double = |yes: TokenType, no: TokenType| {
            let (char_count, next_char) = self.iter.next().unwrap();
            self.char_count = char_count;
            Some(Token {
                content: &self.string[char_count - 2..char_count],
                line: self.line_count,
                token_type: if next_char == '=' { yes } else { no },
            })
        };
        fn is_lead_identifier(c: char) -> bool {
            c.is_alphabetic() || c == '_'
        }
        match next_char {
            '0'..='9' => {
                let start = char_count;
                while '0' <= self.peek() && self.peek() <= '9' {
                    self.char_count = self.iter.next().unwrap().0;
                }
                if self.peek() == '.' {
                    self.iter.next();
                    while '0' <= self.peek() && self.peek() <= '9' {
                        self.char_count = self.iter.next().unwrap().0;
                    }
                }
                Some(Token {
                    line: self.line_count,
                    content: &self.string[start..self.char_count + 1],
                    token_type: Number,
                })
            }
            '(' => single!(LeftParen),
            ')' => single!(RightParen),
            '{' => single!(LeftBrace),
            '}' => single!(RightBrace),
            ';' => single!(Semicolon),
            ',' => single!(Comma),
            '.' => single!(Dot),
            '-' => single!(Minus),
            '+' => single!(Plus),
            '*' => single!(Star),
            '/' => {
                if self.peek() == '/' {
                    while self.peek() != '\n' {
                        self.char_count = self.iter.next().unwrap().0;
                    }
                    return self.next();
                }
                single!(Slash)
            }
            '"' => {
                let start = self.char_count;
                while self.peek() != '"' {
                    if self.iter.next().unwrap().1 == '\n' {
                        self.line_count += 1;
                    }
                }
                self.char_count = self.iter.next().unwrap().0;
                Some(Token {
                    content: &self.string[start..self.char_count],
                    line: self.line_count,
                    token_type: StringLiteral,
                })
            }

            '!' => double(BangEqual, Bang),
            '=' => double(EqualEqual, Equal),
            '<' => double(LessEqual, Less),
            '>' => double(GreaterEqual, Greater),
            ' ' | '\t' | '\r' => self.next(),
            '\n' => {
                self.line_count += 1;
                self.next()
            }
            _ if is_lead_identifier(next_char) => {
                let start = self.char_count;
                while self.peek().is_alphanumeric() {
                    self.char_count = self.iter.next().unwrap().0;
                }
                let content = &self.string[start..self.char_count + 1];
                let rest = |rest: &str, tt: TokenType| -> TokenType {
                    if content == rest {
                        tt
                    } else {
                        Identifier
                    }
                };
                let mut chars = content.chars();
                Some(Token {
                    line: self.line_count,
                    content,
                    token_type: match chars.next().unwrap() {
                        'a' => rest("and", And),
                        'c' => rest("class", Class),
                        'e' => rest("else", Else),
                        'i' => rest("if", If),
                        'n' => rest("nil", Nil),
                        'o' => rest("or", Or),
                        'p' => rest("print", Print),
                        'r' => rest("return", Return),
                        's' => rest("super", Super),
                        'v' => rest("var", Var),
                        'w' => rest("while", While),
                        'f' => match chars.next().unwrap() {
                            'a' => rest("false", False),
                            'o' => rest("for", For),
                            'u' => rest("fun", Fun),
                            _ => Identifier,
                        },
                        't' => match chars.next().unwrap() {
                            'h' => rest("this", This),
                            'r' => rest("true", True),
                            _ => Identifier,
                        },
                        _ => Identifier,
                    },
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use std::fs::read_to_string;

    use super::*;
    #[test]
    fn scan() {
        let x = read_to_string("tests/programs/fib.lox").unwrap();
        for t in x.as_str().scanner() {
            println!("{:#?}", t);
        }
    }
}

#[derive(Debug)]
pub struct TokenDebug {
    pub content: String,
    pub line: LineNo,
    pub token_type: TokenType,
}

impl<'a> Into<TokenDebug> for Token<'a> {
    fn into(self) -> TokenDebug {
        let Token {
            content,
            line,
            token_type,
        } = self;
        TokenDebug {
            content: String::from(content),
            line,
            token_type,
        }
    }
}

pub type Scanning = Vec<TokenDebug>;

pub trait Scanned {
    fn scan(&self) -> Scanning;
}

impl Scanned for &str {
    fn scan(&self) -> Scanning {
        self.scanner().map(|t| t.into()).collect()
    }
}
