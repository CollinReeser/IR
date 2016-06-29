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

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Type::I8 => write!(f, "i8"),
            &Type::I16 => write!(f, "i16"),
            &Type::I32 => write!(f, "i32"),
            &Type::I64 => write!(f, "i64"),
            &Type::F32 => write!(f, "f32"),
            &Type::F64 => write!(f, "f64"),
            &Type::UserType (ref s) => write!(f, "{}", s),
        }
    }
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
            println!("add {}:{} {} {}", v1, t, v2, v3);
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

fn parse_var_type_pair(it: &mut Peekable<Iter<Token>>)
    -> Option<(Variable, Type)>
{
    return if let Some (&&Token::VarName (ref target, ref tl)) = it.peek() {
        it.next();

        if let Some (&&Token::Colon (ref tl)) = it.peek() {
            it.next();

            if let Some (type_node) = parse_type(it) {
                Some ((
                    Variable {name: target.to_owned()}, type_node
                ))
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
        None
    };
}

fn parse_binary_input_vars(it: &mut Peekable<Iter<Token>>)
    -> Option<(Variable, Variable)>
{
    return if let Some (&&Token::VarName (ref left_src, ref tl)) = it.peek() {
        it.next();

        if let Some (&&Token::VarName (ref right_src, _)) = it.peek() {
            it.next();

            Some ((
                Variable {name: left_src.to_owned()},
                Variable {name: right_src.to_owned()}
            ))
        }
        else {
            panic!("Expected ':', got trash: {:?}", tl);
        }
    }
    else {
        None
    };
}

fn parse_add(mut it: &mut Peekable<Iter<Token>>) -> Option<Node> {
    return if let Some (&&Token::AddKeyword (ref tl)) = it.peek() {
        it.next();

        if let Some ((target_var, target_type)) = parse_var_type_pair(&mut it) {
            if let Some ((left_src, right_src))
                = parse_binary_input_vars(&mut it)
            {
                Some (Node::AddInst (
                    target_type,
                    target_var,
                    left_src,
                    right_src,
                ))
            }
            else {
                panic!("Expected binary input vars, got trash: {:?}", tl);
            }
        }
        else {
            panic!("Expected <var>:<type> pair, got trash: {:?}", tl);
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