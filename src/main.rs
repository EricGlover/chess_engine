#![warn(unused_extern_crates)]
use chess_engine::board::Coordinate;
use chess_engine::chess_notation;
use chess_engine::{chess_notation::pgn, game};
use chess_engine::board::PieceType;
use getopts::Options;
use std::io::Read;
use std::{env, path};
use std::fs::{self, File, Metadata};
use std::path::Path;
use regex::*;
use chess_engine::bit_board;
use pgn::Game as notated_game;


fn print_help_menu() {
    println!("For ai vs ai game \ncargo run -- --ai\n");
    println!("To read a pgn from /Games and have the AI consider it \ncargo run -- --sim\n");
    println!("For help menu run \ncargo run -- --help\n");
    println!("For human vs ai game \ncargo run\n");
}

fn main() {
    // testing bit boards 
    bit_board::test();
    return;








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
            let mut contents:String = match std::fs::read_to_string(&path) {
                Err(err) => panic!("couldn't read {}: {}", display, err),
                Ok(file) => file,
            };
            contents = contents.trim_start_matches('\u{feff}').replace("\r\n", "\n");
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
