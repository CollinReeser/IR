use std::path::Path;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::error::Error;

extern crate ir;

use ir::ir_lexer::*;

fn main() {
    let path = Path::new("lex.rs");
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
        tokens.extend(tokenize_line(&s, row));

        row += 1;

        s.clear();
    }

    print_tokens(&tokens);
}
