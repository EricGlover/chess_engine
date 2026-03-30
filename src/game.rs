use crate::ai;
use crate::ai::evaluator::evaluate;
use crate::board::*;
use crate::board_console_printer::print_board;
use crate::chess_notation;
use crate::chess_notation::pgn::Game as PgnGame;
use crate::chess_notation::{fen_reader, parse_move, print_move};
use crate::move_generator::Move;
use chrono::{DateTime, Local};
use std::fs::{self, File, Metadata};
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::time::{Duration, Instant};

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
    ai: ai::Ai,
    ai2: ai::Ai,
    start_time: String,
    result: GameResult,
    enable_logging: bool,
    game_start: Instant
}

impl Game {
    pub fn new() -> Game {
        let mut ai = ai::Ai::new(Color::Black);
        ai.default_search_depth = 4;
        let mut ai2 = ai::Ai::new(Color::White);
        ai.default_search_depth = 4;
        Game {
            board: fen_reader::make_board(fen_reader::INITIAL_BOARD),
            ai,
            ai2,
            moves: vec![],
            start_time: Local::now().format("%Y-%m-%d_%H%M%S").to_string(),
            result: GameResult::InProgress,
            enable_logging: false,
            game_start: Instant::now()
        }
    }

    pub fn get_turn(&self) -> u32 {
        return (self.moves.len() as u32 / 2 ) + 1
    }

    pub fn get_time_elapsed(&self) -> Duration {
        self.game_start.elapsed()
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

    // write the current game as a pgn file with a FEN of the ending position
    // at the end of the file
    fn write_log(&self) {
        if (!self.enable_logging) {
            return;
        }

        //check for GameLogs directory
        let res = fs::metadata("./GameLogs");
        let is_dir: bool = match res {
            Ok(f) => f.is_dir(),
            Err(err) => false,
        };

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

    fn ai_make_move(&mut self, ai: &mut ai::Ai) {
        println!("{} to move", ai.color());
        print_board(&self.board);

        let m = ai.make_move(&self.board, None).unwrap();
        let log = print_move(&m, &self.board);
        println!("{} transposition table hits", ai.transposition_table_hits);
        println!("{} moves \n{}", ai.color(), log);
        self.moves.push(log);
        self.board.make_move_mut(&m);
    }

    fn ai1_make_move(&mut self) {
        println!("{} to move", self.ai.color());
        print_board(&self.board);

        let m = self.ai.make_move(&self.board, None).unwrap();
        let log = print_move(&m, &self.board);
        println!(
            "{} transposition table hits",
            self.ai.transposition_table_hits
        );
        println!("{} moves \n{}", self.ai.color(), log);
        self.moves.push(log);
        self.board.make_move_mut(&m);
    }
    fn ai2_make_move(&mut self) {
        println!("{} to move", self.ai2.color());
        print_board(&self.board);

        let m = self.ai2.make_move(&self.board, None).unwrap();
        let log = print_move(&m, &self.board);
        println!(
            "{} transposition table hits",
            self.ai2.transposition_table_hits
        );
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

    pub fn print_ai_stats_for_last_move(&self, ai: &ai::Ai) {
        println!(
            "{} transposition table hits",
            ai.transposition_table_hits
        );
        println!(
            "moves evaluated {}, time elapsed {:?}",
            ai.minimax_calls(),
            ai.time_elapsed().unwrap()
        );
    }

    pub fn run_sim_game(mut self, moves: Vec<Move>) {
        let mut white_to_move = true;
        for _move in moves {
            println!("Game time elasped : {:?}", self.get_time_elapsed());
            if (white_to_move) {
                //PLAYER 1'S TURN
                println!("{} to move", self.ai2.color());
                print_board(&self.board);
                // let evaluation = evaluate(&self.board, None, None);

                let m = self.ai2.make_move(&self.board, None).unwrap();
                let log = print_move(&m, &self.board);
                self.print_ai_stats_for_last_move(&self.ai2);
                println!("{} AI moves \n{}", self.ai2.color(), log);

                let log = print_move(&_move, &self.board);
                println!("{} player choose move \n{}", self.ai2.color(), log);
                self.moves.push(log);
                self.board.make_move_mut(&_move);
                white_to_move = false;
            } else {
                //PLAYER 2'S TURN
                println!("{} to move", self.ai.color());
                print_board(&self.board);

                let m = self.ai.make_move(&self.board, None).unwrap();
                let log = print_move(&m, &self.board);
                self.print_ai_stats_for_last_move(&self.ai);
                println!("{} AI moves \n{}", self.ai.color(), log);
                let log = print_move(&_move, &self.board);
                println!("{} player choose move \n{}", self.ai.color(), log);
                self.moves.push(log);
                self.board.make_move_mut(&_move);

                // if evaluation.is_checkmate() {
                //     self.end_game(evaluation.mated_player.unwrap().opposite());
                // }
                white_to_move = true;
            }
        }
        println!("The game ended here ");
        let t1 = match  self.ai.total_time_elapsed_during_search() {
            Some(duration) => duration.as_secs(),
            None => 0
        };
        let t2 = match  self.ai2.total_time_elapsed_during_search() {
            Some(duration) => duration.as_secs(),
            None => 0
        };
        if t1 > 0 {
            println!("AI searched {} moves, over {} seconds, at a rate of {} moves / second ", self.ai.total_minimax_calls(), t1, self.ai.total_minimax_calls() / t1 as u128);
        } else {
            println!("AI searched {} moves, over {} seconds", self.ai.total_minimax_calls(), t1);
        }
        if t2 > 0 {
            println!("AI searched {} moves, over {} seconds, at a rate of {} moves / second ", self.ai2.total_minimax_calls(), t2, self.ai2.total_minimax_calls() / t2 as u128);
        } else {
            println!("AI searched {} moves, over {} seconds", self.ai2.total_minimax_calls(), t2);
        }
        
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
            self.print_ai_stats_for_last_move(&self.ai);
            if eval.is_checkmate() {
                self.end_game(eval.mated_player.unwrap().opposite());
                break;
            }
            print_board(&self.board);
            self.write_log();
        }
    }
}
