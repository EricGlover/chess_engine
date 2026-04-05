use std::collections::HashMap;
use std::hash::Hash;

use crate::bit_board::{self, BitBoard};
use crate::board::{BoardTrait, CastlingRights, Color, Coordinate, Piece, PieceType, Square};
use crate::chess_notation::pgn::Game;
use crate::move_generator::Move;

// might be worthwhile to add pointers to things
// or add a list of moves
#[derive(Debug, Eq, PartialEq)]
pub struct GameState {
    player_to_move: Color,
    white_castling_rights: CastlingRights,
    black_castling_rights: CastlingRights,
    en_passant_target: Option<Coordinate>,
    half_move_clock: u32,
    full_move_number: u32,
    pub board: BitBoard,
    dirty_squares: bool,
    dirty_pieces: bool,
    squares: Vec<Square>,
    pieces: HashMap<u64, Piece>,
    _squares: [[Square; 8]; 8],
}

// @todo ::
impl BoardTrait for GameState {
    fn clone(&self) -> Box<dyn BoardTrait> {
        Box::new(GameState::starting_game())
    }

    // info about game going on
    fn player_to_move(&self) -> Color {
        self.player_to_move
    }
    fn en_passant_target(&self) -> Option<Coordinate> {
        self.en_passant_target
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
        self.white_castling_rights
    }
    fn black_castling_rights(&self) -> CastlingRights {
        self.black_castling_rights
    }

    // getting squares
    //@todo
    fn squares_list(&self) -> Vec<&Square> {
        let squares = Vec::new();
        return squares;
    }
    fn get_rank(&self, y: u8) -> Vec<&Square> {
        let squares = Vec::new();
        return squares;
    }
    fn get_files(&self) -> Vec<Vec<&Square>> {
        let files: Vec<Vec<&Square>> = Vec::new();
        return files;
    }
    // fn get_squares(&self) -> Vec<Vec<&Square>> {
    //     let squares: Vec<Vec<&Square>> = Vec::new();
    //     if self.dirty_squares {
    //         // self.init_squares();
    //     }
    //     return squares;
    // }

    // moves
    //@todo
    fn make_move_mut(&mut self, m: &Move) {}
    fn unmake_move_mut(&mut self, m: &Move) {}

    // getting and setting pieces
    fn place_piece(&mut self, piece: Piece, at: &Coordinate) {
        self.board.set_piece(piece.piece_type, piece.color, *at);
        let idx = BitBoard::coordinate_to_idx(*at);
        self.pieces.insert(idx, piece);
    }
    fn remove_piece(&mut self, piece: &Piece) -> Piece {
        if piece.at().is_none() {
            //error
            return Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(1, 1)));
        }
        let idx = BitBoard::coordinate_to_idx(*piece.at().unwrap());
        let piece = self.pieces.remove(&idx);
        return Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(1, 1)));
    }
    fn has_piece(&self, at: &Coordinate) -> bool {
        self.board.is_piece_at_coordinate(at)
    }
    //@todo figure out our piece list
    //@todo, board.get_piece_at -> Option<Piece>
    fn get_piece_at(&self, at: &Coordinate) -> Option<&Piece> {
        let idx = BitBoard::coordinate_to_idx(*at);
        return self.pieces.get(&idx);
    }
    fn get_kings(&self) -> Vec<&Piece> {
        let mut kings: Vec<&Piece> = Vec::new();
        for idx in self.board.get_piece_types_idx(PieceType::King) {
            let piece_opt = self.pieces.get(&idx);
            if piece_opt.is_some() {
                kings.push(piece_opt.unwrap());
            }
        }

        return kings;
    }
    fn get_pieces(&self, color: Color, piece_type: PieceType) -> Vec<&Piece> {
        let mut pieces: Vec<&Piece> = Vec::new();

        for idx in self.board.get_piece_types_by_color_idx(piece_type, color) {
            let piece_opt = self.pieces.get(&idx);
            if piece_opt.is_some() {
                pieces.push(piece_opt.unwrap());
            }
        }
        return pieces;
    }
    fn get_all_pieces(&self, color: Color) -> Vec<&Piece> {
        return self.pieces.values().collect();
    }

    fn get_castling_rights_changes_if_piece_moves(&self, piece: &Piece) -> Option<CastlingRights> {
        None
    }

    fn get_castling_rights_changes_if_piece_is_captured(
        &self,
        piece: &Piece,
    ) -> Option<CastlingRights> {
        None
    }
}

