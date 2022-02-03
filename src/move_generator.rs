// use test::Bencher;

mod attack_map;
mod chess_move;
mod path;
mod pin;
mod pseudo_legal_move_generator;

pub use attack_map::*;
pub use chess_move::*;
pub use path::*;
pub use pin::*;

use crate::board::*;
use crate::board_console_printer::print_board;
use crate::move_generator::path::*;
use crate::move_generator::pseudo_legal_move_generator::*;
use std::fmt;
use std::fmt::Formatter;

#[cfg(test)]
mod bench {
    use super::*;
    use crate::ai::{ai, AiSearch};
    use crate::chess_notation::fen_reader;
    use crate::chess_notation::fen_reader::*;
    use crate::move_generator::chess_move::MoveType;

    use test::Bencher;
    #[bench]
    fn bench_perft(b: &mut Bencher) {
        let board = fen_reader::make_initial_board();
        let mut ai = ai::new_with_search(Color::White, AiSearch::Minimax);
        b.iter(|| {
            // ai.make_move(&board, Some(0));
            // ai.make_move(&board, Some(1));
            // ai.make_move(&board, Some(2));
            ai.make_move(&board, Some(3));
            // ai.make_move(&board, Some(4));
            // ai.make_move(&board, Some(5));
        })
    }
    #[bench]
    fn bench_gen_find_pinned_pieces(b: &mut Bencher) {
        let white_bishop_pinned =
            "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPPBPPPP/RN1QKB1R b KQkq - 3 3";
        let board = make_board(white_bishop_pinned);
        b.iter(|| {
            let mut pins = find_pinned_pieces(&board, Color::White);
        })
    }

    #[bench]
    fn bench_gen_attack_vectors(b: &mut Bencher) {
        let black_mates = "rnb1k1nr/pp2pp1p/Q5pb/2pp4/2PP4/N7/PP1qPPPP/R3KBNR w KQkq - 0 7";
        let board = make_board(black_mates);
        b.iter(|| {
            let moves = gen_attack_vectors(&board, Color::White);
        })
    }

    #[bench]
    fn bench_gen_pseudo_legal_moves(b: &mut Bencher) {
        let black_mates = "rnb1k1nr/pp2pp1p/Q5pb/2pp4/2PP4/N7/PP1qPPPP/R3KBNR w KQkq - 0 7";
        let board = make_board(black_mates);
        let initial_board = fen_reader::make_board(fen_reader::INITIAL_BOARD);
        b.iter(|| {
            let moves = gen_pseudo_legal_moves(&board, Color::White);
            let moves = gen_pseudo_legal_moves(&board, Color::Black);
            let moves = gen_pseudo_legal_moves(&initial_board, Color::White);
            let moves = gen_pseudo_legal_moves(&initial_board, Color::Black);
        })
    }

    #[bench]
    fn gen_pseudo_legal_moves_initial_board_white(b: &mut Bencher) {
        let initial_board = fen_reader::make_board(fen_reader::INITIAL_BOARD);
        b.iter(|| {
            let moves = gen_pseudo_legal_moves(&initial_board, Color::White);
        })
    }

