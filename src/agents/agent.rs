use crate::neutrino_board::{GameBoard, TurnMove};


pub(crate) trait Agent {
    /**
     * Returns the move that the agent chooses for a given board.
     */
    fn get_move(&mut self, board: &GameBoard) -> TurnMove;
}