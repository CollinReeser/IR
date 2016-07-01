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
pub struct VarTypePair {
    pub name: String,
    pub typename: Type,
}

impl fmt::Display for VarTypePair {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "%{}:{}", self.name, self.typename)
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct FuncSig {
    pub name: String,
    pub typename: Type,
    pub arglist: Vec<VarTypePair>,
}

impl fmt::Display for FuncSig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut farglist = String::new();

        if let Some ((last_elem, firsts)) = self.arglist.split_last() {
            for arg in firsts {
                farglist.push_str(
                    &format!("{}, ", arg)
                );
            }

            farglist.push_str(
                &format!("{}", last_elem)
            );
        }

        write!(f, "func @{}:{} ({})", self.name, self.typename, farglist)
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum LetValue {
    LetVariable (Variable),
    LetInteger (i64),
}

impl fmt::Display for LetValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &LetValue::LetVariable (ref v) => write!(f, "%{}", v.name),
            &LetValue::LetInteger (i) => write!(f, "{}", i),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Stmt {
    AddInst  (VarTypePair, Variable, Variable),
    SubInst  (VarTypePair, Variable, Variable),
    LetInst (VarTypePair, LetValue),
}

#[derive(Debug)]
#[derive(Clone)]
pub enum Node {
    FuncDef (FuncSig, Vec<Stmt>),
}

