extern crate ir;

use ir::ir_lexer::*;
use ir::ir_parser::*;
use ir::ir_typechecker::*;
use ir::ir_reg_allocer::*;

use std::env;

extern crate getopts;
use getopts::Options;

use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.reqopt("f", "file", "Input file to parse", "FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    let filename = match matches.opt_str("f") {
        Some(x) => x,
        None => {
            println!("Must provide a -f filename");
            return;
        },
    };

    let path = Path::new(&filename);
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

    if let Some(node) = parse(&tokens) {
        if !typecheck(&node) {
            panic!("Source does not typecheck!");
        }

        let rig = generate_rig(&node);

        println!("{}", dump_dot_format(&rig));
    }

}

