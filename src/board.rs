mod board_stuff;
use crate::board_console_printer::print_board;
use crate::fen_reader;
use crate::move_generator::{gen_pseudo_legal_moves, Move};
pub use board_stuff::*;
use std::fmt;
use std::fmt::Formatter;

mod test {
    use crate::board::*;
    use crate::fen_reader;

    #[test]
    fn test_get_pieces() {
        let board = fen_reader::make_board(fen_reader::BLACK_IN_CHECK);
        let pieces = board.get_pieces(Color::Black, PieceType::King);
        assert_eq!(pieces.len(), 1, "there is one black king");
        let found_black_king = pieces.get(0).unwrap();
        let black_king = Piece::new(Color::Black, PieceType::King, Some(Coordinate::new(5,8)));
        assert_eq!(&black_king, found_black_king);
    }

    #[test]
    fn test_clone() {
        let board = fen_reader::make_board(fen_reader::BLACK_IN_CHECK);
        let cloned = board.clone();
        // assert_eq!(board, cloned);
    }

    #[test]
    fn test_in_check() {
        let board = fen_reader::make_board(fen_reader::BLACK_IN_CHECK);
        assert!(board.is_in_check(Color::Black));
        assert!(!board.is_in_check(Color::White));
        let board = fen_reader::make_board(fen_reader::WHITE_IN_CHECK);
        assert!(!board.is_in_check(Color::Black));
        assert!(board.is_in_check(Color::White));
    }

    #[test]
    fn test_get_files() {
        let board = fen_reader::make_board(fen_reader::INITIAL_BOARD);
        let files = board.get_files();
        for (j, row) in files.iter().enumerate() {
            for (i, s) in row.iter().enumerate() {
                assert_eq!((i + 1) as u8, s.coordinate.y());
                assert_eq!((j + 1) as u8, s.coordinate.x());
            }
        }
    }
}

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

#[derive(Debug)]
pub struct Board {
    pub player_to_move: Color,
    white_can_castle_king_side: bool,
    white_can_castle_queen_side: bool,
    black_can_castle_king_side: bool,
    black_can_castle_queen_side: bool,
    pub en_passant_target: Option<Coordinate>,
    pub half_move_clock: u8,
    pub full_move_number: u8,
    squares: Vec<Vec<Square>>,
}

