use std::{collections::HashMap, f64::consts::SQRT_2, time::{Duration, Instant}, ops::{Index, IndexMut}};
use rand::{rng, seq::IteratorRandom};

use crate::{agents::{agent::Agent, random_agent::{self, RandomAgent}}, neutrino_board::{GameBoard, Player, TurnMove}};

type NodeIndex = usize;

#[derive(Clone)]
pub struct Node {
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
    
    fn is_leaf(&self) -> bool {
        self.board.actions().len() == self.children.len() /*|| self.board.is_terminal() */
    }
}

enum SelectionResult {
    NonTerminal(NodeIndex),
    Terminal(NodeIndex, f64)

}
pub(crate) struct MonteCarloTreeSearch  {
    nodes: Vec<Node>,
    root: NodeIndex,
    time_out: Duration
}

impl MonteCarloTreeSearch {
    pub(crate) fn new(board: &GameBoard, time_out: Duration) -> Self {
        let root = Node::new(board.clone(), None);
        Self { nodes: vec![root], root: 0, time_out }
    }
    
    fn root_player(&self) -> Player {
        self.nodes[self.root].board.to_move()
    }
    /**
     * Select a node to expand.
     */
    fn select(&self) -> SelectionResult {
        let mut node_index = self.root;
        if self[node_index].number_of_playouts == 0 {
            return SelectionResult::NonTerminal(node_index)
        }
        
        while self[node_index].is_leaf() {
            //if node is terminal, return it should not be expanded.
            let node = &self[node_index];
            if node.board.is_terminal() {
                let result = node.board.utility(self.root_player()).expect("We should have a winner here");
                return SelectionResult::Terminal(node_index, result);
            }
            //buggy
            
            let actions = node.board.actions();
            assert!(!actions.is_empty(), "Trying to find actions from terminal state node:\n {}", self[node_index].board);
            //let action: TurnMove = random_agent.get_move(&self.nodes[node].board);
            let choosen_action = actions.into_iter().max_by(|x,y | {
                let x_node = node.children.get(x).unwrap();
                let x_value = self.ucb1(*x_node);
                let y_node = node.children.get(y).unwrap();
                let y_value = self.ucb1(*y_node);
                //println!("comparing {} with {}", x_value, y_value);
                x_value.total_cmp(&y_value)
            }).expect("There should be a move here!");

            let child = node.children.get(&choosen_action).unwrap();
            node_index = *child
        }
        SelectionResult::NonTerminal(node_index)
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
        let root_player = self.root_player();
        let game_board = self.nodes[node_index].board.clone();
        //playout policy
        let terminal_state = game_board.actions().into_iter().find(|action| game_board.result(action.clone().clone()).is_terminal());
        match terminal_state {
            Some(action) => {
                let terminal_game_board = game_board.result(action);
                terminal_game_board.utility(root_player).unwrap()
            },
            None => Self::random_playout(game_board, root_player)
        }
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
            .filter(|(index, _node)| path_to_root.contains(index)) {
                node.number_of_playouts += 1;
                if winner_is_root && node.board.to_move() == root_player {
                    node.utility += 1
                } else if !winner_is_root && node.board.to_move() != root_player {
                    node.utility += 1
                }
        }
    }

    fn random_playout(mut game_board: GameBoard, root_player: Player) -> f64 {
        let mut random_agent = RandomAgent::default();
        while !game_board.is_terminal() {
            let random_move = random_agent.get_move(&game_board);
            game_board = game_board.result(random_move);
        }
        game_board.utility(root_player).expect("Game board should be in a terminal state.")
    }

    fn ucb1(&self, node_index: NodeIndex) -> f64 {
        let node = &self.nodes[node_index];
        let exploration_constant = SQRT_2;
        let parent = &self.nodes[node.parent.expect("ucb doesn't work on the root...")];

        (node.utility / node.number_of_playouts) as f64 + exploration_constant * f64::sqrt(f64::ln(parent.number_of_playouts as f64) / node.number_of_playouts as f64)
    }
    

}

impl IndexMut<usize> for MonteCarloTreeSearch {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl Index<usize> for MonteCarloTreeSearch {
    type Output = Node;

    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl Agent for MonteCarloTreeSearch {
    fn get_move(&mut self, board: &GameBoard) -> TurnMove {
        let start_time = Instant::now();
        while start_time.elapsed() < self.time_out {
            match self.select() {
                SelectionResult::NonTerminal(selected_node) => {
                    let expanded_node = self.expand(selected_node);
                    let result = self.simulate(expanded_node);
                    self.back_propagate(expanded_node, result);
                },
                SelectionResult::Terminal(expanded_node, result) => self.back_propagate(expanded_node, result),
            }
            
        };
        
        let action = self.nodes[self.root].children
            .iter()
            .max_by_key(|(_, index)| self.nodes[**index].number_of_playouts)
            .expect("We should have found at least one move")
            .0;
        action.clone()
    }
}