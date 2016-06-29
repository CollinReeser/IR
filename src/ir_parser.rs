use ir_lexer::*;
use std::fmt;

use std::iter::Peekable;
use std::slice::Iter;

#[derive(Debug)]
#[derive(Clone)]
pub enum Type {
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    UserType (String),
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Variable {
    pub name: String,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%{}", self.name)
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Node {
    AddInst (Type, Variable, Variable, Variable),
}

pub fn print_ast(node: &Node) {
    match node {
        &Node::AddInst (ref t, ref v1, ref v2, ref v3) => {
            println!("{}:{:?} = {} + {}", v1, t, v2, v3);
        }
        // _ => {},
    }
}

fn parse_type(it: &mut Peekable<Iter<Token>>) -> Option<Type> {
    return if let Some (&&Token::I8Keyword (_)) = it.peek() {
        it.next();

        Some (Type::I8)
    }
    else if let Some (&&Token::I16Keyword (_)) = it.peek() {
        it.next();

        Some (Type::I16)
    }
    else if let Some (&&Token::I32Keyword (_)) = it.peek() {
        it.next();

        Some (Type::I32)
    }
    else if let Some (&&Token::I64Keyword (_)) = it.peek() {
        it.next();

        Some (Type::I64)
    }
    else if let Some (&&Token::F32Keyword (_)) = it.peek() {
        it.next();

        Some (Type::F32)
    }
    else if let Some (&&Token::F64Keyword (_)) = it.peek() {
        it.next();

        Some (Type::F64)
    }
    else {
        None
    };
}

fn parse_add(it: &mut Peekable<Iter<Token>>) -> Option<Node> {
    return if let Some (&&Token::AddKeyword (ref tl)) = it.peek() {
        it.next();

        if let Some (&&Token::VarName (ref target, ref tl)) = it.peek() {
            it.next();

            if let Some (&&Token::Colon (ref tl)) = it.peek() {
                it.next();

                if let Some (type_node) = parse_type(it) {
                    if let Some (&&Token::VarName (ref src_left, ref tl)) = it.peek() {
                        it.next();

                        if let Some (&&Token::VarName (ref src_right, _)) = it.peek() {
                            it.next();

                            Some (Node::AddInst (
                                type_node,
                                Variable {name: target.to_owned()},
                                Variable {name: src_left.to_owned()},
                                Variable {name: src_right.to_owned()},
                            ))
                        }
                        else {
                            panic!("Expected variable name, got trash: {:?}", tl);
                        }
                    }
                    else {
                        panic!("Expected variable name, got trash: {:?}", tl);
                    }
                }
                else {
                    panic!("Expected type, got trash: {:?}", tl);
                }
            }
            else {
                panic!("Expected ':', got trash: {:?}", tl);
            }
        }
        else {
            panic!("Expected variable name, got trash: {:?}", tl);
        }
    }
    else {
        None
    }
}

pub fn parse(tokens: &Vec<Token>) -> Node {
    let mut it = tokens.iter().peekable();

    while let Some (_) = it.peek() {
        if let Some (node) = parse_add(&mut it) {
            return node;
        }
    }

    return Node::AddInst (
        Type::I8,
        Variable {name: "s".to_string()},
        Variable {name: "s".to_string()},
        Variable {name: "s".to_string()}
    );
}
