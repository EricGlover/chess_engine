mod pseudo_legal_move_generator;

use crate::board::*;
use crate::fen_reader::make_board;
#[cfg(test)]
use crate::fen_reader::make_initial_board;
use crate::move_generator::pseudo_legal_move_generator::*;
use std::fmt;
use std::fmt::Formatter;

// @todo : test
/**

**/

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Move {
    pub piece: Piece,
    pub from: Coordinate,
    pub to: Coordinate,
    pub promoted_to: Option<PieceType>, // pawn promotion
    pub is_capture: bool,
    pub is_castling: bool,
    pub is_check: bool,     // @todo : set these in game when eval happens ?
    pub is_checkmate: bool, // @todo : set these in game when eval happens ?
    pub rook: Option<Piece>,
    pub rook_from: Option<Coordinate>,
    pub rook_to: Option<Coordinate>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} moving from {} to {} ",
            self.piece.piece_type, self.from, self.to
        )
    }
}

impl Move {
    pub fn new(from: Coordinate, to: Coordinate, piece: Piece, is_capture: bool) -> Move {
        Move {
            piece,
            from,
            to,
            promoted_to: None,
            is_castling: false,
            is_capture,
            rook: None,
            rook_from: None,
            rook_to: None,
            is_check: false,
            is_checkmate: false,
        }
    }

    pub fn pawn_promotion(
        from: Coordinate,
        to: Coordinate,
        piece: Piece,
        promoted_type: PieceType,
        is_capture: bool,
    ) -> Move {
        Move {
            piece,
            from,
            to,
            promoted_to: Some(promoted_type),
            is_castling: false,
            is_capture,
            rook: None,
            rook_from: None,
            rook_to: None,
            is_check: false,
            is_checkmate: false,
        }
    }

    pub fn castle_king_side(color: Color) -> Move {
        let (from, to) = Move::king_side_castle_coordinates(color, PieceType::King);
        let (rook_from, rook_to) = Move::king_side_castle_coordinates(color, PieceType::Rook);
        Move {
            piece: Piece::new(color, PieceType::King, Some(from.clone())),
            from,
            to,
            promoted_to: None,
            is_castling: true,
            is_capture: false,
            rook: Some(Piece::new(color, PieceType::Rook, Some(rook_from.clone()))),
            rook_from: Some(rook_from),
            rook_to: Some(rook_to),
            is_check: false,
            is_checkmate: false,
        }
    }
    pub fn castle_queen_side(color: Color) -> Move {
        let (from, to) = Move::queen_side_castle_coordinates(color, PieceType::King);
        let (rook_from, rook_to) = Move::queen_side_castle_coordinates(color, PieceType::Rook);
        Move {
            piece: Piece::new(color, PieceType::King, Some(from.clone())),
            from,
            to,
            promoted_to: None,
            is_castling: true,
            is_capture: false,
            rook: Some(Piece::new(color, PieceType::Rook, Some(rook_from.clone()))),
            rook_from: Some(rook_from),
            rook_to: Some(rook_to),
            is_check: false,
            is_checkmate: false,
        }
    }
    pub fn is_king_side_castle(&self) -> bool {
        self.rook_from.is_some() && self.rook_from.unwrap().x() == 8
    }
    pub fn is_queen_side_castle(&self) -> bool {
        self.rook_from.is_some() && self.rook_from.unwrap().x() == 1
    }
    pub fn king_side_castle_coordinates(
        color: Color,
        piece_type: PieceType,
    ) -> (Coordinate, Coordinate) {
        let y: u8 = if color == Color::White { 1 } else { 8 };
        match piece_type {
            PieceType::King => {
                let from = Coordinate::new(5, y);
                let to = Coordinate::new(7, y);
                return (from, to);
            }
            PieceType::Rook => {
                let from = Coordinate::new(8, y);
                let to = Coordinate::new(6, y);
                return (from, to);
            }
            _ => panic!("invalid"),
        }
    }
    pub fn queen_side_castle_coordinates(
        color: Color,
        piece_type: PieceType,
    ) -> (Coordinate, Coordinate) {
        let y: u8 = if color == Color::White { 1 } else { 8 };
        match piece_type {
            PieceType::King => {
                let from = Coordinate::new(5, y);
                let to = Coordinate::new(3, y);
                return (from, to);
            }
            PieceType::Rook => {
                let from = Coordinate::new(1, y);
                let to = Coordinate::new(4, y);
                return (from, to);
            }
            _ => panic!("invalid"),
        }
    }
}

