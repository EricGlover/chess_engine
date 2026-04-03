use crate::bit_board::BitBoard;
use crate::board::{CastlingRights, Color, Coordinate, Piece, PieceType};
use crate::chess_notation::pgn::Game;

// might be worthwhile to add pointers to things
// or add a list of moves
#[derive(Debug, Eq, PartialEq)]
pub struct GameState {
    player_to_move: Color,
    white_castling_rights: CastlingRights,
    black_castling_rights: CastlingRights,
    en_passant_target: Option<Coordinate>,
    half_move_clock: u32,
    full_move_number: u32,
    pub board: BitBoard,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player_to_move: Color::White,
            white_castling_rights: CastlingRights::new(true, true),
            black_castling_rights: CastlingRights::new(true, true),
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 1,
            board: BitBoard::new(),
        }
    }
    pub fn make_game_state(
        player_to_move: Color,
        white_can_castle_king_side: bool,
        white_can_castle_queen_side: bool,
        black_can_castle_king_side: bool,
        black_can_castle_queen_side: bool,
        en_passant_target: Option<Coordinate>,
        half_move_clock: u32,
        full_move_number: u32,
        board: BitBoard,
    ) -> GameState {
        GameState {
            player_to_move,
            white_castling_rights: CastlingRights::new(
                white_can_castle_king_side,
                white_can_castle_queen_side,
            ),
            black_castling_rights: CastlingRights::new(
                black_can_castle_king_side,
                black_can_castle_queen_side,
            ),
            en_passant_target,
            half_move_clock,
            full_move_number,
            board,
        }
    }
    pub fn starting_game() -> GameState {
        GameState {
            player_to_move: Color::White,
            white_castling_rights: CastlingRights::new(true, true),
            black_castling_rights: CastlingRights::new(true, true),
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 1,
            board: BitBoard::new(),
        }
    }

    pub fn get_castling_rights(&self, color: Color) -> &CastlingRights {
        return match color {
            Color::White => &self.white_castling_rights,
            Color::Black => &self.black_castling_rights,
        };
    }

    pub fn get_castling_rights_changes_if_piece_moves(
        &self,
        piece: &Piece,
    ) -> Option<CastlingRights> {
        let current = match piece.color {
            Color::White => self.white_castling_rights,
            Color::Black => self.black_castling_rights,
        };
        if current.none() {
            None
        } else if piece.piece_type == PieceType::King {
            Some(CastlingRights::new(
                current.king_side(),
                current.queen_side(),
            ))
        } else if piece.piece_type == PieceType::Rook {
            // which rook bro ?
            if current.king_side() && piece.at().unwrap().x() == 8 {
                Some(CastlingRights::new(true, false))
            } else if current.queen_side() && piece.at().unwrap().x() == 1 {
                Some(CastlingRights::new(false, true))
            } else {
                None
            }
        } else {
            None
        }
    }
}
