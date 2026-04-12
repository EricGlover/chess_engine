// use test::Bencher;

mod chess_move;
pub mod move_log;
pub mod path;
pub mod pin;
pub mod plmg;
pub mod pseudo_legal_move_generator;

pub use chess_move::*;

use crate::bit_board::BitBoard;
use crate::board::*;
use crate::board_console_printer;
use crate::board_console_printer::print_board;
use crate::chess_notation::pgn::Game;
use crate::game;
use crate::game_state;
use crate::game_state::GameState;
use crate::move_generator::path::*;
use crate::move_generator::plmg::gen_attacks_for_square;
use crate::move_generator::plmg::ROOK_ATTACKS;
use crate::move_generator::pseudo_legal_move_generator::*;
use move_log::MoveLog;
use pin::Pin;
use std::fmt;
use std::fmt::Formatter;
use std::iter::Iterator;

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

// later this could become more generalized, but right now
// it's only used for finding pinned pieces
// same for attack vectors
// ignores blocking pieces
// don't ignore same color pieces that are in the way
fn find_attacking_pieces<'a>(
    game_state: &'a GameState,
    attackers_color: Color,
    attack_coordinate: &Coordinate,
) -> Vec<&'a Piece> {
    let mut attacking_pieces: Vec<&Piece> = vec![];
    // how to make sure the pieces returned are unique ?
    // pieces can't attack the same square twice , so we're good
    let idx = BitBoard::coordinate_to_idx(*attack_coordinate);
    let candidate_indices = plmg::get_slider_pieces_indices_attacking_idx(
        game_state.get_board_ref(),
        idx,
        attackers_color,
    );
    return candidate_indices
        .iter()
        .map(|idx| {
            let c = BitBoard::idx_to_coordinate(*idx);
            let piece = game_state.get_piece_at(&c).unwrap();
            return plmg::gen_vectors_for(game_state, &piece);
        })
        .flatten()
        .filter(|m| {
            return &m.to == attack_coordinate;
        })
        .map(|m| {
            return game_state.get_piece_at(&m.from).unwrap();
        })
        .collect();
}

// is path empty
// if only one piece get the piece
// if only one piece find the path of coordinates it can move to
// from is attacker
// to is king
pub fn get_attack_path_from_to<'a>(
    game_state: &'a GameState,
    from: &Coordinate,
    to: &Coordinate,
) -> (bool, Option<&'a Piece>, u64) {
    let mut is_empty = true;
    let mut pinned_piece = None;
    let from_idx = BitBoard::coordinate_to_idx(*from);
    let to_idx = BitBoard::coordinate_to_idx(*to);
    let start_bit = BitBoard::coordinate_to_bit(*from);
    let end_bit = BitBoard::coordinate_to_bit(*to);
    let board = game_state.get_board_ref();
    let all_pieces_board = board.get_piece_board();
    let mut path = 0;
    let mut pieces_found = 0;
    //rook path ?
    let mut attack_path: u64 = 0;
    let line = BitBoard::get_rook_path_for_bits(start_bit, end_bit);
    if line > 0 {
        attack_path = line;
    }
    let diagonal = BitBoard::get_diagonal_for_bits(start_bit, end_bit);
    if diagonal > 0 {
        attack_path = diagonal;
    }
    if to_idx > from_idx {
        let up_board: u64 = !(start_bit - 1);
        let down_board = end_bit - 1;
        path = up_board & attack_path & down_board;
        pieces_found = all_pieces_board & path;
    } else {
        // we want the right shifted stuff , include the attacker
        let up_board: u64 = !(end_bit - 1) ^ end_bit;
        let down_board = (start_bit - 1) | start_bit;

        path = up_board & attack_path & down_board;
        pieces_found = all_pieces_board & path;
    }
    if u64::count_ones(pieces_found) == 1 {
        return (true, None, 0);
    } else if u64::count_ones(pieces_found) == 2 {
        let other_piece_bit = pieces_found & !start_bit;
        let piece = game_state.get_piece_at_idx(BitBoard::get_index_of_bit(other_piece_bit));
        return (false, piece, path);
    } else {
        return (is_empty, pinned_piece, 0);
    }
}

fn find_pinned_pieces(game_state: &GameState, defender_color: Color) -> Vec<Pin> {
    let attacker_color = defender_color.opposite();
    //@todo generate legal? moves

    // get defender king
    let king = game_state.get_king(defender_color).unwrap();

    // get pieces that can attack king (ignoring our own pieces)
    let attacking_pieces = find_attacking_pieces(game_state, attacker_color, &king.at().unwrap());

    // use piece.at and king.at to generate a range of Coordinates where pieces can interpose at
    let mut pins = vec![];
    for &attacking_piece in attacking_pieces.iter() {
        // if piece is knight skip
        // if piece is one square away from the king then skip
        // assume King and Pawn can't attack the enemy king / from more than a square away
        let t = attacking_piece.piece_type;
        if t == PieceType::Queen || t == PieceType::Bishop || t == PieceType::Rook {
            let from = attacking_piece.at().unwrap();
            let to = king.at().unwrap();
            let (is_empty, piece_opt, path) = get_attack_path_from_to(game_state, from, to);
            if !is_empty {
                let pinned_piece = piece_opt.unwrap();
                pins.push(Pin {
                    pinned_piece,
                    pinned_at: *pinned_piece.at().unwrap(),
                    pinned_by: attacking_piece,
                    pinned_to: king,
                    can_move_to_board: path,
                })
            }
        }
    }
    pins
}

