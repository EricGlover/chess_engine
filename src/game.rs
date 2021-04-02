use chess_engine::board::*;
use chess_engine::board::{Board, Coordinate};
use chess_engine::board_console_printer::print_board;
use chess_engine::chess_notation;
use chess_engine::fen_reader;
use chess_engine::move_generator::{gen_moves, print_move_list, Move};
use chess_engine::AI;
use std::io;
use std::io::prelude::*;

pub struct Game {
    board: Board,
    ai: AI::AI,
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: fen_reader::read(fen_reader::INITIAL_BOARD),
            ai: AI::AI::new(Color::Black),
        }
    }

    pub fn run(mut self) {
        let stdin = io::stdin();
        println!("You're playing white.");
        print_board(&self.board);
        println!("What's your move?");
        for line in stdin.lock().lines() {
            let command = line.unwrap().clone();
            let m = chess_notation::read_move(command.as_str(), &self.board, Color::White);
            if m.is_none() {
                println!("That move is illegal!");
                continue;
            }
            self.board.make_move_mut(&m.unwrap());
            // print eval
            let (white_eval, black_eval) = AI::evaluator::evaluate(&self.board);
            println!("white eval {}, black eval {}", white_eval, black_eval);
            print_board(&self.board);
            // black moves now
            let m = self.ai.make_move(&self.board);
            self.board.make_move_mut(&m);
            println!("Black moves... {}", m);
            println!("white eval {}, black eval {}", white_eval, black_eval);
            print_board(&self.board);
        }
    }
}
