use crate::board::BoardTrait;
use crate::board::*;

//@todo
pub fn clone(board: &dyn BoardTrait) -> Box<dyn BoardTrait> {
    unimplemented!();
    Box::new(BoardRef::new())
}

pub struct BoardRef {
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
impl BoardTrait for BoardRef {
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
        // update white to move flag
        self.player_to_move = mov.piece.color.opposite();

        let enemy_piece = self.remove_piece_at(&mov.to);

        // update 50 move rule draw counter
        if mov.captured.is_none() || mov.piece.piece_type != PieceType::Pawn {
            self.half_move_clock = self.half_move_clock + 1;
        } else {
            self.half_move_clock = 0;
        }

        // is this a capture
        if enemy_piece.is_some() {
            let enemy_piece = enemy_piece.unwrap();
            if enemy_piece.color == mov.piece.color {
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
        if mov.promoted_to.is_some() && mov.piece.piece_type == PieceType::Pawn {
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
            self.move_piece(mov.rook.unwrap(), &mov.rook_to.unwrap());
        }

        //@todo check castling rights

        // update move counter
        if mov.piece.color == Color::Black {
            self.full_move_number = self.full_move_number + 1;
        }
    }

    fn unmake_move_mut(&mut self, mov: &Move) {
        self.move_piece(mov.piece, &mov.from);

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
                mov.piece.color,
                mov.captured.unwrap(),
                Some(mov.to.clone()),
            ));
        }

        // if it was promoted, then switch it's type
        if mov.promoted_to.is_some() {
            let mut piece = self.remove_piece(mov.piece);
            piece.piece_type = mov.promoted_to.unwrap().clone();
        }

        // castling
        if mov.is_castling && mov.rook_from.is_some() && mov.rook_to.is_some() {
            match mov.piece.color {
                Color::White => {
                    self.white_castling_rights = self.previous_white_castling_rights.clone();
                }
                Color::Black => {
                    self.black_castling_rights = self.previous_black_castling_rights.clone();
                }
            }
            // move the rook back
            self.move_piece(mov.rook.unwrap(), &mov.rook_from.unwrap());
        }

        // update white to move flag
        self.player_to_move = self.player_to_move.opposite();

        // rollback the move counter
        if mov.piece.color == Color::Black {
            self.full_move_number = self.full_move_number - 1;
        }
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
        self.find_pieces(|&square| square.piece.is_some())
    }
}

impl BoardRef {
    pub fn make_board(
        player_to_move: Color,
        white_can_castle_king_side: bool,
        white_can_castle_queen_side: bool,
        black_can_castle_king_side: bool,
        black_can_castle_queen_side: bool,
        en_passant_target: Option<Coordinate>,
        half_move_clock: u8,
        full_move_number: u8,
    ) -> BoardRef {
        BoardRef {
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
            squares: BoardRef::make_squares(),
        }
    }
    pub fn new() -> BoardRef {
        BoardRef {
            player_to_move: Color::White,
            white_castling_rights: CastlingRights::new(true, true),
            black_castling_rights: CastlingRights::new(true, true),
            previous_white_castling_rights: CastlingRights::new(true, true),
            previous_black_castling_rights: CastlingRights::new(true, true),
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 0,
            squares: BoardRef::make_squares(),
        }
    }
    pub fn from(board: &dyn BoardTrait) -> BoardRef {
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
        BoardRef {
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
    fn clone(&self) -> BoardRef {
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
        BoardRef {
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

    pub fn _clone(&self) -> BoardRef {
        self.clone()
    }

    fn move_piece(&mut self, piece: &Piece, to: &Coordinate) {
        // get the square for the piece moved
        let square = self
            .find_square_mut(|square| square.piece.map_or(false, |p| p == *piece))
            .unwrap();
        let moved_piece = square.remove_piece().unwrap();
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
}
