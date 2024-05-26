use crate::SourceLocation;

pub struct Ast {
    root: Node,
}

pub struct Node {
    children: Vec<Node>,
    location: SourceLocation,
}

pub enum NodeType {}
