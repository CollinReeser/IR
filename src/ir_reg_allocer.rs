use ir_parser::*;

use petgraph::*;

use std::collections::HashMap;
use std::collections::HashSet;

// Left Vec<String> is remove list
// Right Vec<String> is add list
fn stmt_liveness<'a, 'b>(stmt: &'a Stmt)
    -> (Vec<&'b str>, Vec<&'b str>)
    where 'a: 'b
{
    return match stmt {
        &Stmt::AddInst (
            VarTypePair {ref name, typename: _},
            Variable {name: ref l_rval},
            Variable {name: ref r_rval},
        ) => {
            return (
                vec!(&name),
                vec!(&l_rval, &r_rval)
            )
        }
        &Stmt::SubInst (
            VarTypePair {ref name, typename: _},
            Variable {name: ref l_rval},
            Variable {name: ref r_rval},
        ) => {
            return (
                vec!(&name),
                vec!(&l_rval, &r_rval)
            )
        }
        &Stmt::LetInst (
            VarTypePair {ref name, typename: _},
            ref let_val,
        ) => {
            match let_val {
                &LetValue::LetVariable (Variable {name: ref rval}) => {
                    return (
                        vec!(&name),
                        vec!(&rval)
                    )
                }
                &LetValue::LetInteger (_) => {
                    return (
                        vec!(&name),
                        vec!()
                    )
                }
            }
        }
        &Stmt::RetInst (ref ret_opt) => {
            match ret_opt {
                &Some (Variable {ref name}) => {
                    return (
                        vec!(),
                        vec!(&name)
                    )
                }
                &None => {
                    return (
                        vec!(),
                        vec!()
                    )
                }
            }
        }
        &Stmt::CallInst (
            VarTypePair {ref name, typename: _},
            Function {name: _},
            ref vars_rval,
        ) => {
            let mut rvars: Vec<&str> = Vec::new();
            for var_rval in vars_rval {
                rvars.push(&var_rval.name);
            }
            return (
                vec!(&name),
                rvars
            )
        }
    }
}

fn get_funcdef_liveness_ranges<'a, 'b>(stmts: &'a Vec<Stmt>)
    -> Vec<HashSet<&'b str>>
    where 'a: 'b
{
    let mut livesets = Vec::new();
    let mut prev_liveset: HashSet<&str> = HashSet::new();

    for stmt in stmts.iter().rev() {
        let (remove_list, add_list) = stmt_liveness(&stmt);

        let mut liveset: HashSet<&str> = HashSet::new();

        for prev in prev_liveset.drain() {
            liveset.insert(&prev);
        }
        for add in add_list {
            liveset.insert(&add);
        }
        for remove in remove_list {
            liveset.remove(remove);
        }

        livesets.push(liveset.clone());
        prev_liveset = liveset;
    }

    return livesets;
}

fn add_liveset_to_rig<'a>(
    liveset: HashSet<&'a str>, rig: &mut GraphMap<&'a str, i64>
) {
    for val in liveset.iter() {

        let mut single_elem: HashSet<&str> = HashSet::new();
        single_elem.insert(val);

        let difference: HashSet<&str> = liveset.difference(&single_elem)
                                                  .cloned()
                                                  .collect();

        if !rig.contains_node(val) {
            rig.add_node(val);
        }

        for other in difference.iter() {
            if !rig.contains_node(other) {
                rig.add_node(other);
            }

            rig.add_edge(val, other, 1);
        }
    }
}

pub fn dump_dot_format(rig: &GraphMap<&str, i64>) -> String {

    let mut s = String::new();
    s.push_str("graph {\n");

    let mut node_indices = HashMap::new();

    for (i, node) in rig.nodes().enumerate() {
        s.push_str(&format!("    {} [label=\"{}\"]\n", i, node));

        node_indices.insert(node, i);
    }

    let mut already_linked = HashSet::new();

    for node in rig.nodes() {
        for (connected, _) in rig.edges(node) {
            match (node_indices.get(node), node_indices.get(connected)) {
                (Some (i), Some (j)) => {
                    let fmt = format!("    {} -- {}\n", i, j);
                    let key = if i <= j {
                        format!("{} {}\n", i, j)
                    }
                    else {
                        format!("{} {}\n", j, i)
                    };


                    if !already_linked.contains(&key) {
                        s.push_str(&fmt);
                        already_linked.insert(key);
                    }
                }
                _ => panic!("Unreachable"),
            }
        }
    }

    s.push_str("}");

    return s;
}

pub fn generate_rig(ast: &Node) -> GraphMap<&str, i64> {
    let rig = match ast {
        &Node::FuncDef (_, ref stmts) => {
            let mut rig: GraphMap<&str, i64> = GraphMap::new();

            let mut liveness_ranges = get_funcdef_liveness_ranges(&stmts);

            println!("{:?}", liveness_ranges);

            for liveset in liveness_ranges.drain(..) {
                add_liveset_to_rig(liveset, &mut rig);
            }

            rig
        }
    };

    return rig;
}
