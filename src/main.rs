#![warn(unused_extern_crates)]
use chess_engine::ai::ai;
use chess_engine::ai::AiSearch;
use chess_engine::board::Color;
use chess_engine::chess_notation::fen_reader;
use chess_engine::game;
use getopts::Options;
use std::env;

use std::time::Instant;

fn print_help_menu() {
    println!("For ai vs ai game \ncargo run -- --ai\n");
    println!("For human vs ai game \ncargo run\n");
}

fn test_perft() {
    let board = fen_reader::make_initial_board();
    let now = Instant::now();
    let mut ai = ai::new_with_search(Color::White, AiSearch::Minimax);
    ai.make_move(&board, Some(4));
    let nodes_at_depth: u64 = ai.minimax_calls() as u64;
    let elapsed_time = now.elapsed();

    println!();
    println!("Nodes searched:   {}", nodes_at_depth);
    println!("Time elapsed :    {} ms", elapsed_time.as_millis());
    if elapsed_time.as_secs() > 0 {
        println!(
            "Nodes searched per second : {} ",
            nodes_at_depth / elapsed_time.as_secs()
        );
    } else if elapsed_time.as_millis() > 0 {
        println!(
            "Nodes searched per milliseconds : {} ",
            nodes_at_depth / (elapsed_time.as_millis() as u64)
        );
    } else if elapsed_time.as_micros() > 0 {
        println!(
            "Nodes searched per microseconds : {} ",
            nodes_at_depth / (elapsed_time.as_micros() as u64)
        );
    }
}

fn main() {
    // test_perft();
    // return;
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
