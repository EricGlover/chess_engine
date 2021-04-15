// use test::Bencher;

mod path;
mod pseudo_legal_move_generator;

use crate::board::*;
use crate::board_console_printer::print_board;
use crate::fen_reader;
use crate::fen_reader::make_board;
#[cfg(test)]
use crate::fen_reader::make_initial_board;
use crate::move_generator::path::*;
use crate::move_generator::pseudo_legal_move_generator::*;
use std::fmt;
use std::fmt::Formatter;

// @todo : test
/**

**/
#[cfg(test)]
mod tests {
    use super::*;
    // use test::Bencher;
    use crate::Ai::AI;

    #[test]
    fn perft_initial_position() {
        let board = fen_reader::make_initial_board();
        let mut ai = AI::new(Color::White);
        ai.make_move(&board, Some(0));
        assert_eq!(ai.minimax_calls(), 1, "1 node visited at depth 0");
        ai.make_move(&board, Some(1));
        assert_eq!(ai.minimax_calls(), 20, "20 nodes visited at depth 1");
        ai.make_move(&board, Some(2));
        assert_eq!(ai.minimax_calls(), 400, "400 nodes visited at depth 2");
        ai.make_move(&board, Some(3));
        assert_eq!(ai.minimax_calls(), 8902, "8902 nodes visited at depth 3");
        ai.make_move(&board, Some(4));
        assert_eq!(
            ai.minimax_calls(),
            197281,
            "197281 nodes visited at depth 4"
        );
        ai.make_move(&board, Some(5));
        assert_eq!(
            ai.minimax_calls(),
            4865609,
            "4865609 nodes visited at depth 5"
        );
    }

    #[test]
    fn test_find_moves_to_resolve_check() {
        let mut moves = vec![];
        let white_queen_checks = "rnb1k1nr/pp2pp1p/6pb/1Qpp4/qPPP4/N7/P3PPPP/R1B1KBNR b KQkq - 2 7";
        let board = fen_reader::make_board(white_queen_checks);
        // knight interpose
        let knight = board.get_piece_at(&Coordinate::new(2, 8)).unwrap();
        let knight_at = knight.at().unwrap();
        moves.push(Move::new(knight_at, Coordinate::new(4, 7), knight, false));
        moves.push(Move::new(knight_at, Coordinate::new(3, 6), knight, false));
        // bishop interpose
        let bishop = board.get_piece_at(&Coordinate::new(3, 8)).unwrap();
        let bishop_at = bishop.at().unwrap();
        moves.push(Move::new(bishop_at, bishop_at.add(1, -1), bishop, false));
        // queen captures
        let queen = board.get_piece_at(&Coordinate::new(1, 4)).unwrap();
        let queen_at = queen.at().unwrap();
        moves.push(Move::new(queen_at, queen_at.add(1, 1), queen, true));
        // king move left one, right one
        let king = board.get_piece_at(&Coordinate::new(5, 8)).unwrap();
        let king_at = king.at().unwrap();
        moves.push(Move::new(king_at, king_at.add(-1, 0), king, false));
        moves.push(Move::new(king_at, king_at.add(1, 0), king, false));

        let checks = get_checks(&board, Color::Black);
        let possible_moves = gen_pseudo_legal_moves(&board, Color::Black);
        println!("possible moves");
        possible_moves.iter().for_each(|m| println!("{}", m));
        let found_moves = find_moves_to_resolve_check(&board, &checks, &possible_moves);

        let moves: Vec<&Move> = moves.iter().collect();

        assert_eq!(
            found_moves.len(),
            moves.len(),
            "Same number of saving moves"
        );
        for m in moves.iter() {
            println!("{} is a move ", m);
        }
        for &m in moves.iter() {
            // find move

            let found = found_moves.iter().any(|m2| m2 == m);
            if !found {
                println!("{} looking for ", m);
            }
            assert_eq!(found, true, "a way out of check was not found");
        }

        // assert!(move_list_eq(&found_moves, &moves), "There are four moves that get black out of check.");
        // assert_eq!(found_moves, moves, "There are four moves that get black out of check.");
    }