pub fn print_ast(node: &Node) {
    match node {
        &Node::FuncDef (ref sig, ref stmt_list) => {
            println!("{} {{", sig);
            for stmt in stmt_list {
                match stmt {
                    &Stmt::AddInst (ref vtp, ref v2, ref v3) => {
                        println!("    add   {} {} {}", vtp, v2, v3);
                    }
                    &Stmt::SubInst (ref vtp, ref v2, ref v3) => {
                        println!("    sub   {} {} {}", vtp, v2, v3);
                    }
                    &Stmt::LetInst (ref vtp, ref v2) => {
                        println!("    let   {} {}", vtp, v2);
                    }
                }
            }
            println!("}}");
        }
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

fn parse_var_type_pair(mut it: &mut Peekable<Iter<Token>>)
    -> Option<VarTypePair>
{
    return if let Some (&&Token::VarName (ref varname, ref tl)) = it.peek() {
        it.next();

        if let Some (&&Token::Colon (ref tl)) = it.peek() {
            it.next();

            if let Some (type_node) = parse_type(&mut it) {
                Some (
                    VarTypePair {name: varname.to_owned(), typename: type_node}
                )
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

fn parse_func_type_pair(mut it: &mut Peekable<Iter<Token>>)
    -> Option<(String, Type)>
{
    return if let Some (&&Token::FuncName (ref funcname, ref tl)) = it.peek() {
        it.next();

        if let Some (&&Token::Colon (ref tl)) = it.peek() {
            it.next();

            if let Some (type_node) = parse_type(&mut it) {
                Some ((
                    funcname.to_owned(), type_node
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

fn parse_binary_input_vars(mut it: &mut Peekable<Iter<Token>>)
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

fn parse_let_value(mut it: &mut Peekable<Iter<Token>>)
    -> Option<LetValue>
{
    return if let Some (&&Token::Integer (i, _)) = it.peek() {
        it.next();

        Some (LetValue::LetInteger (i))
    }
    else if let Some (&&Token::VarName (ref varname, _)) = it.peek() {
        it.next();

        Some (LetValue::LetVariable (Variable {name: varname.to_owned()}))
    }
    else {
        None
    };
}

fn parse_let(mut it: &mut Peekable<Iter<Token>>) -> Option<Stmt> {
    return if let Some (&&Token::LetKeyword (ref tl)) = it.peek() {
        it.next();

        if let Some (dest_var_type_pair) = parse_var_type_pair(&mut it) {
            if let Some (let_value) = parse_let_value(&mut it)
            {
                Some (Stmt::LetInst (
                    dest_var_type_pair,
                    let_value,
                ))
            }
            else {
                panic!("Expected unary input var, got trash: {:?}", tl);
            }
        }
        else {
            panic!("Expected <var>:<type> pair, got trash: {:?}", tl);
        }
    }
    else {
        None
    };
}

fn parse_add(mut it: &mut Peekable<Iter<Token>>) -> Option<Stmt> {
    return if let Some (&&Token::AddKeyword (ref tl)) = it.peek() {
        it.next();

        if let Some (target_var_type_pair) = parse_var_type_pair(&mut it) {
            if let Some ((left_src, right_src))
                = parse_binary_input_vars(&mut it)
            {
                Some (Stmt::AddInst (
                    target_var_type_pair,
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
    };
}

fn parse_sub(mut it: &mut Peekable<Iter<Token>>) -> Option<Stmt> {
    return if let Some (&&Token::SubKeyword (ref tl)) = it.peek() {
        it.next();

        if let Some (target_var_type_pair) = parse_var_type_pair(&mut it) {
            if let Some ((left_src, right_src))
                = parse_binary_input_vars(&mut it)
            {
                Some (Stmt::SubInst (
                    target_var_type_pair,
                    left_src,
                    right_src,
                ))
            }
            else {
                panic!("Expected binary input vars: {:?}", tl);
            }
        }
        else {
            panic!("Expected <var>:<type> pair: {:?}", tl);
        }
    }
    else {
        None
    };
}

fn parse_arg_list (mut it: &mut Peekable<Iter<Token>>)
    -> Vec<VarTypePair>
{
    let mut arg_list = Vec::new();

    while let Some (target_var_type_pair) = parse_var_type_pair(&mut it) {
        arg_list.push(target_var_type_pair);

        if let Some (&&Token::Comma(_)) = it.peek() {
            it.next();
        }
        else {
            break;
        }
    }

    return arg_list;
}

fn parse_func_sig(mut it: &mut Peekable<Iter<Token>>)
    -> Option<FuncSig>
{
    return if let Some ((func_name, func_type))
        = parse_func_type_pair(&mut it)
    {

        if let Some (&&Token::LParen (ref tl)) = it.peek() {
            it.next();

            let arg_list = parse_arg_list(it);

            if let Some (&&Token::RParen (_)) = it.peek() {
                it.next();

                Some (
                    FuncSig {
                        name: func_name,
                        typename: func_type,
                        arglist: arg_list,
                    }
                )
            }
            else {
                panic!("Expected ')', got trash: {:?} {:?}", it.peek(), tl);
            }
        }
        else {
            panic!("Expected '(', got trash somewhere");
        }
    }
    else {
        None
    };
}

fn parse_func(mut it: &mut Peekable<Iter<Token>>) -> Option<Node> {
    return if let Some (&&Token::FuncKeyword (ref tl)) = it.peek() {
        it.next();

        if let Some (target_func_sig) = parse_func_sig(&mut it) {

            if let Some (&&Token::LBrace (ref tl)) = it.peek() {
                it.next();

                let stmt_list = parse_statements(&mut it);

                if let Some (&&Token::RBrace (_)) = it.peek() {
                    it.next();

                    Some (
                        Node::FuncDef (
                            target_func_sig,
                            stmt_list,
                        )
                    )
                }
                else {
                    panic!("Expected '}}', got trash: {:?}", tl);
                }
            }
            else {
                panic!("Expected '{{', got trash: {:?}", tl);
            }
        }
        else {
            panic!("Expected 'FuncSig'");
        }
    }
    else {
        None
    };
}

fn parse_statement(mut it: &mut Peekable<Iter<Token>>) -> Option<Stmt> {
    return if let Some (node) = parse_add(&mut it) {
        Some (node)
    }
    else if let Some (node) = parse_sub(&mut it) {
        Some (node)
    }
    else if let Some (node) = parse_let(&mut it) {
        Some (node)
    }
    else {
        None
    };
}

fn parse_statements(mut it: &mut Peekable<Iter<Token>>) -> Vec<Stmt> {
    let mut stmts = Vec::new();

    while let Some (node) = parse_statement(&mut it) {
        stmts.push(node);
    }

    return stmts;
}

pub fn parse(tokens: &Vec<Token>) -> Option<Node> {
    let mut it = tokens.iter().peekable();

    return parse_func(&mut it);
}
