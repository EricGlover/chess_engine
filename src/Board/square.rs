use crate::board::*;

#[derive(Debug, Eq, PartialEq)]
pub struct Square {
    coordinate: Coordinate,
    piece: Option<Piece>,
    color: Color,
}

impl Square {
    pub fn new(coordinate: Coordinate, piece: Option<Piece>, color: Color) -> Square {
        Square {
            coordinate,
            piece,
            color,
        }
    }
    pub fn coordinate(&self) -> &Coordinate {
        &self.coordinate
    }
    pub fn piece(&self) -> Option<&Piece> {
        self.piece.as_ref()
    }
    pub fn color(&self) -> &Color {
        &self.color
    }
    pub fn place_piece(&mut self, mut piece: Piece) {
        piece.set_at(self.coordinate.clone());
        self.piece = Some(piece)
    }
    pub fn remove_piece(&mut self) -> Option<Piece> {
        self.piece.take()
    }
    pub fn _clone(&self) -> Square {
        Square {
            coordinate: self.coordinate.clone(),
            piece: self.piece.clone(),
            color: self.color.clone(),
        }
    }
}