pub fn generate_checks(game_state: &GameState, color_being_checked: Color) -> Vec<Move> {
    let king = game_state.get_king(color_being_checked).unwrap();
    let idx = BitBoard::coordinate_to_idx(*king.at().unwrap());
    return gen_attacks_for_square(game_state, idx, color_being_checked);
}

// @todo : sort this nonsense out
// @todo: consider using a board_get_all_pieces_ref instead of cloning the pieces
//@todo : find_checks_from_moves()
// @todo: piece lists for fast lookups

pub fn find_checks<'a, I>(
    game_state: &GameState,
    color_being_checked: Color,
    moves: I,
) -> Vec<&'a Move>
where
    I: Iterator<Item = &'a Move>,
{
    let m = vec![];
    if let Some(king) = game_state.get_king(color_being_checked) {
        if let Some(king_at) = king.at() {
            return moves.filter(|&m| m.to == *king_at).collect();
        }
    }
    return m;
}

// get checks against color
pub fn get_checks(game_state: &GameState, color_being_checked: Color) -> Vec<Move> {
    let moves = gen_pseudo_legal_moves(game_state, color_being_checked.opposite());
    let king_pieces = game_state.get_pieces(color_being_checked, PieceType::King);
    if king_pieces.len() == 0 {
        return vec![];
    }
    let king = king_pieces.get(0).unwrap();
    let at = king.at().unwrap();
    moves.into_iter().filter(|m| &m.to == at).collect()
}

fn find_checks_from_moves<'a>(
    game_state: &GameState,
    moves: &'a Vec<Move>,
    color_being_checked: Color,
) -> Vec<&'a Move> {
    let king_pieces = game_state.get_pieces(color_being_checked, PieceType::King);
    if king_pieces.len() == 0 {
        return vec![];
    }
    let king = king_pieces.get(0).unwrap();
    let at = king.at().unwrap();
    moves.into_iter().filter(|&m| &m.to == at).collect()
}

fn king_escapes(game_state: &GameState, m: &Move) -> bool {
    let mut m = m.clone();
    let piece = game_state.get_piece_at(&m.from).unwrap();
    if piece.piece_type != PieceType::King {
        println!("NOT KING");
        return false;
    }
    let mut fresh_board = &mut game_state.clone_to_game_state();
    fresh_board.make_move_mut(&mut m);
    let checks = generate_checks(fresh_board, piece.color);
    println!("{}", checks.len());
    return checks.len() == 0;
}

// fn find_moves_to_resolve_check<'a>(board: &dyn BoardTrait, checks: &Vec<Move>, possible_moves: &'a Vec<Move>) -> Vec<&'a Move> {
fn find_moves_to_resolve_check(
    game_state: &GameState,
    checks: &Vec<Move>,
    possible_moves: &Vec<Move>,
    pins: Option<&Vec<Pin>>,
    color_being_checked: Color,
) -> Vec<Move> {
    let mut moves: Vec<Move> = possible_moves.iter().map(|&m| m.clone()).collect();
    let king = game_state.get_king(color_being_checked).unwrap();
    let king_at = king.at().unwrap();
    

    // if 2 checks then the king must flee
    if checks.len() >= 2 {
        // generate safe king moves
        return moves
            .into_iter()
            .filter(|&m| {
                if &m.from == king_at {
                    return king_escapes(game_state, &m);
                } else {
                    return false;
                }
            })
            .collect();
    }

    // if it's only one check then you can interpose, flee, or capture
    if checks.len() == 1 {
        let check_move = checks.get(0).unwrap();
        let check_from = check_move.from;
        let check_to = check_move.to;
        let path = path::make_path_bit_board(&check_from, &check_to, true, false);
        let are_pins = pins.is_some();
        let pins = pins.unwrap();
        // is the attacker defended?
        let checker_idx = BitBoard::coordinate_to_idx(check_from);
        let checker_is_defended = plmg::gen_attacks_for_square(game_state, checker_idx, color_being_checked.opposite()).len() > 0;
        return moves
            .into_iter()
            .filter(|m| {
                let is_capture = check_from == m.to;
                if &m.from == king_at {
                    if m.is_king_side_castle() || m.is_queen_side_castle() {
                        return false;
                    }
                    if !checker_is_defended && is_capture {
                        return true;
                    } else {
                        return king_escapes(game_state, m);
                    }
                }
                let to_bit = BitBoard::coordinate_to_bit(m.to);
                // if moving piece is not the king
                // is the piece pinned to the king already (from some other direction) ?
                if are_pins {
                    let piece = game_state.get_piece_at(&m.from).unwrap();
                    if is_pinned(piece, pins) {
                        return false;
                    }
                }
                let is_interposing_move = BitBoard::bit_on_bit_board(to_bit, path);
                return is_interposing_move || is_capture;
            })
            .collect();
    }
    return moves;
}

fn find_moves_to_resolve_check_brute_force<'a>(
    game_state: &GameState,
    checks: Vec<&Move>,
    possible_moves: Vec<&'a Move>,
    color: Color,
) -> Vec<&'a Move> {
    let mut new_game_state = game_state.clone_to_game_state();
    return possible_moves
        .into_iter()
        .filter(|&m| {
            let mut m_clone = m.clone();
            // @todo pins
            if m.is_king_side_castle() || m.is_queen_side_castle() {
                return false;
            }
            if game_state.get_piece_at(&m.from).is_none() {
                // board_console_printer::print_bit_board(&game_state.get_board());
                println!("{:?}", m);
                panic!("attempting to move a piece that's not there");
            }
            new_game_state.make_move_mut(&mut m_clone);
            let enemy_moves = gen_pseudo_legal_moves(&new_game_state, color.opposite());
            let remaining_checks = find_checks(&new_game_state, color, enemy_moves.iter());
            let no_checks = remaining_checks.len() == 0;
            // println!("{}", m);
            // println!("{}", remaining_checks.len());
            new_game_state.unmake_move_mut(&mut m_clone);
            return no_checks;
        })
        .collect();

    // return *possible_moves;
}

