use ll_lexer::lexer;

mod rule_table;
mod symbol;

use rule_table::get_rt;
use symbol::{get_symbol, LexSym};

fn get_arg() -> String {
    let mut args: Vec<String> = std::env::args().collect();

    if args.len() == 1 {
        eprintln!("usage:\texpr");
        std::process::exit(1);
    }

    return args.remove(1);
}

// #[derive(Debug)]
enum Node {
    Value(NodeValue),
    MinorOp(NodeMinorOp),
}

struct NodeValue {
    val: u32,
}

impl NodeValue {
    fn new(val: u32) -> NodeValue {
        return NodeValue { val };
    }
}

struct NodeMinorOp {
    op: LexSym,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl NodeMinorOp {
    fn new(op: LexSym, left: Option<Node>, right: Option<Node>) -> NodeMinorOp {
        let left = if let Some(node) = left {
            Some(Box::new(node))
        } else {
            None
        };

        let right = if let Some(node) = right {
            Some(Box::new(node))
        } else {
            None
        };

        return NodeMinorOp { op, left, right };
    }
}

fn print_node_prof(prof: u32) {
    for _ in 0..prof {
        print!("\t");
    }
}

fn print_node_value(node: &NodeValue, prof: u32) {
    print_node_prof(prof);
    println!("{}", node.val);
}

fn print_node_op(node: &NodeMinorOp, prof: u32) {
    if let Some(left) = &node.left {
        print_node(&left, prof + 1);
    } else {
        print_node_prof(prof + 1);
        println!("NULL");
    }

    print_node_prof(prof);
    println!("{:?}", node.op);

    if let Some(right) = &node.right {
        print_node(&right, prof + 1);
    } else {
        print_node_prof(prof + 1);
        println!("NULL");
    }
}

fn print_node(node: &Node, prof: u32) {
    match node {
        Node::Value(val) => print_node_value(val, prof),
        Node::MinorOp(op) => print_node_op(op, prof),
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        print_node(self, 0);
        Ok(())
    }
}

fn parse_node_bracket(lexed: &[LexSym]) -> (Node, &[LexSym]) {
    let mut opened_bracket = 0;
    let mut pos_r_bracket = 0;

    for i in lexed.iter() {
        if *i == LexSym::TsRBracket {
            if opened_bracket == 1 {
                break;
            } else {
                opened_bracket -= 1;
            }
        } else if *i == LexSym::TsLBracket {
            opened_bracket += 1;
        }

        pos_r_bracket += 1;
    }

    let (content, _) = parse_expr(&lexed[1..pos_r_bracket]).unwrap();

    return (content, &lexed[pos_r_bracket + 1..]);
}

fn parse_node(lexed: &[LexSym]) -> (Node, &[LexSym]) {
    return match lexed[0] {
        LexSym::TsNbr(n) => (Node::Value(NodeValue::new(n)), &lexed[1..]),
        LexSym::TsLBracket => parse_node_bracket(lexed),
        _ => (Node::Value(NodeValue::new(0)), &lexed[1..]),
    };
}

fn parse_expr(lexed: &[LexSym]) -> Option<(Node, &[LexSym])> {
    if lexed.len() == 0 {
        return None;
    }

    let (left, mut lexed) = parse_node(lexed);

    if lexed.len() == 0 {
        return Some((left, lexed));
    }

    // match lexed[0] {
    // LexSym::TsPlus | LexSym::TsLess => {
    let op = lexed[0];

    lexed = &lexed[1..];

    let (right, lexed) = parse_expr(lexed).unwrap();

    let root = Node::MinorOp(NodeMinorOp::new(op, Some(left), Some(right)));
    // }
    // Some(_) => return Err("qwer".to_string()),
    // None => return Ok(left),
    // }

    return Some((root, lexed));
}

fn parse(lexed: &Vec<LexSym>) -> Option<Node> {
    let (parsed, _) = parse_expr(&lexed[..])?;

    return Some(parsed);
}

fn main() {
    let lexed = lexer(get_arg(), get_rt(), &get_symbol);
    let parsed = parse(&lexed.unwrap());

    println!("{:?}", parsed.unwrap());
}
