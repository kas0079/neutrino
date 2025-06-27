use std::{cell::RefCell, rc::Weak};

struct Tree<NodeContent> {
    root: Node<NodeContent>
}

struct Node<NodeContent> {
    nodeContent: NodeContent,
    parent: Weak<RefCell<Node<NodeContent>>>,
    children: Vec<Node<NodeContent>>
}

impl <NodeContent> Node<NodeContent> {
    
}