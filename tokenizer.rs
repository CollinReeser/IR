use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug)]
enum Token {
    Number (i64),
    Ident (String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semicolon,
    DoubleColon,
    Colon,
    RThinArrow,
    RFatArrow,
    Equals,
    DoubleEquals,
    Minus,
    Bang,
    Dot,
    DotDot,
    Pipe,
    DoublePipe,
    Ampersand,
    DoubleAmpersand,
    LWakka,
    RWakka,
    QuestionMark,
    Comma,
    LBracket,
    RBracket,
    Underscore,

    StrLit (String),
}

fn print_tokens(tokens: &Vec<Token>) {
    for t in tokens {
        println!("{:?}", t);
    }
}

fn tokenize_str(it: &mut Peekable<Chars>) -> Option<Token> {
    let mut s = String::new();

    return if let Some (&'"') = it.peek() {
        it.next();

        let mut escaped = false;

        while let Some (c) = it.next() {
            if c == '"' && !escaped {
                break;
            }
            if c == '\\' && !escaped {
                escaped = true;
            }
            else {
                escaped = false;
            }

            s.push(c);
        }

        Some (Token::StrLit(s))
    }
    else {
        None
    }
}

fn tokenize_number(it: &mut Peekable<Chars>) -> Option<Token> {
    let mut s = String::new();

    while let Some (&c) = it.peek() {
        if c.is_digit(10) {
            s.push(c);
            it.next();
        }
        else {
            break;
        }
    }

    if s.len() == 0 {
        return None;
    }

    match s.parse::<i64>() {
        Ok (i) => return Some (Token::Number(i)),
        Err (msg) => panic!("Unexpected failure: {}", msg),
    }
}

fn tokenize_ident(it: &mut Peekable<Chars>) -> Option<Token> {
    let mut s = String::new();

    while let Some (&c) = it.peek() {
        if c.is_digit(10) || c.is_alphabetic() || c == '_' {
            s.push(c);
            it.next();
        }
        else {
            break;
        }
    }

    return if s.len() == 0 {
        None
    }
    else if s == "_" {
        Some (Token::Underscore)
    }
    else {
        Some (Token::Ident(s))
    }
}

fn tokenize_op(it: &mut Peekable<Chars>) -> Option<Token> {
    return if let Some (&c) = it.peek() {
        match c {
            '!' => { it.next(); Some (Token::Bang)         }
            '(' => { it.next(); Some (Token::LParen)       }
            ')' => { it.next(); Some (Token::RParen)       }
            '{' => { it.next(); Some (Token::LBrace)       }
            '}' => { it.next(); Some (Token::RBrace)       }
            '[' => { it.next(); Some (Token::LBracket)     }
            ']' => { it.next(); Some (Token::RBracket)     }
            '<' => { it.next(); Some (Token::LWakka)       }
            '>' => { it.next(); Some (Token::RWakka)       }
            ';' => { it.next(); Some (Token::Semicolon)    }
            '?' => { it.next(); Some (Token::QuestionMark) }
            ',' => { it.next(); Some (Token::Comma)        }
            ':' => {
                it.next();
                if let Some (&':') = it.peek() {
                    it.next();

                    Some (Token::DoubleColon)
                }
                else {
                    Some (Token::Colon)
                }
            }
            '&' => {
                it.next();
                if let Some (&'&') = it.peek() {
                    it.next();

                    Some (Token::DoubleAmpersand)
                }
                else {
                    Some (Token::Ampersand)
                }
            }
            '|' => {
                it.next();
                if let Some (&'|') = it.peek() {
                    it.next();

                    Some (Token::DoublePipe)
                }
                else {
                    Some (Token::Pipe)
                }
            }
            '-' => {
                it.next();
                if let Some (&'>') = it.peek() {
                    it.next();

                    Some (Token::RThinArrow)
                }
                else {
                    Some (Token::Minus)
                }
            }
            '=' => {
                it.next();
                if let Some (&'>') = it.peek() {
                    it.next();

                    Some (Token::RFatArrow)
                }
                else if let Some (&'=') = it.peek() {
                    it.next();

                    Some (Token::DoubleEquals)
                }
                else {
                    Some (Token::Equals)
                }
            }
            '.' => {
                it.next();
                if let Some (&'.') = it.peek() {
                    it.next();

                    Some (Token::DotDot)
                }
                else {
                    Some (Token::Dot)
                }
            }
            _ => None,
        }
    }
    else {
        None
    }
}

fn tokenize_line(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut it = line.chars().peekable();

    while let Some (&c) = it.peek() {
        if c.is_whitespace() {
            it.next();
        }
        else if c == '#' {
            break;
        }
        else if let Some (str_tok) = tokenize_str(&mut it) {
            tokens.push(str_tok);
        }
        else if let Some (op_tok) = tokenize_op(&mut it) {
            tokens.push(op_tok);
        }
        else if let Some (num_tok) = tokenize_number(&mut it) {
            tokens.push(num_tok);
        }
        else if let Some (ident_tok) = tokenize_ident(&mut it) {
            tokens.push(ident_tok);
        }
        else {
            it.next();
        }
    }

    return tokens;
}

fn main() {
    let path = Path::new("tokenizer.rs");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!(
            "couldn't open {}: {}", display, why.description()
        ),
        Ok(file) => BufReader::new(file),
    };

    let mut s = String::new();
    let mut tokens = Vec::new();
    while file.read_line(&mut s).unwrap() > 0 {
        print!("Line: {}", s);

        tokens.extend(tokenize_line(&s));

        s.clear();
    }

    print_tokens(&tokens);
}
