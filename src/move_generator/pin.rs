use crate::move_generator::Piece;
use crate::board::Coordinate;

#[derive(Eq, PartialEq, Debug)]
pub struct Pin<'a> {
    pub pinned_piece: &'a Piece,
    pub pinned_by: &'a Piece,
    pub pinned_at: Coordinate,
    pub pinned_to: &'a Piece,
    pub can_move_to_board: u64,
}