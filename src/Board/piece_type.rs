use std::fmt::{Error, Formatter};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            PieceType::King => "King",
            PieceType::Queen => "Queen",
            PieceType::Bishop => "Bishop",
            PieceType::Knight => "Knight",
            PieceType::Rook => "Rook",
            PieceType::Pawn => "Pawn",
        };
        write!(f, "{}", string)
    }
}

impl PieceType {
    pub fn from(char: &str) -> Option<PieceType> {
        match char {
            "p" => Some(PieceType::Pawn),
            "n" => Some(PieceType::Knight),
            "b" => Some(PieceType::Bishop),
            "r" => Some(PieceType::Rook),
            "q" => Some(PieceType::Queen),
            "k" => Some(PieceType::King),
            _ => None,
        }
    }
    pub fn to(&self) -> &str {
        match self {
            PieceType::King => "k",
            PieceType::Queen => "q",
            PieceType::Bishop => "b",
            PieceType::Knight => "n",
            PieceType::Rook => "r",
            PieceType::Pawn => "p",
        }
    }
}