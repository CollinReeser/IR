use ir_parser::*;

use petgraph::*;

use std::collections::HashMap;
use std::collections::HashSet;
use rand::*;

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn new() -> Self {
        return Color {
            red: random::<u8>() % 0xFF,
            green: random::<u8>() % 0xFF,
            blue: random::<u8>() % 0xFF
        };
    }
}

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

fn active_edges(rig: &GraphMap<&str, i64>, active_node: &str) -> i64 {
    let mut active_count = 0;

    for node in rig.nodes() {
        if node != active_node {
            continue;
        }

        for (_, &i) in rig.edges(node) {
            if i > 0 {
                active_count += 1;
            }
        }
    }

    return active_count;
}

fn disconnect_k_connected_nodes<'a>(rig: &mut GraphMap<&'a str, i64>, k: i64)
    -> Vec<&'a str>
{
    let mut disconnected_nodes = Vec::new();

    let nodes = rig.nodes().collect::<Vec<&str>>();

    let mut iter = nodes.iter();
    for node in nodes.iter() {
        let active_count = active_edges(&rig, node);

        if k != active_count {
            continue;
        }

        let mut disconnected = false;

        for neighbor in iter.clone() {
            if node != neighbor {
                if let Some (i) = rig.edge_weight_mut(node, neighbor) {
                    if *i > 0 {
                        disconnected = true;
                        *i = 0;
                    }
                }
            }
        }

        if disconnected {
            disconnected_nodes.push(*node);
        }

        iter.next();
    }

    return disconnected_nodes;
}

fn reconnect_edges(rig: &mut GraphMap<&str, i64>) {
    let nodes = rig.nodes().collect::<Vec<&str>>();

    let mut iter = nodes.iter();
    for node in nodes.iter() {
        for neighbor in iter.clone() {

            if node != neighbor {
                if let Some (i) = rig.edge_weight_mut(node, neighbor) {
                    *i = 1;
                }
            }
        }

        iter.next();
    }
}

/// Given a graph and a target K, first reconnect any disconnected edges, then
/// attempt to iteratively disconnect nodes from the graph to generate a
/// coloring stack, then (regardless of success or failure) reconnect any
/// disconnected edges, then return either Some(coloring stack) or None
fn generate_coloring_stack<'a>(mut rig: &mut GraphMap<&'a str, i64>, k: i64)
    -> Option<Vec<&'a str>>
{
    reconnect_edges(rig);

    let mut k_minus = k - 1;

    let mut stack = Vec::new();

    let mut ignore = HashSet::new();

    while k_minus > 0 {
        let mut sub_stack = disconnect_k_connected_nodes(&mut rig, k_minus);

        if sub_stack.len() > 0 {
            for node in sub_stack.drain(..) {
                if ignore.get(node).is_none() {
                    stack.push(node);
                    ignore.insert(node);
                }
            }
        }
        else {
            k_minus -= 1;
        }
    }

    let mut failure = false;

    for node in rig.nodes() {
        let active_count = active_edges(&rig, node);

        if active_count == 0 {
            if ignore.get(node).is_none() {
                stack.push(node);
                ignore.insert(node);
            }
        }
        else {
            failure = true;
        }
    }

    reconnect_edges(rig);

    return if !failure {
        Some(stack)
    }
    else {
        None
    };

}

fn color_mappings<'a>(
    rig: &mut GraphMap<&'a str, i64>, stack: &Vec<&'a str>, k: i64
)
    -> HashMap<&'a str, Color>
{
    let mut colors = HashSet::new();

    for _ in 0..k {
        colors.insert(Color::new());
    }

    let mut color_map: HashMap<&'a str, Color> = HashMap::new();

    for val in stack {
        for node in rig.nodes() {
            if *val == node {
                let mut colors_taken = HashSet::new();

                for neighbor in rig.neighbors(node) {
                    if let Some (color) = color_map.get(neighbor) {
                        colors_taken.insert(color.clone());
                    }
                }

                let mut assigned = false;

                for color in colors.iter() {
                    if !colors_taken.contains(color) {
                        color_map.insert(val, color.clone());
                        assigned = true;
                    }
                }

                if !assigned {
                    panic!("Could not assign color to node {}", val);
                }
            }
        }
    }

    return color_map;
}

pub fn dump_colored_graph(rig: &mut GraphMap<&str, i64>) -> String {
    if let Some((stack, used_k)) = find_minimum_k(rig, 16) {
        let color_map = color_mappings(rig, &stack, used_k);

        let mut s = String::new();
        s.push_str("graph {\n");

        let mut node_indices = HashMap::new();

        for (i, node) in rig.nodes().enumerate() {
            if let Some (color) = color_map.get(node) {
                s.push_str(
                    &format!(
                        "    {} [label=\"{}\", style=filled, fillcolor=\"#{:x}{:x}{:x}\"]\n",
                        i, node, color.red, color.green, color.blue
                    )
                );
            }
            else {
                s.push_str(&format!("    {} [label=\"{}\"]\n", i, node));
            }

            node_indices.insert(node, i);
        }

        let mut already_linked = HashSet::new();

        for node in rig.nodes() {
            for (connected, &edge_val) in rig.edges(node) {
                if edge_val == 0 {
                    continue;
                }

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
    else {
        return "".to_owned();
    }
}

/// Iteratively increase K until a sufficient K is found such that the graph can
/// be K-colored. Yield both the coloring stack and the determined-sufficient K
/// on success. bound is one past the maximum K to test.
pub fn find_minimum_k<'a>(mut rig: &mut GraphMap<&'a str, i64>, bound: i64)
    -> Option<(Vec<&'a str>, i64)>
{
    let mut k = 2;
    loop {
        if let Some (stack) = generate_coloring_stack(&mut rig, k) {
            return Some((stack, k));
        }
        else {
            k += 1;
        }

        if k == bound {
            return None;
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
        for (connected, &edge_val) in rig.edges(node) {
            if edge_val == 0 {
                continue;
            }

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

            for liveset in liveness_ranges.drain(..) {
                add_liveset_to_rig(liveset, &mut rig);
            }

            rig
        }
    };

    return rig;
}
