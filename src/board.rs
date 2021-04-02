mod board_stuff;
pub use board_stuff::*;
use crate::move_generator::{gen_attack_moves, Move};
use std::fmt;
use std::fmt::Formatter;
use crate::fen_reader;


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
    pub fn at(&self) -> Option<Coordinate> {
        self.at
    }
}

pub struct Player {
    time_used: u16,      // milliseconds
    time_remaining: u16, // milliseconds
    name: String,
}

#[derive(Debug)]
pub struct Square {
    pub coordinate: Coordinate,
    pub piece: Option<Piece>,
    pub color: Color,
}

#[test]
fn test_in_check() {
    let board = fen_reader::read(fen_reader::BLACK_IN_CHECK);
    assert!(board.is_in_check(Color::Black));
    assert!(!board.is_in_check(Color::White));
    let board = fen_reader::read(fen_reader::WHITE_IN_CHECK);
    assert!(!board.is_in_check(Color::Black));
    assert!(board.is_in_check(Color::White));
}

#[test]
fn test_get_files() {
    let board = fen_reader::read(fen_reader::INITIAL_BOARD);
    let files = board.get_files();
    for (j, row) in files.iter().enumerate() {
        for (i, s) in row.iter().enumerate() {
            assert_eq!((i + 1) as u8, s.coordinate.y);
            assert_eq!((j + 1) as u8, s.coordinate.x);
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub white_to_move: bool,               //@todo : update these
    pub white_can_castle_king_side: bool,  //@todo : update these
    pub white_can_castle_queen_side: bool, //@todo : update these
    pub black_can_castle_king_side: bool,  //@todo : update these
    pub black_can_castle_queen_side: bool, //@todo : update these
    pub en_passant_target: Option<Coordinate>,
    pub half_move_clock: u8,  //@todo : update these
    pub full_move_number: u8, //@todo : update these
    squares: Vec<Vec<Square>>,
}

impl Board {
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

    pub fn squares_list(&self) -> Vec<&Square> {
        self.squares
            .iter()
            .map(|vec| {
                return vec.iter();
            })
            .flatten()
            .collect()
    }

    // fn test_files() {
    //
    // }

    pub fn get_files(&self) -> Vec<Vec<&Square>> {
        let mut files: Vec<Vec<&Square>> = vec![];
        {
            let mut x = 0;
            let row_length = self.squares.get(0).unwrap().len();
            while x < row_length {
                // for each row get square at x
                let file: Vec<&Square> = self.squares.iter().map(|row| {
                    row.get(x).unwrap()
                }).collect();
                files.push(file);
                x = x + 1;
            }
        }
        files
    }

    pub fn get_squares(&self) -> &Vec<Vec<Square>> {
        &self.squares
    }

    // change return to piece list or something ?
    pub fn is_in_check(&self, color: Color) -> bool {
        let moves = gen_attack_moves(self, color.opposite());
        let king = self.get_king(color).unwrap();
        let at = king.at().unwrap();
        for m in moves {
            if m.to == at {
                return true;
            }
        }
        false
    }

    // doesn't check legality of moves
    pub fn make_move_mut(&mut self, m: &Move) {
        let enemy_piece = self.remove_piece(&m.to);
        // is this a capture
        if enemy_piece.is_none() && m.piece.piece_type != PieceType::Pawn {
            // update 50 move rule draw counter
            self.half_move_clock = self.half_move_clock + 1;
        } else if enemy_piece.is_some() {
            let enemy_piece = enemy_piece.unwrap();
            if enemy_piece.color == m.piece.color {
                // put it back
                self.place_piece(enemy_piece, m.to.clone());
                panic!("invalid move");
            }
        }

        // get piece to move
        let mut moving_piece = self.remove_piece(&m.from).unwrap();

        // if it gets promoted, then switch it's type
        if m.promoted_to.is_some() && m.piece.piece_type == PieceType::Pawn {
            moving_piece.piece_type = m.promoted_to.unwrap();
        }

        // move the piece ( update the piece and square )
        self.place_piece(moving_piece, m.to.clone());

        // castling
        if m.is_castling && m.rook_from.is_some() && m.rook_to.is_some() {
            self.make_move_mut(&Move::new(
                m.rook_from.unwrap(),
                m.rook_to.unwrap(),
                m.rook.unwrap(),
            ))
        }

        // update move counter
        if m.piece.color == Color::Black {
            self.full_move_number = self.full_move_number + 1;
        }
    }

    pub fn make_move(&self, m: &Move) -> Board {
        let mut board = self.clone();
        board.make_move_mut(m);
        board
    }

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

    pub fn _clone(&self) -> Board {
        self.clone()
    }

    fn clone(&self) -> Board {
        let mut squares: Vec<Vec<Square>> = vec![];
        for row in self.squares.iter() {
            let mut new_row: Vec<Square> = vec![];
            for square in row.iter() {
                new_row.push(Square {
                    coordinate: square.coordinate.clone(),
                    piece: square.piece.clone(),
                    color: square.color.clone(),
                });
            }
            squares.push(new_row);
        }
        Board {
            white_to_move: self.white_to_move,
            white_can_castle_king_side: self.white_can_castle_king_side,
            white_can_castle_queen_side: self.white_can_castle_queen_side,
            black_can_castle_king_side: self.black_can_castle_king_side,
            black_can_castle_queen_side: self.black_can_castle_queen_side,
            en_passant_target: self.en_passant_target.clone(),
            half_move_clock: self.half_move_clock,
            full_move_number: self.full_move_number,
            squares,
        }
    }

    fn get_king(&self, color: Color) -> Option<Piece> {
        for row in self.squares.iter() {
            for square in row.iter() {
                if square.piece.is_none() {
                    continue;
                }
                let piece = square.piece.unwrap();
                if piece.piece_type == PieceType::King && piece.color == color {
                    return Some(piece.clone());
                }
            }
        }
        None
    }

    fn remove_piece(&mut self, at: &Coordinate) -> Option<Piece> {
        let square = self.get_square_mut(at);
        let piece = square.piece;
        square.piece = None;
        piece
    }

    fn is_on_board(c: Coordinate) -> bool {
        c.x >= LOW_X && c.x <= HIGH_X && c.y >= LOW_Y && c.y <= HIGH_Y
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

        for (_, y) in (1..9).enumerate() {
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
}
