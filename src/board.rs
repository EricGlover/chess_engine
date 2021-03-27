#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq,  Eq, Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

impl Piece {
    pub fn make_white_pawn() -> Piece {
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
        }
    }
    pub fn make_black_pawn() -> Piece {
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
        }
    }
    pub fn make_white_rook() -> Piece {
        Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        }
    }
    pub fn make_black_rook() -> Piece {
        Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        }
    }
    pub fn make_white_knight() -> Piece {
        Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        }
    }
    pub fn make_black_knight() -> Piece {
        Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        }
    }
    pub fn make_white_bishop() -> Piece {
        Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        }
    }
    pub fn make_black_bishop() -> Piece {
        Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        }
    }
    pub fn make_white_queen() -> Piece {
        Piece {
            piece_type: PieceType::Queen,
            color: Color::White,
        }
    }
    pub fn make_black_queen() -> Piece {
        Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
        }
    }
    pub fn make_white_king() -> Piece {
        Piece {
            piece_type: PieceType::King,
            color: Color::White,
        }
    }
    pub fn make_black_king() -> Piece {
        Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        }
    }
}

#[derive(Debug,  PartialEq, Eq, Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug,  PartialEq, Eq, Copy, Clone)]
pub struct Coordinate {
    pub x: u8, // a - h (traditional coordinates)
    pub y: u8, // 1 - 8 (traditional coordinates)
}

#[derive(Debug)]
pub struct Board {
    squares: Vec<Vec<Square>>,
    // squares: [[Square; 8]; 8],
    // squares: [Square; 64],
}

#[derive(Debug)]
pub struct Square {
    pub coordinate: Coordinate,
    pub piece: Option<Piece>,
    pub color: Color,
}

impl Board {
    pub fn place_piece(&mut self, piece : Piece, at: &Coordinate) {
        let mut square = self.squares
            .get_mut((at.y - 1) as usize)
            .unwrap()
            .get_mut((at.x - 1) as usize)
            .unwrap();
        square.piece = Some(piece);

        // let mut square = self.squares[(at.y - 1) as usize][(at.x - 1) as usize];
        // square.piece = Some(piece);
    }

    pub fn has_piece(&self, at: &Coordinate) -> bool {
        self.squares[(at.y - 1) as usize][(at.x - 1) as usize]
            .piece
            .is_some()
    }

    pub fn get_piece_at(&self, at: &Coordinate) -> Option<Piece> {
        let square = self.get_square(at);
        if square.piece.is_some() {
            return Some(square.piece.unwrap().clone());
        } else {
            return None;
        }
    }

    fn get_square(&self, at: &Coordinate) -> &Square {
        &self.squares[(at.y - 1) as usize][(at.x - 1) as usize]
    }

    fn make_squares() -> Vec<Vec<Square>> {
        let mut vec: Vec<Vec<Square>> = vec![];

        for (i, y) in (1..9).enumerate() {
            let mut row: Vec<Square> = Vec::new();
            for (j, x) in (1..9).enumerate() {
                // odd numbered rows have black squares on even x's
                let color: Color;
                if y % 2 == 0 {
                    // even row , white is even, black is odd
                    color = if x % 2 == 0 {
                        Color::White
                    } else {
                        Color::Black
                    }
                } else {
                    // odd row , white is odd , black is even
                    color = if x % 2 == 0 {
                        Color::Black
                    } else {
                        Color::White
                    }
                }
                row.push(Square {
                    coordinate: Coordinate { y, x },
                    piece: None,
                    color,
                });
            }
            vec.push(row);
        }
        return vec;
    }

    //@todo : clean up the architecture here, should it pass in a format and matrix display ?
    pub fn get_squares(&self) -> Vec<&Square> {
        return self
            .squares
            .iter()
            .map(|vec| {
                return vec.iter().rev();
            })
            .flatten()
            .collect();
    }

    pub fn new() -> Board {
        Board {
            squares: Board::make_squares(),
        }
    }
}
