use crate::node::{Node, NodeOp, NodeValue};
use crate::symbol::LexSym;

pub fn compute(parsed: &Node) -> i32 {
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