    #[test]
    fn test_find_attacking_pieces() {
        let white_bishop_pinned =
            "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPPBPPPP/RN1QKB1R b KQkq - 3 3";
        let board = make_board(white_bishop_pinned);
        let mut king_pieces = board.get_pieces(Color::White, PieceType::King);
        assert!(king_pieces.get(0).is_some(), "king not found");
        let king = king_pieces.remove(0);
        let mut attacking_pieces = find_attacking_pieces(&board, Color::Black, &king.at().unwrap());
        assert_eq!(
            attacking_pieces.len(),
            1,
            "one piece should be attacking the king"
        );
        let piece = attacking_pieces.pop().unwrap();
        assert_eq!(piece.color, Color::Black, "piece is black");
        assert_eq!(
            piece.at().unwrap(),
            Coordinate::new(2, 4),
            " piece is at 2, 4"
        );
        assert_eq!(piece.piece_type, PieceType::Bishop, "piece is a bishop");
    }

    #[test]
    fn test_find_pinned_pieces() {
        // pinned by black bishop, can capture or move 1
        let white_bishop_pinned =
            "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPPBPPPP/RN1QKB1R b KQkq - 3 3";
        let board = make_board(white_bishop_pinned);
        // diagonal from pinning piece to one space before the king
        // it'd be neat to make diagonal from / to function, and file from / to, and rank from / to
        let mut pins = find_pinned_pieces(&board, Color::White);
        assert_eq!(pins.len(), 1, "There is one pin");
        let bishop = board.get_piece_at(&Coordinate::new(2, 4)).unwrap();
        let white_bishop = board.get_piece_at(&Coordinate::new(4, 2)).unwrap();
        let king = board.get_piece_at(&Coordinate::new(5, 1)).unwrap();
        let can_move_to = vec![Coordinate::new(2, 4), Coordinate::new(3, 3)];

        let found_pin = pins.pop().unwrap();

        let expected_pin = Pin {
            pinned_piece: white_bishop,
            pinned_by: bishop,
            pinned_to: king,
            can_move_to: can_move_to.clone(),
        };

        assert_eq!(found_pin.pinned_piece, white_bishop);
        assert_eq!(found_pin.pinned_by, bishop);
        assert_eq!(found_pin.pinned_to, king);
        assert_eq!(found_pin.can_move_to, can_move_to);

        assert_eq!(
            found_pin, expected_pin,
            "Black bishop pins white bishop to king"
        );

        // am I pinned if you're pinned ?
        let pinned_piece_attacks_kings =
            "rnb1k1nr/ppp2qpp/8/B1b1p2Q/3p4/1K2P2P/PPP2PP1/RN3B1R w kq - 0 17";
    }