fn is_pinned(piece: &Piece, pinned_pieces: &Vec<Pin>) -> bool {
    let at = *piece.at().unwrap();
    pinned_pieces.iter().any(|p| p.pinned_at == at)
}
fn get_pin<'a, 'b>(piece: &'b Piece, pinned_pieces: &'a Vec<Pin>) -> Option<&'a Pin<'a>> {
    let at = *piece.at().unwrap();
    pinned_pieces.iter().find(|p| p.pinned_at == at)
}

// @todo pass attacker moves so you only calculate it once
pub fn gen_legal_moves(game_state: &GameState, color: Color) -> Vec<Move> {
    // generate enemy moves
    // let enemy_moves = gen_pseudo_legal_moves(game_state, color.opposite());
    // look for pins
    let pinned_pieces = find_pinned_pieces(game_state, color);
    // look for checks
    let checks = generate_checks(game_state, color);
    let mut moves = gen_pseudo_legal_moves(game_state, color);

    if checks.len() > 0 {
        let resolve_checks_moves = find_moves_to_resolve_check(game_state, &checks, &moves, Some(&pinned_pieces), color);
        return resolve_checks_moves;
    }

    // deal with castling through check....

    // if not in check, will this move expose my king ?
    return moves
        .into_iter()
        .filter(|m| {
            let opt = game_state.get_piece_at(&m.from);
            if opt.is_none() {
                // board_console_printer::print_bit_board(&game_state.get_board());
                println!("{:?}", m);
                panic!("attempting to move a piece that's not there");
            }
            // is this piece pinned ?
            let piece = opt.unwrap();
            if is_pinned(&piece, &pinned_pieces) {
                let pin = get_pin(&piece, &pinned_pieces).unwrap();
                // check if the pinned piece can move here
                let to_bit = BitBoard::coordinate_to_bit(m.to);
                return BitBoard::bit_on_bit_board(to_bit, pin.can_move_to_board);
            }
            if m.is_king_side_castle() || m.is_queen_side_castle() {

            }
            true
        })
        .collect();
    // add pgn notation
}

// @todo : write tests and benchmarks
// ignores enemy captures
pub fn gen_attack_vectors(game_state: &GameState, color: Color) -> Vec<Move> {
    let mut vector_moves: Vec<Move> = game_state
        .get_pieces(color, PieceType::Bishop)
        .into_iter()
        .map(|piece| plmg::gen_vectors_for(game_state, &piece))
        .flatten()
        .collect();
    game_state
        .get_pieces(color, PieceType::Rook)
        .into_iter()
        .map(|piece| plmg::gen_vectors_for(game_state, &piece))
        .flatten()
        .for_each(|m| vector_moves.push(m));
    game_state
        .get_pieces(color, PieceType::Queen)
        .into_iter()
        .map(|piece| plmg::gen_vectors_for(game_state, &piece))
        .flatten()
        .for_each(|m| vector_moves.push(m));
    return vector_moves;
}

// PSEUDO LEGAL MOVE GENERATION
// determines what moves the pieces can legally do
// does not check whether the player can legally do that move
// for instance : no checking pins or checks , etc...
pub fn gen_pseudo_legal_moves(game_state: &GameState, color: Color) -> Vec<Move> {
    game_state
        .get_all_pieces(color)
        .into_iter()
        .map(|p| plmg::gen_moves_for(game_state, &p))
        .flatten()
        .collect()
}

#[cfg(test)]
mod bench {
    use super::*;
    use crate::ai::{Ai, AiSearch};
    use crate::chess_notation::fen_reader;
    use crate::chess_notation::fen_reader::*;
    use crate::move_generator::chess_move::MoveType;
    use test::{black_box, Bencher};

    #[bench]
    fn bench_perft(b: &mut Bencher) {
        let mut game_state = GameState::starting_game();
        let mut ai = Ai::new_with_search(Color::White, AiSearch::Minimax);
        b.iter(|| {
            // ai.make_move(&board, Some(0));
            // ai.make_move(&board, Some(1));
            // ai.make_move(&board, Some(2));
            ai.make_move(&mut game_state, Some(3));
            // ai.make_move(&board, Some(4));
            // ai.make_move(&board, Some(5));
        })
    }

    #[bench]
    fn bench_find_checks(b: &mut Bencher) {
        let game_state = GameState::starting_game();
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        b.iter(|| {
            let checks = find_checks(&game_state, Color::Black, w_moves.iter());
            let checks = find_checks(&game_state, Color::White, b_moves.iter());
        });
        let game_state = fen_reader::make_game_state(fen_reader::BLACK_IN_CHECK);
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        b.iter(|| {
            let checks = find_checks(&game_state, Color::Black, w_moves.iter());
            let checks = find_checks(&game_state, Color::White, b_moves.iter());
        });
        let game_state = fen_reader::make_game_state(fen_reader::WHITE_IN_CHECK);
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        b.iter(|| {
            let checks = find_checks(&game_state, Color::Black, w_moves.iter());
            let checks = find_checks(&game_state, Color::White, b_moves.iter());
        });
    }