impl Board {
    pub fn make_board(
        player_to_move: Color,
        white_can_castle_king_side: bool,
        white_can_castle_queen_side: bool,
        black_can_castle_king_side: bool,
        black_can_castle_queen_side: bool,
        en_passant_target: Option<Coordinate>,
        half_move_clock: u8,
        full_move_number: u8,
    ) -> Board {
        Board {
            player_to_move,
            white_can_castle_king_side,
            white_can_castle_queen_side,
            black_can_castle_king_side,
            black_can_castle_queen_side,
            en_passant_target,
            half_move_clock,
            full_move_number,
            squares: Board::make_squares(),
        }
    }
    pub fn new() -> Board {
        Board {
            player_to_move: Color::White,
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

    pub fn can_castle_queen_side(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_can_castle_queen_side,
            Color::Black => self.black_can_castle_queen_side,
        }
    }

    pub fn can_castle_king_side(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_can_castle_king_side,
            Color::Black => self.black_can_castle_king_side,
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

    pub fn get_files(&self) -> Vec<Vec<&Square>> {
        let mut files: Vec<Vec<&Square>> = vec![];
        {
            let mut x = 0;
            let row_length = self.squares.get(0).unwrap().len();
            while x < row_length {
                // for each row get square at x
                let file: Vec<&Square> =
                    self.squares.iter().map(|row| row.get(x).unwrap()).collect();
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
        let moves = gen_pseudo_legal_moves(self, color.opposite());
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
        // update white to move flag
        self.player_to_move = m.piece.color.opposite();

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
            match moving_piece.color {
                Color::White => {
                    self.white_can_castle_king_side = false;
                    self.white_can_castle_queen_side = false;
                }
                Color::Black => {
                    self.black_can_castle_king_side = false;
                    self.black_can_castle_queen_side = false;
                }
            }
            self.make_move_mut(&Move::new(
                m.rook_from.unwrap(),
                m.rook_to.unwrap(),
                m.rook.unwrap(),
                false,
            ))
        }

        // update move counter
        if m.piece.color == Color::Black {
            self.full_move_number = self.full_move_number + 1;
        }
    }

    // pub fn unmake_move(&self, m: &Move)

    pub fn make_move(&self, m: &Move) -> Board {
        let mut board = self.clone();
        board.make_move_mut(m);
        board
    }

    pub fn place_piece(&mut self, mut piece: Piece, at: Coordinate) {
        if at.is_valid_coordinate() {
            piece.at = Some(at);
            self.get_square_mut(&at).piece = Some(piece);
        }
    }

    pub fn has_piece(&self, at: &Coordinate) -> bool {
        if !at.is_valid_coordinate() {
            return false;
        }
        self.squares[(at.y() - 1) as usize][(at.x() - 1) as usize]
            .piece
            .is_some()
    }

    pub fn get_pieces_in(&self, area: Vec<Coordinate>) -> Vec<(Coordinate, Option<Piece>)> {
        panic!("not implemented");
        vec![]
    }

    pub fn get_piece_at(&self, at: &Coordinate) -> Option<Piece> {
        if !at.is_valid_coordinate() {
            return None;
        }
        let square = self.get_square(at);
        if square.piece.is_some() {
            return Some(square.piece.unwrap().clone());
        } else {
            return None;
        }
    }

    pub fn get_kings(&self)-> Vec<Piece> {
        vec![]
    }

    pub fn get_pieces(&self, color: Color, piece_type: PieceType) -> Vec<Piece> {
        // self.squares
        //     .iter()
        //     .flatten()
        //     .filter(|&square| {
        //         if square.piece.is_none() {
        //             return false;
        //         }
        //         let piece = square.piece.unwrap();
        //         return piece.piece_type == piece_type && piece.color == color;
        //     })
        //     .map(|square| square.piece.unwrap().clone())
        //     .collect()
        let mut pieces: Vec<Piece> = vec![];
        // @todo : try filtering
        for row in self.squares.iter() {
            for square in row.iter() {
                if square.piece.is_none() {
                    continue;
                }
                let piece = square.piece.unwrap();
                if piece.piece_type == piece_type && piece.color == color {
                    pieces.push(piece.clone());
                }
            }
        }
        pieces
    }

    pub fn get_all_pieces(&self, color: Color) -> Vec<Piece> {
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
            player_to_move: self.player_to_move,
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
        let mut pieces = self.get_pieces(color, PieceType::King);
        if pieces.len() == 0 {
            return None;
        }
        Some(pieces.remove(0))
    }

    fn remove_piece(&mut self, at: &Coordinate) -> Option<Piece> {
        let square = self.get_square_mut(at);
        let piece = square.piece;
        square.piece = None;
        piece
    }

    fn is_on_board(c: Coordinate) -> bool {
        c.x() >= LOW_X && c.x() <= HIGH_X && c.y() >= LOW_Y && c.y() <= HIGH_Y
    }

    fn get_square(&self, at: &Coordinate) -> &Square {
        self.squares
            .get((at.y() - 1) as usize)
            .unwrap()
            .get((at.x() - 1) as usize)
            .unwrap()
    }

    fn get_square_mut(&mut self, at: &Coordinate) -> &mut Square {
        self.squares
            .get_mut((at.y() - 1) as usize)
            .unwrap()
            .get_mut((at.x() - 1) as usize)
            .unwrap()
    }

    fn make_squares() -> Vec<Vec<Square>> {
        let mut vec: Vec<Vec<Square>> = Vec::with_capacity(8);
        for (_, y) in (1..9).enumerate() {
            let mut row: Vec<Square> = Vec::with_capacity(8);
            for (i, x) in (1..9).enumerate() {
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
                    coordinate: Coordinate::new(x, y),
                    piece: None,
                    color,
                });
            }
            vec.push(row);
        }
        return vec;
    }
}
