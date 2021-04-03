use crate::board::*;
use crate::board::{Board, Coordinate};
use crate::board_console_printer::print_board;
use crate::chess_notation;
use crate::chess_notation::pgn::make_move_log;
use crate::fen_reader;
use crate::move_generator::Move;
use crate::AI;
use std::io;
use std::io::prelude::*;

pub struct Game {
    board: Board,
    moves: Vec<String>,
    ai: AI::AI,
    ai2: AI::AI,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: fen_reader::make_board(fen_reader::INITIAL_BOARD),
            ai: AI::AI::new(Color::Black),
            ai2: AI::AI::new(Color::White),
            moves: vec![],
        }
    }

    pub fn run_ai_versus_ai(mut self) {
        loop {
            println!("White to move");
            print_board(&self.board);

            let m = self.ai2.make_move(&self.board).unwrap();
            let log = make_move_log(&m, &self.board);
            println!("White moves \n{}", log);
            self.moves.push(log);
            self.board.make_move_mut(&m);

            println!("Black to move");
            print_board(&self.board);

            let m = self.ai.make_move(&self.board).unwrap();
            let log = make_move_log(&m, &self.board);
            println!("Black moves \n{}", log);
            self.moves.push(log);
            self.board.make_move_mut(&m);
        }
    }

    pub fn run_human_versus_ai(mut self) {
        let stdin = io::stdin();
        println!("You're playing white.");
        print_board(&self.board);
        println!("What's your move?");
        for line in stdin.lock().lines() {
            //@ todo : check for checkmate
            // white move
            let command = line.unwrap().clone();
            let m = chess_notation::read_move(command.as_str(), &self.board, Color::White);
            if m.is_none() {
                println!("That move is illegal!");
                continue;
            }
            let m = m.unwrap();
            let log = make_move_log(&m, &self.board);
            println!("move = \n{}", log);
            self.moves.push(log);
            self.board.make_move_mut(&m);

            // print eval
            let eval = AI::evaluator::evaluate(&self.board);
            println!("white eval {}", eval);
            print_board(&self.board);

            // black moves now
            let m = self.ai.make_move(&self.board).unwrap();
            let log = make_move_log(&m, &self.board);
            println!("move = \n{}", log);
            self.moves.push(log);
            self.board.make_move_mut(&m);
            // self.moves.push(m);
            println!("Black moves... {}", m);
            let eval = AI::evaluator::evaluate(&self.board);
            println!("white eval {}", eval);
            print_board(&self.board);
        }
    }
}
