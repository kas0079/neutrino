use rand::{rngs::ThreadRng, seq::IndexedRandom};

use crate::{agents::agent::Agent, neutrino_board::{GameBoard, TurnMove}};



pub(crate) struct RandomAgent {
    seed: ThreadRng
}

impl RandomAgent {
    fn new(seed: ThreadRng) -> Self {
        Self {seed}
    }
}

impl Default for RandomAgent {
    fn default() -> Self {
        Self { seed: Default::default() }
    }
}

impl Agent for RandomAgent {
    fn get_move(&mut self, board: &GameBoard) -> TurnMove {
        let possible_move = board.actions();
        let random_move = possible_move.choose(&mut self.seed).unwrap();
        random_move.clone()
    }
}