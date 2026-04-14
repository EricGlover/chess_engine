use std::collections::HashMap;

use rand::seq::index;

use crate::bit_board::BitBoard;
use crate::board::{BoardTrait, CastlingRights, Color, Coordinate, Piece, PieceType, Square};
use crate::chess_notation::pgn::Game;
use crate::game_state;
use crate::move_generator::{gen_legal_moves, plmg, Move, MoveType};

// might be worthwhile to add pointers to things
// or add a list of moves
#[derive(Debug, Eq, PartialEq)]
pub struct GameState {
    player_to_move: Color,
    white_castling_rights: CastlingRights,
    black_castling_rights: CastlingRights,
    en_passant_target: Option<Coordinate>,
    half_move_clock: u16,
    full_move_number: u16,
    board: BitBoard,
    dirty_squares: bool,
    dirty_pieces: bool,
    squares: Vec<Square>,
    pieces: HashMap<u8, Piece>,
    is_drawn: bool,
}

// @todo :: test
impl BoardTrait for GameState {
    fn clone(&self) -> Box<dyn BoardTrait> {
        Box::new(self.clone_to_game_state())
    }

    // info about game going on
    fn player_to_move(&self) -> Color {
        self.player_to_move
    }
    fn en_passant_target(&self) -> Option<Coordinate> {
        self.en_passant_target
    }
    fn half_move_clock(&self) -> u16 {
        self.half_move_clock
    }
    fn full_move_number(&self) -> u16 {
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
        self.squares.iter().collect()
    }
    fn get_rank(&self, y: u8) -> Vec<&Square> {
        let mut rank: Vec<&Square> = Vec::new();
        for x in 1..=8u8 {
            let c = Coordinate::new(x, y);
            let idx = BitBoard::coordinate_to_idx(c);
            rank.push(&self.squares[(idx - 1) as usize]);
        }
        return rank;
    }
    fn get_files(&self) -> Vec<Vec<&Square>> {
        let mut files: Vec<Vec<&Square>> = Vec::new();
        for x in 1..=8u8 {
            let mut file: Vec<&Square> = vec![];
            for y in 1..=8u8 {
                let c = Coordinate::new(x as u8, y as u8);
                let idx = BitBoard::coordinate_to_idx(c);
                file.push(&self.squares[(idx - 1) as usize]);
            }
            files.push(file);
        }
        return files;
    }

    // moves
    fn make_move_mut(&mut self, mut m: &mut Move) {
        let from_idx = BitBoard::coordinate_to_idx(m.from);
        let to_idx = BitBoard::coordinate_to_idx(m.to);
        if let Some(mut piece_to_move) = self.pieces.remove(&from_idx) {
            // update white to move flag
            self.player_to_move = piece_to_move.color.opposite();

            // update 50 move rule draw counter
            if m.captured.is_none() && piece_to_move.piece_type != PieceType::Pawn {
                self.half_move_clock = self.half_move_clock + 1;
            } else {
                self.half_move_clock = 0;
            }

            if self.half_move_clock >= 50 {
                self.is_drawn = true;
            }

            // update castling rights
            if m.castling_rights_removed().some() {
                let removed = m.castling_rights_removed();
                if removed.king_side() {
                    match piece_to_move.color {
                        Color::White => {
                            *self.white_castling_rights.king_side_mut() = false;
                        }
                        Color::Black => {
                            *self.black_castling_rights.king_side_mut() = false;
                        }
                    }
                }
                if removed.queen_side() {
                    match piece_to_move.color {
                        Color::White => {
                            *self.white_castling_rights.queen_side_mut() = false;
                        }
                        Color::Black => {
                            *self.black_castling_rights.queen_side_mut() = false;
                        }
                    }
                }
            }

            // update move counter
            if piece_to_move.color == Color::Black {
                self.full_move_number = self.full_move_number + 1;
            }

            // update draw clock
            m.old_half_move_clock = Some(self.half_move_clock);

            // do any special logic,
            match m.move_type() {
                MoveType::Castling { rook_from, rook_to } => {
                    // self.move_piece(rook_from, rook_to);
                    let mut rook_to_move = self.remove_piece_at(rook_from);
                    self.place_piece(rook_to_move, rook_to);
                }
                // if it gets promoted, then switch it's type
                MoveType::Promotion(promoted_to) => {
                    piece_to_move.piece_type = promoted_to.clone();
                }
                MoveType::EnPassant => {}
                MoveType::Move => {}
            }
            // remove the captured piece
            if m.captured.is_some() {
                let mut captured_at = m.to;
                if m.move_type() == &MoveType::EnPassant {
                    // println!("trying to en passant {}", m);
                    captured_at = plmg::get_en_passant_piece(
                        &piece_to_move,
                        &self.en_passant_target.unwrap(),
                    );
                    // println!("{}", captured_at);
                }
                let removed_piece = self.remove_piece_at(&captured_at);
            }

            // en passant
            m.old_en_passant_target = self.en_passant_target;
            self.en_passant_target = m.en_passant_target;

            // move the piece ( update the piece, piece map , square, and board )

            self.pieces.insert(from_idx, piece_to_move);
            self.remove_piece(&piece_to_move);
            self.place_piece(piece_to_move, &m.to);
        } else {
            println!("{:?}", m);
            panic!("trying to remove a piece that isn't there.");
        }
    }
    fn unmake_move_mut(&mut self, mut m: &mut Move) {
        let from_idx = BitBoard::coordinate_to_idx(m.from);
        let to_idx = BitBoard::coordinate_to_idx(m.to);
        if let Some(mut piece_to_move) = self.pieces.remove(&to_idx) {
            // roll back white to move flag
            self.player_to_move = piece_to_move.color;

            // roll back castling rights changes
            if m.castling_rights_removed().some() {
                let removed = m.castling_rights_removed();
                if removed.king_side() {
                    match piece_to_move.color {
                        Color::White => {
                            *self.white_castling_rights.king_side_mut() = true;
                        }
                        Color::Black => {
                            *self.black_castling_rights.king_side_mut() = true;
                        }
                    }
                }
                if removed.queen_side() {
                    match piece_to_move.color {
                        Color::White => {
                            *self.white_castling_rights.queen_side_mut() = true;
                        }
                        Color::Black => {
                            *self.black_castling_rights.queen_side_mut() = true;
                        }
                    }
                }
            }

            // roll back move counter
            if piece_to_move.color == Color::Black {
                self.full_move_number = self.full_move_number - 1;
            }

            // roll back en passant
            self.en_passant_target = m.old_en_passant_target;
            // roll back half move clock
            self.half_move_clock = m.old_half_move_clock.unwrap();

            // special logic needed for different move types
            match m.move_type() {
                MoveType::Castling { rook_from, rook_to } => {
                    // move the rook back
                    let mut rook_to_move = self.remove_piece_at(rook_to);
                    self.place_piece(rook_to_move, rook_from);
                }
                // if it gets promoted, then switch it's type
                MoveType::Promotion(promoted_to) => {
                    piece_to_move.piece_type = PieceType::Pawn;
                }
                MoveType::EnPassant => {}
                MoveType::Move => {}
            }
            // add it to our hash map before we remove it from everything else
            self.pieces.insert(to_idx, piece_to_move);
            let piece_to_move = self.remove_piece(&piece_to_move);

            // handle putting back captured pieces
            if m.captured.is_some() {
                if m.move_type() == &MoveType::EnPassant {
                    let en_passant_at = Coordinate::new(m.to.x(), m.from.y());
                    // place down the captured piece
                    self.place_piece(
                        Piece::new(
                            piece_to_move.color.opposite(),
                            PieceType::Pawn,
                            Some(en_passant_at),
                        ),
                        &en_passant_at,
                    );
                } else {
                    // place down the captured piece
                    let mut p = Piece::new(
                        piece_to_move.color.opposite(),
                        m.captured.unwrap(),
                        Some(m.to),
                    );
                    self.place_piece(p, &m.to);
                }
            }

            // move the piece ( update the piece, piece map , square, and board )
            self.place_piece(piece_to_move, &m.from);
        } else {
            println!("{:?}", m);
            panic!("trying to remove a piece that isn't there.");
        }
    }

