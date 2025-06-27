use crate::{neutrino_board::{GameBoard, Player, TurnMove}, Agent};

pub(crate) struct MinimaxAgent {
    //TODO maybe add max-depth as a field
    player: Player
}

impl MinimaxAgent {
    pub(crate) fn new(player: Player) -> Self {
        Self {player}
    } 

    fn max_value(&self, board: &GameBoard) -> (f64, Option<TurnMove>) {
        if board.is_terminal() {
            return (board.utility(self.player).unwrap(), None)
        }
        let mut value = f64::NEG_INFINITY;
        let mut best_action: Option<TurnMove> = None;
        for action in board.actions() {
            let (subtree_value, _) = self.min_value(board);
            if subtree_value > value {
                value = subtree_value;
                best_action = Some(action);
            }
        }
        return (value, best_action)
    }

    fn min_value(&self, board: &GameBoard) -> (f64, Option<TurnMove>) {
        if board.is_terminal() {
            return (board.utility(self.player).unwrap(), None)
        }
        let mut value = f64::INFINITY;
        let mut best_action: Option<TurnMove> = None;
        for action in board.actions() {
            let (subtree_value, _) = self.max_value(board);
            if subtree_value < value {
                value = subtree_value;
                best_action = Some(action);
            }
        }
        return (value, best_action)
    }

}

impl Agent for MinimaxAgent {
    fn get_move(&mut self, board: &GameBoard) -> TurnMove {
        let (_, action) = self.max_value(board);
        action.expect("Expected to find possible moves.")
    }
}