    #[bench]
    fn bench_get_checks(b: &mut Bencher) {
        let game_state = GameState::starting_game();
        b.iter(|| {
            let checks = get_checks(&game_state, Color::Black);
            let checks = get_checks(&game_state, Color::White);
        });
        let game_state = fen_reader::make_game_state(fen_reader::BLACK_IN_CHECK);
        b.iter(|| {
            let checks = get_checks(&game_state, Color::Black);
            let checks = get_checks(&game_state, Color::White);
        });
        let game_state = fen_reader::make_game_state(fen_reader::WHITE_IN_CHECK);
        b.iter(|| {
            let checks = get_checks(&game_state, Color::Black);
            let checks = get_checks(&game_state, Color::White);
        });
    }

    #[bench]
    fn bench_gen_checks(b: &mut Bencher) {
        let game_state = GameState::starting_game();
        b.iter(|| {
            let checks = generate_checks(&game_state, Color::Black);
            let checks = generate_checks(&game_state, Color::White);
        });

        let game_state = fen_reader::make_game_state(fen_reader::BLACK_IN_CHECK);
        b.iter(|| {
            let checks = generate_checks(&game_state, Color::Black);
            let checks = generate_checks(&game_state, Color::White);
        });

        let game_state = fen_reader::make_game_state(fen_reader::WHITE_IN_CHECK);
        b.iter(|| {
            let checks = generate_checks(&game_state, Color::Black);
            let checks = generate_checks(&game_state, Color::White);
        });
    }

    #[bench]
    fn bench_find_moves_to_resolve_check(b: &mut Bencher) {
        let game_state = GameState::starting_game();
        let white_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let black_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let white_checks = generate_checks(&game_state, Color::Black);
        let black_checks = generate_checks(&game_state, Color::White);
        let black_pins = find_pinned_pieces(&game_state, Color::Black);
        let white_pins = find_pinned_pieces(&game_state, Color::White);
        b.iter(|| {
            let checks =
                find_moves_to_resolve_check(&game_state, &white_checks, &black_moves, Some(&black_pins), Color::Black);
            let checks =
                find_moves_to_resolve_check(&game_state, &black_checks, &white_moves, Some(&white_pins), Color::White);
        });

        let game_state = fen_reader::make_game_state(fen_reader::BLACK_IN_CHECK);
        let white_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let black_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let white_checks = generate_checks(&game_state, Color::Black);
        let black_checks = generate_checks(&game_state, Color::White);
        let black_pins = find_pinned_pieces(&game_state, Color::Black);
        let white_pins = find_pinned_pieces(&game_state, Color::White);
        b.iter(|| {
            let checks =
                find_moves_to_resolve_check(&game_state, &white_checks, &black_moves, Some(&black_pins), Color::Black);
            let checks =
                find_moves_to_resolve_check(&game_state, &black_checks, &white_moves, Some(&white_pins), Color::White);
        });

        let game_state = fen_reader::make_game_state(fen_reader::WHITE_IN_CHECK);
        let white_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let black_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let white_checks = generate_checks(&game_state, Color::Black);
        let black_checks = generate_checks(&game_state, Color::White);
        let black_pins = find_pinned_pieces(&game_state, Color::Black);
        let white_pins = find_pinned_pieces(&game_state, Color::White);
        b.iter(|| {
            let checks =
                find_moves_to_resolve_check(&game_state, &white_checks, &black_moves, Some(&black_pins), Color::Black);
            let checks =
                find_moves_to_resolve_check(&game_state, &black_checks, &white_moves, Some(&white_pins), Color::White);
        });
    }

    #[bench]
    fn bench_find_moves_to_resolve_check_brute_force(b: &mut Bencher) {
        let game_state = GameState::starting_game();
        let white_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let black_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let white_checks = generate_checks(&game_state, Color::Black);
        let black_checks = generate_checks(&game_state, Color::White);
        b.iter(|| {
            let checks = find_moves_to_resolve_check_brute_force(
                &game_state,
                white_checks.iter().collect(),
                black_moves.iter().collect(),
                Color::Black,
            );
            let checks = find_moves_to_resolve_check_brute_force(
                &game_state,
                black_checks.iter().collect(),
                white_moves.iter().collect(),
                Color::White,
            );
        });

        let game_state = fen_reader::make_game_state(fen_reader::BLACK_IN_CHECK);
        let white_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let black_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let white_checks = generate_checks(&game_state, Color::Black);
        let black_checks = generate_checks(&game_state, Color::White);
        b.iter(|| {
            let checks = find_moves_to_resolve_check_brute_force(
                &game_state,
                white_checks.iter().collect(),
                black_moves.iter().collect(),
                Color::Black,
            );
            let checks = find_moves_to_resolve_check_brute_force(
                &game_state,
                black_checks.iter().collect(),
                white_moves.iter().collect(),
                Color::White,
            );
        });

        let game_state = fen_reader::make_game_state(fen_reader::WHITE_IN_CHECK);
        let white_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let black_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let white_checks = generate_checks(&game_state, Color::Black);
        let black_checks = generate_checks(&game_state, Color::White);
        b.iter(|| {
            let checks = find_moves_to_resolve_check_brute_force(
                &game_state,
                white_checks.iter().collect(),
                black_moves.iter().collect(),
                Color::Black,
            );
            let checks = find_moves_to_resolve_check_brute_force(
                &game_state,
                black_checks.iter().collect(),
                white_moves.iter().collect(),
                Color::White,
            );
        });
    }

