use ir_parser::*;

use std::collections::HashMap;

fn typecheck_stmts<'a, 'b>(
    stmts: &'a Vec<Stmt>, ret_type: &'a Type,
    sym_tab: &'b mut HashMap<&'b str, &'b Type>
) -> bool
where 'a: 'b
{
    for stmt in stmts {
        match stmt {
            &Stmt::AddInst (ref dest_lval, ref left_rval, ref right_rval) => {
                if let Some (left_type)
                    = sym_tab.get::<str>(&&left_rval.name)
                {
                    if !is_promotable_to(&left_type, &dest_lval.typename) {
                        return false;
                    }
                }
                else {
                    return false;
                }
                if let Some (right_type)
                    = sym_tab.get::<str>(&&right_rval.name)
                {
                    if !is_promotable_to(&right_type, &dest_lval.typename) {
                        return false;
                    }
                }
                else {
                    return false;
                }

                if sym_tab.contains_key::<str>(&&dest_lval.name) {
                    return false;
                }

                sym_tab.insert(&dest_lval.name, &dest_lval.typename);
            }
            &Stmt::SubInst (ref dest_lval, ref left_rval, ref right_rval) => {
                if let Some (left_type)
                    = sym_tab.get::<str>(&&left_rval.name)
                {
                    if !is_promotable_to(&left_type, &dest_lval.typename) {
                        return false;
                    }
                }
                else {
                    return false;
                }
                if let Some (right_type)
                    = sym_tab.get::<str>(&&right_rval.name)
                {
                    if !is_promotable_to(&right_type, &dest_lval.typename) {
                        return false;
                    }
                }
                else {
                    return false;
                }

                if sym_tab.contains_key::<str>(&&dest_lval.name) {
                    return false;
                }

                sym_tab.insert(&dest_lval.name, &dest_lval.typename);
            }
            &Stmt::LetInst (ref dest_lval, ref src_rval) => {
                match src_rval {
                    &LetValue::LetInteger (_) => {
                        match dest_lval.typename {
                            Type::Ptr (_) => {
                                return false;
                            }
                            Type::UserType (_) => {
                                return false;
                            }
                            _ => {}
                        }
                    }
                    &LetValue::LetVariable (ref var) => {
                        if let Some (val_type)
                            = sym_tab.get::<str>(&&var.name)
                        {
                            if !is_promotable_to(
                                &val_type, &dest_lval.typename
                            ) {
                                return false;
                            }
                        }
                    }
                }

                if sym_tab.contains_key::<str>(&&dest_lval.name) {
                    return false;
                }

                sym_tab.insert(&dest_lval.name, &dest_lval.typename);
            }
            &Stmt::RetInst (ref rval) => {
                if let &Some (ref var) = rval {
                    if let Some (val_type)
                        = sym_tab.get::<str>(&&var.name)
                    {
                        if !is_promotable_to(&val_type, &ret_type) {
                            return false;
                        }
                    }
                    else {
                        if !is_promotable_to(&Type::Void, &ret_type) {
                            return false;
                        }
                    }
                }
            }
        }
    }

    return true;
}

fn typecheck_funcdef<'a, 'b>(
    sig: &'a FuncSig, stmts: &'a Vec<Stmt>,
    sym_tab: &'b mut HashMap<&'b str, &'b Type>
) -> bool
where 'a: 'b
{
    for arg in &sig.arglist {
        if sym_tab.contains_key::<str>(&&arg.name) {
            return false;
        }

        sym_tab.insert(&arg.name, &arg.typename);
    }

    return typecheck_stmts(stmts, &sig.typename, sym_tab);
}

pub fn typecheck(ast: &Node) -> bool {
    let mut sym_tab = HashMap::new();

    match ast {
        &Node::FuncDef (ref sig, ref stmts) => {
            if !typecheck_funcdef(sig, stmts, &mut sym_tab) {
                return false;
            }
        }
    }

    return true;
}
