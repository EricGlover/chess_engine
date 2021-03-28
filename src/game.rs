use chess_engine::board::*;
use chess_engine::board::{Board, Coordinate};
use chess_engine::board_console_printer::print_board;
use chess_engine::fen_reader;
use chess_engine::move_generator::{gen_moves, Move};
use std::borrow::Borrow;
use std::io;
use std::io::empty;
use std::io::prelude::*;

#[test]
fn read_move_test() {
    let board = fen_reader::read(fen_reader::INITIAL_BOARD);
    let s = "Ra2";
    let s2 = "a4";
    let m = read_move(s, &board, Color::White);
    let m2 = read_move(s2, &board, Color::White).unwrap();
    let a1 = Coordinate::from("a1");
    let a2 = Coordinate::from("a2");
    let a4 = Coordinate::from("a4");
    let rook = Piece::new(Color::White, PieceType::Rook, Some(a1.clone()));
    let pawn = Piece::new(Color::White, PieceType::Pawn, Some(a2.clone()));
    assert!(m.is_none());
    assert_eq!(m2, Move::new(a2.clone(), a4.clone(), pawn.clone()));
}

fn parse_move(str: &str) -> (PieceType, Coordinate) {
    // need to generate moves to determine which piece can move there
    // piece specifier is uppercase
    let mut chars = str.chars().collect::<Vec<char>>();
    let first = chars.get(0).unwrap();

    let piece_type: PieceType = if first.to_lowercase().to_string() != first.to_string() {
        println!("is piece specifier");
        let t = PieceType::from(first.to_lowercase().to_string().as_str()).unwrap();
        chars.remove(0);
        t
    } else {
        PieceType::Pawn
    };
    let s: String = chars.splice((0..2), std::iter::empty()).collect();
    let to = Coordinate::from(s.as_str());
    (piece_type, to)
}

// change this to result error ?
// doesn't return illegal moves, return None if not possible
fn read_move(str: &str, board: &Board, color: Color) -> Option<Move> {
    // figure out what they're trying to move and where
    let (piece_type, to) = parse_move(str);

    // find what piece they're talking about by looking through the possible moves
    let mut moves = gen_moves(board, color);
    moves
        .into_iter()
        .find(|m| m.piece.piece_type == piece_type && m.to == to)
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

    pub fn run(mut self) {
        let stdin = io::stdin();
        println!("You're playing white.");
        print_board(&self.board);
        println!("What's your move?");
        for line in stdin.lock().lines() {
            let command = line.unwrap().clone();
            loop {
                // you play as white for now
                let m = read_move(command.as_str(), &self.board, Color::White);
                if m.is_none() {
                    println!("That move is illegal!");
                } else {
                    self.board.make_move(m.unwrap());
                    print_board(&self.board);
                    break;
                }
            }
            // black moves now
        }
    }
}
