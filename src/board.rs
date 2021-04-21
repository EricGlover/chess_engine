mod board_stuff;

use crate::board_console_printer::print_board;
use crate::fen_reader;
use crate::move_generator::{gen_pseudo_legal_moves, Move};
pub use board_stuff::*;
use std::fmt;
use std::fmt::Formatter;

// getting pieces && squares return references
pub trait BoardTrait {
    // constructors
    // fn make_board(
    //     player_to_move: Color,
    //     white_can_castle_king_side: bool,
    //     white_can_castle_queen_side: bool,
    //     black_can_castle_king_side: bool,
    //     black_can_castle_queen_side: bool,
    //     en_passant_target: Option<Coordinate>,
    //     half_move_clock: u8,
    //     full_move_number: u8,
    // ) -> Board;
    // fn new() -> Board;

    fn clone(&self) -> Box<dyn BoardTrait>;

    // info about game going on
    fn player_to_move(&self) -> Color;
    fn en_passant_target(&self) -> Option<Coordinate>;
    fn half_move_clock(&self) -> u8;
    fn full_move_number(&self) -> u8;
    fn can_castle_queen_side(&self, color: Color) -> bool;
    fn can_castle_king_side(&self, color: Color) -> bool;
    fn white_castling_rights(&self) -> CastlingRights;
    fn black_castling_rights(&self) -> CastlingRights;

    // getting squares
    fn squares_list(&self) -> Vec<&Square>;
    fn get_rank(&self, y: u8) -> Vec<&Square>;
    fn get_files(&self) -> Vec<Vec<&Square>>;
    fn get_squares(&self) -> &Vec<Vec<Square>>;

    // moves
    // fn make_move(&self, m: &Move) -> Self where Self: Sized ;
    fn make_move_mut(&mut self, m: &Move);
    // fn unmake_move(&self, m: &Move) -> Self where Self: Sized ;
    fn unmake_move_mut(&mut self, m: &Move);

    // getting and setting pieces
    fn place_piece(&mut self, piece: Piece, at: &Coordinate);
    fn remove_piece(&mut self, piece: &Piece) -> Piece;
    fn has_piece(&self, at: &Coordinate) -> bool;
    // fn get_pieces_in(&self, area: Vec<Coordinate>) -> Vec<(Coordinate, Option<&Piece>)>;
    fn get_piece_at(&self, at: &Coordinate) -> Option<&Piece>;
    fn get_kings(&self) -> Vec<&Piece>;
    fn get_pieces(&self, color: Color, piece_type: PieceType) -> Vec<&Piece>;
    fn get_all_pieces(&self, color: Color) -> Vec<&Piece>;
}

#[cfg(test)]
mod test {
    use crate::board::*;
    use crate::fen_reader;
    use crate::move_generator::gen_legal_moves;

    fn assert_board_is_same(board: &Board, board_2: &Board) {
        assert_eq!(
            board.player_to_move(),
            board_2.player_to_move(),
            "same player_to_move"
        );
        assert_eq!(
            board.white_castling_rights(),
            board_2.white_castling_rights(),
            "same white_castling_rights"
        );
        assert_eq!(
            board.black_castling_rights(),
            board_2.black_castling_rights(),
            "same black_castling_rights"
        );
        assert_eq!(
            board.previous_white_castling_rights, board_2.previous_white_castling_rights,
            "same previous_white_castling_rights"
        );
        assert_eq!(
            board.previous_black_castling_rights, board_2.previous_black_castling_rights,
            "same previous_black_castling_rights"
        );
        assert_eq!(
            board.en_passant_target(),
            board_2.en_passant_target(),
            "same en_passant_target"
        );
        // assert_eq!(board.half_move_clock(), board_2.half_move_clock(), "same half_move_clock");
        assert_eq!(
            board.full_move_number(),
            board_2.full_move_number(),
            "same full_move_number"
        );

        board
            .squares
            .iter()
            .zip(board_2.squares.iter())
            .for_each(|(row, row_2)| {
                row.iter().zip(row_2.iter()).for_each(|(square, square_2)| {
                    assert_eq!(square, square_2, "squares are the same");
                })
            });
    }

    #[test]
    fn test_make_unmake() {
        fn test_board_moves(board: Board) {
            let mut board_2 = board._clone();
            let moves = gen_legal_moves(&board_2, board.player_to_move);
            moves.iter().for_each(|m| {
                println!("making move {}", m);
                println!("making move {:?}", m);
                board_2.make_move_mut(m);
                board_2.unmake_move_mut(m);
                assert_board_is_same(&board, &board_2);
                // assert_eq!(board, board_2, "board is back to what it was");
            });
        }
        println!("testing initial board");
        test_board_moves(fen_reader::make_initial_board());
        println!("testing WHITE_IN_CHECK");
        test_board_moves(fen_reader::make_board(fen_reader::WHITE_IN_CHECK));
        println!("testing TEST_BOARD_1");
        test_board_moves(fen_reader::make_board(fen_reader::TEST_BOARD_1));
        println!("testing TEST_BOARD_2");
        test_board_moves(fen_reader::make_board(fen_reader::TEST_BOARD_2));
        println!("testing BLACK_IN_CHECK");
        test_board_moves(fen_reader::make_board(fen_reader::BLACK_IN_CHECK));
    }

