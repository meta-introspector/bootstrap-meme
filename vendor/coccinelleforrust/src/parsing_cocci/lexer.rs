use std::str::CharIndices;

use crate::syntaxerror;

#[derive(Clone, Debug)]
pub enum Token<'input> {
    AT(usize), //@ (line number)
    COMMA,
    DOT,  //.
    DOTS, //...
    PIPE,
    LINEFEED,
    SEMICOLON,
    DISJLparen,
    DISJRparen,
    CODE(&'input str),
    WORD(&'input str),
}

pub type Spanned<Tok, Loc, Err> = Result<(Loc, Tok, Loc), Err>;

pub struct CocciLexer<'input> {
    stream: &'input str,
    stream_len: usize,
    code: CharIndices<'input>,
    // code: CharIndices<'input>,
    line: usize,
    col: usize,

    last: usize,

    in_header: usize,
    curr_token: Option<Token<'input>>,

    end_of_stream: bool
}

impl<'input> CocciLexer<'input> {
    pub fn new(t: &'input str) -> CocciLexer<'input> {
        CocciLexer {
            stream: t,
            stream_len: t.len(),
            code: t.char_indices(),

            line: 0,
            col: 0,

            last: 0,

            in_header: 4,
            curr_token: None,

            end_of_stream: false
        }
    }
}

impl<'input> Iterator for CocciLexer<'input> {
    type Item = Spanned<Token<'input>, (usize, usize), String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.in_header > 0 {
            let mut inword: bool = false;
            loop {
                self.col += 1;
                match self.code.next() {
                    Some((bytes, '@')) => {
                        self.last = bytes + 2;
                        self.in_header -= 1;
                        if self.in_header == 0 {
                            let (_, ch) = self.code.next().expect("Empty Patch");
                            if ch != '\n' {
                                syntaxerror!(self.line, "No newline");
                            }
                            self.line += 1;
                            self.col = 0;
                        }
                        return Some(Ok(((self.line, self.col), Token::AT(self.line), (self.line, self.col))));
                    }
                    Some((_, ',')) => {
                        return Some(Ok(((self.line, self.col), Token::COMMA, (self.line, self.col))))
                    }
                    Some((_, ';')) => {
                        return Some(Ok((
                            (self.line, self.col),
                            Token::SEMICOLON,
                            (self.line, self.col),
                        )))
                    }
                    Some((_, '\n')) => {
                        self.col = 0;
                        self.line += 1;
                        continue;
                        // return Some(Ok((
                        //     (self.line, self.col),
                        //     Token::LINEFEED,
                        //     (self.line, self.col),
                        // )));
                    }
                    Some((_, ' ')) | Some((_, '\t')) => {
                        continue;
                        // if self.code.peek().unwrap_or((0, 'a')).1.is_alphanumeric() {
                        //     let token = Token::WSPACE(&self.stream[self.last..bytes + 1]);
                        //     self.last = bytes + 1;
                        //     return Some(Ok((token, (self.line, self.col))));
                        // }

                        // continue;
                    }
                    Some((bytes, _)) => {
                        if !inword {
                            self.last = bytes;
                            inword = true;
                        }

                        let c = self.stream.as_bytes()[bytes + 1] as char;
                        if c == ' ' || c == '\t' || c == '@' || c == ';' || c == ',' || c=='\n' {
                            return Some(Ok((
                                (self.line, self.col),
                                Token::WORD(&self.stream[self.last..bytes + 1]),
                                (self.line, self.col),
                            )));
                        }
                    }
                    _ => return None,
                }
            }
        } else {
            loop {
                match &self.curr_token {
                    None => {}
                    Some(Token::DOTS) => {
                        self.code.next();
                        self.code.next();
                        self.col += 2;
                        self.curr_token = None;
                        return Some(Ok(((self.line, self.col), Token::DOTS, (self.line, self.col))));
                    }
                    Some(Token::PIPE) => {
                        self.code.next();
                        self.code.next();
                        self.col += 2;
                        self.curr_token = None;
                        return Some(Ok(((self.line, self.col), Token::PIPE, (self.line, self.col))));
                    }
                    Some(Token::DISJLparen) | Some(Token::DISJRparen) => {
                        self.code.next();
                        self.code.next();
                        self.col = 0;
                        let tmp = self.curr_token.as_ref().unwrap().clone();
                        self.curr_token = None;
                        return Some(Ok((
                            (self.line, self.col),
                            tmp,
                            (self.line, self.col),
                        )));
                    }
                    _ => {
                        panic!("Should not be set to anything else")
                    }
                }

                self.col += 1;
                match self.code.next() {
                    Some((bytes, '\n')) => {
                        self.line += 1;
                        self.col = 0;

                        if bytes + 3 > self.stream_len {
                            self.end_of_stream = true;
                            return Some(Ok((
                                (self.line, self.col),
                                Token::CODE(&self.stream[self.last..]),
                                (self.line, self.col),
                            )));
                        }

                        if &self.stream[bytes..bytes + 3] == "\n(\n" {
                            let tmp = self.last;
                            self.last = bytes + 3;
                            self.curr_token = Some(Token::DISJLparen);
                            return Some(Ok((
                                (self.line, self.col),
                                Token::CODE(&self.stream[tmp..bytes]),
                                (self.line, self.col),
                            )));
                        } else if &self.stream[bytes..bytes + 3] == "\n)\n" {
                            let tmp = self.last;
                            self.last = bytes + 3;
                            self.curr_token = Some(Token::DISJRparen);
                            return Some(Ok((
                                (self.line, self.col),
                                Token::CODE(&self.stream[tmp..bytes]),
                                (self.line, self.col),
                            )));
                        }else if &self.stream[bytes..bytes + 3] == "\n|\n" {
                            let tmp = self.last;
                            self.last = bytes + 3;
                            self.curr_token = Some(Token::PIPE);
                            return Some(Ok((
                                (self.line, self.col),
                                Token::CODE(&self.stream[tmp..bytes]),
                                (self.line, self.col),
                            )));
                        } else {
                            continue;
                        }
                    }
                    Some((bytes, '.')) => {
                        if bytes + 3 > self.stream_len {
                            self.end_of_stream = true;
                            return Some(Ok((
                                (self.line, self.col),
                                Token::CODE(&self.stream[self.last..]),
                                (self.line, self.col),
                            )));
                        }

                        if &self.stream[bytes..bytes + 3] == "..." {
                            let tmp = self.last;
                            self.last = bytes + 3;
                            self.curr_token = Some(Token::DOTS);
                            return Some(Ok((
                                (self.line, self.col),
                                Token::CODE(&self.stream[tmp..bytes]),
                                (self.line, self.col),
                            )));
                        } else {
                            continue;
                        }
                    }
                    Some((_, _)) => {
                        continue;
                    }
                    None => {
                        if !self.end_of_stream {
                            let ret = Some(Ok((
                                (self.line, self.col),
                                Token::CODE(&self.stream[self.last..]),
                                (self.line, self.col),
                            )));
                            self.end_of_stream = true;
                            return ret;
                        } else {
                            return None;
                        }
                    }
                }
            }
        }
    }
}