pub fn print_move(m: &Move) {
    println!(
        "{:?} moving from ({}, {}) to ({},{}) ",
        m.piece.piece_type,
        m.from.x(),
        m.from.y(),
        m.to.x(),
        m.to.y()
    );
}

pub fn print_move_list(moves: &Vec<Move>) {
    for m in moves.iter() {
        print_move(m);
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Pin {
    pub pinned_piece: Piece,
    pub pinned_by: Piece,
    pub pinned_to: Piece,
    pub can_move_to: Vec<Coordinate>,
}
//@todo:::
//
// struct Path {
//     start: Coordinate,
//     direction:
// }
//
// pub struct PathIterator {
//     at: Coordinate,
//     direction: Coordinate,
// }
//
// impl Iterator for PathIterator {
//
// }

#[test]
fn test_find_pinned_pieces() {
    // pinned by black bishop, can capture or move 1
    let white_bishop_pinned = "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPPBPPPP/RN1QKB1R b KQkq - 3 3";
    let board = make_board(white_bishop_pinned);
    // diagonal from pinning piece to one space before the king
    // it'd be neat to make diagonal from / to function, and file from / to, and rank from / to
    let pins = find_pinned_pieces(&board, Color::White);
    assert_eq!(pins.len(), 1, "There is one pin");
    let bishop = board.get_piece_at(&Coordinate::new(2, 4)).unwrap();
    let white_bishop = board.get_piece_at(&Coordinate::new(4, 2)).unwrap();
    let king = board.get_piece_at(&Coordinate::new(5, 1)).unwrap();
    let can_move_to = vec![Coordinate::new(2, 4), Coordinate::new(3, 2)];
    let pin = Pin {
        pinned_piece: white_bishop,
        pinned_by: bishop,
        pinned_to: king,
        can_move_to,
    };
    assert_eq!(pins[0], pin, "Black bishop pins white bishop to king");

    // am I pinned if you're pinned ?
    let pinned_piece_attacks_kings =
        "rnb1k1nr/ppp2qpp/8/B1b1p2Q/3p4/1K2P2P/PPP2PP1/RN3B1R w kq - 0 17";
}

// ignores blocking pieces
fn find_attacking_pieces(board: &Board, attackers_color: Color, attack_coordinate: &Coordinate) -> Vec<Piece>{
    // vec![]
    let moves = gen_attack_moves(board, attackers_color);
    for m in moves {
        if m.to == at {
            return true;
        }
    }
    false
}


#[test]
fn test_get_path() {
    // diagonal
    let a = Coordinate::new(1, 1);
    let b = Coordinate::new(8, 8);
    let path = get_path(&a, &b);
    assert!(path.is_some(), "There is a path");
    let path = path.unwrap();
    assert_eq!(path.len(), 8);
    println!("{:?}", path);
    let expected : Vec<Coordinate> = vec![
        Coordinate::new(1,1),
        Coordinate::new(2,2),
        Coordinate::new(3,3),
        Coordinate::new(4,4),
        Coordinate::new(5,5),
        Coordinate::new(6,6),
        Coordinate::new(7,7),
        Coordinate::new(8,8),
    ];
    assert_eq!(path, expected);

    // right
    let a = Coordinate::new(1, 1);
    let b = Coordinate::new(2, 1);
    let path = get_path(&a, &b);
    assert!(path.is_some(), "There is a path");
    let path = path.unwrap();
    assert_eq!(path.len(), 2);
    let expected :Vec<Coordinate> = vec![
        a.clone(),
        b.clone(),
    ];
    assert_eq!(path, expected);

    // up
    let a = Coordinate::new(1, 1);
    let b = Coordinate::new(1, 2);
    let path = get_path(&a, &b);
    assert!(path.is_some(), "There is a path");
    let path = path.unwrap();
    assert_eq!(path.len(), 2);
    let expected :Vec<Coordinate> = vec![
        a.clone(),
        b.clone(),
    ];
    assert_eq!(path, expected);

    // test no path
    let a = Coordinate::new(1, 1);
    let b = Coordinate::new(2, 3);
    let path = get_path(&a, &b);
    assert!(path.is_none(), "There is no path");
}

// gets a straight path
fn get_path(from : &Coordinate, to : &Coordinate) -> Option<Vec<Coordinate>> {
    let (x_diff, y_diff) = to.diff(&from);

    // if they're on the same rank or file then there's a valid straight path
    // or if |from.x - to.x| == |from.y - to.y|
    fn is_straight(from: &Coordinate, to: &Coordinate) -> bool {
        let (x_diff, y_diff) = to.diff(&from);
        // horizontal || vertical || a straight diagonal
        (from.x() == to.x() || from.y() == to.y()) || (x_diff.abs() == y_diff.abs())
    }
    if !is_straight(&from, &to) {
        return None;
    }
    let delta_x = if x_diff > 0 { 1 } else if x_diff < 0 { - 1 } else { 0 };
    let delta_y = if y_diff > 0 { 1 } else if y_diff < 0 { -1 } else { 0 };
    let mut path:Vec<Coordinate> = vec![];
    let mut current = from.clone();
    while &current != to {
        path.push(current.clone());
        current = current.add(delta_x, delta_y);
        println!("current = {:?}", current);
    }
    path.push(current.clone());
    Some(path)
}

fn find_pinned_pieces(board: &Board, color: Color) -> Vec<Pin> {
    //@todo generate legal? moves
    vec![]
    let mut king_pieces = board.get_pieces(color, PieceType::King);
    if king_pieces.get(0).is_none() {
        return vec![];
    }
    let king = king_pieces.remove(0);
    let attacking_pieces = find_attacking_pieces(board, color, king.at().unwrap());
    // use piece.at and king.at to generate a range of Coordinates where pieces can interpose at
    let mut pins = vec![];

    for attacking_piece in attacking_pieces.iter() {
        // if piece is knight skip
        // if piece is one square away from the king then skip
        // assume King and Pawn can't attack the enemy king / from more than a square away
        let t = attacking_piece.piece_type;
        if t == PieceType::Queen || t == PieceType::Bishop || t == PieceType::Rook {
            let from = attacking_piece.at().unwrap();
            let to = king.at().unwrap();
            // if piece is Queen, Bishop, or Rook then
            // walk through the squares, from attacking piece to the king
            // if only one defender is in those squares then it's a pin
            let path = get_path(&from, &to);

            // let pinned_piece =
            let pin = Pin {
                pinned_piece,
                pinned_by: attacking_piece.clone(),
                pinned_to: king.clone(),
                can_move_to:
            };
            pins.push(pin);
        }
    }

    let pins = attacking_pieces.iter().map(|attacking_piece| {

    }).collect();
    pins
}

// @todo: check if piece is pinned , if pinned check if the move is legal
pub fn gen_legal_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let pieces = board.get_all_pieces(color);
    for piece in pieces.iter() {
        let m = gen_moves_for(board, piece);
        moves.extend(m.into_iter());
    }
    // @todo: fix the infinite loop
    let filtered_moves: Vec<Move> = moves
        .into_iter()
        .filter(|m| {
            let new_board = board.make_move(&m);
            !new_board.is_in_check(m.piece.color)
        })
        .collect();
    filtered_moves
}

// generates all moves to squares on the board
// could be illegal
//@todo: test
pub fn gen_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let pieces = board.get_all_pieces(color);
    for piece in pieces.iter() {
        let m = gen_moves_for(board, piece);
        moves.extend(m.into_iter());
    }
    // @todo: fix the infinite loop
    let filtered_moves: Vec<Move> = moves
        .into_iter()
        .filter(|m| {
            let new_board = board.make_move(&m);
            !new_board.is_in_check(m.piece.color)
        })
        .collect();
    filtered_moves
}

// change this to -> Vec<Coordinate>?
//@todo: test
pub fn gen_attack_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let pieces = board.get_all_pieces(color);
    for piece in pieces.iter() {
        let m = gen_moves_for(board, piece);
        moves.extend(m.into_iter());
    }
    return moves;
}

// //@todo:: this would be so cool
// fn generate_path(starting_at: Coordinate, x: i32, y: i32) -> Iterator {
//
// }
