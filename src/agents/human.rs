use std::{collections::HashMap, io::{stdin, stdout, Write}};

use crate::{agents::agent::Agent, neutrino_board::{GameBoard, MoveType, TurnMove}};


pub(crate) struct Human {
}

impl Human {
    pub(crate) fn new() -> Self {
        Self {  }
    }
    
    fn get_move_map(board: &GameBoard) -> HashMap<MoveType, Vec<TurnMove>> {
        let mut map: HashMap<MoveType, Vec<TurnMove>> = HashMap::new();
        for action in board.actions() {
            if map.contains_key(&action.neutrino_move) {
                map.get_mut(&action.neutrino_move).unwrap().push(action);
            } else {
                map.insert(action.neutrino_move.clone(), vec![action]);
            }
        }
        map
    }
}

impl Default for Human {
    fn default() -> Self {
        Self {  }
    }
}

impl Agent for Human {

    fn get_move(&mut self, board: &GameBoard) -> TurnMove {
        let actions: HashMap<MoveType, Vec<TurnMove>> = Self::get_move_map(board);

        let mut action: Option<TurnMove> = None;
        let stdin = stdin();
        let mut user_selection = String::new();
        while action.is_none() {
            println!("Current board:\n{}", board);
            println!("Available Neutrino Moves:");
            
            //Assumes keys() is stable
            let neutrino_moves: Vec<&MoveType> = actions.keys().collect();

            for (idx, neutrino) in neutrino_moves.iter().enumerate() {
                println!("Neutrino Move {}: {}", idx, neutrino);
            }
            user_selection.clear();
            print!("Please select a Neutrino move ");
            let _ = stdout().flush();

            stdin.read_line(&mut user_selection);
            //TODO continue on error
            let neutrino_move: &MoveType = neutrino_moves[user_selection.trim().parse::<usize>().unwrap()];
            let piece_moves: &Vec<TurnMove> = actions.get(neutrino_move).unwrap();

            for (idx, turn_move) in piece_moves.iter().enumerate() {
                println!("Move {}: {}", idx, turn_move);
                println!("{}", board.result(turn_move.clone()))
            }
            user_selection.clear();
            print!("Please select the rest of the move ");
            let _ = stdout().flush();

            stdin.read_line(&mut user_selection);
            let piece_move_index: usize = user_selection.trim().parse().unwrap();
            let parsed_move: TurnMove = piece_moves.get(piece_move_index).unwrap().clone();
            action = Some(parsed_move)
        }
        action.unwrap()
    }
}