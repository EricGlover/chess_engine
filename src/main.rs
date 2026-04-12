#![warn(unused_extern_crates)]
use chess_engine::{bit_board, search};
use chess_engine::bit_board::BitBoard;
use chess_engine::board::{Board, BoardTrait};
use chess_engine::board::{Color, Coordinate, Piece, PieceType};
use chess_engine::board_console_printer::print_bit_board;
use chess_engine::chess_notation::{self, fen_reader};
use chess_engine::game_state::GameState;
use chess_engine::move_generator::{Move, MoveType, plmg};
use chess_engine::move_generator::pseudo_legal_move_generator;
use chess_engine::{chess_notation::pgn, game, game_state};
use getopts::Options;
use pgn::Game as notated_game;
use regex::*;
use std::fs::{self, File, Metadata};
use std::io::Read;
use std::path::Path;
use std::time::{Duration, Instant};
use std::{env, path};

/*
 * init results of running gen_king moves between old board and bit board
 * over 10 thousand iterations each time with release build
 * bit_boards seem around 50x's faster on average, at worst it was 30 something (38 on first run? )
2144 micro seconds
41 micro seconds, 52x's faster
 */

fn print_help_menu() {
    println!("For ai vs ai game \ncargo run -- --ai\n");
    println!("To read a pgn from /Games and have the AI consider it \ncargo run -- --sim\n");
    println!("For help menu run \ncargo run -- --help\n");
    println!("For human vs ai game \ncargo run\n");
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let debug = true;
    if debug {
        plmg::test();
        // search::test();
        return;
    }


    //@todo:  use get opts for choosing the game modes and stuff
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("a", "ai", "run ai versus ai game");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("s", "sim", "run sim game to test engine");
    opts.optflag("p", "pvp", "run player vs player");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        // println!("in help mode ");
        print_help_menu();
        return;
    }
    if matches.opt_present("p") {
        // @todo:
        return;
    }
    if matches.opt_present("s") {
        // println!("in sim mode ");
        let game = game::Game::new();
        // read pgn file
        //check for Games directory
        // fix this pathing
        let res = fs::metadata("./Games");
        let is_dir: bool = match res {
            Ok(f) => f.is_dir(),
            Err(err) => false,
        };
        if is_dir {
            let mut path_str = format!("./Games/");
            path_str.push_str("1.pgn");
            let path = Path::new(path_str.as_str());
            let display = path.display();
            let mut contents: String = match std::fs::read_to_string(&path) {
                Err(err) => panic!("couldn't read {}: {}", display, err),
                Ok(file) => file,
            };
            contents = contents
                .trim_start_matches('\u{feff}')
                .replace("\r\n", "\n");
            let moves = pgn::moves_from_pgn(contents.as_str());
            game.run_sim_game(moves);
            return;
        }
    }
    let game = game::Game::new();
    if matches.opt_present("ai") {
        game.run_ai_versus_ai();
    } else {
        game.run_human_versus_ai();
    }
}
