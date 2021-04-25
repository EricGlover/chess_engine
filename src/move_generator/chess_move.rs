use crate::move_generator::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum MoveType {
    Move,
    Castling {
        rook_from: Coordinate,
        rook_to: Coordinate,
    },
    EnPassant,
    Promotion(PieceType),
}

// @todo: maybe consider adding the algebraic notation for this move (the pgn)
// @todo : add old castling rights to moves ?
// @todo : add all the info needed for the unmake function , consider this a two-way change object
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Move {
    pub piece: PieceType,
    pub from: Coordinate,
    pub to: Coordinate,
    move_type: MoveType,
    pub captured: Option<PieceType>,
    castling_rights_removed: CastlingRights,
    pub is_check: bool,     // @todo : set these in game when eval happens ?
    pub is_checkmate: bool, // @todo : set these in game when eval happens ?
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{} moving from {} to {} ",
            self.piece, self.from, self.to
        )
    }
}

impl Move {
    pub fn new(
        from: Coordinate,
        to: Coordinate,
        piece: PieceType,
        move_type: MoveType,
        captured: Option<PieceType>,
        castling_rights_removed: Option<CastlingRights>,
    ) -> Move {
        Move {
            piece,
            from,
            to,
            move_type,
            captured,
            castling_rights_removed: castling_rights_removed
                .map_or(CastlingRights::new(false, false), |r| r),
            is_check: false,
            is_checkmate: false,
        }
    }

    pub fn castling_rights_removed(&self) -> &CastlingRights {
        &self.castling_rights_removed
    }

    pub fn move_type(&self) -> &MoveType {
        &self.move_type
    }

    // @todo: static lifetime is suss
    pub fn print_moves(moves: &Vec<Move>) {
        moves.iter().for_each(|m| {
            let str = m.to_string();
            println!("{}", str.as_str());
        })
    }

    pub fn castle_king_side(color: Color) -> Move {
        let (from, to) = Move::king_side_castle_coordinates(color, PieceType::King);
        let (rook_from, rook_to) = Move::king_side_castle_coordinates(color, PieceType::Rook);
        Move {
            piece: PieceType::King,
            from,
            to,
            move_type: MoveType::Castling { rook_from, rook_to },
            castling_rights_removed: CastlingRights::new(true, true),
            captured: None,
            is_check: false,
            is_checkmate: false,
        }
    }
    pub fn castle_queen_side(color: Color) -> Move {
        let (from, to) = Move::queen_side_castle_coordinates(color, PieceType::King);
        let (rook_from, rook_to) = Move::queen_side_castle_coordinates(color, PieceType::Rook);
        Move {
            piece: PieceType::King,
            from,
            to,
            move_type: MoveType::Castling { rook_from, rook_to },
            castling_rights_removed: CastlingRights::new(true, true),
            captured: None,
            is_check: false,
            is_checkmate: false,
        }
    }
    pub fn is_king_side_castle(&self) -> bool {
        match self.move_type {
            MoveType::Castling { rook_from, rook_to:_ } => rook_from.x() == 8,
            _ => false,
        }
    }
    pub fn is_queen_side_castle(&self) -> bool {
        match self.move_type {
            MoveType::Castling { rook_from, rook_to:_ } => rook_from.x() == 1,
            _ => false,
        }
    }
    pub fn king_side_castle_coordinates(
        color: Color,
        piece_type: PieceType,
    ) -> (Coordinate, Coordinate) {
        let y: u8 = if color == Color::White { 1 } else { 8 };
        match piece_type {
            PieceType::King => {
                let from = Coordinate::new(5, y);
                let to = Coordinate::new(7, y);
                return (from, to);
            }
            PieceType::Rook => {
                let from = Coordinate::new(8, y);
                let to = Coordinate::new(6, y);
                return (from, to);
            }
            _ => panic!("invalid"),
        }
    }
    pub fn queen_side_castle_coordinates(
        color: Color,
        piece_type: PieceType,
    ) -> (Coordinate, Coordinate) {
        let y: u8 = if color == Color::White { 1 } else { 8 };
        match piece_type {
            PieceType::King => {
                let from = Coordinate::new(5, y);
                let to = Coordinate::new(3, y);
                return (from, to);
            }
            PieceType::Rook => {
                let from = Coordinate::new(1, y);
                let to = Coordinate::new(4, y);
                return (from, to);
            }
            _ => panic!("invalid"),
        }
    }
}
