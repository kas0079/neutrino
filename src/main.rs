mod neutrino_board;
mod agents;
use crate::agents::agent::Agent;
use crate::agents::human::Human;
use crate::agents::minimax::{self, MinimaxAgent};
use crate::neutrino_board::{GameBoard, Player};
use crate::agents::random_agent::RandomAgent;

fn main() {
    let mut board: GameBoard = GameBoard::default();
    println!("{board}");
    //let mut minimax_agent = MinimaxAgent::new(Player::Player1); //stack overflow
    let mut random_agent = RandomAgent::default();
    let mut human = Human::new();
    let mut num_moves = 0usize;

    while !board.is_terminal() {
        
        let agent_move = if num_moves % 2 == 0 {human.get_move(&board)} else {random_agent.get_move(&board)};
        
        board = board.result(agent_move);
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
