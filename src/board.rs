mod castling_rights;
mod color;
mod coordinate;
mod piece;
mod piece_type;
mod square;

use crate::board_console_printer::print_board;
use crate::chess_notation::fen_reader;
use crate::move_generator::{gen_pseudo_legal_moves, Move, MoveType};
pub use castling_rights::CastlingRights;
pub use color::Color;
pub use coordinate::*;
pub use piece::Piece;
pub use piece_type::PieceType;
pub use square::Square;
use std::fmt;
use std::fmt::Formatter;

// getting pieces && squares return references
pub trait BoardTrait {
    fn clone(&self) -> Box<dyn BoardTrait>;

    // info about game going on
    fn player_to_move(&self) -> Color;
    fn en_passant_target(&self) -> Option<Coordinate>;
    fn half_move_clock(&self) -> u32;
    fn full_move_number(&self) -> u32;
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
    fn get_castling_rights_changes_if_piece_moves(&self, piece: &Piece) -> Option<CastlingRights>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Board {
    player_to_move: Color,
    white_castling_rights: CastlingRights,
    black_castling_rights: CastlingRights,
    previous_white_castling_rights: CastlingRights, // used in unmake move
    previous_black_castling_rights: CastlingRights, // used in unmake move
    en_passant_target: Option<Coordinate>,
    half_move_clock: u32,
    full_move_number: u32,
    squares: Vec<Vec<Square>>,
}

//@todo : piece.at
impl BoardTrait for Board {
    // fn clone(&self) -> Board {
    //
    // }
    fn clone(&self) -> Box<dyn BoardTrait> {
        Box::new(Board {
            player_to_move: self.player_to_move,
            white_castling_rights: self.white_castling_rights.clone(),
            black_castling_rights: self.black_castling_rights.clone(),
            previous_white_castling_rights: self.white_castling_rights.clone(),
            previous_black_castling_rights: self.black_castling_rights.clone(),
            en_passant_target: self.en_passant_target.clone(),
            half_move_clock: self.half_move_clock,
            full_move_number: self.full_move_number,
            squares: self.clone_squares(),
        })
    }
    fn player_to_move(&self) -> Color {
        self.player_to_move
    }
    fn en_passant_target(&self) -> Option<Coordinate> {
        self.en_passant_target.clone()
    }
    fn half_move_clock(&self) -> u32 {
        self.half_move_clock
    }
    fn full_move_number(&self) -> u32 {
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

        match mov.move_type() {
            MoveType::Castling { rook_from, rook_to } => {
                self.move_piece(rook_from, rook_to);
            }
            // if it gets promoted, then switch it's type
            MoveType::Promotion(promoted_to) => {
                moving_piece.piece_type = promoted_to.clone();
            }
            MoveType::EnPassant => {}
            MoveType::Move => {}
        }

        if !mov.castling_rights_removed().none() {
            let removed = mov.castling_rights_removed();
            if removed.king_side() {
                match moving_piece.color {
                    Color::White => {
                        *self.white_castling_rights.king_side_mut() = false;
                    }
                    Color::Black => {
                        *self.black_castling_rights.king_side_mut() = false;
                    }
                }
            }
            if removed.queen_side() {
                match moving_piece.color {
                    Color::White => {
                        *self.white_castling_rights.queen_side_mut() = false;
                    }
                    Color::Black => {
                        *self.black_castling_rights.queen_side_mut() = false;
                    }
                }
            }
        }

        // move the piece ( update the piece and square )
        self.place_piece(moving_piece, &mov.to);

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

        match mov.move_type() {
            MoveType::Castling { rook_from, rook_to } => {
                match moving_piece.color {
                    Color::White => {
                        self.white_castling_rights = self.previous_white_castling_rights.clone();
                    }
                    Color::Black => {
                        self.black_castling_rights = self.previous_black_castling_rights.clone();
                    }
                }
                // move the rook back
                self.move_piece(rook_to, rook_from);
            }
            // if it gets promoted, then switch it's type
            MoveType::Promotion(_) => {
                let mut piece = self.remove_piece(&moving_piece);
                piece.piece_type = PieceType::Pawn;
            }
            MoveType::EnPassant => {}
            MoveType::Move => {}
        }

        if !mov.castling_rights_removed().none() {
            let removed = mov.castling_rights_removed();
            if removed.king_side() {
                match moving_piece.color {
                    Color::White => {
                        *self.white_castling_rights.king_side_mut() = true;
                    }
                    Color::Black => {
                        *self.black_castling_rights.king_side_mut() = true;
                    }
                }
            }
            if removed.queen_side() {
                match moving_piece.color {
                    Color::White => {
                        *self.white_castling_rights.queen_side_mut() = true;
                    }
                    Color::Black => {
                        *self.black_castling_rights.queen_side_mut() = true;
                    }
                }
            }
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
            piece.set_at(at.clone());
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
        self.get_square(at).piece()
    }
    fn get_kings(&self) -> Vec<&Piece> {
        self.find_pieces(|&square| {
            square
                .piece()
                .map_or(false, |piece| piece.piece_type == PieceType::King)
        })
    }

    fn get_pieces(&self, color: Color, piece_type: PieceType) -> Vec<&Piece> {
        self.find_pieces(|&square| {
            square.piece().map_or(false, |piece| {
                piece.piece_type == piece_type && piece.color == color
            })
        })
    }

    fn get_all_pieces(&self, color: Color) -> Vec<&Piece> {
        self.find_pieces(|&square| square.piece().map_or(false, |p| p.color == color))
    }

    fn get_castling_rights_changes_if_piece_moves(&self, piece: &Piece) -> Option<CastlingRights> {
        let current = match piece.color {
            Color::White => self.white_castling_rights,
            Color::Black => self.black_castling_rights,
        };
        if current.none() {
            None
        } else if piece.piece_type == PieceType::King {
            Some(CastlingRights::new(
                current.king_side(),
                current.queen_side(),
            ))
        } else if piece.piece_type == PieceType::Rook {
            // which rook bro ?
            if current.king_side() && piece.at().unwrap().x() == 8 {
                Some(CastlingRights::new(true, false))
            } else if current.queen_side() && piece.at().unwrap().x() == 1 {
                Some(CastlingRights::new(false, true))
            } else {
                None
            }
        } else {
            None
        }
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
        half_move_clock: u32,
        full_move_number: u32,
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
        let squares = board
            .get_squares()
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|square| {
                        Square::new(
                            square.coordinate().clone(),
                            square.piece().map(|p| p.clone()),
                            square.color().clone(),
                        )
                    })
                    .collect()
            })
            .collect();
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
        self.get_square_mut(at).remove_piece()
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
            .map(|square| square.piece().unwrap())
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
                row.push(Square::new(Coordinate::new(x, y), None, color));
            }
            vec.push(row);
        }
        return vec;
    }
    fn clone_squares(&self) -> Vec<Vec<Square>> {
        self.squares
            .iter()
            .map(|row| {
                row.iter()
                    .map(|square| {
                        Square::new(
                            square.coordinate().clone(),
                            square.piece().map(|p| p.clone()),
                            square.color().clone(),
                        )
                    })
                    .collect()
            })
            .collect()
    }
    fn _clone(&self) -> Board {
        Board {
            player_to_move: self.player_to_move,
            white_castling_rights: self.white_castling_rights.clone(),
            black_castling_rights: self.black_castling_rights.clone(),
            previous_white_castling_rights: self.white_castling_rights.clone(),
            previous_black_castling_rights: self.black_castling_rights.clone(),
            en_passant_target: self.en_passant_target.clone(),
            half_move_clock: self.half_move_clock,
            full_move_number: self.full_move_number,
            squares: self.clone_squares(),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::board::*;
    use crate::chess_notation::fen_reader;
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
    fn test_castling_rights() {
        //@todo : use pgn for a comprehensive test
        let mut board = fen_reader::make_board(
            "r3kbnr/ppp2ppp/2n5/3ppb2/2BPP2q/2N5/PPP1NPPP/R1BQK2R w KQkq - 6 6",
        );
        let white_castles = Move::castle_king_side(Color::White);
        let black_castles = Move::castle_queen_side(Color::Black);
        let white_rook = board.get_piece_at(&Coordinate::new(8, 1)).unwrap();
        let white_rook_move = Move::new(
            white_rook.at().unwrap().clone(),
            Coordinate::new(7, 1),
            PieceType::Rook,
            MoveType::Move,
            None,
            board.get_castling_rights_changes_if_piece_moves(white_rook),
        );

        // if we castle rights be gone
        // white
        let old_rights = board.white_castling_rights;
        board.make_move_mut(&white_castles);
        assert!(board.white_castling_rights.none());
        board.unmake_move_mut(&white_castles);
        assert!(board.white_castling_rights.both());

        // black
        let old_rights = board.black_castling_rights;
        board.make_move_mut(&black_castles);
        assert!(board.black_castling_rights.none());
        board.unmake_move_mut(&black_castles);
        assert!(board.black_castling_rights.both());

        // if we move a rook we can't use it to castle
        let old_rights = board.white_castling_rights;
        board.make_move_mut(&white_rook_move);
        assert_eq!(
            board.white_castling_rights,
            CastlingRights::new(false, true)
        );
        board.unmake_move_mut(&white_rook_move);
        assert!(board.white_castling_rights.both());
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
        assert_eq!(square.unwrap().coordinate(), &at, "at 1 1");
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
                assert_eq!((i + 1) as u8, s.coordinate().y());
                assert_eq!((j + 1) as u8, s.coordinate().x());
            }
        }
    }
}
