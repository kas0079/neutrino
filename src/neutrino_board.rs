use std::fmt::Display;



#[derive(Debug, Clone, Copy, PartialEq)]
enum Piece {
    Player1,
    Player2,
    Neutrino
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    Player1,
    Player2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TurnMove {
    neutrino_move: MoveType,
    piece_move: MoveType
}

impl TurnMove {
    fn new(neutrino_move: MoveType, piece_move: MoveType) -> Self {
        Self { neutrino_move, piece_move}
    }
}

impl Display for TurnMove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "({} -> {}) ({} -> {})", self.neutrino_move.from_position, self.neutrino_move.to_position, self.piece_move.from_position, self.piece_move.to_position)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct MoveType {
    from_position: Position,
    to_position: Position,
}

impl MoveType {
    fn new(from_position: Position, to_position: Position) -> Self {
        Self {from_position, to_position}
    }
}


/**
 * A position on the board, use board[row][column]
 *  */
 
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position {
    row: usize,
    column: usize
}
impl From<(usize, usize)> for Position {
    fn from(value: (usize, usize)) -> Self {
        Position {row: value.0, column: value.1}
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.column)
    }
}

impl Position {

    fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    fn up(&self) -> Self {
        Self { row: self.row + 1, column: self.column }
    }

    fn down(&self) -> Self {
        Self { row: self.row - 1, column: self.column }
    }

    fn left(&self) -> Self {
        Self { row: self.row, column: self.column - 1 }
    }