    #[bench]
    fn bench_gen_find_pinned_pieces(b: &mut Bencher) {
        let white_bishop_pinned =
            "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPPBPPPP/RN1QKB1R b KQkq - 3 3";
        let game_state = fen_reader::make_game_state(white_bishop_pinned);
        b.iter(|| {
            let mut pins = find_pinned_pieces(&game_state, Color::White);
        })
    }

    #[bench]
    fn bench_gen_attack_vectors(b: &mut Bencher) {
        let black_mates = "rnb1k1nr/pp2pp1p/Q5pb/2pp4/2PP4/N7/PP1qPPPP/R3KBNR w KQkq - 0 7";
        let game_state = fen_reader::make_game_state(black_mates);
        b.iter(|| {
            let moves = gen_attack_vectors(&game_state, Color::White);
        })
    }

    #[bench]
    fn bench_gen_attack_vectors2(b: &mut Bencher) {
        let game_state = GameState::starting_game();
        b.iter(|| {
            for i in 0..100 {
                black_box({
                    let vector_moves = gen_attack_vectors(&game_state, Color::Black);
                })
            }
        });
        let white_bishop_pinned =
            "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPPBPPPP/RN1QKB1R b KQkq - 3 3";
        let game_state = fen_reader::make_game_state(white_bishop_pinned);
        b.iter(|| {
            for i in 0..100 {
                black_box({
                    let vector_moves = gen_attack_vectors(&game_state, Color::Black);
                })
            }
        });
    }

    #[bench]
    fn bench_gen_pseudo_legal_moves(b: &mut Bencher) {
        let black_mates = "rnb1k1nr/pp2pp1p/Q5pb/2pp4/2PP4/N7/PP1qPPPP/R3KBNR w KQkq - 0 7";
        let game_state = fen_reader::make_game_state(black_mates);
        let init_state = GameState::starting_game();
        b.iter(|| {
            let moves = gen_pseudo_legal_moves(&game_state, Color::White);
            let moves = gen_pseudo_legal_moves(&game_state, Color::Black);
            let moves = gen_pseudo_legal_moves(&init_state, Color::White);
            let moves = gen_pseudo_legal_moves(&init_state, Color::Black);
        })
    }

    #[bench]
    fn gen_pseudo_legal_moves_initial_board_white(b: &mut Bencher) {
        let init_state = GameState::starting_game();
        b.iter(|| {
            let moves = gen_pseudo_legal_moves(&init_state, Color::White);
        })
    }

    #[bench]
    fn bench_gen_legal_moves_mate(b: &mut Bencher) {
        let black_mates = "rnb1k1nr/pp2pp1p/Q5pb/2pp4/2PP4/N7/PP1qPPPP/R3KBNR w KQkq - 0 7";
        let game_state = fen_reader::make_game_state(black_mates);
        let init_state = GameState::starting_game();
        b.iter(|| {
            let moves = gen_legal_moves(&game_state, Color::White);
            let moves = gen_legal_moves(&game_state, Color::Black);
        })
    }
    #[bench]
    fn bench_gen_legal_moves_start(b: &mut Bencher) {
        let init_state = GameState::starting_game();
        b.iter(|| {
            let moves = gen_legal_moves(&init_state, Color::White);
            let moves = gen_legal_moves(&init_state, Color::Black);
        })
    }
    #[bench]
    fn bench_gen_legal_moves_open_pos(b: &mut Bencher) {
        let open_pos = "2r3k1/pp3ppp/1qrb4/3n1bB1/3QN1n1/1B3N1P/PP3PP1/2R1R1K1 b - - 2 20";
        let game_state = fen_reader::make_game_state(open_pos);
        b.iter(|| {
            let moves = gen_legal_moves(&game_state, Color::White);
            let moves = gen_legal_moves(&game_state, Color::Black);
        })
    }
    #[bench]
    fn bench_gen_legal_moves_white_in_check(b: &mut Bencher) {
        let white_in_check = "2r3k1/pp3pp1/1qrb3p/3n1bB1/3QN3/1B3N1P/PP3PPn/2R1RK2 w - - 2 22";
        let game_state = fen_reader::make_game_state(white_in_check);
        b.iter(|| {
            let moves = gen_legal_moves(&game_state, Color::White);
            let moves = gen_legal_moves(&game_state, Color::Black);
        })
    }

