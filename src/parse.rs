use crate::node::{Node, NodeOp, NodeValue};
use crate::symbol::{get_opposite_bracket, LexSym};

static OP_PRIORITIES: &'static [&'static [LexSym]] = &[
    &[LexSym::TsPlus],
    &[LexSym::TsLess],
    &[LexSym::TsTimes, LexSym::TsDivide, LexSym::TsModulo],
    &[LexSym::TsPower],
];

fn parse_node_bracket(lexed: &[LexSym]) -> (Node, &[LexSym]) {
    let mut opened_bracket = 0;
    let mut pos_r_bracket = 0;
    let l_brack = lexed[0];
    let r_brack = get_opposite_bracket(lexed[0]).unwrap();

    for i in lexed.iter() {
        if *i == r_brack {
            if opened_bracket == 1 {
                break;
            } else {
                opened_bracket -= 1;
            }
        } else if *i == l_brack {
            opened_bracket += 1;
        }

        pos_r_bracket += 1;
    }

    let (content, _) = parse_expr(OP_PRIORITIES, &lexed[1..pos_r_bracket]);

    return (content, &lexed[pos_r_bracket + 1..]);
}

fn parse_node(lexed: &[LexSym]) -> (Node, &[LexSym]) {
    return match lexed[0] {
        LexSym::TsNbr(n) => (Node::Value(NodeValue::new(n)), &lexed[1..]),
        LexSym::TsLBracket1 | LexSym::TsLBracket2 | LexSym::TsLBracket3 => {
            parse_node_bracket(lexed)
        }
        _ => unimplemented!(),
    };
}

fn parse_op<'a>(
    lexed: &'a [LexSym],
    left: Node,
    ops: &'static [&'static [LexSym]],
) -> (Node, &'a [LexSym]) {
    if lexed.len() == 0 {
        return (left, lexed);
    }

    return if ops[0].iter().any(|op| *op == lexed[0]) {
        let op = lexed[0];

        let (right, lexed) = parse_expr(ops, &lexed[1..]);

        let node = Node::Op(NodeOp::new(op, left, right));

        (node, lexed)
    } else {
        (left, lexed)
    };
}

fn parse_expr<'a>(ops: &'static [&'static [LexSym]], lexed: &'a [LexSym]) -> (Node, &'a [LexSym]) {
    let (left, lexed) = if ops.len() == 0 {
        return parse_node(lexed);
    } else {
        parse_expr(&ops[1..], lexed)
    };

    let (node, lexed) = parse_op(lexed, left, ops);

    return (node, lexed);
}

fn clear_sub(node: Node, prev_is_less: bool) -> Node {
    match node {
        Node::Value(n) => Node::Value(n),
        Node::Op(NodeOp {
            op: LexSym::TsLess,
            left,
            right,
        }) => {
            return if prev_is_less {
                let left = clear_sub(*left, false);
                let right = clear_sub(*right, false);

                Node::Op(NodeOp::new(LexSym::TsPlus, left, right))
            } else {
                let left = clear_sub(*left, true);
                let right = clear_sub(*right, true);

                Node::Op(NodeOp::new(LexSym::TsLess, left, right))
            }
        }
        Node::Op(NodeOp { op, left, right }) => {
            let left = clear_sub(*left, false);
            let right = clear_sub(*right, false);

            Node::Op(NodeOp::new(op, left, right))
        }
    }
}

pub fn parse(lexed: &Vec<LexSym>) -> Node {
    let (parsed, _) = parse_expr(OP_PRIORITIES, &lexed[..]);

    return clear_sub(parsed, false);
}