    fn right(&self) -> Self {
        Self { row: self.row, column: self.column + 1 }
    }
    fn in_bounds(&self) -> bool {
        if self.row > 4  || self.column > 4 {
            return false
        } else {
            return true
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct GameBoard {
    board: [[Option<Piece>; 5]; 5],
    to_move: Player,
    neutrino_position: Position
}

impl GameBoard {
    fn new(board: [[Option<Piece>; 5]; 5], to_move: Player) -> Self {
        //find neutrino
        let mut neutrino_position: Option<Position> = None;
        for (row_index, row) in board.iter().enumerate() {
            for (column_index, column) in row.iter().enumerate() {
                if column.is_some_and(|x| x == Piece::Neutrino) {
                    neutrino_position = Some((row_index, column_index).into())
                }
            }
        }
        match neutrino_position {
            Some(neutrino_position) => Self { board, to_move, neutrino_position},
            None => panic!("Boards must have a neutrino!"),
        }  
    }
    /**
     * Returns the positions of the player's pieces
     */
    fn pieces(&self, player: Player) -> Vec<Position> {
        let mut positions: Vec<Position> = vec![];
        let player_piece = match player {
            Player::Player1 => Piece::Player1,
            Player::Player2 => Piece::Player2,
        };
        for (row_index, row) in self.board.iter().enumerate() {
            for (column_index, piece) in row.iter().enumerate() {
                if piece.is_some_and(|x| x == player_piece) {
                    positions.push(Position::new(row_index, column_index))
                }
            }
        }
        positions
    }
    /**
     * What player's turn it is.
     */
    pub fn to_move(&self) -> Player {
        self.to_move
    }

    /**
     * A list of all possible moves.
     */
    pub fn actions(&self) -> Vec<TurnMove>{
        //list neutrino moves
        
        let move_in_direction = |board: &GameBoard, start_position: Position, direction: Box<dyn Fn(Position) -> Position>| -> Option<MoveType>{
            let test_position = direction(Position::new(1,1));
            let moves_down = test_position.row == 0;
            let moves_left = test_position.column == 0;
            if start_position.row == 0 && moves_down {
                return None
            }
            if start_position.column == 0 && moves_left {
                return None
            }
            let position_in_direction = direction(start_position);
            let mut end_position = None;
            if position_in_direction.in_bounds() && board.at_position(position_in_direction).is_none()  {
                end_position = Some(position_in_direction)
            }
            if let Some(mut end_position) = end_position {
                while !(end_position.row == 0 && moves_down) && 
                    !(end_position.column == 0 && moves_left) && 
                    direction(end_position).in_bounds() &&
                    board.at_position(direction(end_position)).is_none() {
                    end_position = direction(end_position)
                }
                return Some(MoveType::new(start_position.clone(), end_position))
            }
            return None
        };
        let neutrino_start_position: Position = self.neutrino_position.clone();
        let neutrino_moves: Vec<MoveType> = [
            move_in_direction(self, neutrino_start_position, Box::new(|x:Position| x.up())),
            move_in_direction(self, neutrino_start_position, Box::new(|x:Position| x.down())),
            move_in_direction(self, neutrino_start_position, Box::new(|x:Position| x.left())),
            move_in_direction(self, neutrino_start_position, Box::new(|x:Position| x.right())),
            move_in_direction(self, neutrino_start_position, Box::new(|x:Position| x.up().left())),
            move_in_direction(self, neutrino_start_position, Box::new(|x:Position| x.up().right())),
            move_in_direction(self, neutrino_start_position, Box::new(|x:Position| x.down().left())),
            move_in_direction(self, neutrino_start_position, Box::new(|x:Position| x.down().right()))
        ].into_iter()
        .flatten() //remove None
        .collect();

        //generate moves from each possible neutrino move
        let mut moves: Vec<TurnMove> = vec![];
        let player_pieces = self.pieces(self.to_move());
        let start_row = if self.to_move() == Player::Player1 {
            0
        } else {
            4
        };
        // can't move into the start row if there are already 4 pieces there
        let can_move_to_start_row = player_pieces
            .iter()
            .filter(|position| position.row == start_row)
            .count() < 4;

        for neutrino_move in neutrino_moves {
            let mut moved_neutrino_board = self.clone();
            moved_neutrino_board.board[neutrino_move.from_position.row][neutrino_move.from_position.column] = None;
            moved_neutrino_board.board[neutrino_move.to_position.row][neutrino_move.to_position.column] = Some(Piece::Neutrino);

            // generate list of all the player's pieces
            for player_piece in &player_pieces {
                let player_moves: Vec<MoveType> = [
                    move_in_direction(&moved_neutrino_board, *player_piece, Box::new(|x:Position| x.up())),
                    move_in_direction(&moved_neutrino_board, *player_piece, Box::new(|x:Position| x.down())),
                    move_in_direction(&moved_neutrino_board, *player_piece, Box::new(|x:Position| x.left())),
                    move_in_direction(&moved_neutrino_board, *player_piece, Box::new(|x:Position| x.right())),
                    move_in_direction(&moved_neutrino_board, *player_piece, Box::new(|x:Position| x.up().left())),
                    move_in_direction(&moved_neutrino_board, *player_piece, Box::new(|x:Position| x.up().right())),
                    move_in_direction(&moved_neutrino_board, *player_piece, Box::new(|x:Position| x.down().left())),
                    move_in_direction(&moved_neutrino_board, *player_piece, Box::new(|x:Position| x.down().right()))
                ].into_iter()
                    .flatten()
                    .filter(|piece_move| {
                        !(!can_move_to_start_row
                            && piece_move.from_position.row != start_row
                            && piece_move.to_position.row == start_row)
                    } )
                    .collect();
                player_moves.into_iter()
                    .for_each(|piece_move| {
                        let turn_move = TurnMove::new(neutrino_move.clone(), piece_move);
                        moves.push(turn_move);
                    });
            }
        }

        moves
    }

    /**
     * The GameBoard resulting from a move.
     */
    pub fn result(&self, turn_move: TurnMove) -> Self {
        //set new to_move
        let new_player = match self.to_move() {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };
        //set new board state
        let mut new_board = self.board.clone();

        //move neutrino
        assert!(new_board[turn_move.neutrino_move.from_position.row][turn_move.neutrino_move.from_position.column]
            .is_some_and(|x| x == Piece::Neutrino ));
        let new_neutrino_position: Position = (turn_move.neutrino_move.to_position.row, turn_move.neutrino_move.to_position.column).into();
        assert!(new_board[new_neutrino_position.row][new_neutrino_position.column].is_none());

        new_board[turn_move.neutrino_move.from_position.row][turn_move.neutrino_move.from_position.column] = None;
        new_board[new_neutrino_position.row][new_neutrino_position.column] = Some(Piece::Neutrino);

        //move playerPiece
        let piece = match self.to_move() {
            Player::Player1 => Piece::Player1,
            Player::Player2 => Piece::Player2,
        };
        assert!(new_board[turn_move.piece_move.from_position.row][turn_move.piece_move.from_position.column]
            .is_some_and(|x| x == piece ));
        assert!(new_board[turn_move.piece_move.to_position.row][turn_move.piece_move.to_position.column].is_none());

        new_board[turn_move.piece_move.from_position.row][turn_move.piece_move.from_position.column] = None;
        new_board[turn_move.piece_move.to_position.row][turn_move.piece_move.to_position.column] = Some(piece);

        Self {board: new_board, neutrino_position: new_neutrino_position, to_move: new_player}
    }

    /**
     * Whether the GameBoard is terminal (has a winner). 
     */
    pub fn is_terminal(&self) -> bool {
        if self.neutrino_position.row == 0 || self.neutrino_position.row == 4 {
            return true
        }
        if self.actions().len() == 0 {
            return true
        }
        false
    }
    
    /**
     * The utility (i.e. score) of the GameBoard from a given player's perspective.
     */
    pub fn utility(&self, player: Player) -> Option<f64> {
        if !self.is_terminal() {
            return None
        }

        let neutrino_loss_player1 = self.board[0]
            .iter()
            .any(|x| x.is_some_and(|piece| piece == Piece::Neutrino));
        let neutrino_loss_player2 = self.board[4]
            .iter()
            .any(|x| x.is_some_and(|piece| piece == Piece::Neutrino));
        let captured_neutrino = self.actions().len() == 0;

        let mut result = 0.5;

        if neutrino_loss_player1 {
            result = 0.
        } else if neutrino_loss_player2 {
            result = 1.
        } else if captured_neutrino && self.to_move() == Player::Player1 {
            result = 0.
        } else if captured_neutrino && self.to_move() == Player::Player2 {
            result = 1.0
        }
        //Adjust perspective if necessary
        if player == Player::Player2 {
            result = 1.0 - result
        }
        Some(result)
    }

    fn at_position(&self, position: Position) -> Option<Piece> {
        self.board[position.row][position.column]
    }
}

impl Default for GameBoard {
    fn default() -> Self {
        let mut board: [[Option<Piece>; 5]; 5] = Default::default();
        board[2][2] = Some(Piece::Neutrino);
        
        for column in 0..5 {
            board[0][column] = Some(Piece::Player1);
            board[4][column] = Some(Piece::Player2)
        }
        Self { board: board, neutrino_position: (2,2).into(), to_move: Player::Player1 }
    }
}

impl Display for GameBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut board_string= String::new();

        for row in &self.board {
            let mut row_string = String::new();
            for column in row {
                row_string.push(
                match column {
                    Some(Piece::Neutrino) => 'X',
                    Some(Piece::Player1) => '1',
                    Some(Piece::Player2) => '2',
                    None => '_',
                    }
                )
            }
            //TODO don't \n on the last row.
            row_string.push('\n');
            board_string.push_str(&row_string);
        }
        writeln!(f, "{}", board_string)
    }
}
