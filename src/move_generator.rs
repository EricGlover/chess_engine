use crate::board::*;


pub struct Move {
    piece: Piece,
    from: Coordinate,
    to: Coordinate
}

// legal moves
// any moves
// legal moves on the board

// generates all moves to squares on the board
// could be illegal
pub fn gen_moves(board: &Board, color: Color) -> Vec<Move> {
    let moves : Vec<Move> = Vec::new();
    let pieces = board.get_pieces(color);
    for piece in pieces.iter() {
        // gen_moves_for(board, piece, )
    }
    return moves;
}

fn gen_moves_for(board: &Board, piece: Piece, at: Coordinate) -> Vec<Move> {
    return match piece.piece_type {
        PieceType::King => gen_king_moves(board, at),
        PieceType::Queen => gen_queen_moves(board, at),
        PieceType::Bishop => gen_bishop_moves(board, at),
        PieceType::Knight => gen_knight_moves(board, at),
        PieceType::Rook => gen_rook_moves(board, at),
        PieceType::Pawn => gen_pawn_moves(board, at)
    }
}

fn gen_king_moves(board: &Board, at: Coordinate) -> Vec<Move> {
    vec![]
}
fn gen_queen_moves(board: &Board, at: Coordinate) -> Vec<Move> {
    vec![]
}
fn gen_bishop_moves(board: &Board, at: Coordinate) -> Vec<Move> {
    vec![]
}
fn gen_knight_moves(board: &Board, at: Coordinate) -> Vec<Move> {
    vec![]
}
fn gen_rook_moves(board: &Board, at: Coordinate) -> Vec<Move> {
    vec![]
}
fn gen_pawn_moves(board: &Board, at: Coordinate) -> Vec<Move> {
    vec![]
}

fn is_legal(move_: Move) -> bool {
    true
}