    #[bench]
    fn bench_gen_legal_moves_original_benchmark(b: &mut Bencher) {
        let black_mates = "rnb1k1nr/pp2pp1p/Q5pb/2pp4/2PP4/N7/PP1qPPPP/R3KBNR w KQkq - 0 7";
        let game_state = fen_reader::make_game_state(black_mates);
        let init_state = GameState::starting_game();
        b.iter(|| {
            let moves = gen_legal_moves(&game_state, Color::White);
            let moves = gen_legal_moves(&game_state, Color::Black);
            let moves = gen_legal_moves(&init_state, Color::White);
            let moves = gen_legal_moves(&init_state, Color::Black);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::{Ai, AiSearch};
    use crate::chess_notation::fen_reader;
    use crate::chess_notation::fen_reader::*;
    use crate::move_generator::chess_move::MoveType;
    use test::Bencher;

    #[test]
    fn test_gen_pseudo_legal_moves() {}

    #[test]
    fn perft_initial_position() {
        let mut game_state = GameState::starting_game();
        let mut ai = Ai::new_with_search(Color::White, AiSearch::Minimax);
        ai.make_move(&mut game_state, Some(0));
        assert_eq!(ai.minimax_calls(), 1, "1 node visited at depth 0");
        ai.make_move(&mut game_state, Some(1));
        assert_eq!(ai.minimax_calls(), 20, "20 nodes visited at depth 1");
        ai.make_move(&mut game_state, Some(2));
        assert_eq!(ai.minimax_calls(), 400, "400 nodes visited at depth 2");
        ai.make_move(&mut game_state, Some(3));
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
    fn test_get_attack_path_from_to() {
        // black knight is pinned
        let bishop_fen = "r1bqkbnr/pppnpppp/3p4/1B6/3P4/4P3/PPP2PPP/RNBQK1NR b KQkq - 2 3";
        let game_state = fen_reader::make_game_state(bishop_fen);
        let b5 = Coordinate::new(2, 5);
        let e8 = Coordinate::new(5, 8);
        let d8 = Coordinate::new(4, 7);
        let (is_empty, piece, path) = get_attack_path_from_to(&game_state, &b5, &e8);
        assert_eq!(is_empty, false);
        assert_eq!(piece.is_some(), true);
        assert_eq!(piece.unwrap().at().unwrap(), &d8);
        assert_eq!(u64::count_ones(path), 3);

        // white knight is pinned
        let bishop_fen = "rnbqk1nr/pppp1ppp/8/4p3/1b1P4/8/PPPNPPPP/R1BQKBNR w KQkq - 2 3";
        let game_state = fen_reader::make_game_state(bishop_fen);
        let b4 = Coordinate::new(2, 4);
        let e1 = Coordinate::new(5, 1);
        let d2 = Coordinate::new(4, 2);
        let (is_empty, piece, path) = get_attack_path_from_to(&game_state, &b4, &e1);
        println!(
            "{} {} {}",
            is_empty,
            piece.map_or("NONE", |p| "piece"),
            u64::count_ones(path)
        );
        assert_eq!(is_empty, false);
        assert_eq!(piece.is_some(), true);
        assert_eq!(piece.unwrap().at().unwrap(), &d2);
        assert_eq!(u64::count_ones(path), 3);
        // white bishop pinned by rook
        let rook_fen = "rnbq2k1/pppp1ppp/5n2/6N1/1b2rP2/8/PPPNB1PP/R1BQK2R b KQ - 1 8";
        let game_state = fen_reader::make_game_state(rook_fen);
        let e4 = Coordinate::new(5, 4);
        let e2 = Coordinate::new(5, 2);
        let (is_empty, piece, path) = get_attack_path_from_to(&game_state, &e4, &e1);
        println!(
            "{} {} {}",
            is_empty,
            piece.map_or("NONE", |p| "piece"),
            u64::count_ones(path)
        );
        assert_eq!(is_empty, false);
        assert_eq!(piece.is_some(), true);
        assert_eq!(piece.unwrap().at().unwrap(), &e2);
        assert_eq!(u64::count_ones(path), 3);
    }
    #[test]
    fn test_find_check() {
        // no checks
        let game_state = GameState::starting_game();
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let checks = find_checks(&game_state, Color::Black, w_moves.iter());
        assert_eq!(checks.len(), 0, "black is not in check");
        let checks = find_checks(&game_state, Color::White, b_moves.iter());
        assert_eq!(checks.len(), 0, "black is not in check");

        // single check
        let game_state = fen_reader::make_game_state(fen_reader::BLACK_IN_CHECK);
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let checks = find_checks(&game_state, Color::Black, w_moves.iter());
        assert_eq!(checks.len(), 1, "black is in check");
        let checks = find_checks(&game_state, Color::White, b_moves.iter());
        assert_eq!(checks.len(), 0, "white is not in check");

        let game_state = fen_reader::make_game_state(fen_reader::WHITE_IN_CHECK);
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let checks = find_checks(&game_state, Color::Black, w_moves.iter());
        assert_eq!(checks.len(), 0, "black is not in check");
        let checks = find_checks(&game_state, Color::White, b_moves.iter());
        assert_eq!(checks.len(), 1, "white is in check");

        // double check
        let fen_double_check = "2R1k1nr/pp2pp1p/3nb1pb/1Qpp4/qPPP4/N7/P3PPPP/R1B1KBN1 b Qk - 2 7";
        let game_state = fen_reader::make_game_state(fen_double_check);
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let checks = find_checks(&game_state, Color::Black, w_moves.iter());
        assert_eq!(checks.len(), 2, "black is in double check");

        // if triple check was legal then Nf7# is allowed (triple check loophole baby lol)
        let fen_double_check_2 = "4r1rk/b5pp/8/4N3/2n5/4K3/1Q6/8 w - - 3 8";
        let game_state = fen_reader::make_game_state(fen_double_check_2);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let checks = find_checks(&game_state, Color::White, b_moves.iter());
        assert_eq!(checks.len(), 2, "white is in double check");

        // check with pinned piece
        let fen_pinned_check = "2R3nr/pp2pp1p/4b1pb/1n1p4/qp2k3/N7/P1Q1PPPP/R1BK1BN1 b - - 2 7";
        let game_state = fen_reader::make_game_state(fen_pinned_check);
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let checks = find_checks(&game_state, Color::Black, w_moves.iter());
        assert_eq!(checks.len(), 1, "black is in check");
    }

    #[test]
    fn test_gen_checks() {
        let game_state = GameState::starting_game();
        let checks = generate_checks(&game_state, Color::Black);
        assert_eq!(checks.len(), 0);
        let checks = generate_checks(&game_state, Color::White);
        assert_eq!(checks.len(), 0);

        //
        let game_state = fen_reader::make_game_state(fen_reader::BLACK_IN_CHECK);
        let checks = generate_checks(&game_state, Color::Black);
        for m in checks.iter() {
            println!("{}", m);
        }
        assert_eq!(checks.len(), 1);
        let checks = generate_checks(&game_state, Color::White);
        assert_eq!(checks.len(), 0);

        //
        let game_state = fen_reader::make_game_state(fen_reader::WHITE_IN_CHECK);
        let checks = generate_checks(&game_state, Color::Black);
        assert_eq!(checks.len(), 0);
        let checks = generate_checks(&game_state, Color::White);
        assert_eq!(checks.len(), 1);
    }

    #[test]
    fn test_find_moves_to_resolve_check_brute_force() {
        // no checks
        let game_state = GameState::starting_game();
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        // both should be 0 checks
        // black's moves
        let checks = find_checks(&game_state, Color::Black, w_moves.iter());
        let moves = find_moves_to_resolve_check_brute_force(
            &game_state,
            checks,
            b_moves.iter().collect(),
            Color::Black,
        );
        assert_eq!(b_moves.len(), moves.len());

        // white's moves
        let checks = find_checks(&game_state, Color::White, b_moves.iter());
        let moves = find_moves_to_resolve_check_brute_force(
            &game_state,
            checks,
            w_moves.iter().collect(),
            Color::White,
        );
        assert_eq!(w_moves.len(), moves.len());

        // single check
        // 6 legal moves
        let game_state = fen_reader::make_game_state(fen_reader::BLACK_IN_CHECK);
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let checks = find_checks(&game_state, Color::Black, w_moves.iter());
        for check in checks.iter() {
            println!("===============CHECK========");
            println!("{:?}", check);
            println!("===============CHECK========");
        }
        for b_move in b_moves.iter() {
            println!("{}", b_move);
        }
        let moves = find_moves_to_resolve_check_brute_force(
            &game_state,
            checks,
            b_moves.iter().collect(),
            Color::Black,
        );
        assert_eq!(moves.len(), 6);

        // single check
        // 1 legal move
        let game_state = fen_reader::make_game_state(fen_reader::WHITE_IN_CHECK);
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let checks = find_checks(&game_state, Color::White, b_moves.iter());
        let moves = find_moves_to_resolve_check_brute_force(
            &game_state,
            checks,
            w_moves.iter().collect(),
            Color::White,
        );
        assert_eq!(moves.len(), 1);

        // double check
        // no legal moves
        let fen_double_check = "2R1k1nr/pp2pp1p/3nb1pb/1Qpp4/qPPP4/N7/P3PPPP/R1B1KBN1 b Qk - 2 7";
        let game_state = fen_reader::make_game_state(fen_double_check);
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let checks = find_checks(&game_state, Color::Black, w_moves.iter());
        let moves = find_moves_to_resolve_check_brute_force(
            &game_state,
            checks,
            b_moves.iter().collect(),
            Color::Black,
        );
        assert_eq!(moves.len(), 0);

        // if triple check was legal then Nf7# is allowed (triple check loophole baby lol)
        // five legal moves
        let fen_double_check_2 = "4r1rk/b5pp/8/4N3/2n5/4K3/1Q6/8 w - - 3 8";
        let game_state = fen_reader::make_game_state(fen_double_check_2);
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let checks = find_checks(&game_state, Color::White, b_moves.iter());
        let moves = find_moves_to_resolve_check_brute_force(
            &game_state,
            checks,
            w_moves.iter().collect(),
            Color::White,
        );
        assert_eq!(moves.len(), 5);

        // check with pinned piece
        // 3 legal moves
        let fen_pinned_check = "2R3nr/pp2pp1p/4b1pb/1n1p4/qp2k3/N7/P1Q1PPPP/R1BK1BN1 b - - 2 7";
        let game_state = fen_reader::make_game_state(fen_pinned_check);
        let w_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        let b_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        let checks = find_checks(&game_state, Color::Black, w_moves.iter());
        let moves = find_moves_to_resolve_check_brute_force(
            &game_state,
            checks,
            b_moves.iter().collect(),
            Color::Black,
        );
        assert_eq!(moves.len(), 3);
    }

    #[test]
    fn test_in_check() {
        let game_state = GameState::starting_game();
        let checks = get_checks(&game_state, Color::Black);
        assert!(checks.len() == 0, "black is not in check");
        let checks = get_checks(&game_state, Color::White);
        assert!(checks.len() == 0, "black is not in check");

        let game_state = fen_reader::make_game_state(fen_reader::BLACK_IN_CHECK);
        let checks = get_checks(&game_state, Color::Black);
        assert!(checks.len() > 0, "black is in check");
        let checks = get_checks(&game_state, Color::White);
        assert!(checks.len() == 0, "white is not in check");

        let game_state = fen_reader::make_game_state(fen_reader::WHITE_IN_CHECK);
        let checks = get_checks(&game_state, Color::Black);
        assert!(checks.len() == 0, "black is not in check");
        let checks = get_checks(&game_state, Color::White);
        assert!(checks.len() > 0, "white is in check");
    }

    #[test]
    fn test_find_moves_to_resolve_check() {
        let mut moves = vec![];
        let white_queen_checks = "rnb1k1nr/pp2pp1p/6pb/1Qpp4/qPPP4/N7/P3PPPP/R1B1KBNR b KQkq - 2 7";

        let game_state = fen_reader::make_game_state(white_queen_checks);
        // knight interpose
        let knight = game_state.get_piece_at(&Coordinate::new(2, 8)).unwrap();
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
        let bishop = game_state.get_piece_at(&Coordinate::new(3, 8)).unwrap();
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
        let queen = game_state.get_piece_at(&Coordinate::new(1, 4)).unwrap();
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
        let king = game_state.get_piece_at(&Coordinate::new(5, 8)).unwrap();
        let king_at = king.at().unwrap();
        moves.push(Move::new(
            king_at.clone(),
            king_at.add(-1, 0),
            king.piece_type,
            MoveType::Move,
            None,
            game_state.get_castling_rights_changes_if_piece_moves(king),
            None,
        ));
        moves.push(Move::new(
            king_at.clone(),
            king_at.add(1, 0),
            king.piece_type,
            MoveType::Move,
            None,
            game_state.get_castling_rights_changes_if_piece_moves(king),
            None,
        ));

        let checks = get_checks(&game_state, Color::Black);
        // let white_moves = gen_pseudo_legal_moves(&game_state, Color::White);
        // let checks = find_checks(&game_state, Color::Black, white_moves);
        let possible_moves = gen_pseudo_legal_moves(&game_state, Color::Black);
        println!("possible moves");
        possible_moves.iter().for_each(|m| println!("{}", m));
        let black_pins = find_pinned_pieces(&game_state, Color::Black);
        let white_pins = find_pinned_pieces(&game_state, Color::White);
        let found_moves =
            find_moves_to_resolve_check(&game_state, &checks, &possible_moves, Some(&black_pins), Color::Black);

        let moves: Vec<&Move> = moves.iter().collect();
        println!("Saving king with !");
        for found_move in found_moves.iter() {
            println!("{}", found_move);
        }
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
    fn test_gen_attack_vectors() {
        let white_bishop_pinned =
            "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPPBPPPP/RN1QKB1R b KQkq - 3 3";
        let game_state = fen_reader::make_game_state(white_bishop_pinned);
        let vector_moves = gen_attack_vectors(&game_state, Color::Black);
        assert_eq!(vector_moves.len(), 13);
    }

    #[test]
    fn test_find_attacking_pieces() {
        let white_bishop_pinned =
            "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPPBPPPP/RN1QKB1R b KQkq - 3 3";
        let game_state = fen_reader::make_game_state(white_bishop_pinned);
        let mut king_pieces = game_state.get_pieces(Color::White, PieceType::King);
        assert!(king_pieces.get(0).is_some(), "king not found");
        let king = king_pieces.remove(0);
        let mut attacking_pieces =
            find_attacking_pieces(&game_state, Color::Black, &king.at().unwrap());
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
    fn test_find_pinned_pieces() {
        // pinned by black bishop, can capture or move 1
        let white_bishop_pinned =
            "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPPBPPPP/RN1QKB1R b KQkq - 3 3";
        let game_state = fen_reader::make_game_state(white_bishop_pinned);
        // diagonal from pinning piece to one space before the king
        // it'd be neat to make diagonal from / to function, and file from / to, and rank from / to
        let mut pins = find_pinned_pieces(&game_state, Color::White);
        assert_eq!(pins.len(), 1, "There is one pin");
        let bishop = game_state.get_piece_at(&Coordinate::new(2, 4)).unwrap();
        let white_bishop = game_state.get_piece_at(&Coordinate::new(4, 2)).unwrap();
        let king = game_state.get_piece_at(&Coordinate::new(5, 1)).unwrap();
        let can_move_to = vec![
            Coordinate::new(2, 4),
            Coordinate::new(3, 3),
            Coordinate::new(4, 2),
        ];

        let found_pin = pins.pop().unwrap();

        assert_eq!(found_pin.pinned_piece, white_bishop);
        assert_eq!(found_pin.pinned_by, bishop);
        assert_eq!(found_pin.pinned_to, king);
        // am I pinned if you're pinned ?
        let pinned_piece_attacks_kings =
            "rnb1k1nr/ppp2qpp/8/B1b1p2Q/3p4/1K2P2P/PPP2PP1/RN3B1R w kq - 0 17";
    }

    #[test]
    fn test_get_checks() {
        let board = fen_reader::make_board(
            "rnb1kbnr/pppp1p1p/4pp2/8/8/3BP3/PPPP1PPP/RNB1K1NR b KQkq - 1 4",
        );
    }

    #[test]
    fn test_king_escapes() {
        let white_queen_checks = "rnb1k1nr/pp2pp1p/6pb/1Qpp4/qPPP4/N7/P3PPPP/R1B1KBNR b KQkq - 2 7";
        let game_state = fen_reader::make_game_state(white_queen_checks);
        let king = game_state.get_king(Color::Black).unwrap();
        let king_moves = plmg::gen_king_moves(king, &game_state);
        let successful_moves: Vec<&Move> = king_moves
            .iter()
            .filter(|&m| king_escapes(&game_state, m))
            .collect();
        assert_eq!(successful_moves.len(), 2);
    }

    #[test]
    fn test_gen_legal_moves_checkmate() {
        let black_mates = "rnb1k1nr/pp2pp1p/Q5pb/2pp4/2PP4/N7/PP1qPPPP/R3KBNR w KQkq - 0 7";
        let game_state = fen_reader::make_game_state(black_mates);
        let moves = gen_legal_moves(&game_state, Color::White);
        println!("{:?}", moves);
        assert_eq!(moves.len(), 0, "White has no moves");
        let white_mates = "2kQ4/pp3p2/4p1p1/7p/4P3/8/PP3PPP/3R2K1 b - - 0 21";
        let game_state = fen_reader::make_game_state(white_mates);
        let moves = gen_legal_moves(&game_state, Color::Black);
        assert_eq!(moves.len(), 0, "Black has no moves");
    }
}
