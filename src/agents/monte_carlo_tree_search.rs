use std::{collections::HashMap, f64::consts::SQRT_2, time::{Duration, Instant}};
use rand::{rng, seq::IteratorRandom};

use crate::{agents::{agent::Agent, random_agent::{self, RandomAgent}}, neutrino_board::{GameBoard, Player, TurnMove}};

type NodeIndex = usize;

#[derive(Clone)]
struct Node {
    utility: usize,
    number_of_playouts: usize,
    board: GameBoard,
    children: HashMap<TurnMove, NodeIndex>,
    parent: Option<NodeIndex>
}

impl Node {
    fn new(board: GameBoard, parent: Option<NodeIndex>) -> Self {    
        Self { utility: 0, number_of_playouts: 0, board, children: HashMap::new(), parent }
    }
    
    fn children_fully_populated(&self) -> bool {
        self.board.actions().len() == self.children.len()
    }
}
pub(crate) struct MonteCarloTreeSearch  {
    nodes: Vec<Node>,
    root: usize,
    time_out: Duration
}

impl MonteCarloTreeSearch {
    pub(crate) fn new(board: &GameBoard, time_out: Duration) -> Self {
        let root = Node::new(board.clone(), None);
        Self { nodes: vec![root], root: 0, time_out }
    }
    
    /**
     * Select a node to expand.
     */
    fn select(&self) -> NodeIndex {
        let mut node = self.root;
        let mut random_agent = RandomAgent::default();
        if self.nodes[node].number_of_playouts == 0 {
            return node
        }
        
        while self.nodes[node].children_fully_populated() {
            //buggy
            let actions = self.nodes[node].board.actions();
            //let action: TurnMove = random_agent.get_move(&self.nodes[node].board);
            let choosen_action = actions.into_iter().max_by(|x,y | {
                let x_node = self.nodes[node].children.get(x).unwrap();
                let x_value = self.ucb1(*x_node);
                let y_node = self.nodes[node].children.get(y).unwrap();
                let y_value = self.ucb1(*y_node);
                println!("comparing {} with {}", x_value, y_value);
                x_value.total_cmp(&y_value)
            }).expect("There should be a move here!");

            let child = self.nodes[node].children.get(&choosen_action).unwrap();
            node = *child
        }
        node
    }

    /**
     * Expands the node
     */
    fn expand(&mut self, node: NodeIndex) -> NodeIndex {
        let parent: NodeIndex = node;
        //To be selected the node must have an action that is not yet a child.
        let children = &self.nodes[node].children;
        let mut rng = rng();
        let selected_action = self.nodes[node].board.actions()
            .into_iter()
            .filter(|action| !children.contains_key(action))
            .choose(&mut rng)
            .expect("Node must have an action that is not yet a child, yet choose failed!!");
        let new_board = self.nodes[node].board.result(selected_action.clone());
        let child = Node::new(new_board, Some(parent));
        let node_index = self.nodes.len();
        self.nodes.push(child);
        self.nodes[parent].children.insert(selected_action, node_index);
        node_index    
    }

    /// Simulates moves until a winner is found, with random_agent as playout policy, returns the utility from the POV of the player at the root node.
    fn simulate(&self, node_index: NodeIndex) -> f64 {
        let root_player = self.nodes[self.root].board.to_move();
        let mut game_board = self.nodes[node_index].board.clone();
        let mut random_agent = RandomAgent::default();
        while !game_board.is_terminal() {
            let random_move = random_agent.get_move(&game_board);
            game_board = game_board.result(random_move);
        }
        game_board.utility(root_player).expect("Game board should be in a terminal state.")
    }

    
    fn back_propagate(&mut self, from_node: NodeIndex, result: f64) {
        let root_player  =self.nodes[self.root].board.to_move();
        //scary floating point eq compare!!
        let winner_is_root = result == 1.;
        let mut path_to_root: Vec<NodeIndex> = vec![from_node];
        let mut node_index = from_node;
        loop {
            path_to_root.push(node_index);
            let node = &self.nodes[node_index];
            match node.parent {
                Some(parent) => node_index = parent,
                None => break,
            }
        }
        for (_, node) in self.nodes.iter_mut()
            .enumerate()
            .filter(|(index, node)| path_to_root.contains(index)) {
                node.number_of_playouts += 1;
                if winner_is_root && node.board.to_move() == root_player {
                    node.utility += 1
                } else if !winner_is_root && node.board.to_move() != root_player {
                    node.utility += 1
                }
        }
    }
    
    fn ucb1(&self, node_index: NodeIndex) -> f64 {
        let node = &self.nodes[node_index];
        let exploration_constant = SQRT_2;
        let parent = &self.nodes[node.parent.expect("ucb doesn't work on the root...")];

        (node.utility / node.number_of_playouts) as f64 + exploration_constant * f64::sqrt(f64::ln(parent.number_of_playouts as f64) / node.number_of_playouts as f64)
    }
    

}

impl Agent for MonteCarloTreeSearch {
    fn get_move(&mut self, board: &GameBoard) -> TurnMove {
        let start_time = Instant::now();
        while start_time.elapsed() < self.time_out {
            let selected_node = self.select();
            let expanded_node = self.expand(selected_node);
            let result = self.simulate(expanded_node);
            self.back_propagate(expanded_node, result);
        };
        
        let action = self.nodes[self.root].children
            .iter()
            .max_by_key(|(_, index)| self.nodes[**index].number_of_playouts)
            .expect("We should have found at least one move")
            .0;
        action.clone()
    }
}