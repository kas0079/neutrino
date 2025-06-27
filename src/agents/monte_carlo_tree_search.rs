
use std::{cell::{RefCell, RefMut}, collections::HashMap, rc::Weak};

use graph::prelude::{DirectedALGraph, DirectedCsrGraph, DirectedNeighborsWithValues, NodeValues};
use rand::{rng, seq::IteratorRandom};

use crate::{agents::{agent::Agent, random_agent::RandomAgent}, neutrino_board::{GameBoard, TurnMove}};

#[derive(Clone)]
struct NodeType {
    utility: f64,
    number_of_playouts: usize,
    board: GameBoard,
    children: HashMap<TurnMove, RefCell<NodeType>>,
    parent: RefCell<Weak<NodeType>>
}

impl NodeType {
    fn new(board: GameBoard, parent: Option<Weak<NodeType>>) -> Self {    
        Self { utility: 0., number_of_playouts: 0, board, children: HashMap::new(), parent: if parent.is_some() {RefCell::new(parent.unwrap())} else {RefCell::new(Weak::new())} }
    }
    
    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }
}
pub(crate) struct MonteCarloTreeSearch  {  
    root: RefCell<NodeType>
}

impl MonteCarloTreeSearch {
    pub(crate) fn new(board: &GameBoard) -> Self {
        let root = RefCell::new(NodeType::new(board.clone(), None));
        Self { root }
    }
    
    fn select(&mut self) -> RefCell<NodeType> {
        let mut node = self.root.clone();
        let mut random_agent = RandomAgent::default();
        while !node.borrow().is_leaf() {
            let action = random_agent.get_move(&node.borrow().board);
            node = node.borrow().children.get(&action).unwrap().clone();
            
            //node = node.children.get_mut(&action).unwrap();
        }
        node
    }

    fn expand(&mut self, node: &mut NodeType) {
        for action in &node.board.actions() {
            let new_board = node.board.result(action.clone());
            let child = NodeType::new(new_board, Some(node));
            node.children.insert(action.clone(), child);
        }
        todo!()
    }

    fn simulate() -> Self {
        todo!()
    }

    fn back_propagate() -> Self {
        todo!()
    }

    fn ucb1(node: NodeType) {
        todo!()
    }

}

impl Agent for MonteCarloTreeSearch {
    fn get_move(&mut self, board: &GameBoard) -> TurnMove {
        todo!()
    }
}