    #[test]
    fn test_get_rank() {
        let board = fen_reader::make_initial_board();
        let rank = board.get_rank(1);
        let square = rank.get(0);
        assert!(square.is_some(), "there is a square at 1 1 ");
        let at = Coordinate::new(1, 1);
        assert_eq!(square.unwrap().coordinate, at, "at 1 1");
    }

    #[test]
    fn test_get_pieces() {
        let board = fen_reader::make_board(fen_reader::BLACK_IN_CHECK);
        let pieces = board.get_pieces(Color::Black, PieceType::King);
        assert_eq!(pieces.len(), 1, "there is one black king");
        let found_black_king = pieces.get(0).unwrap();
        let black_king = Piece::new(Color::Black, PieceType::King, Some(Coordinate::new(5, 8)));
        assert_eq!(&&black_king, found_black_king);
    }

    #[test]
    fn test_clone() {
        let board = fen_reader::make_board(fen_reader::BLACK_IN_CHECK);
        let cloned = board.clone();
        // assert_eq!(board, cloned);
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
    pub fn at(&self) -> Option<&Coordinate> {
        self.at.as_ref()
    }
}

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
    pub fn king_side(&self) -> bool {
        self.king_side
    }
    pub fn queen_side(&self) -> bool {
        self.queen_side
    }
}

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
        piece.at = Some(self.coordinate.clone());
        self.piece = Some(piece)
    }
    pub fn remove_piece(&mut self) -> Option<Piece> {
        self.piece.take()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Board {
    player_to_move: Color,
    white_castling_rights: CastlingRights,
    black_castling_rights: CastlingRights,
    previous_white_castling_rights: CastlingRights, // used in unmake move
    previous_black_castling_rights: CastlingRights, // used in unmake move
    en_passant_target: Option<Coordinate>,
    half_move_clock: u8,
    full_move_number: u8,
    squares: Vec<Vec<Square>>,
}

//@todo : piece.at
impl BoardTrait for Board {
    // fn clone(&self) -> Board {
    //
    // }
    fn clone(&self) -> Box<dyn BoardTrait> {
        let mut squares: Vec<Vec<Square>> = vec![];
        for row in self.squares.iter() {
            let mut new_row: Vec<Square> = vec![];
            for square in row.iter() {
                new_row.push(Square::new(
                    square.coordinate.clone(),
                    square.piece.clone(),
                    square.color.clone(),
                ));
            }
            squares.push(new_row);
        }
        Box::new(Board {
            player_to_move: self.player_to_move,
            white_castling_rights: self.white_castling_rights.clone(),
            black_castling_rights: self.black_castling_rights.clone(),
            previous_white_castling_rights: self.white_castling_rights.clone(),
            previous_black_castling_rights: self.black_castling_rights.clone(),
            en_passant_target: self.en_passant_target.clone(),
            half_move_clock: self.half_move_clock,
            full_move_number: self.full_move_number,
            squares,
        })
    }
    fn player_to_move(&self) -> Color {
        self.player_to_move
    }
    fn en_passant_target(&self) -> Option<Coordinate> {
        self.en_passant_target.clone()
    }
    fn half_move_clock(&self) -> u8 {
        self.half_move_clock
    }
    fn full_move_number(&self) -> u8 {
        self.full_move_number
    }
    fn can_castle_queen_side(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_castling_rights.queen_side(),
            Color::Black => self.black_castling_rights.queen_side(),
        }
    }

    fn can_castle_king_side(&self, color: Color) -> bool {
        match color {
            Color::White => self.white_castling_rights.king_side(),
            Color::Black => self.black_castling_rights.king_side(),
        }
    }
    fn white_castling_rights(&self) -> CastlingRights {
        self.white_castling_rights.clone()
    }
    fn black_castling_rights(&self) -> CastlingRights {
        self.black_castling_rights.clone()
    }

    fn squares_list(&self) -> Vec<&Square> {
        self.squares
            .iter()
            .map(|vec| {
                return vec.iter();
            })
            .flatten()
            .collect()
    }

    fn get_rank(&self, y: u8) -> Vec<&Square> {
        if y < 1 || y > 8 {
            panic!("invalid rank");
        }
        // self.squares.get((y - 1) as usize)
        let rank = self.squares.get((y - 1) as usize).unwrap();
        rank.iter().map(|square| square).collect()
    }

    fn get_files(&self) -> Vec<Vec<&Square>> {
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

    fn get_squares(&self) -> &Vec<Vec<Square>> {
        &self.squares
    }

    fn remove_piece(&mut self, piece: &Piece) -> Piece {
        self.get_square_mut(&piece.at().unwrap())
            .remove_piece()
            .unwrap()
    }

    // doesn't check legality of moves
    // fn make_move_mut(&mut self, m: &Move) {
    fn make_move_mut(&mut self, mov: &Move) {
        let moving_piece = self.get_piece_at(&mov.from).unwrap().clone();
        // update white to move flag
        self.player_to_move = moving_piece.color.opposite();

        let enemy_piece = self.remove_piece_at(&mov.to);

        // update 50 move rule draw counter
        if mov.captured.is_none() || moving_piece.piece_type != PieceType::Pawn {
            self.half_move_clock = self.half_move_clock + 1;
        } else {
            self.half_move_clock = 0;
        }

        // is this a capture
        if enemy_piece.is_some() {
            let enemy_piece = enemy_piece.unwrap();
            if enemy_piece.color == moving_piece.color {
                // put it back
                self.place_piece(enemy_piece, &mov.to);
                panic!("invalid move");
            }
        }

        // get piece to move
        let removed = self.remove_piece_at(&mov.from);
        if removed.is_none() {
            println!("{:?}", mov);
            panic!("trying to remove a piece that isn't there.");
        }
        let mut moving_piece = removed.unwrap();

        // if it gets promoted, then switch it's type
        if mov.promoted_to.is_some() && mov.piece == PieceType::Pawn {
            moving_piece.piece_type = mov.promoted_to.unwrap();
        }

        // move the piece ( update the piece and square )
        self.place_piece(moving_piece, &mov.to);

        // castling
        if mov.is_castling && mov.rook_from.is_some() && mov.rook_to.is_some() {
            // @todo : this doesn't really work , you want to be able to roll back multiple moves if needed,
            // because if this is used for searching then it'll be doing that
            match moving_piece.color {
                Color::White => {
                    self.white_castling_rights = CastlingRights::new(false, false);
                }
                Color::Black => {
                    self.black_castling_rights = CastlingRights::new(false, false);
                }
            }
            self.move_piece(mov.rook_from.as_ref().unwrap(), &mov.rook_to.unwrap());
        }

        //@todo check castling rights

        // update move counter
        if moving_piece.color == Color::Black {
            self.full_move_number = self.full_move_number + 1;
        }
    }

    fn unmake_move_mut(&mut self, mov: &Move) {
        let moving_piece = self.get_piece_at(&mov.to).unwrap().clone();

        self.move_piece(&mov.to, &mov.from);

        // replace captured piece
        // update 50 move rule draw counter @todo:::
        // if m.captured.is_none() || m.piece.piece_type != PieceType::Pawn {
        //     self.half_move_clock = self.half_move_clock + 1;
        // } else {
        //     self.half_move_clock = 0;
        // }

        if mov.captured.is_some() {
            // replace piece
            let square = self.get_square_mut(&mov.to);
            square.place_piece(Piece::new(
                moving_piece.color.opposite(),
                mov.captured.unwrap(),
                Some(mov.to.clone()),
            ));
        }

        // if it was promoted, then switch it's type
        if mov.promoted_to.is_some() {
            let mut piece = self.remove_piece(&moving_piece);
            piece.piece_type = mov.promoted_to.unwrap().clone();
        }

        // castling
        if mov.is_castling && mov.rook_from.is_some() && mov.rook_to.is_some() {
            match moving_piece.color {
                Color::White => {
                    self.white_castling_rights = self.previous_white_castling_rights.clone();
                }
                Color::Black => {
                    self.black_castling_rights = self.previous_black_castling_rights.clone();
                }
            }
            // move the rook back
            self.move_piece(mov.rook_to.as_ref().unwrap(), &mov.rook_from.unwrap());
        }
        // rollback the move counter
        if moving_piece.color == Color::Black {
            self.full_move_number = self.full_move_number - 1;
        }

        // update white to move flag
        self.player_to_move = self.player_to_move.opposite();
    }
    fn place_piece(&mut self, mut piece: Piece, at: &Coordinate) {
        if at.is_valid_coordinate() {
            piece.at = Some(at.clone());
            self.get_square_mut(&at).place_piece(piece)
        }
    }
    fn has_piece(&self, at: &Coordinate) -> bool {
        self.get_piece_at(at).is_some()
    }
    // fn get_pieces_in(&self, area: Vec<Coordinate>) -> Vec<(Coordinate, Option<&Piece>)> {
    //
    // }
    fn get_piece_at(&self, at: &Coordinate) -> Option<&Piece> {
        if !at.is_valid_coordinate() {
            return None;
        }
        self.get_square(at).piece.as_ref()
    }
    fn get_kings(&self) -> Vec<&Piece> {
        self.find_pieces(|&square| {
            square
                .piece
                .map_or(false, |piece| piece.piece_type == PieceType::King)
        })
    }

    fn get_pieces(&self, color: Color, piece_type: PieceType) -> Vec<&Piece> {
        self.find_pieces(|&square| {
            square.piece.map_or(false, |piece| {
                piece.piece_type == piece_type && piece.color == color
            })
        })
    }

    fn get_all_pieces(&self, color: Color) -> Vec<&Piece> {
        self.find_pieces(|&square| square.piece.map_or(false, |p| p.color == color))
    }
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
            white_castling_rights: CastlingRights::new(
                white_can_castle_king_side,
                white_can_castle_queen_side,
            ),
            black_castling_rights: CastlingRights::new(
                black_can_castle_king_side,
                black_can_castle_queen_side,
            ),
            previous_white_castling_rights: CastlingRights::new(
                white_can_castle_king_side,
                white_can_castle_queen_side,
            ),
            previous_black_castling_rights: CastlingRights::new(
                black_can_castle_king_side,
                black_can_castle_queen_side,
            ),
            en_passant_target,
            half_move_clock,
            full_move_number,
            squares: Board::make_squares(),
        }
    }
    pub fn new() -> Board {
        Board {
            player_to_move: Color::White,
            white_castling_rights: CastlingRights::new(true, true),
            black_castling_rights: CastlingRights::new(true, true),
            previous_white_castling_rights: CastlingRights::new(true, true),
            previous_black_castling_rights: CastlingRights::new(true, true),
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 0,
            squares: Board::make_squares(),
        }
    }
    pub fn from(board: &dyn BoardTrait) -> Board {
        let mut squares: Vec<Vec<Square>> = vec![];
        for row in board.get_squares() {
            let mut new_row: Vec<Square> = vec![];
            for square in row.iter() {
                new_row.push(Square::new(
                    square.coordinate.clone(),
                    square.piece.clone(),
                    square.color.clone(),
                ));
            }
            squares.push(new_row);
        }
        Board {
            player_to_move: board.player_to_move().clone(),
            white_castling_rights: board.white_castling_rights(),
            black_castling_rights: board.black_castling_rights(),
            previous_white_castling_rights: board.white_castling_rights(),
            previous_black_castling_rights: board.black_castling_rights(),
            en_passant_target: board.en_passant_target().clone(),
            half_move_clock: board.half_move_clock(),
            full_move_number: board.full_move_number(),
            squares,
        }
    }

    fn move_piece(&mut self, at: &Coordinate, to: &Coordinate) {
        // @todo : handle unwrap
        let moved_piece = self.remove_piece_at(at).unwrap();
        // move it back to where it was
        let square = self.get_square_mut(to);
        square.place_piece(moved_piece);
    }

    fn remove_piece_at(&mut self, at: &Coordinate) -> Option<Piece> {
        let square = self.get_square_mut(at);
        let piece = square.piece;
        square.piece = None;
        piece
    }

    fn find_square_mut<F>(&mut self, filter: F) -> Option<&mut Square>
    where
        F: Fn(&&mut Square) -> bool,
    {
        self.squares.iter_mut().flatten().find(filter)
    }

    fn find_pieces<F>(&self, filter: F) -> Vec<&Piece>
    where
        F: Fn(&&Square) -> bool,
    {
        self.squares
            .iter()
            .flatten()
            .filter(filter)
            .map(|square| square.piece.as_ref().unwrap())
            .collect()
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
                row.push(Square::new(Coordinate::new(x, y), None, color));
            }
            vec.push(row);
        }
        return vec;
    }
    fn _clone(&self) -> Board {
        let mut squares: Vec<Vec<Square>> = vec![];
        for row in self.squares.iter() {
            let mut new_row: Vec<Square> = vec![];
            for square in row.iter() {
                new_row.push(Square::new(
                    square.coordinate.clone(),
                    square.piece.clone(),
                    square.color.clone(),
                ));
            }
            squares.push(new_row);
        }
        Board {
            player_to_move: self.player_to_move,
            white_castling_rights: self.white_castling_rights.clone(),
            black_castling_rights: self.black_castling_rights.clone(),
            previous_white_castling_rights: self.white_castling_rights.clone(),
            previous_black_castling_rights: self.black_castling_rights.clone(),
            en_passant_target: self.en_passant_target.clone(),
            half_move_clock: self.half_move_clock,
            full_move_number: self.full_move_number,
            squares,
        }
    }
}
