use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::Enumerate;
use std::iter::Peekable;
use std::path::Path;
use std::str::Chars;

#[derive(Debug)]
#[derive(Clone)]
enum Token {
    AsKeyword (TokLoc),
    BreakKeyword (TokLoc),
    ContinueKeyword (TokLoc),
    ElseKeyword (TokLoc),
    EnumKeyword (TokLoc),
    FnKeyword (TokLoc),
    ForKeyword (TokLoc),
    IfKeyword (TokLoc),
    InKeyword (TokLoc),
    LetKeyword (TokLoc),
    MatchKeyword (TokLoc),
    MutKeyword (TokLoc),
    ReturnKeyword (TokLoc),
    UseKeyword (TokLoc),
    WhileKeyword (TokLoc),
    Number (i64, TokLoc),
    CharLit (String, TokLoc),
    Ident (String, TokLoc),
    StrLit (String, TokLoc),
    Ampersand (TokLoc),
    Bang (TokLoc),
    Colon (TokLoc),
    Comma (TokLoc),
    Dot (TokLoc),
    DotDot (TokLoc),
    DoubleAmpersand (TokLoc),
    DoubleColon (TokLoc),
    DoubleEquals (TokLoc),
    DoublePipe (TokLoc),
    Equals (TokLoc),
    LBrace (TokLoc),
    LBracket (TokLoc),
    LParen (TokLoc),
    LWakka (TokLoc),
    Minus (TokLoc),
    Pipe (TokLoc),
    QuestionMark (TokLoc),
    RBrace (TokLoc),
    RBracket (TokLoc),
    RFatArrow (TokLoc),
    RParen (TokLoc),
    RThinArrow (TokLoc),
    RWakka (TokLoc),
    Semicolon (TokLoc),
    Underscore (TokLoc),
}

#[derive(Debug)]
#[derive(Clone)]
struct TokLoc {
    row: u64,
    col: u64,
}

fn print_tokens(tokens: &Vec<Token>) {
    for t in tokens {
        println!("{:?}", t);
    }
}

fn tokenize_char(it: &mut Peekable<Enumerate<Chars>>, row: u64)
    -> Option<Token>
{
    let mut s = String::new();

    return if let Some (&(col, '\'')) = it.peek() {
        it.next();

        let mut escaped = false;
        let mut index = 0;

        while let Some ((_, c)) = it.next() {
            if index > 2 || (c == '\'' && index == 0) {
                panic!("bad char");
            }
            else if c == '\'' && !escaped {
                break;
            }
            else if c == '\\' && !escaped {
                escaped = true;
            }
            else {
                escaped = false;
            }

            s.push(c);
            index += 1;
        }

        Some (Token::CharLit(s, TokLoc {row: row, col: col as u64}))
    }
    else {
        None
    }
}

