
use std::{cell::{RefCell, RefMut}, collections::HashMap, rc::Weak};

use rand::{rng, seq::IteratorRandom};

use crate::{agents::{agent::Agent, random_agent::RandomAgent}, neutrino_board::{GameBoard, TurnMove}};

type NodeIndex = usize;
#[derive(Clone)]
struct NodeType {
    utility: f64,
    number_of_playouts: usize,
    board: GameBoard,
    children: HashMap<TurnMove, NodeIndex>,
    parent: Option<NodeIndex>
}

impl NodeType {
    fn new(board: GameBoard, parent: Option<NodeIndex>) -> Self {    
        Self { utility: 0., number_of_playouts: 0, board, children: HashMap::new(), parent }
    }
    
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}
pub(crate) struct MonteCarloTreeSearch  {
    nodes: Vec<NodeType>,
    root: usize
}

impl MonteCarloTreeSearch {
    pub(crate) fn new(board: &GameBoard) -> Self {
        let root = NodeType::new(board.clone(), None);
        Self { nodes: vec![root], root: 0 }
    }
    
    fn select(&self) -> NodeIndex {
        let mut node = self.root;
        let mut random_agent = RandomAgent::default();
        while !self.nodes[node].is_leaf() {
            let action: TurnMove = random_agent.get_move(&self.nodes[node].board);
            let child = self.nodes[node].children.get(&action).unwrap();
            node = *child
        }
        node
    }
    
    fn expand(&mut self, node: NodeIndex) {
        let parent: NodeIndex = node;
        for action in self.nodes[node].board.actions() {
            let new_board = self.nodes[node].board.result(action.clone());
            let child = NodeType::new(new_board, Some(parent));
            let nodeIndex = self.nodes.len();
            self.nodes.push(child);
            self.nodes[parent].children.insert(action, nodeIndex);
        }
    }
    /*
    fn simulate() -> Self {
        todo!()
    }

    fn back_propagate() -> Self {
        todo!()
    }

    fn ucb1(node: NodeType) {
        todo!()
    }
    */

}

impl Agent for MonteCarloTreeSearch {
    fn get_move(&mut self, board: &GameBoard) -> TurnMove {
        todo!()
    }
}