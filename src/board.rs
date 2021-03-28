#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    at: Option<Coordinate>
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType, at: Option<Coordinate>) -> Piece {
        Piece {
            piece_type,
            color,
            at,
        }
    }
    pub fn at(&self) -> Option<Coordinate> {
        self.at
    }
    pub fn make_white_pawn() -> Piece {
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::White,
            at: None,
        }
    }
    pub fn make_black_pawn() -> Piece {
        Piece {
            piece_type: PieceType::Pawn,
            color: Color::Black,
            at: None,
        }
    }
    pub fn make_white_rook() -> Piece {
        Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
            at: None,
        }
    }
    pub fn make_black_rook() -> Piece {
        Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
            at: None,
        }
    }
    pub fn make_white_knight() -> Piece {
        Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
            at: None,
        }
    }
    pub fn make_black_knight() -> Piece {
        Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
            at: None,
        }
    }
    pub fn make_white_bishop() -> Piece {
        Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
            at: None,
        }
    }
    pub fn make_black_bishop() -> Piece {
        Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
            at: None,
        }
    }
    pub fn make_white_queen() -> Piece {
        Piece {
            piece_type: PieceType::Queen,
            color: Color::White,
            at: None,
        }
    }
    pub fn make_black_queen() -> Piece {
        Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
            at: None,
        }
    }
    pub fn make_white_king() -> Piece {
        Piece {
            piece_type: PieceType::King,
            color: Color::White,
            at: None,
        }
    }
    pub fn make_black_king() -> Piece {
        Piece {
            piece_type: PieceType::King,
            color: Color::Black,
            at: None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Coordinate {
    pub x: u8, // a - h (traditional coordinates)
    pub y: u8, // 1 - 8 (traditional coordinates)
}

impl Coordinate {
    // this is not pretty.... don't judge me
    pub fn from(str: &str) -> Coordinate {
        let mut x:u8 = 0;
        if str.contains("1") {
            x = 1;
        }
        else if str.contains("2") {
            x = 2;
        }
        else if str.contains("3") {
            x = 3;
        }
        else if str.contains("4") {
            x = 4;
        }
        else if str.contains("5") {
            x = 5;
        }
        else if str.contains("6") {
            x = 6;
        }
        else if str.contains("7") {
            x = 7;
        }
        else if str.contains("8") {
            x = 8;
        }
        let mut y : u8 = 0;
        if str.contains("a") {
            y = 1;
        }
        else if str.contains("b") {
            y = 2;
        }
        else if str.contains("c") {
            y = 3;
        }
        else if str.contains("d") {
            y = 4;
        }
        else if str.contains("e") {
            y = 5;
        }
        else if str.contains("f") {
            y = 6;
        }
        else if str.contains("g") {
            y = 7;
        }
        else if str.contains("h") {
            y = 8;
        }
        Coordinate {x, y}
    }
}

#[derive(Debug)]
pub struct Board {
    pub white_to_move: bool,
    pub white_can_castle_king_side: bool,
    pub white_can_castle_queen_side: bool,
    pub black_can_castle_king_side: bool,
    pub black_can_castle_queen_side: bool,
    pub en_passant_target: Option<Coordinate>,
    pub half_move_clock: u8,
    pub full_move_number: u8,
    squares: Vec<Vec<Square>>,
}

#[derive(Debug)]
pub struct Square {
    pub coordinate: Coordinate,
    pub piece: Option<Piece>,
    pub color: Color,
}

impl Board {
    pub fn place_piece(&mut self, mut piece: Piece, at: Coordinate) {
        piece.at = Some(at);
        self.get_square_mut(&at).piece = Some(piece);
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

    pub fn get_pieces(&self, color: Color) -> Vec<Piece> {
        let mut pieces = Vec::<Piece>::new();
        for row in self.squares.iter() {
            for square in row.iter() {
                if square.piece.is_none() {
                    continue;
                }
                let piece = square.piece.unwrap();
                if piece.color == color {
                    pieces.push(piece.clone());
                }
            }
        }
        return pieces;
    }

    fn get_square(&self, at: &Coordinate) -> &Square {
        self.squares
            .get((at.y - 1) as usize)
            .unwrap()
            .get((at.x - 1) as usize)
            .unwrap()
    }

    fn get_square_mut(&mut self, at: &Coordinate) -> &mut Square {
        self.squares
            .get_mut((at.y - 1) as usize)
            .unwrap()
            .get_mut((at.x - 1) as usize)
            .unwrap()
    }

    fn make_squares() -> Vec<Vec<Square>> {
        let mut vec: Vec<Vec<Square>> = vec![];

        for (_, y)  in (1..9).enumerate() {
            let mut row: Vec<Square> = Vec::new();
            for (_, x) in (1..9).enumerate() {
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
            white_to_move: true,
            white_can_castle_king_side: true,
            white_can_castle_queen_side: true,
            black_can_castle_king_side: true,
            black_can_castle_queen_side: true,
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 0,
            squares: Board::make_squares(),
        }
    }
}
