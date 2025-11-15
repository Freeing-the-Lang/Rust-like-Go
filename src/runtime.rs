use std::collections::HashMap;
use crate::parser::Node;
use crate::builtins;

pub fn run(nodes: Vec<Node>) {
    let mut env = HashMap::new();

    for node in nodes {
        match node {
            Node::Bind(name, expr) => {
                let v = eval(&expr, &env);
                env.insert(name, v);
            }

            Node::Print(expr) => {
                let v = eval(&expr, &env);
                builtins::print_value(v);
            }

            _ => {}
        }
    }
}

fn eval(node: &Node, env: &HashMap<String, i64>) -> i64 {
    match node {
        Node::Number(n) => *n,
        Node::Ident(id) => *env.get(id).unwrap_or(&0),
        Node::Add(a, b) => eval(a, env) + eval(b, env),
        _ => 0,
    }
}
