use crate::board::*;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    at: Option<Coordinate>,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.color, self.piece_type)
    }
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType, at: Option<Coordinate>) -> Piece {
        Piece {
            piece_type,
            color,
            at,
        }
    }
    pub fn at(&self) -> Option<&Coordinate> {
        self.at.as_ref()
    }
    pub fn set_at(&mut self, at: Coordinate) {
        self.at = Some(at);
    }
}
