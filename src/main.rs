mod neutrino_board;
use rand::prelude::*;
use crate::neutrino_board::{GameBoard, Player};

fn main() {
    let mut rng = rand::rng();
    let mut board: GameBoard = GameBoard::default();
    println!("{board}");

    let mut num_moves = 0usize;
    while !board.is_terminal() {
        let possible_move = board.actions();
        let random_move = possible_move.choose(&mut rng).unwrap();
        board = board.result(random_move.clone());
        num_moves += 1;
        if num_moves % 1 == 0 {
            println!("move {num_moves}");
            println!("{board}\n");
        }
    }
    println!("Terminal state!!!");
    println!("The utility is {}", board.utility(Player::Player1).unwrap());
    println!("{board}");

    
}
