// use console::{style, Attribute};
use chess_engine::fen_reader;
use chess_engine::board_console_printer;
// use matrix_display::*;
// use std::thread;
// use std::time::Duration;

use chess_engine::board::*;
// use console::Term;
// use std::cell::Cell;

fn main() {
    // I can use the terminal but I don't quite see the point .... at least it's interactive / clearable
    // use terminal to print a board .....

    // let term = Term::stdout();
    //
    // term.write_line(style("Hello World! !♔♔♔♔♔♔").cyan().attr(Attribute::Bold).to_string().as_str());
    // thread::sleep(Duration::from_millis(2000));
    // term.clear_line();

    // println!("Hello, world!♔♔♔♔♔♔♕");
    // println!("This is {} neat", style("quite").cyan());

    // how to print board
    // squares background color set to black and white
    // pieces color set to red and green ?
    // pieces are letters (maybe use ascii art later
    // use the matrix thing for the background colors

    // let board = fen_reader::read(fen_reader::INITIAL_BOARD);
    // println!("reading board for {}", fen_reader::INITIAL_BOARD);
    // board_console_printer::print_board(board.get_squares());
    // let board = fen_reader::read(fen_reader::TEST_BOARD_1);
    // println!("reading board for {}", fen_reader::TEST_BOARD_1);
    // board_console_printer::print_board(board.get_squares());
    let board = fen_reader::read(fen_reader::TEST_BOARD_2);
    println!("reading board for {}", fen_reader::TEST_BOARD_2);
    board_console_printer::print_board(board.get_squares());
    return;

    let  piece = Piece::make_white_king();
    let mut square = Square{coordinate: Coordinate{x:1, y:1}, piece: Some(piece), color: Color::Black};
    println!("{:?}", square);
    square.piece = None;
    println!("{:?}", square);
    let mut vec :Vec<Square>= Vec::new();
    vec.push(square);
    println!("{:?}", vec);

    println!("{:?}", vec[0]);
    vec[0].piece = Some(piece);
    println!("{:?}", vec[0]);


    let mut board = Board::new();
    board.get_squares();
    let piece = Piece::make_white_king();
    board = board.place_piece(piece, &Coordinate {x: 1, y: 8});
    println!("{:?}", board);
    return;

    // let _board = Board::new();
    // let squares = Board::make_squares();
    // println!("{:?}", squares);
    // let board: Vec<Option<Piece>> = Board::make_initial_board();
    // print_board(&board);
}