fn tokenize_ident(it: &mut Peekable<Enumerate<Chars>>, row: u64)
    -> Option<Token>
{
    let mut s = String::new();

    let mut col_capture = 0;

    if let Some (&(col, _)) = it.peek() {
        col_capture = col;
    }

    while let Some (&(_, c)) = it.peek() {
        if c.is_digit(10) || c.is_alphabetic() || c == '_' {
            s.push(c);
            it.next();
        }
        else {
            break;
        }
    }

    let keyword = match s.as_ref() {
        "as"       => Some (
            Token::AsKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "break"    => Some (
            Token::BreakKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "continue" => Some (
            Token::ContinueKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "else"     => Some (
            Token::ElseKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "enum"     => Some (
            Token::EnumKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "fn"       => Some (
            Token::FnKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "for"      => Some (
            Token::ForKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "if"       => Some (
            Token::IfKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "in"       => Some (
            Token::InKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "let"      => Some (
            Token::LetKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "match"    => Some (
            Token::MatchKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "mut"      => Some (
            Token::MutKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "return"   => Some (
            Token::ReturnKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "use"      => Some (
            Token::UseKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        "while"    => Some (
            Token::WhileKeyword (TokLoc {row: row, col: col_capture as u64})
        ),
        _          => None,
    };

    return if s.len() == 0 {
        None
    }
    else if let Some (_) = keyword {
        keyword
    }
    else if s == "_" {
        Some (Token::Underscore (TokLoc {row: row, col: col_capture as u64}))
    }
    else {
        Some (Token::Ident(s, TokLoc {row: row, col: col_capture as u64}))
    }
}

fn tokenize_number(it: &mut Peekable<Enumerate<Chars>>, row: u64)
    -> Option<Token>
{
    let mut s = String::new();

    let mut col_capture = 0;

    if let Some (&(col, _)) = it.peek() {
        col_capture = col;
    }

    while let Some (&(_, c)) = it.peek() {
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
        Ok (i) => return Some (
            Token::Number(i, TokLoc {row: row, col: col_capture as u64})
        ),
        Err (msg) => panic!("Unexpected failure: {}", msg),
    }
}

fn tokenize_op(it: &mut Peekable<Enumerate<Chars>>, row: u64) -> Option<Token> {
    return if let Some (&(col, c)) = it.peek() {
        match c {
            '!' => {
                it.next();
                Some (Token::Bang (TokLoc {row: row, col: col as u64}))
            }
            '(' => {
                it.next();
                Some (Token::LParen (TokLoc {row: row, col: col as u64}))
            }
            ')' => {
                it.next();
                Some (Token::RParen (TokLoc {row: row, col: col as u64}))
            }
            '{' => {
                it.next();
                Some (Token::LBrace (TokLoc {row: row, col: col as u64}))
            }
            '}' => {
                it.next();
                Some (Token::RBrace (TokLoc {row: row, col: col as u64}))
            }
            '[' => {
                it.next();
                Some (Token::LBracket (TokLoc {row: row, col: col as u64}))
            }
            ']' => {
                it.next();
                Some (Token::RBracket (TokLoc {row: row, col: col as u64}))
            }
            '<' => {
                it.next();
                Some (Token::LWakka (TokLoc {row: row, col: col as u64}))
            }
            '>' => {
                it.next();
                Some (Token::RWakka (TokLoc {row: row, col: col as u64}))
            }
            ';' => {
                it.next();
                Some (Token::Semicolon (TokLoc {row: row, col: col as u64}))
            }
            '?' => {
                it.next();
                Some (Token::QuestionMark (TokLoc {row: row, col: col as u64}))
            }
            ',' => {
                it.next();
                Some (Token::Comma (TokLoc {row: row, col: col as u64}))
            }
            ':' => {
                it.next();
                if let Some (&(col, ':')) = it.peek() {
                    it.next();

                    Some (
                        Token::DoubleColon (
                            TokLoc {row: row, col: col as u64}
                        )
                    )
                }
                else {
                    Some (Token::Colon (TokLoc {row: row, col: col as u64}))
                }
            }
            '&' => {
                it.next();
                if let Some (&(col, '&')) = it.peek() {
                    it.next();

                    Some (
                        Token::DoubleAmpersand (
                            TokLoc {row: row, col: col as u64}
                        )
                    )
                }
                else {
                    Some (Token::Ampersand (TokLoc {row: row, col: col as u64}))
                }
            }
            '|' => {
                it.next();
                if let Some (&(col, '|')) = it.peek() {
                    it.next();

                    Some (
                        Token::DoublePipe (
                            TokLoc {row: row, col: col as u64}
                        )
                    )
                }
                else {
                    Some (Token::Pipe (TokLoc {row: row, col: col as u64}))
                }
            }
            '-' => {
                it.next();
                if let Some (&(col, '>')) = it.peek() {
                    it.next();

                    Some (
                        Token::RThinArrow (TokLoc {row: row, col: col as u64})
                    )
                }
                else {
                    Some (Token::Minus (TokLoc {row: row, col: col as u64}))
                }
            }
            '=' => {
                it.next();
                if let Some (&(col, '>')) = it.peek() {
                    it.next();

                    Some (Token::RFatArrow (TokLoc {row: row, col: col as u64}))
                }
                else if let Some (&(col, '=')) = it.peek() {
                    it.next();

                    Some (
                        Token::DoubleEquals (
                            TokLoc {row: row, col: col as u64}
                        )
                    )
                }
                else {
                    Some (Token::Equals (TokLoc {row: row, col: col as u64}))
                }
            }
            '.' => {
                it.next();
                if let Some (&(col, '.')) = it.peek() {
                    it.next();

                    Some (Token::DotDot (TokLoc {row: row, col: col as u64}))
                }
                else {
                    Some (Token::Dot (TokLoc {row: row, col: col as u64}))
                }
            }
            _ => None,
        }
    }
    else {
        None
    }
}

fn tokenize_str(it: &mut Peekable<Enumerate<Chars>>, row: u64)
    -> Option<Token>
{
    let mut s = String::new();

    return if let Some (&(col, '"')) = it.peek() {
        it.next();

        let mut escaped = false;

        while let Some ((_, c)) = it.next() {
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

        Some (Token::StrLit(s, TokLoc {row: row, col: col as u64}))
    }
    else {
        None
    }
}

fn tokenize_line(line: &str, row: u64) -> Vec<Token> {
    let mut tokens = Vec::new();

    let mut it = line.chars().enumerate().peekable();

    while let Some (&(_, c)) = it.peek() {
        if c.is_whitespace() {
            it.next();
        }
        else if c == '#' {
            break;
        }
        else if let Some (str_tok) = tokenize_str(&mut it, row) {
            tokens.push(str_tok);
        }
        else if let Some (char_tok) = tokenize_char(&mut it, row) {
            tokens.push(char_tok);
        }
        else if let Some (op_tok) = tokenize_op(&mut it, row) {
            tokens.push(op_tok);
        }
        else if let Some (num_tok) = tokenize_number(&mut it, row) {
            tokens.push(num_tok);
        }
        else if let Some (ident_tok) = tokenize_ident(&mut it, row) {
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

    let mut row = 0;

    while file.read_line(&mut s).unwrap() > 0 {
        // print!("Line: {}", s);

        tokens.extend(tokenize_line(&s, row));

        row += 1;

        s.clear();
    }

    print_tokens(&tokens);
}
