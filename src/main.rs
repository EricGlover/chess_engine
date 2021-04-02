mod game;

use chess_engine::board::*;
use chess_engine::board_console_printer;
use chess_engine::fen_reader;
use chess_engine::move_generator;
use chess_engine::AI;
use std::io::prelude::*;
use std::io::{empty, BufReader, Read};

/**
alpha - beta searching
**/

/**
chess move reader

<piece_specifier><piece_file | piece_rank | piece_file && piece_rank><captures><file><rank>
piece_specifier = ['R', 'B', 'N', 'Q', 'K']
piece_file = [a-h][1-8]
captures = 'x'
file = [a-h]
rank = [1-8]
**/

fn main() {
    let game = game::Game::new();
    game.run();
}
