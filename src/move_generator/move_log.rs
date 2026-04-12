use crate::board::{Coordinate, PieceType};
use crate::move_generator::{Move, MoveType};



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