impl GameState {
    pub fn new() -> GameState {
        let mut g = GameState {
            player_to_move: Color::White,
            white_castling_rights: CastlingRights::new(true, true),
            black_castling_rights: CastlingRights::new(true, true),
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 1,
            board: BitBoard::new(),
            dirty_squares: true,
            dirty_pieces: true,
            squares: Vec::new(),
            pieces: HashMap::new(),
            _squares: GameState::make_squares(),
        };
        g.update_pieces();
        return g;
    }
    pub fn make_game_state(
        player_to_move: Color,
        white_can_castle_king_side: bool,
        white_can_castle_queen_side: bool,
        black_can_castle_king_side: bool,
        black_can_castle_queen_side: bool,
        en_passant_target: Option<Coordinate>,
        half_move_clock: u32,
        full_move_number: u32,
        board: BitBoard,
    ) -> GameState {
        let mut g = GameState {
            player_to_move,
            white_castling_rights: CastlingRights::new(
                white_can_castle_king_side,
                white_can_castle_queen_side,
            ),
            black_castling_rights: CastlingRights::new(
                black_can_castle_king_side,
                black_can_castle_queen_side,
            ),
            en_passant_target,
            half_move_clock,
            full_move_number,
            board,
            dirty_squares: true,
            dirty_pieces: true,
            squares: Vec::new(),
            pieces: HashMap::new(),
            _squares: GameState::make_squares(),
        };
        g.update_pieces();
        return g;
    }
    pub fn starting_game() -> GameState {
        let mut g = GameState {
            player_to_move: Color::White,
            white_castling_rights: CastlingRights::new(true, true),
            black_castling_rights: CastlingRights::new(true, true),
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 1,
            board: BitBoard::new(),
            dirty_squares: true,
            dirty_pieces: true,
            squares: Vec::new(),
            pieces: HashMap::new(),
            _squares: GameState::make_squares(),
        };
        g.update_pieces();
        return g;
    }

