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

enum Node {
    Value(NodeValue),
    Op(NodeOp),
}

struct NodeValue {
    val: u32,
}

impl NodeValue {
    fn new(val: u32) -> NodeValue {
        return NodeValue { val };
    }
}

struct NodeOp {
    op: LexSym,
    left: Box<Node>,
    right: Box<Node>,
}

impl NodeOp {
    fn new(op: LexSym, left: Node, right: Node) -> NodeOp {
        return NodeOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        };
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

fn print_node_op(node: &NodeOp, prof: u32) {
    print_node(&node.left, prof + 1);
    print_node_prof(prof);
    println!("{:?}", node.op);
    print_node(&node.right, prof + 1);
}

fn print_node(node: &Node, prof: u32) {
    match node {
        Node::Value(val) => print_node_value(val, prof),
        Node::Op(op) => print_node_op(op, prof),
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
        _ => unimplemented!(),
    };
}

fn parse_high_prior_op(lexed: &[LexSym]) -> Option<(Node, &[LexSym])> {
    let (left, mut lexed) = parse_node(lexed);

    if lexed.len() == 0 {
        return Some((left, lexed));
    }

    return match lexed[0] {
        LexSym::TsTimes | LexSym::TsDivide | LexSym::TsModulo => {
            let op = lexed[0];

            lexed = &lexed[1..];

            let (right, lexed) = parse_high_prior_op(lexed).unwrap();

            let root = Node::Op(NodeOp::new(op, left, right));

            Some((root, lexed))
        }
        _ => Some((left, lexed)),
    };
}

fn parse_expr(lexed: &[LexSym]) -> Option<(Node, &[LexSym])> {
    if lexed.len() == 0 {
        return None;
    }

    let (left, mut lexed) = parse_high_prior_op(lexed)?;

    if lexed.len() == 0 {
        return Some((left, lexed));
    }

    let op = lexed[0];

    lexed = &lexed[1..];

    let (right, lexed) = parse_expr(lexed).unwrap();

    let root = Node::Op(NodeOp::new(op, left, right));

    return Some((root, lexed));
}

fn parse(lexed: &Vec<LexSym>) -> Option<Node> {
    let (parsed, _) = parse_expr(&lexed[..])?;

    return Some(parsed);
}

fn compute(parsed: &Node) -> i32 {
    return match parsed {
        Node::Op(NodeOp {
            op: LexSym::TsPlus,
            left,
            right,
        }) => compute(left) + compute(right),
        Node::Op(NodeOp {
            op: LexSym::TsLess,
            left,
            right,
        }) => compute(left) - compute(right),
        Node::Op(NodeOp {
            op: LexSym::TsTimes,
            left,
            right,
        }) => compute(left) * compute(right),
        Node::Op(NodeOp {
            op: LexSym::TsDivide,
            left,
            right,
        }) => compute(left) / compute(right),
        Node::Op(NodeOp {
            op: LexSym::TsModulo,
            left,
            right,
        }) => compute(left) % compute(right),
        Node::Value(NodeValue { val }) => *val as i32,
        _ => unimplemented!(),
    };
}

fn main() {
    let lexed = lexer(get_arg(), get_rt(), &get_symbol);
    let parsed = parse(&lexed.unwrap()).unwrap();

    // println!("{:?}", parsed.unwrap());
    println!("{:?}", compute(&parsed));
}
