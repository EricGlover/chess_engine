use crate::board::*;
use crate::fen_reader;
use crate::move_generator::*;

//@todo::
pub fn move_to_notation(m: &Move) -> String {
    String::from("NOT IMPLEMENTED")
}

pub fn parse_move(str: &str) -> (PieceType, Coordinate) {
    // need to generate moves to determine which piece can move there
    // piece specifier is uppercase
    let mut chars = str.chars().collect::<Vec<char>>();
    let first = chars.get(0).unwrap();

    let piece_type: PieceType = if first.to_lowercase().to_string() != first.to_string() {
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
pub fn read_move(str: &str, board: &Board, color: Color) -> Option<Move> {
    // figure out what they're trying to move and where
    let (piece_type, to) = parse_move(str);

    // find what piece they're talking about by looking through the possible moves
    let mut moves = gen_moves(board, color);
    print_move_list(&moves);
    moves
        .into_iter()
        .find(|m| m.piece.piece_type == piece_type && m.to == to)
}

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