    // @note : definitely not the prettiest function every but whatever.
    pub fn update_pieces(&mut self) {
        if self.dirty_pieces {
            //empty the hash map
            let mut white_pawns: Vec<Piece> = Vec::new();
            let mut white_knights: Vec<Piece> = Vec::new();
            let mut white_rooks: Vec<Piece> = Vec::new();
            let mut white_bishops: Vec<Piece> = Vec::new();
            let mut white_queens: Vec<Piece> = Vec::new();
            let mut white_kings: Vec<Piece> = Vec::new();

            let mut black_pawns: Vec<Piece> = Vec::new();
            let mut black_knights: Vec<Piece> = Vec::new();
            let mut black_rooks: Vec<Piece> = Vec::new();
            let mut black_bishops: Vec<Piece> = Vec::new();
            let mut black_queens: Vec<Piece> = Vec::new();
            let mut black_kings: Vec<Piece> = Vec::new();

            ///
            for idx in (1..=64u64) {
                let v = self.pieces.remove(&idx);
                if v.is_some() {
                    let piece: Piece = v.unwrap();
                    if piece.color == Color::White {
                        match piece.piece_type {
                            PieceType::King => white_kings.push(piece),
                            PieceType::Queen => white_queens.push(piece),
                            PieceType::Bishop => white_bishops.push(piece),
                            PieceType::Knight => white_knights.push(piece),
                            PieceType::Rook => white_rooks.push(piece),
                            PieceType::Pawn => white_pawns.push(piece),
                        }
                    } else {
                        match piece.piece_type {
                            PieceType::King => black_kings.push(piece),
                            PieceType::Queen => black_queens.push(piece),
                            PieceType::Bishop => black_bishops.push(piece),
                            PieceType::Knight => black_knights.push(piece),
                            PieceType::Rook => black_rooks.push(piece),
                            PieceType::Pawn => black_pawns.push(piece),
                        }
                    }
                }
            }

            // get the indices of the where the pieces are from the bit-board
            // grab an appropriate piece from the list, update it's location
            // and set it in dictionary

            /** WHITE PIECES  */
            for idx in self
                .board
                .get_piece_types_by_color_idx(PieceType::Pawn, Color::White)
            {
                // grab a white pawn
                let opt = white_pawns.pop();
                let at = BitBoard::idx_to_coordinate(idx);
                if opt.is_some() {
                    let mut pawn = opt.unwrap();
                    pawn.set_at(at);
                    self.pieces.insert(idx, pawn);
                } else {
                    let pawn = Piece::new(Color::White, PieceType::Pawn, Some(at));
                    self.pieces.insert(idx, pawn);
                }
            }
            for idx in self
                .board
                .get_piece_types_by_color_idx(PieceType::Knight, Color::White)
            {
                // grab a white pawn
                let opt = white_knights.pop();
                let at = BitBoard::idx_to_coordinate(idx);
                if opt.is_some() {
                    let mut pawn = opt.unwrap();

                    pawn.set_at(at);
                    self.pieces.insert(idx, pawn);
                } else {
                    let pawn = Piece::new(Color::White, PieceType::Knight, Some(at));
                    self.pieces.insert(idx, pawn);
                }
            }
            for idx in self
                .board
                .get_piece_types_by_color_idx(PieceType::Bishop, Color::White)
            {
                // grab a white pawn
                let opt = white_bishops.pop();
                let at = BitBoard::idx_to_coordinate(idx);
                if opt.is_some() {
                    let mut pawn = opt.unwrap();

                    pawn.set_at(at);
                    self.pieces.insert(idx, pawn);
                } else {
                    let pawn = Piece::new(Color::White, PieceType::Bishop, Some(at));
                    self.pieces.insert(idx, pawn);
                }
            }
            for idx in self
                .board
                .get_piece_types_by_color_idx(PieceType::Rook, Color::White)
            {
                // grab a white pawn
                let opt = white_rooks.pop();
                let at = BitBoard::idx_to_coordinate(idx);
                if opt.is_some() {
                    let mut pawn = opt.unwrap();

                    pawn.set_at(at);
                    self.pieces.insert(idx, pawn);
                } else {
                    let pawn = Piece::new(Color::White, PieceType::Rook, Some(at));
                    self.pieces.insert(idx, pawn);
                }
            }
            for idx in self
                .board
                .get_piece_types_by_color_idx(PieceType::Queen, Color::White)
            {
                // grab a white pawn
                let opt = white_queens.pop();
                let at = BitBoard::idx_to_coordinate(idx);
                if opt.is_some() {
                    let mut pawn = opt.unwrap();

                    pawn.set_at(at);
                    self.pieces.insert(idx, pawn);
                } else {
                    let pawn = Piece::new(Color::White, PieceType::Queen, Some(at));
                    self.pieces.insert(idx, pawn);
                }
            }
            for idx in self
                .board
                .get_piece_types_by_color_idx(PieceType::King, Color::White)
            {
                // grab a white pawn
                let opt = white_kings.pop();
                let at = BitBoard::idx_to_coordinate(idx);
                if opt.is_some() {
                    let mut pawn = opt.unwrap();

                    pawn.set_at(at);
                    self.pieces.insert(idx, pawn);
                } else {
                    let pawn = Piece::new(Color::White, PieceType::King, Some(at));
                    self.pieces.insert(idx, pawn);
                }
            }
            /** BLACK PIECES  */
            for idx in self
                .board
                .get_piece_types_by_color_idx(PieceType::Pawn, Color::Black)
            {
                // grab a white pawn
                let opt = black_pawns.pop();
                let at = BitBoard::idx_to_coordinate(idx);
                if opt.is_some() {
                    let mut pawn = opt.unwrap();

                    pawn.set_at(at);
                    self.pieces.insert(idx, pawn);
                } else {
                    let pawn = Piece::new(Color::Black, PieceType::Pawn, Some(at));
                    self.pieces.insert(idx, pawn);
                }
            }
            for idx in self
                .board
                .get_piece_types_by_color_idx(PieceType::Knight, Color::Black)
            {
                // grab a white pawn
                let opt = black_knights.pop();
                let at = BitBoard::idx_to_coordinate(idx);
                if opt.is_some() {
                    let mut pawn = opt.unwrap();

                    pawn.set_at(at);
                    self.pieces.insert(idx, pawn);
                } else {
                    let pawn = Piece::new(Color::Black, PieceType::Knight, Some(at));
                    self.pieces.insert(idx, pawn);
                }
            }
            for idx in self
                .board
                .get_piece_types_by_color_idx(PieceType::Bishop, Color::Black)
            {
                // grab a white pawn
                let opt = black_bishops.pop();
                let at = BitBoard::idx_to_coordinate(idx);
                if opt.is_some() {
                    let mut pawn = opt.unwrap();

                    pawn.set_at(at);
                    self.pieces.insert(idx, pawn);
                } else {
                    let pawn = Piece::new(Color::Black, PieceType::Bishop, Some(at));
                    self.pieces.insert(idx, pawn);
                }
            }
            for idx in self
                .board
                .get_piece_types_by_color_idx(PieceType::Rook, Color::Black)
            {
                // grab a white pawn
                let opt = black_rooks.pop();
                let at = BitBoard::idx_to_coordinate(idx);
                if opt.is_some() {
                    let mut pawn = opt.unwrap();

                    pawn.set_at(at);
                    self.pieces.insert(idx, pawn);
                } else {
                    let pawn = Piece::new(Color::Black, PieceType::Rook, Some(at));
                    self.pieces.insert(idx, pawn);
                }
            }
            for idx in self
                .board
                .get_piece_types_by_color_idx(PieceType::Queen, Color::Black)
            {
                // grab a white pawn
                let opt = black_queens.pop();
                let at = BitBoard::idx_to_coordinate(idx);
                if opt.is_some() {
                    let mut pawn = opt.unwrap();

                    pawn.set_at(at);
                    self.pieces.insert(idx, pawn);
                } else {
                    let pawn = Piece::new(Color::Black, PieceType::Queen, Some(at));
                    self.pieces.insert(idx, pawn);
                }
            }
            for idx in self
                .board
                .get_piece_types_by_color_idx(PieceType::King, Color::Black)
            {
                // grab a white pawn
                let opt = black_kings.pop();
                let at = BitBoard::idx_to_coordinate(idx);
                if opt.is_some() {
                    let mut pawn = opt.unwrap();

                    pawn.set_at(at);
                    self.pieces.insert(idx, pawn);
                } else {
                    let pawn = Piece::new(Color::Black, PieceType::King, Some(at));
                    self.pieces.insert(idx, pawn);
                }
            }
            self.dirty_pieces = false;
        }
    }

