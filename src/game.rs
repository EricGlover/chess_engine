use crate::ai;
use crate::ai::evaluator::evaluate;
use crate::board::*;
use crate::board_console_printer::print_board;
use crate::chess_notation;
use crate::chess_notation::pgn::Game as PgnGame;
use crate::chess_notation::{fen_reader, parse_move, print_move};
use crate::move_generator::Move;
use chrono::{DateTime, Local};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

pub struct Player {
    time_used: u16,      // milliseconds
    time_remaining: u16, // milliseconds
    name: String,
}

#[derive(Debug, Copy, Clone)]
pub enum GameResult {
    InProgress,
    Draw,
    Win { winning_player: Color },
}

pub struct Game {
    board: Board,
    moves: Vec<String>,
    ai: ai::ai,
    ai2: ai::ai,
    start_time: String,
    result: GameResult,
}

impl Game {
    pub fn new() -> Game {
        let mut ai = ai::ai::new(Color::Black);
        ai.default_search_depth = 4;
        let mut ai2 = ai::ai::new(Color::White);
        ai.default_search_depth = 4;
        Game {
            board: fen_reader::make_board(fen_reader::INITIAL_BOARD),
            ai,
            ai2,
            moves: vec![],
            start_time: Local::now().format("%Y-%m-%d_%H%M%S").to_string(),
            result: GameResult::InProgress,
        }
    }

    pub fn result(&self) -> GameResult {
        self.result.clone()
    }

    pub fn moves(&self) -> Vec<String> {
        self.moves.clone()
    }

    pub fn make_move(&mut self, move_: &Move) {
        let log = print_move(&move_, &self.board);
        println!("move = \n{}", log);
        self.moves.push(log);
        self.board.make_move_mut(&move_);
    }

    pub fn make_moves(&mut self, moves: Vec<(Move, Option<Move>)>) {
        for (w_move, b_move) in moves {
            self.make_move(&w_move);
            if b_move.is_some() {
                self.make_move(b_move.as_ref().unwrap())
            }
        }
    }

    fn write_log(&self) {
        let path_str = format!("./GameLogs/{}-output.txt", self.start_time);
        let path = Path::new(path_str.as_str());
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        let pgn = PgnGame::new_from_game(&self);
        let fen = fen_reader::make_fen(&self.board);
        let log = format!("{}\n{}", pgn, fen);

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
        let log = print_move(&m, &self.board);
        println!("{} transposition table hits", self.ai.transposition_table_hits);
        println!("{} moves \n{}", self.ai.color(), log);
        self.moves.push(log);
        self.board.make_move_mut(&m);
    }
    fn ai2_make_move(&mut self) {
        println!("{} to move", self.ai2.color());
        print_board(&self.board);

        let m = self.ai2.make_move(&self.board, None).unwrap();
        let log = print_move(&m, &self.board);
        println!("{} transposition table hits", self.ai2.transposition_table_hits);
        println!("{} moves \n{}", self.ai2.color(), log);
        self.moves.push(log);
        self.board.make_move_mut(&m);
    }

    fn end_game(&mut self, winner: Color) {
        self.result = GameResult::Win {
            winning_player: winner,
        };
        print_board(&self.board);
        self.write_log();
        println!("{} wins", winner);
    }

    pub fn run_ai_versus_ai(mut self) {
        loop {
            self.ai2_make_move();
            let evaluation = evaluate(&self.board, None, None);
            if evaluation.is_checkmate() {
                self.end_game(evaluation.mated_player.unwrap().opposite());
            }
            self.write_log();
            self.ai1_make_move();
            let evaluation = evaluate(&self.board, None, None);
            if evaluation.is_checkmate() {
                self.end_game(evaluation.mated_player.unwrap().opposite());
            }
            self.write_log();
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
            let m = parse_move(command.as_str(), &self.board, Color::White);
            if m.is_none() {
                println!("That move is illegal!");
                continue;
            }
            let m = m.unwrap();
            let log = print_move(&m, &self.board);
            println!("move = \n{}", log);
            self.moves.push(log);
            self.board.make_move_mut(&m);
            self.write_log();

            // print eval
            let eval = ai::evaluator::evaluate(&self.board, None, None);
            println!("eval {}", eval.score);
            if eval.is_checkmate() {
                self.end_game(eval.mated_player.unwrap().opposite());
                break;
            }

            print_board(&self.board);
            // black moves now
            let m = self.ai.make_move(&mut self.board, None).unwrap();
            let log = print_move(&m, &self.board);
            println!("move = \n{}", log);
            self.moves.push(log);
            self.board.make_move_mut(&m);
            // self.moves.push(m);
            println!("Black moves... {}", m);
            let eval = ai::evaluator::evaluate(&self.board, None, None);
            println!("eval {}", eval.score);
            println!(
                "moves evaluated {}, time elapsed {:?}",
                self.ai.minimax_calls(),
                self.ai.time_elapsed().unwrap()
            );
            if eval.is_checkmate() {
                self.end_game(eval.mated_player.unwrap().opposite());
                break;
            }
            print_board(&self.board);
            self.write_log();
        }
    }
}
