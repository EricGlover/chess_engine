// works with bitboards
use crate::bit_board::BitBoard;
use crate::board::{Color, Coordinate};
use crate::board::{HIGH_X, HIGH_Y, LOW_X, LOW_Y};

/**
 * @todo 
one square move, two square move, capturing diagonally forward, pawn promotion, en passant
**/
pub fn gen_pawn_moves(board: &BitBoard, piece: &Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    return moves;
}

/** HELPER FUNCTIONS  */
fn square_is_empty(board: &BitBoard, at: &Coordinate) -> bool {
    board.is_piece_at_coordinate(at)
}

// if square is off board || square has friendly price => false
fn square_occupiable_by(board: &BitBoard, at: &Coordinate, color: Color) -> bool {
    if !is_on_board(at) {
        return false;
    }
    board.get_piece_at(at).map_or(true, |p| p.color != color)
}

fn has_enemy_piece(board: &BitBoard, at: &Coordinate, own_color: Color) -> bool {
    if !is_on_board(at) {
        return false;
    }
    board
        .get_piece_at(at)
        .map_or(false, |piece| piece.color == own_color.opposite())
}

fn is_on_board(c: &Coordinate) -> bool {
    c.x() >= LOW_X && c.x() <= HIGH_X && c.y() >= LOW_Y && c.y() <= HIGH_Y
}
