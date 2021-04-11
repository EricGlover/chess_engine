use crate::board::*;
use crate::board::{Board, Coordinate};
use crate::board_console_printer::print_board;
use crate::chess_notation;
use crate::chess_notation::pgn::make_move_log;
use crate::fen_reader;
use crate::move_generator::Move;
use crate::AI;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use crate::AI::evaluator::evaluate;

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

    pub fn moves(&self) -> Vec<String> {
        self.moves.clone()
    }

    fn write_log(&self) {
        let path = Path::new("./GameLogs/output.txt");
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        let log = self.moves.join("\n");

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(log.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    fn ai1_make_move(&mut self) {
        println!("{} to move", self.ai.color());
        print_board(&self.board);

        let m = self.ai.make_move(&self.board, None).unwrap();
        let log = make_move_log(&m, &self.board);
        println!("{} moves \n{}", self.ai.color(), log);
        self.moves.push(log);
        self.board.make_move_mut(&m);
    }
    fn ai2_make_move(&mut self) {
        println!("{} to move", self.ai2.color());
        print_board(&self.board);

        let m = self.ai2.make_move(&self.board, None).unwrap();
        let log = make_move_log(&m, &self.board);
        println!("{} moves \n{}", self.ai2.color(), log);
        self.moves.push(log);
        self.board.make_move_mut(&m);
    }

    fn end_game(&self, winner: Color) {
        print_board(&self.board);
        self.write_log();
        println!("{} wins", winner);
    }

    pub fn run_ai_versus_ai(mut self) {
        loop {
            self.ai1_make_move();
            let evaluation = evaluate(&self.board);
            if evaluation.is_checkmate() {
                self.end_game(evaluation.mated_player.unwrap().opposite());
            }
            self.ai2_make_move();
            let evaluation = evaluate(&self.board);
            if evaluation.is_checkmate() {
                self.end_game(evaluation.mated_player.unwrap().opposite());
            }
        }
    }

    pub fn run_human_versus_ai(mut self) {
        let stdin = io::stdin();
        println!("You're playing white.");
        print_board(&self.board);
        println!("What's your move?");
        for line in stdin.lock().lines() {
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

            self.write_log();

            // print eval
            let eval = AI::evaluator::evaluate(&self.board);
            println!("eval {}", eval.score);
            if eval.is_checkmate() {
                self.end_game(eval.mated_player.unwrap().opposite());
                break;
            }

            print_board(&self.board);
            // black moves now
            let m = self.ai.make_move(&self.board, None).unwrap();
            let log = make_move_log(&m, &self.board);
            println!("move = \n{}", log);
            self.moves.push(log);
            self.board.make_move_mut(&m);
            // self.moves.push(m);
            println!("Black moves... {}", m);
            let eval = AI::evaluator::evaluate(&self.board);
            println!("eval {}", eval.score);
            println!("moves evaluated {}, time elapsed {:?}", self.ai.minimax_calls(), self.ai.time_elapsed().unwrap());
            if eval.is_checkmate() {
                self.end_game(eval.mated_player.unwrap().opposite());
                break;
            }
            print_board(&self.board);
            self.write_log();
        }
    }
}
