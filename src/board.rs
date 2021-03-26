#[derive(Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Copy, Clone, Debug)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Debug, Copy, Clone)]
pub struct Coordinate {
    pub x: u8, // a - h (traditional coordinates)
    pub y: u8, // 1 - 8 (traditional coordinates)
}

#[derive(Debug, Copy, Clone)]
pub struct Square {
    pub coordinate: Coordinate,
    pub piece: Option<Piece>,
    pub color: Color,
}

#[derive(Debug, Clone)]
pub struct Board {
    squares: Vec<Vec<Square>>,
}

impl<'a> Board {
    pub fn place_piece(mut self, piece: Piece, at : &Coordinate) -> Self {
        self.squares[(at.y - 1) as usize][(at.x - 1) as usize].piece = Some(piece);
        return self;
    }
    pub fn make_squares() -> Vec<Vec<Square>> {
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

    // @todo: try reading fen
    pub fn place_pieces() {
        let s = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    }


    //@todo : clean up the architecture here, should it pass in a format and matrix display ?
    pub fn get_squares(&self) -> Vec<&Square>{
        return self.squares
            .iter()
            .map(| vec| {
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
    // @todo: place the pieces
    //

    //
    // fn print(&self) {
    //     let format = Format::new(7, 3);
    //     let board_cells = self.squares
    //         .iter()
    //         .enumerate()
    //         .map(|(i, row)| {
    //             // @todo: check if None ?
    //             // ansi 8 bit color scheme
    //             let mut foreground = 0;
    //             let mut value = ' ';
    //             if x.is_some() {
    //                 let piece = x.unwrap();
    //                 foreground =  match piece.color {
    //                     Color::Black => 1,  // red
    //                     Color::White => 5,  // purple
    //                 };
    //                 value = match piece.piece_type {
    //                     PieceType::King => 'K',
    //                     PieceType::Queen => 'Q',
    //                     PieceType::Bishop => 'B',
    //                     PieceType::Knight => 'N',
    //                     PieceType::Rook => 'R',
    //                     PieceType::Pawn => 'P',
    //                 }
    //             }
    //
    //             // @todo : change to use square color later
    //             let mut ansi_bg = 0;
    //             if i % 2 + (i / 8) % 2 == 1 {
    //                 ansi_bg = 7;
    //             }
    //             cell::Cell::new(value, foreground, ansi_bg)
    //         })
    //         .collect::<Vec<_>>();
    //     let mut data = matrix::Matrix::new(8, board_cells);
    //     let mut display = MatrixDisplay::new(&format, &mut data);
    //     display.print(&mut std::io::stdout(), &style::BordersStyle::None);
    // }

    pub fn make_initial_board() -> Vec<Option<Piece>> {
        vec![
            Some(Piece::make_black_rook()),
            Some(Piece::make_black_knight()),
            Some(Piece::make_black_bishop()),
            Some(Piece::make_black_queen()),
            Some(Piece::make_black_king()),
            Some(Piece::make_black_bishop()),
            Some(Piece::make_black_knight()),
            Some(Piece::make_black_rook()),
            Some(Piece::make_black_pawn()),
            Some(Piece::make_black_pawn()),
            Some(Piece::make_black_pawn()),
            Some(Piece::make_black_pawn()),
            Some(Piece::make_black_pawn()),
            Some(Piece::make_black_pawn()),
            Some(Piece::make_black_pawn()),
            Some(Piece::make_black_pawn()),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            None,
            Some(Piece::make_white_pawn()),
            Some(Piece::make_white_pawn()),
            Some(Piece::make_white_pawn()),
            Some(Piece::make_white_pawn()),
            Some(Piece::make_white_pawn()),
            Some(Piece::make_white_pawn()),
            Some(Piece::make_white_pawn()),
            Some(Piece::make_white_pawn()),
            Some(Piece::make_white_rook()),
            Some(Piece::make_white_knight()),
            Some(Piece::make_white_bishop()),
            Some(Piece::make_white_queen()),
            Some(Piece::make_white_king()),
            Some(Piece::make_white_bishop()),
            Some(Piece::make_white_knight()),
            Some(Piece::make_white_rook()),
        ]
    }
}