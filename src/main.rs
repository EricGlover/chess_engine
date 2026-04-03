#![warn(unused_extern_crates)]
use chess_engine::bit_board;
use chess_engine::bit_board::BitBoard;
use chess_engine::board::Board;
use chess_engine::board::{Color, Coordinate, Piece, PieceType};
use chess_engine::chess_notation;
use chess_engine::game_state::GameState;
use chess_engine::move_generator::plmg;
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

/**
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
    let debug = false;
    if debug {
        // testing bit boards
        let mut game_state = GameState::starting_game();
        println!("{} piece count ", game_state.board.get_piece_count());
        println!("{} piece count ", game_state.board.get_piece_count());
        // this feels wrong ...
        {
            // let board = &mut game_state.board;
            // board.clear();
        }
        println!("{} piece count ", game_state.board.get_piece_count());
        println!("{} piece count ", game_state.board.get_piece_count());

        let p = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(3, 6)));
        let moves = plmg::gen_pawn_moves(&game_state.board, &p, &game_state);

        println!("found {} moves", moves.len());
        for m in moves {
            println!("{}", m);
        }

        return;

        // let iterations = 10000;

        // let p = Piece::new(Color::White, PieceType::King, Some(Coordinate::new(3, 3)));

        // let board2 = Board::new();
        // let start1 = Instant::now();
        // for i in (0..iterations) {
        //     let moves = pseudo_legal_move_generator::gen_moves_for(&board2, &p);
        // }
        // let elapsed = start1.elapsed();
        // println!("{} micro seconds", elapsed.as_micros());

        // let board = BitBoard::new();
        // let start2 = Instant::now();
        // for i in (0..iterations) {
        //     let moves = plmg::gen_king_moves(&board, &p);
        // }
        // let elapsed2 = start2.elapsed();
        // println!(
        //     "{} micro seconds, {}x's faster",
        //     elapsed2.as_micros(),
        //     elapsed.as_micros() / elapsed2.as_micros()
        // );
        // return;

        // testing bit boards
        bit_board::test();
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
