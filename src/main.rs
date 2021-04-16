#![warn(unused_extern_crates)]
use chess_engine::game;
use getopts::Options;
use std::env;

fn print_help_menu() {
    println!("For ai vs ai game \ncargo run -- --ai\n");
    println!("For human vs ai game \ncargo run\n");
}

fn main() {
    //@todo:  use get opts for choosing the game modes and stuff
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("a", "ai", "run ai versus ai game");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_help_menu();
        return;
    }
    let game = game::Game::new();
    if matches.opt_present("ai") {
        game.run_ai_versus_ai();
    } else {
        game.run_human_versus_ai();
    }
}
