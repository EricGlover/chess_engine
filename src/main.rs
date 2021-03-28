use chess_engine::board::*;
use chess_engine::board_console_printer;
use chess_engine::fen_reader;
use chess_engine::move_generator;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, Read};

/**
chess move reader

<piece_specifier><piece_file | piece_rank | piece_file && piece_rank><captures><file><rank>
piece_specifier = ['R', 'B', 'N', 'Q', 'K']
piece_file = [a-h][1-8]
captures = 'x'
file = [a-h]
rank = [1-8]
**/

mod Game {
    use chess_engine::board::*;
    use chess_engine::board::{Board, Coordinate};
    use chess_engine::fen_reader;
    use chess_engine::move_generator::Move;
    use std::borrow::Borrow;
    use std::io;
    use std::io::prelude::*;

    #[test]
    fn read_move_test() {
        let board = fen_reader::read(fen_reader::INITIAL_BOARD);
        let s = "Ra2";
        let s2 = "a4";
        let m = read_move(s, &board);
        let m2 = read_move(s2, &board);
        let a1 = Coordinate::from("a1");
        let a2 = Coordinate::from("a2");
        let a4 = Coordinate::from("a4");
        let rook = Piece::new(Color::White, PieceType::Rook, Some(a1.clone()));
        let pawn = Piece::new(Color::White, PieceType::Pawn, Some(a2.clone()));
        assert_eq!(m, Move::new(a1.clone(), a2.clone(), rook.clone()));
        assert_eq!(m2, Move::new(a2.clone(), a4.clone(), pawn.clone()));
    }

    // change this to result error ?
    fn read_move(str: &str, board: &Board) -> Move {
        // need to generate moves to determine which piece can move there
        // piece specifier is uppercase
        let chars = str.chars().collect::<Vec<char>>();
        let first = chars.get(0).unwrap();

        let piece_type = if first.to_lowercase().to_string() != first.to_string() {
            println!("is piece specifier");
            PieceType::from(first.to_lowercase().to_string().as_str()).unwrap()
        } else {
            PieceType::Pawn
        };
        println!("{}", str);
        println!("{:?}", chars);
        println!("{:?}", piece_type);

        let a1 = Coordinate::from("a1");
        let a2 = Coordinate::from("a2");
        let a4 = Coordinate::from("a4");
        let rook = Piece::new(Color::White, PieceType::Rook, Some(a1.clone()));
        let pawn = Piece::new(Color::White, PieceType::Pawn, Some(a2.clone()));
        Move::new(a1.clone(), a2.clone(), rook.clone())
    }

    pub struct Game {
        board: Board,
    }

    impl Game {
        pub fn new() -> Game {
            Game {
                board: fen_reader::read(fen_reader::INITIAL_BOARD),
            }
        }

        pub fn run(&self) {
            let stdin = io::stdin();
            for line in stdin.lock().lines() {
                // you play as white for now
                let command = line.unwrap();
                let m = read_move(command.as_str(), &self.board);
            }
        }
    }
}

fn main() {
    let game = Game::Game::new();
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
