use crate::symbol::LexSym;

pub enum Node {
    Value(NodeValue),
    Op(NodeOp),
}

pub struct NodeValue {
    pub val: u32,
}

impl NodeValue {
    pub fn new(val: u32) -> NodeValue {
        return NodeValue { val };
    }
}

pub struct NodeOp {
    pub op: LexSym,
    pub left: Box<Node>,
    pub right: Box<Node>,
}

impl NodeOp {
    pub fn new(op: LexSym, left: Node, right: Node) -> NodeOp {
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