    #[bench]
    fn bench_gen_legal_moves(b: &mut Bencher) {
        let black_mates = "rnb1k1nr/pp2pp1p/Q5pb/2pp4/2PP4/N7/PP1qPPPP/R3KBNR w KQkq - 0 7";
        let board = make_board(black_mates);
        let initial_board = fen_reader::make_board(fen_reader::INITIAL_BOARD);
        b.iter(|| {
            let moves = gen_legal_moves(&board, Color::White);
            let moves = gen_legal_moves(&board, Color::Black);
            let moves = gen_legal_moves(&initial_board, Color::White);
            let moves = gen_legal_moves(&initial_board, Color::Black);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::{ai, AiSearch};
    use crate::chess_notation::fen_reader;
    use crate::chess_notation::fen_reader::*;
    use crate::move_generator::chess_move::MoveType;
    use test::Bencher;

    #[test]
    fn test_gen_pseudo_legal_moves() {}

    #[test]
    fn perft_initial_position() {
        let board = fen_reader::make_initial_board();
        let mut ai = ai::new_with_search(Color::White, AiSearch::Minimax);
        ai.make_move(&board, Some(0));
        assert_eq!(ai.minimax_calls(), 1, "1 node visited at depth 0");
        ai.make_move(&board, Some(1));
        assert_eq!(ai.minimax_calls(), 20, "20 nodes visited at depth 1");
        ai.make_move(&board, Some(2));
        assert_eq!(ai.minimax_calls(), 400, "400 nodes visited at depth 2");
        ai.make_move(&board, Some(3));
        assert_eq!(ai.minimax_calls(), 8902, "8902 nodes visited at depth 3");
        // ai.make_move(&board, Some(4));
        // assert_eq!(
        //     ai.minimax_calls(),
        //     197281,
        //     "197281 nodes visited at depth 4"
        // );
        // ai.make_move(&board, Some(5));
        // assert_eq!(
        //     ai.minimax_calls(),
        //     4865609,
        //     "4865609 nodes visited at depth 5"
        // );
    }

    #[test]
    fn test_in_check() {
        let board = fen_reader::make_board(fen_reader::BLACK_IN_CHECK);
        let potential_check_moves = gen_pseudo_legal_moves(&board, Color::White);
        let checks = get_checks(&board, Color::Black, &potential_check_moves);
        assert!(checks.len() > 0, "black is in check");

        let potential_check_moves = gen_pseudo_legal_moves(&board, Color::Black);
        let checks = get_checks(&board, Color::White, &potential_check_moves);
        assert!(checks.len() == 0, "white is not in check");

        let board = fen_reader::make_board(fen_reader::WHITE_IN_CHECK);

        let potential_check_moves = gen_pseudo_legal_moves(&board, Color::White);
        let checks = get_checks(&board, Color::Black, &potential_check_moves);
        assert!(checks.len() == 0, "black is not in check");

        let potential_check_moves = gen_pseudo_legal_moves(&board, Color::Black);
        let checks = get_checks(&board, Color::White, &potential_check_moves);
        assert!(checks.len() > 0, "white is in check");
    }

    #[test]
    fn test_find_moves_to_resolve_check() {
        let mut moves = vec![];
        let white_queen_checks = "rnb1k1nr/pp2pp1p/6pb/1Qpp4/qPPP4/N7/P3PPPP/R1B1KBNR b KQkq - 2 7";
        let board = fen_reader::make_board(white_queen_checks);
        // knight interpose
        let knight = board.get_piece_at(&Coordinate::new(2, 8)).unwrap();
        let knight_at = knight.at().unwrap();
        moves.push(Move::new(
            knight_at.clone(),
            Coordinate::new(4, 7),
            knight.piece_type,
            MoveType::Move,
            None,
            None,
            None,
        ));
        moves.push(Move::new(
            knight_at.clone(),
            Coordinate::new(3, 6),
            knight.piece_type,
            MoveType::Move,
            None,
            None,
            None,
        ));
        // bishop interpose
        let bishop = board.get_piece_at(&Coordinate::new(3, 8)).unwrap();
        let bishop_at = bishop.at().unwrap();
        moves.push(Move::new(
            bishop_at.clone(),
            bishop_at.add(1, -1),
            bishop.piece_type,
            MoveType::Move,
            None,
            None,
            None,
        ));
        // queen captures
        let queen = board.get_piece_at(&Coordinate::new(1, 4)).unwrap();
        let queen_at = queen.at().unwrap();
        moves.push(Move::new(
            queen_at.clone(),
            queen_at.add(1, 1),
            queen.piece_type,
            MoveType::Move,
            Some(PieceType::Queen),
            None,
            None,
        ));
        // king move left one, right one
        let king = board.get_piece_at(&Coordinate::new(5, 8)).unwrap();
        let king_at = king.at().unwrap();
        moves.push(Move::new(
            king_at.clone(),
            king_at.add(-1, 0),
            king.piece_type,
            MoveType::Move,
            None,
            board.get_castling_rights_changes_if_piece_moves(king),
            None,
        ));
        moves.push(Move::new(
            king_at.clone(),
            king_at.add(1, 0),
            king.piece_type,
            MoveType::Move,
            None,
            board.get_castling_rights_changes_if_piece_moves(king),
            None,
        ));

        let potential_checks = gen_pseudo_legal_moves(&board, Color::White);
        let checks = get_checks(&board, Color::Black, &potential_checks);

        let possible_moves = gen_pseudo_legal_moves(&board, Color::Black);
        // println!("possible moves");
        // possible_moves.iter().for_each(|m| println!("{}", m));

        let found_moves = find_moves_to_resolve_check(&board, &checks, &possible_moves);
        println!("found moves");
        found_moves.iter().for_each(|m| println!("{}", m));

        let moves: Vec<&Move> = moves.iter().collect();
        println!("correct moves");
        moves.iter().for_each(|m| println!("{}", m));
        print_board(&board);
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
            &Coordinate::new(2, 4),
            " piece is at 2, 4"
        );
        assert_eq!(piece.piece_type, PieceType::Bishop, "piece is a bishop");
    }

    #[test]
    fn test_get_checks() {
        let board = fen_reader::make_board(
            "rnb1kbnr/pppp1p1p/4pp2/8/8/3BP3/PPPP1PPP/RNB1K1NR b KQkq - 1 4",
        );
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
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct MoveLog {
    pub piece_type: PieceType,
    pub from: Coordinate,
    pub to: Coordinate,
    pub promoted_to: Option<PieceType>,
    // pub captured: PieceType,
    // pub is_check : bool,
    // pub is_checkmate: bool,
    // pub is_king_side_castle: bool,
    // pub is_queen_side_castle: bool,
}

impl MoveLog {
    pub fn new(m: &Move) -> MoveLog {
        let promoted_to = match m.move_type() {
            MoveType::Promotion(t) => Some(t.clone()),
            _ => None,
        };
        MoveLog {
            piece_type: m.piece,
            from: m.from.clone(),
            to: m.to.clone(),
            promoted_to,
        }
    }
}

pub fn print_move(m: &Move) {
    println!(
        "{:?} moving from ({}, {}) to ({},{}) ",
        m.piece,
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

// ignores blocking pieces
// don't ignore same color pieces that are in the way
// @todo : replace this with attack map stuff
fn find_attacking_pieces<'a>(
    board: &'a dyn BoardTrait,
    attackers_color: Color,
    attack_coordinate: &Coordinate,
) -> Vec<&'a Piece> {
    let mut attacking_pieces: Vec<&Piece> = vec![];
    // how to make sure the pieces returned are unique ?
    // pieces can't attack the same square twice , so we're good

    // generator moves while ignoring blocking enemy pieces
    let moves = gen_attack_vectors(board, attackers_color);
    for m in moves {
        if &m.to == attack_coordinate {
            let piece = board.get_piece_at(&m.from).unwrap();
            attacking_pieces.push(piece);
        }
    }
    attacking_pieces
}

// @todo : sort this nonsense out
// @todo: consider using a board_get_all_pieces_ref instead of cloning the pieces
//@todo : find_checks_from_moves()
// @todo: piece lists for fast lookups

// get checks against color
pub fn get_checks(
    board: &dyn BoardTrait,
    color_being_checked: Color,
    potential_check_moves: &Vec<Move>,
) -> Vec<Move> {
    let king_pieces = board.get_pieces(color_being_checked, PieceType::King);
    if king_pieces.len() == 0 {
        return vec![];
    }
    let king = king_pieces.get(0).unwrap();
    let at = king.at().unwrap();
    potential_check_moves
        .into_iter()
        .filter(|&m| &m.to == at)
        .map(|m| m.clone())
        .collect()
}

fn find_checks_from_moves<'a>(
    board: &dyn BoardTrait,
    moves: &'a Vec<Move>,
    color_being_checked: Color,
) -> Vec<&'a Move> {
    let king_pieces = board.get_pieces(color_being_checked, PieceType::King);
    if king_pieces.len() == 0 {
        return vec![];
    }
    let king = king_pieces.get(0).unwrap();
    let at = king.at().unwrap();
    moves.into_iter().filter(|&m| &m.to == at).collect()
}

// fn find_moves_to_resolve_check<'a>(board: &dyn BoardTrait, checks: &Vec<Move>, possible_moves: &'a Vec<Move>) -> Vec<&'a Move> {
// @todo: this doesn't work in so many ways.... :/
fn find_moves_to_resolve_check(
    board: &dyn BoardTrait,
    checks: &Vec<Move>,
    possible_moves: &Vec<Move>,
) -> Vec<Move> {
    let moves: Vec<Move> = possible_moves.iter().map(|&m| m.clone()).collect();

    // if no checks , BOOM problem is solved
    if checks.len() == 0 {
        return possible_moves.iter().map(|&m| m.clone()).collect();
    }

    // worry about the king fleeing into an attack from another piece later
    // @todo Make an Attack Map Board thing
    fn king_safely_flees(board: &dyn BoardTrait, m: &Move) -> bool {
        //@todo : use attack map to determine if king can flee there
        let king = board.get_piece_at(&m.from).unwrap();
        if king.piece_type != PieceType::King {
            return false;
        }
        let mut fresh_board = board.clone();
        fresh_board.make_move_mut(m);
        let potential_checks = gen_pseudo_legal_moves(&*fresh_board, king.color.opposite());
        get_checks(&*fresh_board, king.color, &potential_checks).len() == 0
    }
    possible_moves
        .into_iter()
        .filter(|m| {
            // the move needs to resolve all the checks
            // @todo : rewrite this to optimize it a bit
            checks.iter().all(|check| {
                println!("{:?}", m);
                let piece = board.get_piece_at(&m.from).unwrap();
                let path_option = path::get_path_to(&check.from, &check.to);
                let is_interposable = path_option.is_some();
                // the king itself can not interpose
                if (is_interposable && m.piece != PieceType::King) {
                    let path = path_option.unwrap();
                    let interpose_path = &path[1..(path.len() - 1)];
                    if interpose_path.iter().any(|coordinate| coordinate == &m.to) {
                        return true;
                    }
                }
                if check.from == m.to {
                    return true;
                } else if king_safely_flees(board, &m) {
                    return true;
                }
                return false;
            })
        })
        .map(|m| m.clone())
        .collect()
}

// @todo pass attacker moves so you only calculate it once
// @todo : write out the algorithm for this
pub fn gen_legal_moves(board: &dyn BoardTrait, color: Color) -> Vec<Move> {
    // generate pseudo legal moves
    let moves = gen_pseudo_legal_moves(board, color);

    // generate places where the opponent can attack to determine if there's a check
    // @todo : write a custom function for this that works out from where the king is
    let potential_checks = gen_pseudo_legal_moves(board, color.opposite());

    // are pseudo legal moves and attack vectors the same ?
    // check castling moves
    // NOTE : THE KING CAN NOT PASS THROUGH SQUARES BEING TARGETED BY ENEMY PIECES BUT THE ROOK CAN

    // if in check do any of these moves resolve it ?
    // let checks = find_checks_from_moves(board, &moves, color.opposite());
    let checks = get_checks(board, color, &potential_checks);
    if checks.len() > 0 {
        return find_moves_to_resolve_check(board, &checks, &moves);

        let mut new_board = board.clone();
        return moves
            .into_iter()
            .filter(|m| {
                if board.get_piece_at(&m.from).is_none() {
                    print_board(board);
                    println!("{:?}", m);
                    panic!("attempting to move a piece that's not there");
                }

                let color = board.get_piece_at(&m.from).unwrap().color;
                new_board.make_move_mut(m);
                let has_checks = get_checks(&*new_board, color, &potential_checks).len() == 0;
                new_board.unmake_move_mut(m);
                has_checks
            })
            .collect();
    } else {
        let pinned_pieces = find_pinned_pieces(board, color);

        fn is_pinned(piece: &Piece, pinned_pieces: &Vec<Pin>) -> bool {
            pinned_pieces.iter().any(|p| p.pinned_piece == piece)
        }
        fn get_pin<'a, 'b>(piece: &'b Piece, pinned_pieces: &'a Vec<Pin>) -> Option<&'a Pin<'a>> {
            pinned_pieces.iter().find(|p| p.pinned_piece == piece)
        }

        // if not in check, will this move expose my king ?
        moves
            .into_iter()
            .filter(|m| {
                if board.get_piece_at(&m.from).is_none() {
                    print_board(board);
                    println!("{:?}", m);
                    panic!("attempting to move a piece that's not there");
                }
                // is this piece pinned ?
                let piece = board.get_piece_at(&m.from).unwrap();
                if is_pinned(&piece, &pinned_pieces) {
                    let pin = get_pin(&piece, &pinned_pieces).unwrap();
                    // check if the pinned piece can move here
                    return pin.can_move_to.iter().any(|c| c == &m.to);
                }
                true
            })
            .collect()
    }
    // add pgn notation
}

// ignores enemy captures
pub fn gen_attack_vectors(board: &dyn BoardTrait, color: Color) -> Vec<Move> {
    board
        .get_all_pieces(color)
        .into_iter()
        .map(|piece| gen_vectors_for(board, piece))
        .flatten()
        .collect()
}

// PSEUDO LEGAL MOVE GENERATION
// determines what moves the pieces can legally do
// does not check whether the player can legally do that move
// for instance : no checking pins or checks , etc...
pub fn gen_pseudo_legal_moves(board: &dyn BoardTrait, color: Color) -> Vec<Move> {
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
