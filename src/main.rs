mod game;

use chess_engine::board::*;
use chess_engine::board_console_printer;
use chess_engine::fen_reader;
use chess_engine::move_generator;
use std::io;
use std::io::prelude::*;
use std::io::{empty, BufReader, Read};

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
    return;

    // loop {
    //     match handle.read(&mut buf) {
    //         Ok(0) => break,
    //         Ok(n) => {
    //             BufReader::new()
    //             for byte in &buf[..n] {
    //
    //                 println!("{}", byte);
    //             }
    //         },
    //         Err(err) => {
    //             println!("err: {}", err);
    //             break;
    //         },
    //     }
    // }

    let board = fen_reader::read(fen_reader::INITIAL_BOARD);
    println!("reading board for {}", fen_reader::INITIAL_BOARD);
    board_console_printer::print_board(&board);

    let moves = move_generator::gen_moves(&board, Color::White);

    println!("NUMBER OF MOVES {:?}", moves.len());
    println!("MOVES {:?}", moves);
    for m in moves.iter() {
        println!(
            "{:?} moving from ({}, {}) to ({},{}) ",
            m.piece.piece_type, m.from.x, m.from.y, m.to.x, m.to.y
        );
    }

    return;

    let board = fen_reader::read(fen_reader::TEST_BOARD_1);
    println!("reading board for {}", fen_reader::TEST_BOARD_1);
    board_console_printer::print_board(&board);

    let board = fen_reader::read(fen_reader::TEST_BOARD_2);
    println!("reading board for {}", fen_reader::TEST_BOARD_2);
    board_console_printer::print_board(&board);

    // return;

    // let  piece = Piece::make_white_king();
    // let mut square = Square{coordinate: Coordinate{x:1, y:1}, piece: Some(piece), color: Color::Black};
    // println!("{:?}", square);
    // square.piece = None;
    // println!("{:?}", square);
    // let mut vec :Vec<Square>= Vec::new();
    // vec.push(square);
    // println!("{:?}", vec);
    //
    // println!("{:?}", vec[0]);
    // vec[0].piece = Some(piece);
    // println!("{:?}", vec[0]);
    //
    //
    // let mut board = Board::new();
    // board.get_squares();
    // let piece = Piece::make_white_king();
    // board = board.place_piece(piece, &Coordinate {x: 1, y: 8});
    // println!("{:?}", board);
    // return;

    // let _board = Board::new();
    // let squares = Board::make_squares();
    // println!("{:?}", squares);
    // let board: Vec<Option<Piece>> = Board::make_initial_board();
    // print_board(&board);
}
