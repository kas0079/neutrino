use crate::{neutrino_board::{GameBoard, Player, TurnMove}, Agent};

pub(crate) struct MinimaxAgent {
    //TODO maybe add max-depth as a field
    player: Player,
    max_depth: usize
}

impl MinimaxAgent {
    pub(crate) fn new(player: Player, max_depth: usize) -> Self {
        Self {player, max_depth}
    } 

    fn max_value(&self, board: &GameBoard, depth: usize) -> (f64, Option<TurnMove>) {
        if board.is_terminal() {
            return (board.utility(self.player).unwrap(), None)
        }

        let mut value = f64::NEG_INFINITY;
        let mut best_action: Option<TurnMove> = None;
        for action in board.actions() {
            let (subtree_value, _) = self.min_value(board, depth + 1);
            if subtree_value > value {
                value = subtree_value;
                best_action = Some(action);
            }
        }
        return (value, best_action)
    }

    fn min_value(&self, board: &GameBoard, depth: usize) -> (f64, Option<TurnMove>) {
        if board.is_terminal() {
            return (board.utility(self.player).unwrap(), None)
        }
        let mut value = f64::INFINITY;
        let mut best_action: Option<TurnMove> = None;
        for action in board.actions() {
            let (subtree_value, _) = self.max_value(board, depth + 1);
            if subtree_value < value {
                value = subtree_value;
                best_action = Some(action);
            }
        }
        return (value, best_action)
    }
    /**
     * Return a move
     */
    fn get_move_optional(&mut self, board: &GameBoard) -> Option<TurnMove> {
            let (_, action) = self.max_value(board, 0);
            action
        }
}

impl Agent for MinimaxAgent {
    fn get_move(&mut self, board: &GameBoard) -> TurnMove {
        let (_, action) = self.max_value(board, 0);
        action.expect("Expected to find possible moves, yet max_depth was reached.")
    }
}