    pub fn get_en_passant_target(&self) -> Option<Coordinate> {
        self.en_passant_target
    }

    pub fn get_castling_rights(&self, color: Color) -> &CastlingRights {
        return match color {
            Color::White => &self.white_castling_rights,
            Color::Black => &self.black_castling_rights,
        };
    }

    pub fn get_castling_rights_changes_if_piece_moves(
        &self,
        piece: &Piece,
    ) -> Option<CastlingRights> {
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
    fn init_squares(mut self) {
        //@todo
    }

    pub fn make_squares() -> [[Square; 8]; 8] {
        [
            [
                Square::new(Coordinate::new(1, 1), None, Color::Black),
                Square::new(Coordinate::new(2, 1), None, Color::White),
                Square::new(Coordinate::new(3, 1), None, Color::Black),
                Square::new(Coordinate::new(4, 1), None, Color::White),
                Square::new(Coordinate::new(5, 1), None, Color::Black),
                Square::new(Coordinate::new(6, 1), None, Color::White),
                Square::new(Coordinate::new(7, 1), None, Color::Black),
                Square::new(Coordinate::new(8, 1), None, Color::White),
            ],
            [
                Square::new(Coordinate::new(1, 2), None, Color::White),
                Square::new(Coordinate::new(2, 2), None, Color::Black),
                Square::new(Coordinate::new(3, 2), None, Color::White),
                Square::new(Coordinate::new(4, 2), None, Color::Black),
                Square::new(Coordinate::new(5, 2), None, Color::White),
                Square::new(Coordinate::new(6, 2), None, Color::Black),
                Square::new(Coordinate::new(7, 2), None, Color::White),
                Square::new(Coordinate::new(8, 2), None, Color::Black),
            ],
            [
                Square::new(Coordinate::new(1, 3), None, Color::Black),
                Square::new(Coordinate::new(2, 3), None, Color::White),
                Square::new(Coordinate::new(3, 3), None, Color::Black),
                Square::new(Coordinate::new(4, 3), None, Color::White),
                Square::new(Coordinate::new(5, 3), None, Color::Black),
                Square::new(Coordinate::new(6, 3), None, Color::White),
                Square::new(Coordinate::new(7, 3), None, Color::Black),
                Square::new(Coordinate::new(8, 3), None, Color::White),
            ],
            [
                Square::new(Coordinate::new(1, 4), None, Color::White),
                Square::new(Coordinate::new(2, 4), None, Color::Black),
                Square::new(Coordinate::new(3, 4), None, Color::White),
                Square::new(Coordinate::new(4, 4), None, Color::Black),
                Square::new(Coordinate::new(5, 4), None, Color::White),
                Square::new(Coordinate::new(6, 4), None, Color::Black),
                Square::new(Coordinate::new(7, 4), None, Color::White),
                Square::new(Coordinate::new(8, 4), None, Color::Black),
            ],
            [
                Square::new(Coordinate::new(1, 5), None, Color::Black),
                Square::new(Coordinate::new(2, 5), None, Color::White),
                Square::new(Coordinate::new(3, 5), None, Color::Black),
                Square::new(Coordinate::new(4, 5), None, Color::White),
                Square::new(Coordinate::new(5, 5), None, Color::Black),
                Square::new(Coordinate::new(6, 5), None, Color::White),
                Square::new(Coordinate::new(7, 5), None, Color::Black),
                Square::new(Coordinate::new(8, 5), None, Color::White),
            ],
            [
                Square::new(Coordinate::new(1, 6), None, Color::White),
                Square::new(Coordinate::new(2, 6), None, Color::Black),
                Square::new(Coordinate::new(3, 6), None, Color::White),
                Square::new(Coordinate::new(4, 6), None, Color::Black),
                Square::new(Coordinate::new(5, 6), None, Color::White),
                Square::new(Coordinate::new(6, 6), None, Color::Black),
                Square::new(Coordinate::new(7, 6), None, Color::White),
                Square::new(Coordinate::new(8, 6), None, Color::Black),
            ],
            [
                Square::new(Coordinate::new(1, 7), None, Color::Black),
                Square::new(Coordinate::new(2, 7), None, Color::White),
                Square::new(Coordinate::new(3, 7), None, Color::Black),
                Square::new(Coordinate::new(4, 7), None, Color::White),
                Square::new(Coordinate::new(5, 7), None, Color::Black),
                Square::new(Coordinate::new(6, 7), None, Color::White),
                Square::new(Coordinate::new(7, 7), None, Color::Black),
                Square::new(Coordinate::new(8, 7), None, Color::White),
            ],
            [
                Square::new(Coordinate::new(1, 8), None, Color::White),
                Square::new(Coordinate::new(2, 8), None, Color::Black),
                Square::new(Coordinate::new(3, 8), None, Color::White),
                Square::new(Coordinate::new(4, 8), None, Color::Black),
                Square::new(Coordinate::new(5, 8), None, Color::White),
                Square::new(Coordinate::new(6, 8), None, Color::Black),
                Square::new(Coordinate::new(7, 8), None, Color::White),
                Square::new(Coordinate::new(8, 8), None, Color::Black),
            ],
        ]
    }
}