    #[test]
    fn test_gen_legal_moves_checkmate() {
        let black_mates = "rnb1k1nr/pp2pp1p/Q5pb/2pp4/2PP4/N7/PP1qPPPP/R3KBNR w KQkq - 0 7";
        let board = make_board(black_mates);
        let moves = gen_legal_moves(&board, Color::White);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 0, "White has no moves");
        let white_mates = "2kQ4/pp3p2/4p1p1/7p/4P3/8/PP3PPP/3R2K1 b - - 0 21";
        let board = make_board(white_mates);
        let moves = gen_legal_moves(&board, Color::Black);
        assert_eq!(moves.len(), 0, "Black has no moves");
    }

    // #[bench]
    // fn bench_gen_find_pinned_pieces(b: &mut Bencher) {
    //     let white_bishop_pinned =
    //         "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPPBPPPP/RN1QKB1R b KQkq - 3 3";
    //     let board = make_board(white_bishop_pinned);
    //     b.iter(|| {
    //         let mut pins = find_pinned_pieces(&board, Color::White);
    //     })
    // }
    //
    // #[bench]
    // fn bench_gen_attack_vectors(b: &mut Bencher) {
    //     let black_mates = "rnb1k1nr/pp2pp1p/Q5pb/2pp4/2PP4/N7/PP1qPPPP/R3KBNR w KQkq - 0 7";
    //     let board = make_board(black_mates);
    //     b.iter(|| {
    //         let moves = gen_attack_vectors(&board, Color::White);
    //     })
    // }
    //
    // #[bench]
    // fn bench_gen_pseudo_legal_moves(b: &mut Bencher) {
    //     let black_mates = "rnb1k1nr/pp2pp1p/Q5pb/2pp4/2PP4/N7/PP1qPPPP/R3KBNR w KQkq - 0 7";
    //     let board = make_board(black_mates);
    //     let initial_board = fen_reader::make_board(fen_reader::INITIAL_BOARD);
    //     b.iter(|| {
    //         let moves = gen_pseudo_legal_moves(&board, Color::White);
    //         let moves = gen_pseudo_legal_moves(&board, Color::Black);
    //         let moves = gen_pseudo_legal_moves(&initial_board, Color::White);
    //         let moves = gen_pseudo_legal_moves(&initial_board, Color::Black);
    //     })
    //
    // }
    //
    // #[bench]
    // fn bench_gen_legal_moves(b: &mut Bencher) {
    //     let black_mates = "rnb1k1nr/pp2pp1p/Q5pb/2pp4/2PP4/N7/PP1qPPPP/R3KBNR w KQkq - 0 7";
    //     let board = make_board(black_mates);
    //     let initial_board = fen_reader::make_board(fen_reader::INITIAL_BOARD);
    //     b.iter(|| {
    //         let moves = gen_legal_moves(&board, Color::White);
    //         let moves = gen_legal_moves(&board, Color::Black);
    //         let moves = gen_legal_moves(&initial_board, Color::White);
    //         let moves = gen_legal_moves(&initial_board, Color::Black);
    //     })
    // }
}

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

    pub fn print_moves(moves: &Vec<Move>) {
        moves.iter().for_each(|m| println!("{}", m))
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

pub fn print_move_list(moves: &Vec<&Move>) {
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

fn move_list_eq(m: &Vec<&Move>, m2: &Vec<&Move>) -> bool {
    if m.len() != m2.len() {
        return false;
    }
    for &&mov in m.iter() {
        let mut found = false;
        for &&mov_2 in m2.iter() {
            if (mov.from == mov_2.from) && (mov.to == mov_2.to) {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    return true;
}

// ignores blocking pieces
// don't ignore same color pieces that are in the way
fn find_attacking_pieces(
    board: &Board,
    attackers_color: Color,
    attack_coordinate: &Coordinate,
) -> Vec<Piece> {
    let mut attacking_pieces: Vec<Piece> = vec![];
    // how to make sure the pieces returned are unique ?
    // pieces can't attack the same square twice , so we're good

    // generator moves while ignoring blocking enemy pieces
    let moves = gen_attack_vectors(board, attackers_color);
    for m in moves {
        if &m.to == attack_coordinate {
            attacking_pieces.push(m.piece.clone());
        }
    }
    attacking_pieces
}

fn find_pinned_pieces(board: &Board, defender_color: Color) -> Vec<Pin> {
    let attacker_color = defender_color.opposite();
    //@todo generate legal? moves

    // get defender king
    let mut king_pieces = board.get_pieces(defender_color, PieceType::King);
    if king_pieces.get(0).is_none() {
        return vec![];
    }
    let king = king_pieces.remove(0);

    // get pieces that can attack king (ignoring our own pieces)
    let attacking_pieces = find_attacking_pieces(board, attacker_color, &king.at().unwrap());

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
            let path = get_path_to(&from, &to);
            if path.is_none() {
                panic!("invalid path")
            }
            let mut path = path.unwrap();
            // remove the kings part of the path
            path.pop();
            // @todo: refactor this
            let mut defenders: Vec<Piece> = vec![];

            for coordinate in path.iter() {
                let piece = board.get_piece_at(coordinate);
                if piece.is_none() {
                    continue;
                } else {
                    let piece = piece.unwrap();
                    if piece.color == attacker_color {
                        continue;
                    } else {
                        defenders.push(piece.clone());
                    }
                }
            }
            if defenders.len() == 1 {
                let mut can_move_to = path.clone();
                // piece can move to where the king is, but can move to the attacker
                can_move_to.pop();
                let pinned_piece = defenders.pop().unwrap();
                let pin = Pin {
                    pinned_piece,
                    pinned_by: attacking_piece.clone(),
                    pinned_to: king.clone(),
                    can_move_to,
                };
                pins.push(pin);
            }
        }
    }
    pins
}

// @todo : sort this nonsense out
// @todo: consider using a board_get_all_pieces_ref instead of cloning the pieces

// get checks against color
pub fn get_checks(board: &Board, color_being_checked: Color) -> Vec<Move> {
    let moves = gen_pseudo_legal_moves(board, color_being_checked.opposite());
    let mut king_pieces = board.get_pieces(color_being_checked, PieceType::King);
    if king_pieces.len() == 0 {
        return vec![];
    }
    let king = king_pieces.get(0).unwrap();
    let at = king.at().unwrap();
    moves.into_iter().filter(|m| m.to == at).collect()
}

// fn find_moves_to_resolve_check<'a>(board: &Board, checks: &Vec<Move>, possible_moves: &'a Vec<Move>) -> Vec<&'a Move> {
// @todo: this doesn't work in so many ways.... :/
fn find_moves_to_resolve_check(
    board: &Board,
    checks: &Vec<Move>,
    possible_moves: &Vec<Move>,
) -> Vec<Move> {
    let mut moves: Vec<Move> = possible_moves.iter().map(|m| m.clone()).collect();

    // if no checks , BOOM problem is solved
    if checks.len() == 0 {
        return moves;
    }

    // worry about the king fleeing into an attack from another piece later
    // @todo
    fn king_safely_flees(board: &Board, m: &Move) -> bool {
        if m.piece.piece_type != PieceType::King {
            return false;
        }
        let new_board = board.make_move(m);
        get_checks(&new_board, m.piece.color).len() == 0
    }
    moves
        .into_iter()
        .filter(|m| {
            checks.iter().all(|check| {
                let path = path::get_path_to(&check.from, &check.to)
                    .unwrap_or_else(|| panic!("illegal move"));
                let interpose_path = &path[1..(path.len() - 1)];
                let is_interposing_move =
                    interpose_path.iter().any(|coordinate| coordinate == &m.to);
                let is_capture = check.from == m.to;
                king_safely_flees(board, &m)
                    || (is_interposing_move && m.piece.piece_type != PieceType::King)
                    || is_capture
            })
        })
        .collect()
}

// @todo pass attacker moves so you only calculate it once
pub fn gen_legal_moves(board: &Board, color: Color) -> Vec<Move> {
    let moves = gen_pseudo_legal_moves(board, color);

    // if in check do any of these moves resolve it ?
    // let enemy_moves = gen_attack_moves(board, color.opposite());
    let checks = get_checks(board, color);
    if checks.len() > 0 {
        return moves
            .into_iter()
            .filter(|m| {
                let new_board = board.make_move(m);
                get_checks(&new_board, m.piece.color).len() == 0
            })
            .collect();
        // return find_moves_to_resolve_check(board, &checks, &moves);
    } else {
        let pinned_pieces = find_pinned_pieces(board, color);
        fn is_pinned(piece: &Piece, pinned_pieces: &Vec<Pin>) -> bool {
            pinned_pieces.iter().any(|p| &p.pinned_piece == piece)
        }
        fn get_pin<'a>(piece: &Piece, pinned_pieces: &'a Vec<Pin>) -> Option<&'a Pin> {
            pinned_pieces.iter().find(|p| &p.pinned_piece == piece)
        }
        // if not in check, will this move expose my king ?
        let filtered_moves: Vec<Move> = moves
            .into_iter()
            .filter(|m| {
                // is this piece pinned ?
                if is_pinned(&m.piece, &pinned_pieces) {
                    let pin = get_pin(&m.piece, &pinned_pieces).unwrap();
                    // check if the pinned piece can move here
                    return pin.can_move_to.iter().any(|c| c == &m.to);
                }
                true
            })
            .collect();
        return filtered_moves;
    }
}

// ignores enemy captures
pub fn gen_attack_vectors(board: &Board, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let pieces = board.get_all_pieces(color);
    for piece in pieces.iter() {
        let m = gen_vectors_for(board, piece);
        moves.extend(m.into_iter());
    }
    return moves;
}

// change this to -> Vec<Coordinate>?
//@todo: test
// PSEUDO LEGAL MOVE GENERATION
pub fn gen_pseudo_legal_moves(board: &Board, color: Color) -> Vec<Move> {
    board
        .get_all_pieces(color)
        .into_iter()
        .map(|p| gen_moves_for(board, &p))
        .flatten()
        .collect()
}

// //@todo:: this would be so cool
// fn generate_path(starting_at: Coordinate, x: i32, y: i32) -> Iterator {
//
// }
