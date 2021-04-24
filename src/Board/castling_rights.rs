#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct CastlingRights {
    king_side: bool,
    queen_side: bool,
}
impl CastlingRights {
    pub fn new(king_side: bool, queen_side: bool) -> CastlingRights {
        CastlingRights {
            king_side,
            queen_side,
        }
    }
    pub fn both(&self) -> bool {
        self.king_side && self.queen_side
    }
    pub fn none(&self) -> bool {
        !self.king_side && !self.queen_side
    }
    pub fn king_side(&self) -> bool {
        self.king_side
    }
    pub fn queen_side(&self) -> bool {
        self.queen_side
    }
    pub fn king_side_mut(&mut self) -> &mut bool {
        &mut self.king_side
    }
    pub fn queen_side_mut(&mut self) -> &mut bool {
        &mut self.queen_side
    }
}