    // getting and setting pieces
    // update the piece, piece map , square, and board
    fn place_piece(&mut self, mut piece: Piece, at: &Coordinate) {
        let idx = BitBoard::coordinate_to_idx(*at);
        let existing_piece = self.pieces.get(&idx);
        if existing_piece.is_some() {
            panic!(
                "tried to place a piece at {}, but {} is already there.",
                at,
                existing_piece.unwrap()
            );
        }

        piece.set_at(*at);
        self.board.set_piece(piece.piece_type, piece.color, *at);

        self.pieces.insert(idx, piece);
        if let Some(square) = self.squares.get_mut((idx - 1) as usize) {
            square.set_piece_to(&piece);
        } else {
            panic!("Tried to place piece {}\n...square not found", piece);
        }
    }
    // update the piece, piece map , square, and board
    fn remove_piece(&mut self, piece: &Piece) -> Piece {
        if piece.at().is_none() {
            //error
            panic!("tried to remove piece without coordinate \n{}", piece);
            // return Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(1, 1)));
        }
        let at = *piece.at().unwrap();
        return self.remove_piece_at(&at);
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
        return self.pieces.values().filter(|&p| p.color == color).collect();
    }

    fn get_castling_rights_changes_if_piece_moves(&self, piece: &Piece) -> Option<CastlingRights> {
        let current = match piece.color {
            Color::White => self.white_castling_rights,
            Color::Black => self.black_castling_rights,
        };
        if current.none() {
            return None;
        }
        if let Some(at) = piece.at() {
            if piece.piece_type == PieceType::King {
                return Some(CastlingRights::new(
                    current.king_side(),
                    current.queen_side(),
                ));
            } else if piece.piece_type == PieceType::Rook {
                // which rook bro ?
                if current.king_side() && at.x() == 8 {
                    return Some(CastlingRights::new(true, false));
                } else if current.queen_side() && at.x() == 1 {
                    return Some(CastlingRights::new(false, true));
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            return None;
        }
    }

    fn get_castling_rights_changes_if_piece_is_captured(
        &self,
        piece: &Piece,
    ) -> Option<CastlingRights> {
        self.get_castling_rights_changes_if_piece_moves(piece)
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
            is_drawn: false,
        };
        g.update_pieces();
        g.update_squares();
        return g;
    }
    pub fn make_game_state(
        player_to_move: Color,
        white_can_castle_king_side: bool,
        white_can_castle_queen_side: bool,
        black_can_castle_king_side: bool,
        black_can_castle_queen_side: bool,
        en_passant_target: Option<Coordinate>,
        half_move_clock: u16,
        full_move_number: u16,
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
            is_drawn: false,
        };
        g.update_pieces();
        g.update_squares();
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
            is_drawn: false,
        };
        g.update_pieces();
        g.update_squares();
        return g;
    }

    pub fn clone_to_game_state(&self) -> GameState {
        GameState {
            player_to_move: self.player_to_move,
            white_castling_rights: self.white_castling_rights,
            black_castling_rights: self.black_castling_rights,
            en_passant_target: self.en_passant_target,
            half_move_clock: self.half_move_clock,
            full_move_number: self.full_move_number,
            board: self.board.clone(),
            dirty_squares: self.dirty_squares,
            dirty_pieces: self.dirty_pieces,
            squares: self.squares.iter().map(|s| s._clone()).collect(),
            pieces: self.pieces.clone(),
            is_drawn: false,
        }
    }
    pub fn get_is_draw(&self) -> bool {
        self.is_drawn
    }
    pub fn get_player_to_move(&self) -> Color {
        self.player_to_move
    }

    pub fn get_half_move_clock(&self) -> u16 {
        self.half_move_clock
    }
    pub fn get_full_move_number(&self) -> u16 {
        self.full_move_number
    }
    pub fn get_board(&self) -> BitBoard {
        self.board.clone()
    }
    pub fn get_board_ref(&self) -> &BitBoard {
        &self.board
    }
    pub fn get_dirty_squares(&self) -> bool {
        self.dirty_squares
    }
    pub fn get_dirty_pieces(&self) -> bool {
        self.dirty_pieces
    }

    pub fn get_king(&self, color: Color) -> Option<&Piece> {
        let indices = self
            .board
            .get_piece_types_by_color_idx(PieceType::King, color);
        if let Some(idx) = indices.get(0) {
            if let Some(king) = self.pieces.get(idx) {
                return Some(king);
            }
        }
        return None;
    }

    pub fn get_piece_at_idx(&self, idx: u8) -> Option<&Piece> {
        self.pieces.get(&idx)
    }

    fn remove_piece_at(&mut self, at: &Coordinate) -> Piece {
        let idx = BitBoard::coordinate_to_idx(*at);
        // update piece map
        let piece_opt = self.pieces.remove(&idx);
        if let Some(piece) = piece_opt {
            // update bit_board
            self.board
                .remove_piece(piece.piece_type, piece.color, piece.at().unwrap().clone());

            // update squares
            if let Some(square) = self.squares.get_mut((idx - 1) as usize) {
                square.remove_piece();
            } else {
                panic!("Tried to remove piece {}\n...square not found", piece);
            }
            if piece_opt.is_none() {
                panic!("piece not found {}", idx);
            }
            let mut piece = piece_opt.unwrap();
            piece.remove();
            return piece;
        } else {
            panic!("tried to remove piece \npiece not found at idx {}", idx);
        }
    }

    pub fn get_piece_list(&self) -> Vec<&Piece> {
        // if self.dirty_pieces {
        //     self.update_pieces();
        // }
        return self.pieces.values().collect();
    }

    pub fn get_piece_map(&self) -> HashMap<u8, Piece> {
        return self.pieces.clone();
    }

    pub fn update_squares(&mut self) {
        if self.dirty_squares {
            if self.squares.len() == 0 {
                self.squares = GameState::make_squares()
            }
            //place pieces
            for (idx, piece) in self.pieces.iter() {
                if let Some(square) = self.squares.get_mut((idx - 1) as usize) {
                    square.set_piece_to(piece);
                } else {
                    panic!("{} {}", idx, piece);
                }
            }
        }
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

            for idx in 1..=64u8 {
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

            /* WHITE PIECES  */
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
            /* BLACK PIECES  */
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
            if current.both() {
                Some(CastlingRights::new(true, true))
            } else if current.king_side() {
                Some(CastlingRights::new(true, false))
            } else if current.queen_side() {
                Some(CastlingRights::new(false, true))
            } else {
                None
            }
        } else if piece.piece_type == PieceType::Rook {
            // which rook bro ?
            let piece_at = piece.at().unwrap();
            let rook_at_kingside = match piece.color {
                Color::White => &Coordinate::new(8, 1) == piece_at,
                Color::Black => &Coordinate::new(8, 8) == piece_at,
            };
            let rook_at_queenside = match piece.color {
                Color::White => &Coordinate::new(1, 1) == piece_at,
                Color::Black => &Coordinate::new(1, 8) == piece_at,
            };
            if current.king_side() && rook_at_kingside {
                Some(CastlingRights::new(true, false))
            } else if current.queen_side() && rook_at_queenside {
                Some(CastlingRights::new(false, true))
            } else {
                None
            }
        } else {
            None
        }
    }

    // note squares[idx], bit_board[idx_b] where idx + 1 = idx_b = Coordinate(blah)
    pub fn make_squares() -> Vec<Square> {
        vec![
            Square::new(Coordinate::new(1, 1), None, Color::Black),
            Square::new(Coordinate::new(2, 1), None, Color::White),
            Square::new(Coordinate::new(3, 1), None, Color::Black),
            Square::new(Coordinate::new(4, 1), None, Color::White),
            Square::new(Coordinate::new(5, 1), None, Color::Black),
            Square::new(Coordinate::new(6, 1), None, Color::White),
            Square::new(Coordinate::new(7, 1), None, Color::Black),
            Square::new(Coordinate::new(8, 1), None, Color::White),
            Square::new(Coordinate::new(1, 2), None, Color::White),
            Square::new(Coordinate::new(2, 2), None, Color::Black),
            Square::new(Coordinate::new(3, 2), None, Color::White),
            Square::new(Coordinate::new(4, 2), None, Color::Black),
            Square::new(Coordinate::new(5, 2), None, Color::White),
            Square::new(Coordinate::new(6, 2), None, Color::Black),
            Square::new(Coordinate::new(7, 2), None, Color::White),
            Square::new(Coordinate::new(8, 2), None, Color::Black),
            Square::new(Coordinate::new(1, 3), None, Color::Black),
            Square::new(Coordinate::new(2, 3), None, Color::White),
            Square::new(Coordinate::new(3, 3), None, Color::Black),
            Square::new(Coordinate::new(4, 3), None, Color::White),
            Square::new(Coordinate::new(5, 3), None, Color::Black),
            Square::new(Coordinate::new(6, 3), None, Color::White),
            Square::new(Coordinate::new(7, 3), None, Color::Black),
            Square::new(Coordinate::new(8, 3), None, Color::White),
            Square::new(Coordinate::new(1, 4), None, Color::White),
            Square::new(Coordinate::new(2, 4), None, Color::Black),
            Square::new(Coordinate::new(3, 4), None, Color::White),
            Square::new(Coordinate::new(4, 4), None, Color::Black),
            Square::new(Coordinate::new(5, 4), None, Color::White),
            Square::new(Coordinate::new(6, 4), None, Color::Black),
            Square::new(Coordinate::new(7, 4), None, Color::White),
            Square::new(Coordinate::new(8, 4), None, Color::Black),
            Square::new(Coordinate::new(1, 5), None, Color::Black),
            Square::new(Coordinate::new(2, 5), None, Color::White),
            Square::new(Coordinate::new(3, 5), None, Color::Black),
            Square::new(Coordinate::new(4, 5), None, Color::White),
            Square::new(Coordinate::new(5, 5), None, Color::Black),
            Square::new(Coordinate::new(6, 5), None, Color::White),
            Square::new(Coordinate::new(7, 5), None, Color::Black),
            Square::new(Coordinate::new(8, 5), None, Color::White),
            Square::new(Coordinate::new(1, 6), None, Color::White),
            Square::new(Coordinate::new(2, 6), None, Color::Black),
            Square::new(Coordinate::new(3, 6), None, Color::White),
            Square::new(Coordinate::new(4, 6), None, Color::Black),
            Square::new(Coordinate::new(5, 6), None, Color::White),
            Square::new(Coordinate::new(6, 6), None, Color::Black),
            Square::new(Coordinate::new(7, 6), None, Color::White),
            Square::new(Coordinate::new(8, 6), None, Color::Black),
            Square::new(Coordinate::new(1, 7), None, Color::Black),
            Square::new(Coordinate::new(2, 7), None, Color::White),
            Square::new(Coordinate::new(3, 7), None, Color::Black),
            Square::new(Coordinate::new(4, 7), None, Color::White),
            Square::new(Coordinate::new(5, 7), None, Color::Black),
            Square::new(Coordinate::new(6, 7), None, Color::White),
            Square::new(Coordinate::new(7, 7), None, Color::Black),
            Square::new(Coordinate::new(8, 7), None, Color::White),
            Square::new(Coordinate::new(1, 8), None, Color::White),
            Square::new(Coordinate::new(2, 8), None, Color::Black),
            Square::new(Coordinate::new(3, 8), None, Color::White),
            Square::new(Coordinate::new(4, 8), None, Color::Black),
            Square::new(Coordinate::new(5, 8), None, Color::White),
            Square::new(Coordinate::new(6, 8), None, Color::Black),
            Square::new(Coordinate::new(7, 8), None, Color::White),
            Square::new(Coordinate::new(8, 8), None, Color::Black),
        ]
    }
    pub fn find_pieces_can_move_to_square(
        &self,
        color: Color,
        piece_type: PieceType,
        to: Coordinate,
    ) -> Vec<&Piece> {
        let moves = gen_legal_moves(self, color);
        let pieces = self.get_pieces(color, piece_type);
        let mut found_pieces = Vec::new();
        for &piece in self.get_piece_list().iter() {
            if piece.piece_type == piece_type && piece.color == color {
                let a = piece.at();
                if a.is_none() {
                    continue;
                }
                let at = piece.at().unwrap();
                //find move
                let found = moves.iter().any(|&m| {
                    return (m.from.x() == at.x() && m.from.y() == at.y())
                        && (m.to.x() == to.x() && m.to.y() == to.y());
                });
                if found {
                    found_pieces.push(piece);
                }
            }
        }
        return found_pieces;
        // return self.find_pieces(|&square| {

        //     square.piece().map_or(false, |piece| {
        //         if piece.piece_type == piece_type && piece.color == color {
        //             let a = piece.at();
        //             if a.is_none() {
        //                 return false;
        //             }
        //             let at = piece.at().unwrap();
        //             //find move
        //             return moves.iter().any(|&m| {
        //                 return (m.from.x() == at.x() && m.from.y() == at.y())
        //                     && (m.to.x() == to.x() && m.to.y() == to.y());
        //             });
        //         } else {
        //             return false;
        //         }
        //     })
        // });
    }
    pub fn assert_valid_state(&self) -> bool {
        let squares = self.squares_list();
        let piece_map = self.get_piece_map();
        let bit_board = self.get_board();
        let mut is_valid = true;
        for x in 1..=8u8 {
            for y in 1..=8u8 {
                let at = Coordinate::new(x, y);
                let idx = BitBoard::coordinate_to_idx(at);
                let square_opt = squares.iter().map(|s| *s).find(|&s| *s.coordinate() == at);
                if !square_opt.is_some() {
                    println!("square should be findable");
                    is_valid = false;
                }
                let square = square_opt.unwrap();
                let piece_opt = bit_board.get_piece_at(&at);
                if let Some(piece) = piece_map.get(&idx) {
                    if piece_opt.is_none() {
                        println!("piece should be in bit board");
                        is_valid = false;
                    }
                    let piece_in_board = piece_opt.unwrap();
                    if *piece.at().unwrap() != at {
                        println!("piece at blank");
                        is_valid = false;
                    }
                    if *piece != *square.piece().unwrap() {
                        println!("square piece != piece");
                        is_valid = false;
                    }
                    if *piece != piece_in_board {
                        println!("piece != piece_in board");
                        is_valid = false;
                    }
                } else {
                    if piece_opt.is_some() {
                        println!("piece should not be in bit board\n{}", piece_opt.unwrap());
                        is_valid = false;
                    }
                    if square.piece().is_some() {
                        println!("piece should not be in square\n{}", square.piece().unwrap());
                        is_valid = false;
                    }
                    if self.pieces.get(&idx).is_some() {
                        println!(
                            "piece should not be in hash map\n{}",
                            self.pieces.get(&idx).is_none()
                        );
                        is_valid = false;
                    }
                }
            }
        }
        return is_valid;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ai::{self, Ai},
        board_console_printer::print_bit_board,
        chess_notation::fen_reader,
        game, move_generator,
    };

    fn assert_valid_state(game_state: &GameState) {
        let squares = game_state.squares_list();
        let piece_map = game_state.get_piece_map();
        let bit_board = game_state.get_board();
        for x in (1..=8u8) {
            for y in (1..=8u8) {
                let at = Coordinate::new(x, y);
                let idx = BitBoard::coordinate_to_idx(at);
                let square_opt = squares.iter().map(|s| *s).find(|&s| *s.coordinate() == at);
                assert!(square_opt.is_some(), "square should be findable");
                let square = square_opt.unwrap();
                let piece_opt = bit_board.get_piece_at(&at);
                if let Some(piece) = piece_map.get(&idx) {
                    assert!(piece_opt.is_some(), "piece should be in bit board");
                    let piece_in_board = piece_opt.unwrap();
                    assert_eq!(*piece.at().unwrap(), at);
                    assert_eq!(*piece, *square.piece().unwrap());
                    assert_eq!(*piece, piece_in_board);
                } else {
                    assert!(piece_opt.is_none(), "piece should not be in bit board");
                    assert!(square.piece().is_none(), "piece should not be in square");
                    assert!(
                        game_state.pieces.get(&idx).is_none(),
                        "piece should not be in hash map"
                    );
                }
            }
        }
    }

    #[test]
    fn test_squares_list() {}
    #[test]
    fn test_get_rank() {}
    #[test]
    fn test_get_files() {}

    #[test]
    fn test_make_unmake_castles() {
        let fen = "r3k2r/pppbbp2/2np1q1p/1N2p1p1/2BPP2n/2P2N2/PP2QPPP/R1B1K2R w KQkq - 0 1";
        let mut game_state = fen_reader::make_game_state(fen);
        let mut moves = gen_legal_moves(&game_state, Color::White);
        // try to manually castle
        let castle_move_opt = moves
            .iter_mut()
            .find(|m| m.is_king_side_castle() || m.is_queen_side_castle());
        if let Some(castle_move) = castle_move_opt {
            game_state.make_move_mut(castle_move);
            game_state.unmake_move_mut(castle_move);
            assert_valid_state(&game_state);
        } else {
            assert!(false, "castling not found");
        }

        let mut moves = gen_legal_moves(&game_state, Color::White);
        for m in moves.iter_mut() {
            game_state.make_move_mut(m);
            game_state.unmake_move_mut(m);
            assert_valid_state(&game_state);
        }

        let mut ai = Ai::new_with_search(Color::White, ai::AiSearch::Minimax);
        ai.make_move(&mut game_state, Some(4));
        assert!(true);
    }
    #[test]
    fn test_make_unmake_en_passant() {
        // black can en passant
        let fen = "rnbqkbnr/1pp1pp1p/8/P2P2P1/pP1p2p1/8/2P1PP1P/RNBQKBNR b KQkq b3 0 1";
        let mut game_state = fen_reader::make_game_state(fen);
        println!("{}", game_state.en_passant_target.unwrap());
        assert_eq!(game_state.en_passant_target.is_some(), true);
        // let mut moves = gen_legal_moves(&game_state, Color::White);
        let mut moves = gen_legal_moves(&game_state, Color::Black);
        println!("moves generated");
        for m in moves.iter_mut() {
            println!("making {}", m);
            game_state.make_move_mut(m);
            println!("unmaking {}", m);
            game_state.unmake_move_mut(m);
            assert_valid_state(&game_state);
        }

        let fen = "rnbqkbnr/1pp1pp1p/8/P2P2P1/p2p2p1/8/1PP1PP1P/RNBQKBNR w KQkq - 0 1";
        let mut game_state = fen_reader::make_game_state(fen);
        assert_eq!(game_state.en_passant_target.is_some(), false);
        let mut moves = gen_legal_moves(&game_state, Color::White);
        println!("moves generated");
        for m in moves.iter_mut() {
            game_state.make_move_mut(m);
            game_state.unmake_move_mut(m);
        }
        let mut ai = Ai::new(Color::White);
        ai.make_move(&mut game_state, Some(2));
        assert!(true);
    }

    #[test]
    fn test_unmake_move_mut_captures() {
        // fairly simple first postion
        let fen_scandi = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2";
        let mut game_state = fen_reader::make_game_state(fen_scandi);
        //exd5
        let from = Coordinate::new(5, 4);
        let to = from.add(-1, 1);
        let mut takes_move = Move::new(
            from,
            to,
            PieceType::Pawn,
            MoveType::Move,
            Some(PieceType::Pawn),
            None,
            None,
        );
        game_state.make_move_mut(&mut takes_move);
        game_state.unmake_move_mut(&mut takes_move);
        assert_valid_state(&game_state);

        // check with fen
        let result_fen = fen_reader::make_fen(&game_state);
        assert_eq!(result_fen, fen_scandi, "fen should be equal");

        // weird position with lots of captures
        // half move is broken btw, so we manually set it to 0 for the moment
        let captures_fen = "4kb1r/1pp2ppp/4pn2/2rp4/q1BPP1b1/p1Pn1N1P/4KPP1/RNBQ3R b k - 0 15";
        let mut game_state = fen_reader::make_game_state(captures_fen);
        let a4 = Coordinate::new(1, 4);
        let d1 = Coordinate::new(4, 1);
        let mut takes_move = Move::new(
            a4,
            d1,
            PieceType::Queen,
            MoveType::Move,
            Some(PieceType::Queen),
            None,
            None,
        );
        game_state.make_move_mut(&mut takes_move);
        game_state.unmake_move_mut(&mut takes_move);
        assert_valid_state(&game_state);
        let result_fen = fen_reader::make_fen(&game_state);
        assert_eq!(captures_fen, result_fen, "fen should be equal");
    }

    #[test]
    fn test_make_move_captures_pawn_promotion() {
        let fen = "r3k1r1/1b1p1p2/p3pp2/B1b4p/P3P3/1B3P2/1pP4P/R2K1R2 b q - 1 22";
        let mut game_state = fen_reader::make_game_state(fen);
        let b2 = Coordinate::new(2, 2);
        let a1 = Coordinate::new(1, 1);
        let mut m = Move::new(
            b2,
            a1,
            PieceType::Pawn,
            MoveType::Promotion(PieceType::Queen),
            Some(PieceType::Rook),
            None,
            None,
        );
        game_state.make_move_mut(&mut m);
        let result_fen = fen_reader::make_fen(&game_state);
        let after_fen = "r3k1r1/1b1p1p2/p3pp2/B1b4p/P3P3/1B3P2/2P4P/q2K1R2 w q - 0 23";
        assert_eq!(after_fen, result_fen, "fen should be equal");
        assert_valid_state(&game_state);
    }

    fn test_make_unmake_captures_pawn_promotion() {
        let fen = "r3k1r1/1b1p1p2/pB2pp2/P1b4p/4P3/1B3P2/1pP4P/R2K1R2 b q - 0 22";
        let mut game_state = fen_reader::make_game_state(fen);
        let b2 = Coordinate::new(2, 2);
        let a1 = Coordinate::new(1, 1);
        let mut m = Move::new(
            b2,
            a1,
            PieceType::Pawn,
            MoveType::Promotion(PieceType::Queen),
            Some(PieceType::Rook),
            None,
            None,
        );
        game_state.make_move_mut(&mut m);
        let res_fen = "r3k1r1/1b1p1p2/pB2pp2/P1b4p/4P3/1B3P2/2P4P/q2K1R2 w q - 0 23";
        let result_fen = fen_reader::make_fen(&game_state);
        assert_eq!(res_fen, result_fen, "fen should be equal");
        assert_valid_state(&game_state);
        game_state.unmake_move_mut(&mut m);
        let result_fen = fen_reader::make_fen(&game_state);
        assert_eq!(fen, result_fen, "fen should be equal");
        assert_valid_state(&game_state);

        // try all moves
        let mut moves = move_generator::gen_legal_moves(&game_state, Color::Black);
        for m1 in moves.iter_mut() {
            game_state.make_move_mut(m1);
            assert_valid_state(&game_state);
            game_state.unmake_move_mut(m1);
            assert_valid_state(&game_state);
        }
    }

    #[test]
    fn test_unmake_move_captures_pawn_promotion() {
        let fen = "r3k1r1/1b1p1p2/p3pp2/B1b4p/P3P3/1B3P2/1pP4P/R2K1R2 b q - 1 22";
    }

    #[test]
    fn test_make_move_mut_captures() {
        // fairly simple first postion
        let fen_scandi = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2";
        let fen_after = "rnbqkbnr/ppp1pppp/8/3P4/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 2";
        let mut game_state = fen_reader::make_game_state(fen_scandi);
        //exd5
        let from = Coordinate::new(5, 4);
        let to = from.add(-1, 1);
        let mut takes_move = Move::new(
            from,
            to,
            PieceType::Pawn,
            MoveType::Move,
            Some(PieceType::Pawn),
            None,
            None,
        );
        game_state.make_move_mut(&mut takes_move);
        assert_valid_state(&game_state);

        // check with fen
        let result_fen = fen_reader::make_fen(&game_state);
        assert_eq!(result_fen, fen_after, "fen should be equal");

        // weird position with lots of captures
        let captures_fen = "4kb1r/1pp2ppp/4pn2/2rp4/q1BPP1b1/p1Pn1N1P/4KPP1/RNBQ3R b k - 14 15";
        let mut game_state = fen_reader::make_game_state(captures_fen);
        let a4 = Coordinate::new(1, 4);
        let d1 = Coordinate::new(4, 1);
        let mut takes_move = Move::new(
            a4,
            d1,
            PieceType::Queen,
            MoveType::Move,
            Some(PieceType::Queen),
            None,
            None,
        );
        game_state.make_move_mut(&mut takes_move);
        assert_valid_state(&game_state);
        let fen_after = "4kb1r/1pp2ppp/4pn2/2rp4/2BPP1b1/p1Pn1N1P/4KPP1/RNBq3R w k - 0 16";
        let result_fen = fen_reader::make_fen(&game_state);
        assert_eq!(fen_after, result_fen, "fen should be equal");
    }

    #[test]
    fn test_make_move_mut() {
        let mut game_state = GameState::starting_game();

        // E4
        let from = Coordinate::new(5, 2);
        let to = from.add(0, 2);
        // let piece = Piece::new(Color::White, PieceType::Pawn, Some(from));
        let mut e4_move = Move::new(from, to, PieceType::Pawn, MoveType::Move, None, None, None);
        game_state.make_move_mut(&mut e4_move);

        assert_valid_state(&game_state);

        let e4_pawn = game_state.get_piece_at(&to);
        assert!(e4_pawn.is_some(), "pawn not found at e4");
        let e2 = game_state.get_piece_at(&from);
        assert!(e2.is_none(), "pawn should not be at e2");

        // check state things
        // bit board
        let bit_board = game_state.get_board();
        let pawn_board = bit_board.get_pawns_board();
        let all_pieces = bit_board.get_piece_board();
        let e4 = bit_board.get_piece_at(&to);
        assert!(e4.is_some(), "pawn not found at e4 on bitboard");
        let e2 = bit_board.get_piece_at(&from);
        BitBoard::print_bitboard(pawn_board);
        BitBoard::print_bitboard(all_pieces);
        assert!(e2.is_none(), "pawn should not be found at e2 on bitboard");

        // hash map
        let piece_map = game_state.get_piece_map();
        let from_idx = BitBoard::coordinate_to_idx(from);
        let to_idx = BitBoard::coordinate_to_idx(to);
        assert!(
            piece_map.get(&from_idx).is_none(),
            "old pawn data found in hash map"
        );
        assert!(
            piece_map.get(&to_idx).is_some(),
            "pawn not found in hash map"
        );
        if let Some(piece) = piece_map.get(&to_idx) {
            assert_eq!(
                piece,
                &Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(5, 4)))
            );
        }
        // squares
        // let find_square = |
        let squares = game_state.squares_list();
        assert_eq!(squares.len(), 64, "there should be 64 squares.");

        let found_square = squares.iter().find(|&&square| *square.coordinate() == from);
        assert!(found_square.is_some(), "e2 should be found ");
        let found_s = found_square.unwrap();
        assert!(found_s.piece().is_none(), "e2 should be empty");

        let found_square = squares.iter().find(|&&square| *square.coordinate() == to);
        assert!(found_square.is_some(), "e4 should be found ");
        let found_s = found_square.unwrap();
        assert!(found_s.piece().is_some(), "e4 should not be empty");
        let found_piece = found_s.piece().unwrap();
        assert_eq!(
            *found_piece,
            Piece::new(Color::White, PieceType::Pawn, Some(to)),
            " piece should be a white pawn"
        );

        // for x in (1..=8) {
        //     let row_1_x = Coordinate::new(x, 1);
        //     let row_7_x = Coordinate::new(x, 7);
        //     let row_8_x = Coordinate::new(x, 8);
        //     let mut coords_to_check:Vec<Coordinate> = vec![row_1_x, row_7_x, row_8_x];
        //     // we already moved e2
        //     if x != 5 {
        //         let row_2_x = Coordinate::new(x, 2);
        //         coords_to_check.push(row_2_x);
        //     }
        //     // for coordinate in coords_to_check {

        //     // }
        // }

        // E5
        let from = Coordinate::new(5, 7);
        let to = from.add(0, -2);
        let mut e5_move = Move::new(from, to, PieceType::Pawn, MoveType::Move, None, None, None);
        game_state.make_move_mut(&mut e5_move);
        let e5_pawn = game_state.get_piece_at(&to);
        assert!(e5_pawn.is_some(), "pawn not found at e5");
        let e7 = game_state.get_piece_at(&from);
        assert!(e7.is_none(), "pawn should not be at e7");
    }
    #[test]
    fn test_unmake_move_mut() {
        let mut game_state = GameState::starting_game();
        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            fen_reader::make_fen(&game_state),
            start_fen,
            "fen should be equal"
        );
        // E4
        let from = Coordinate::new(5, 2);
        let to = from.add(0, 2);
        // let piece = Piece::new(Color::White, PieceType::Pawn, Some(from));
        let mut e4_move = Move::new(from, to, PieceType::Pawn, MoveType::Move, None, None, None);

        game_state.make_move_mut(&mut e4_move);
        game_state.unmake_move_mut(&mut e4_move);
        assert_eq!(
            fen_reader::make_fen(&game_state),
            start_fen,
            "fen should be equal"
        );
        assert_valid_state(&game_state);

        let e4_pawn = game_state.get_piece_at(&to);
        assert!(e4_pawn.is_none(), "pawn found at e4");
        let e2 = game_state.get_piece_at(&from);
        assert!(e2.is_some(), "pawn should be at e2");

        // check state things
        // bit board
        let bit_board = game_state.get_board();
        let pawn_board = bit_board.get_pawns_board();
        let all_pieces = bit_board.get_piece_board();
        let e4 = bit_board.get_piece_at(&to);
        assert!(e4.is_none(), "pawn found at e4 on bitboard");
        let e2 = bit_board.get_piece_at(&from);
        BitBoard::print_bitboard(pawn_board);
        BitBoard::print_bitboard(all_pieces);
        assert!(e2.is_some(), "pawn should be found at e2 on bitboard");

        // hash map
        let piece_map = game_state.get_piece_map();
        let from_idx = BitBoard::coordinate_to_idx(from);
        let to_idx = BitBoard::coordinate_to_idx(to);
        assert!(
            piece_map.get(&from_idx).is_some(),
            "old pawn data found in hash map"
        );
        assert!(
            piece_map.get(&to_idx).is_none(),
            "pawn not found in hash map"
        );
        if let Some(piece) = piece_map.get(&to_idx) {
            assert_eq!(
                piece,
                &Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(5, 2)))
            );
        }

        // squares
        // let find_square = |
        let squares = game_state.squares_list();
        assert_eq!(squares.len(), 64, "there should be 64 squares.");

        let found_square = squares.iter().find(|&&square| *square.coordinate() == from);
        assert!(found_square.is_some(), "e2 should be found ");
        let found_s = found_square.unwrap();
        assert!(found_s.piece().is_some(), "e2 should not be empty");

        // check e2
        let found_square = squares.iter().find(|&&square| *square.coordinate() == to);
        assert!(found_square.is_some(), "e4 should be found ");
        let found_piece = found_s.piece().unwrap();
        assert_eq!(
            *found_piece,
            Piece::new(Color::White, PieceType::Pawn, Some(from)),
            " piece should be a white pawn"
        );

        // check e4
        let found_s = found_square.unwrap();
        assert!(found_s.piece().is_none(), "e4 should be empty");
    }
    #[test]
    fn test_place_piece() {
        // simple case , add piece to empty board
        let mut game_state = GameState::starting_game();
        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            fen_reader::make_fen(&game_state),
            start_fen,
            "fen should be equal"
        );

        let e4 = Coordinate::new(5, 4);
        let piece = Piece::new(Color::White, PieceType::Knight, Some(e4));
        game_state.place_piece(piece, &e4);

        assert_valid_state(&game_state);

        let added_fen = "rnbqkbnr/pppppppp/8/8/4N3/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            fen_reader::make_fen(&game_state),
            added_fen,
            "fen should be equal"
        );
    }
    #[test]
    fn test_remove_piece() {
        let added_fen = "rnbqkbnr/pppppppp/8/8/4N3/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut game_state = fen_reader::make_game_state(added_fen);
        assert_eq!(
            fen_reader::make_fen(&game_state),
            added_fen,
            "fen should be equal"
        );
        assert_valid_state(&game_state);

        let e4 = Coordinate::new(5, 4);
        let piece = Piece::new(Color::White, PieceType::Knight, Some(e4));
        game_state.remove_piece_at(&e4);

        assert_valid_state(&game_state);

        let start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        assert_eq!(
            fen_reader::make_fen(&game_state),
            start_fen,
            "fen should be equal"
        );
    }
    #[test]
    fn test_has_piece() {}
    #[test]
    fn test_get_piece_at() {
        let game_state = GameState::starting_game();
        let a2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(1, 2)));
        let e2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(5, 2)));
        let h2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(8, 2)));
        let a7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(1, 7)));
        let d7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(4, 7)));
        let h7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(8, 7)));
        let pawns: Vec<Piece> = vec![a2_pawn, e2_pawn, h2_pawn, a7_pawn, d7_pawn, h7_pawn];
        for pawn in pawns.iter() {
            if let Some(at) = pawn.at() {
                let found = game_state.get_piece_at(at);
                if let Some(found_p) = found {
                    //stuff
                    assert_eq!(pawn, found_p);
                } else {
                    panic!("piece not found!");
                }
            }
        }
        let coordinates: Vec<Coordinate> = vec![
            Coordinate::new(4, 4),
            Coordinate::new(3, 3),
            Coordinate::new(1, 5),
            Coordinate::new(1, 3),
            Coordinate::new(8, 6),
            Coordinate::new(8, 3),
        ];
        for c in coordinates.iter() {
            let found = game_state.get_piece_at(c);
            if let Some(found_p) = found {
                //stuff
                panic!("piece found! at {}, when no piece is there", c);
            }
        }
    }
    #[test]
    fn test_get_kings() {}
    #[test]
    fn test_get_pieces() {
        let game_state = GameState::starting_game();
        let a2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(1, 2)));
        let b2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(2, 2)));
        let c2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(3, 2)));
        let d2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(4, 2)));
        let e2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(5, 2)));
        let f2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(6, 2)));
        let g2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(7, 2)));
        let h2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(8, 2)));
        //
        let a7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(1, 7)));
        let b7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(2, 7)));
        let c7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(3, 7)));
        let d7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(4, 7)));
        let e7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(5, 7)));
        let f7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(6, 7)));
        let g7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(7, 7)));
        let h7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(8, 7)));
        //
        let a1_p = Piece::new(Color::White, PieceType::Rook, Some(Coordinate::new(1, 1)));
        let b1_p = Piece::new(Color::White, PieceType::Knight, Some(Coordinate::new(2, 1)));
        let c1_p = Piece::new(Color::White, PieceType::Bishop, Some(Coordinate::new(3, 1)));
        let d1_p = Piece::new(Color::White, PieceType::Queen, Some(Coordinate::new(4, 1)));
        let e1_p = Piece::new(Color::White, PieceType::King, Some(Coordinate::new(5, 1)));
        let f1_p = Piece::new(Color::White, PieceType::Bishop, Some(Coordinate::new(6, 1)));
        let g1_p = Piece::new(Color::White, PieceType::Knight, Some(Coordinate::new(7, 1)));
        let h1_p = Piece::new(Color::White, PieceType::Rook, Some(Coordinate::new(8, 1)));
        //
        let a8_p = Piece::new(Color::Black, PieceType::Rook, Some(Coordinate::new(1, 8)));
        let b8_p = Piece::new(Color::Black, PieceType::Knight, Some(Coordinate::new(2, 8)));
        let c8_p = Piece::new(Color::Black, PieceType::Bishop, Some(Coordinate::new(3, 8)));
        let d8_p = Piece::new(Color::Black, PieceType::Queen, Some(Coordinate::new(4, 8)));
        let e8_p = Piece::new(Color::Black, PieceType::King, Some(Coordinate::new(5, 8)));
        let f8_p = Piece::new(Color::Black, PieceType::Bishop, Some(Coordinate::new(6, 8)));
        let g8_p = Piece::new(Color::Black, PieceType::Knight, Some(Coordinate::new(7, 8)));
        let h8_p = Piece::new(Color::Black, PieceType::Rook, Some(Coordinate::new(8, 8)));
        let pieces: Vec<Piece> = vec![
            a2_pawn, b2_pawn, c2_pawn, d2_pawn, e2_pawn, f2_pawn, g2_pawn, h2_pawn, a7_pawn,
            b7_pawn, c7_pawn, d7_pawn, e7_pawn, f7_pawn, g7_pawn, h7_pawn, a8_p, b8_p, c8_p, d8_p,
            e8_p, f8_p, g8_p, h8_p, a1_p, b1_p, c1_p, d1_p, e1_p, f1_p, g1_p, h1_p,
        ];
        let white_pawns = vec![
            a2_pawn, b2_pawn, c2_pawn, d2_pawn, e2_pawn, f2_pawn, g2_pawn, h2_pawn,
        ];
        let black_pawns = vec![
            a7_pawn, b7_pawn, c7_pawn, d7_pawn, e7_pawn, f7_pawn, g7_pawn, h7_pawn,
        ];
        let white_knights = vec![b1_p, g1_p];

        let mut results = game_state.get_pieces(Color::White, PieceType::Pawn);
        for piece in pieces.iter() {}
    }
    #[test]
    fn test_get_all_pieces() {
        let game_state = GameState::starting_game();
        let a2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(1, 2)));
        let b2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(2, 2)));
        let c2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(3, 2)));
        let d2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(4, 2)));
        let e2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(5, 2)));
        let f2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(6, 2)));
        let g2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(7, 2)));
        let h2_pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(8, 2)));
        //
        let a7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(1, 7)));
        let b7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(2, 7)));
        let c7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(3, 7)));
        let d7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(4, 7)));
        let e7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(5, 7)));
        let f7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(6, 7)));
        let g7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(7, 7)));
        let h7_pawn = Piece::new(Color::Black, PieceType::Pawn, Some(Coordinate::new(8, 7)));
        //
        let a1_p = Piece::new(Color::White, PieceType::Rook, Some(Coordinate::new(1, 1)));
        let b1_p = Piece::new(Color::White, PieceType::Knight, Some(Coordinate::new(2, 1)));
        let c1_p = Piece::new(Color::White, PieceType::Bishop, Some(Coordinate::new(3, 1)));
        let d1_p = Piece::new(Color::White, PieceType::Queen, Some(Coordinate::new(4, 1)));
        let e1_p = Piece::new(Color::White, PieceType::King, Some(Coordinate::new(5, 1)));
        let f1_p = Piece::new(Color::White, PieceType::Bishop, Some(Coordinate::new(6, 1)));
        let g1_p = Piece::new(Color::White, PieceType::Knight, Some(Coordinate::new(7, 1)));
        let h1_p = Piece::new(Color::White, PieceType::Rook, Some(Coordinate::new(8, 1)));
        //
        let a8_p = Piece::new(Color::Black, PieceType::Rook, Some(Coordinate::new(1, 8)));
        let b8_p = Piece::new(Color::Black, PieceType::Knight, Some(Coordinate::new(2, 8)));
        let c8_p = Piece::new(Color::Black, PieceType::Bishop, Some(Coordinate::new(3, 8)));
        let d8_p = Piece::new(Color::Black, PieceType::Queen, Some(Coordinate::new(4, 8)));
        let e8_p = Piece::new(Color::Black, PieceType::King, Some(Coordinate::new(5, 8)));
        let f8_p = Piece::new(Color::Black, PieceType::Bishop, Some(Coordinate::new(6, 8)));
        let g8_p = Piece::new(Color::Black, PieceType::Knight, Some(Coordinate::new(7, 8)));
        let h8_p = Piece::new(Color::Black, PieceType::Rook, Some(Coordinate::new(8, 8)));
        let white_pieces: Vec<Piece> = vec![
            a2_pawn, b2_pawn, c2_pawn, d2_pawn, e2_pawn, f2_pawn, g2_pawn, h2_pawn, a1_p, b1_p,
            c1_p, d1_p, e1_p, f1_p, g1_p, h1_p,
        ];
        let black_pieces: Vec<Piece> = vec![
            a7_pawn, b7_pawn, c7_pawn, d7_pawn, e7_pawn, f7_pawn, g7_pawn, h7_pawn, a8_p, b8_p,
            c8_p, d8_p, e8_p, f8_p, g8_p, h8_p,
        ];
        let mut results = game_state.get_all_pieces(Color::White);
        assert_eq!(
            results.len(),
            white_pieces.len(),
            "total pieces found should be equal."
        );
        for piece in white_pieces.iter() {
            let found = white_pieces.iter().any(|p| p == piece);
            if !found {
                panic!("{} not found", piece);
            }
        }
        let mut results = game_state.get_all_pieces(Color::Black);
        assert_eq!(
            results.len(),
            black_pieces.len(),
            "total pieces found should be equal."
        );
        for piece in black_pieces.iter() {}
    }

    #[test]
    fn test_en_passant() {
        let fen = "r2q1rk1/p2p1ppp/1pn1pn2/2p1P1B1/1b1P4/2N2N1P/PPP2PP1/R2Q1RK1 b - - 0 6";
        let mut game_state = fen_reader::make_game_state(fen);
        let d7 = Coordinate::new(4, 7);
        let d5 = d7.add(0, -2);
        let b_moves = gen_legal_moves(&game_state, Color::Black);
        let mut p_move = b_moves.into_iter().find(|m| {
            return m.from == d7 && m.to == d5;
        });
        assert!(p_move.is_some());
        let mut p_move = p_move.unwrap();
        game_state.make_move_mut(&mut p_move);
        let w_moves = gen_legal_moves(&game_state, Color::White);
        let en_passant = w_moves
            .into_iter()
            .find(|m| m.move_type() == &MoveType::EnPassant);
        assert!(en_passant.is_some());
    }
    #[test]
    fn test_get_castling_rights_changes_if_piece_moves() {}
    #[test]
    fn test_get_castling_rights_changes_if_piece_is_captured() {}
}
