extern crate ir;

use ir::ir_lexer::*;
use ir::ir_parser::*;

fn main() {
    let mut tokens = Vec::new();

    tokens.push(Token::AddKeyword ({TokLoc {row: 0, col: 0}}));
    tokens.push(Token::VarName ("target".to_string(), {TokLoc {row: 0, col: 0}}));
    tokens.push(Token::Colon ({TokLoc {row: 0, col: 0}}));
    tokens.push(Token::I8Keyword ({TokLoc {row: 0, col: 0}}));
    tokens.push(Token::VarName ("src_left".to_string(), {TokLoc {row: 0, col: 0}}));
    tokens.push(Token::VarName ("src_right".to_string(), {TokLoc {row: 0, col: 0}}));

    let node = parse(&tokens);

    print_ast(&node);
}
