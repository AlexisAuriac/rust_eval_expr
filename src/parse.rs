use crate::node::{Node, NodeOp, NodeValue};
use crate::symbol::{get_opposite_bracket, LexSym};

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

    let (content, _) = parse_expr(&lexed[1..pos_r_bracket]);

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

fn parse_power(lexed: &[LexSym]) -> (Node, &[LexSym]) {
    let (left, mut lexed) = parse_node(lexed);

    if lexed.len() == 0 {
        return (left, lexed);
    }

    return match lexed[0] {
        LexSym::TsPower => {
            let op = lexed[0];

            lexed = &lexed[1..];

            let (right, lexed) = parse_power(lexed);

            let root = Node::Op(NodeOp::new(op, left, right));

            (root, lexed)
        }
        _ => (left, lexed),
    };
}

fn parse_high_prior_op(lexed: &[LexSym]) -> (Node, &[LexSym]) {
    let (left, mut lexed) = parse_power(lexed);

    if lexed.len() == 0 {
        return (left, lexed);
    }

    return match lexed[0] {
        LexSym::TsTimes | LexSym::TsDivide | LexSym::TsModulo => {
            let op = lexed[0];

            lexed = &lexed[1..];

            let (right, lexed) = parse_high_prior_op(lexed);

            let root = Node::Op(NodeOp::new(op, left, right));

            (root, lexed)
        }
        _ => (left, lexed),
    };
}

fn parse_expr(lexed: &[LexSym]) -> (Node, &[LexSym]) {
    let (left, mut lexed) = parse_high_prior_op(lexed);

    if lexed.len() == 0 {
        return (left, lexed);
    }

    let op = lexed[0];

    lexed = &lexed[1..];

    let (right, lexed) = parse_expr(lexed);

    let root = Node::Op(NodeOp::new(op, left, right));

    return (root, lexed);
}

pub fn parse(lexed: &Vec<LexSym>) -> Node {
    let (parsed, _) = parse_expr(&lexed[..]);

    return parsed;
}
