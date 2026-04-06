use crate::bit_board;
use crate::board::{Board, CastlingRights, Color, Coordinate, Piece, PieceType};
use crate::board_console_printer::print_board;
use crate::move_generator::plmg;
use std::fmt;
use std::fmt::{Error, Formatter};

/*
 * board indices
57 58 59 60 61 62 63 64
49 50 51 52 53 54 55 56
41 42 43 44 45 46 47 48
33 34 35 36 37 38 39 40
25 26 27 28 29 30 31 32
17 18 19 20 21 22 23 24
 9 10 11 12 13 14 15 16
 1  2  3  4  5  6  7  8
 */

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct BitBoard {
    pieces: u64,
    white_pieces: u64,
    black_pieces: u64,
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    kings: u64,
}

const a1: u64 = 1;
const a2: u64 = 1 << 8;
const a3: u64 = 1 << 16;
const a4: u64 = 1 << 24;
const a5: u64 = 1 << 32;
const a6: u64 = 1 << 40;
const a7: u64 = 1 << 48;
const a8: u64 = 1 << 56;
pub const A_FILE: u64 = 72340172838076673;
pub const B_FILE: u64 = 144680345676153346;
pub const C_FILE: u64 = 289360691352306692;
pub const D_FILE: u64 = 578721382704613384;
pub const E_FILE: u64 = 1157442765409226768;
pub const F_FILE: u64 = 2314885530818453536;
pub const G_FILE: u64 = 4629771061636907072;
pub const H_FILE: u64 = 9259542123273814144;
pub const ROW_1: u64 = 255;
pub const ROW_2: u64 = 65280;
pub const ROW_3: u64 = 16711680;
pub const ROW_4: u64 = 4278190080;
pub const ROW_5: u64 = 1095216660480;
pub const ROW_6: u64 = 280375465082880;
pub const ROW_7: u64 = 71776119061217280;
pub const ROW_8: u64 = 18374686479671623680;
pub const LIGHT_SQUARES: u64 = 6172840429334713770;
pub const DARK_SQUARES: u64 = 12273903644374837845;
pub const ALL_SQUARES: u64 = 18446744073709551615;

// dark_diagonals going from down left to up right
pub const DARK_DIAGONAL_1: u64 = 144396663052566528;
pub const DARK_DIAGONAL_2: u64 = 577588855528488960;
pub const DARK_DIAGONAL_3: u64 = 2310355422147575808;
pub const DARK_DIAGONAL_4: u64 = 9241421688590303745;
pub const DARK_DIAGONAL_5: u64 = 141012904183812;
pub const DARK_DIAGONAL_6: u64 = 2151686160;
pub const DARK_DIAGONAL_7: u64 = 32832;
pub const DARK_DIAGONALS_UP_RIGHT: u64 = 12273903644374837845;
pub const DARK_ARRAY_UP_RIGHT: [u64; 7] = [
    DARK_DIAGONAL_1,
    DARK_DIAGONAL_2,
    DARK_DIAGONAL_3,
    DARK_DIAGONAL_4,
    DARK_DIAGONAL_5,
    DARK_DIAGONAL_6,
    DARK_DIAGONAL_7,
];
pub const DARK_DIAGONAL_A: u64 = 66052;
pub const DARK_DIAGONAL_B: u64 = 4328785936;
pub const DARK_DIAGONAL_C: u64 = 283691315109952;
pub const DARK_DIAGONAL_D: u64 = 145249953336295424;
pub const DARK_DIAGONAL_E: u64 = 580999813328273408;
pub const DARK_DIAGONAL_F: u64 = 2323998145211531264;
pub const DARK_DIAGONALS_UP_LEFT: u64 = 3050531607520062036;
pub const DARK_ARRAY_UP_LEFT: [u64; 6] = [
    DARK_DIAGONAL_A,
    DARK_DIAGONAL_B,
    DARK_DIAGONAL_C,
    DARK_DIAGONAL_D,
    DARK_DIAGONAL_E,
    DARK_DIAGONAL_F,
];

// light square diagonals
pub const LIGHT_DIAGONAL_1: u64 = 288794425616760832;
pub const LIGHT_DIAGONAL_2: u64 = 1155177711073755136;
pub const LIGHT_DIAGONAL_3: u64 = 4620710844295151872;
pub const LIGHT_DIAGONAL_4: u64 = 36099303471055874;
pub const LIGHT_DIAGONAL_5: u64 = 550831656968;
pub const LIGHT_DIAGONAL_6: u64 = 8405024;
pub const LIGHT_DIAGONALS_UP_RIGHT: u64 = 6100782835296785706;
pub const LIGHT_ARRAY_UP_RIGHT: [u64; 6] = [
    LIGHT_DIAGONAL_1,
    LIGHT_DIAGONAL_2,
    LIGHT_DIAGONAL_3,
    LIGHT_DIAGONAL_4,
    LIGHT_DIAGONAL_5,
    LIGHT_DIAGONAL_6,
];

pub const LIGHT_DIAGONAL_A: u64 = 258;
pub const LIGHT_DIAGONAL_B: u64 = 16909320;
pub const LIGHT_DIAGONAL_C: u64 = 1108169199648;
pub const LIGHT_DIAGONAL_D: u64 = 72624976668147840;
pub const LIGHT_DIAGONAL_E: u64 = 290499906672525312;
pub const LIGHT_DIAGONAL_F: u64 = 1161999622361579520;
pub const LIGHT_DIAGONAL_G: u64 = 4647714815446351872;
pub const LIGHT_DIAGONALS_UP_LEFT: u64 = 6172840429334713770;
pub const LIGHT_ARRAY_UP_LEFT: [u64; 7] = [
    LIGHT_DIAGONAL_A,
    LIGHT_DIAGONAL_B,
    LIGHT_DIAGONAL_C,
    LIGHT_DIAGONAL_D,
    LIGHT_DIAGONAL_E,
    LIGHT_DIAGONAL_F,
    LIGHT_DIAGONAL_G,
];
pub const WHITE_KINGSIDE_CASTLE_BLOCKERS: u64 = 96;
pub const WHITE_QUEENSIDE_CASTLE_BLOCKERS: u64 = 14;
pub const BLACK_KINGSIDE_CASTLE_BLOCKERS: u64 = 6917529027641081856;
pub const BLACK_QUEENSIDE_CASTLE_BLOCKERS: u64 = 1008806316530991104;

// dark diagonals 6, down right to up left
// dark diagonals 7, down left to up right
// all dark squares = sum of diagonals

impl BitBoard {
    pub fn new() -> BitBoard {
        return BitBoard::init_pieces();
    }

    pub fn init_from_pieces(pieces: Vec<Piece>) -> BitBoard {
        let mut board = BitBoard {
            pieces: 0u64,
            white_pieces: 0u64,
            black_pieces: 0u64,
            pawns: 0u64,
            knights: 0u64,
            bishops: 0u64,
            rooks: 0u64,
            queens: 0u64,
            kings: 0u64,
        };
        for piece in pieces {
            if piece.at().is_some() {
                let at = piece.at().unwrap();
                board.set_piece(piece.piece_type, piece.color, *at);
            }
        }
        return board;
    }

    pub fn get_piece_board(&self) -> u64 {
        self.pieces
    }

    pub fn get_white_pieces_board(&self) -> u64 {
        self.white_pieces
    }
    pub fn get_black_pieces_board(&self) -> u64 {
        self.black_pieces
    }
    pub fn get_pawns_board(&self) -> u64 {
        self.pawns
    }
    pub fn get_knights_board(&self) -> u64 {
        self.knights
    }
    pub fn get_bishops_board(&self) -> u64 {
        self.bishops
    }
    pub fn get_rooks_board(&self) -> u64 {
        self.rooks
    }
    pub fn get_queens_board(&self) -> u64 {
        self.queens
    }
    pub fn get_kings_board(&self) -> u64 {
        self.kings
    }

    pub fn get_square_color_at(coordinate: Coordinate) -> Color {
        let bit = BitBoard::coordinate_to_bit(coordinate);
        return BitBoard::get_square_color(bit);
    }

    pub fn get_square_color(bit: u64) -> Color {
        if BitBoard::bit_on_bit_board(bit, LIGHT_SQUARES) {
            return Color::White;
        } else {
            return Color::Black;
        }
    }

    pub fn get_diagonals_vec_for_bit(bit: u64) -> Vec<u64> {
        let mut diagonals: Vec<u64> = Vec::new();
        let color = BitBoard::get_square_color(bit);
        let mut arr1: &[u64; 7] = &LIGHT_ARRAY_UP_LEFT;
        let mut arr2: &[u64; 6] = &LIGHT_ARRAY_UP_RIGHT;
        if color == Color::Black {
            arr1 = &DARK_ARRAY_UP_RIGHT;
            arr2 = &DARK_ARRAY_UP_LEFT;
        }
        let d1 = arr1
            .iter()
            .find(|&&board| BitBoard::bit_on_bit_board(bit, board));
        let d2 = arr2
            .iter()
            .find(|&&board| BitBoard::bit_on_bit_board(bit, board));
        if d1.is_some() {
            diagonals.push(*d1.unwrap());
        }
        if d2.is_some() {
            diagonals.push(*d2.unwrap());
        }
        return diagonals;
    }

    pub fn get_diagonals_for_bit(bit: u64) -> u64 {
        let color = BitBoard::get_square_color(bit);
        let mut arr1: &[u64; 7] = &LIGHT_ARRAY_UP_LEFT;
        let mut arr2: &[u64; 6] = &LIGHT_ARRAY_UP_RIGHT;
        if color == Color::Black {
            arr1 = &DARK_ARRAY_UP_RIGHT;
            arr2 = &DARK_ARRAY_UP_LEFT;
        }
        let mut diagonals: u64 = 0;
        let d1 = arr1
            .iter()
            .find(|&&board| BitBoard::bit_on_bit_board(bit, board));
        let d2 = arr2
            .iter()
            .find(|&&board| BitBoard::bit_on_bit_board(bit, board));
        if d1.is_some() {
            diagonals = diagonals | d1.unwrap();
        }
        if d2.is_some() {
            diagonals = diagonals | d2.unwrap();
        }
        return diagonals;
    }

    pub fn get_file_for_bit(bit: u64) -> u64 {
        if BitBoard::bit_on_bit_board(bit, A_FILE) {
            return A_FILE;
        }
        if BitBoard::bit_on_bit_board(bit, B_FILE) {
            return B_FILE;
        }
        if BitBoard::bit_on_bit_board(bit, C_FILE) {
            return C_FILE;
        }
        if BitBoard::bit_on_bit_board(bit, D_FILE) {
            return D_FILE;
        }
        if BitBoard::bit_on_bit_board(bit, E_FILE) {
            return E_FILE;
        }
        if BitBoard::bit_on_bit_board(bit, F_FILE) {
            return F_FILE;
        }
        if BitBoard::bit_on_bit_board(bit, G_FILE) {
            return G_FILE;
        }
        if BitBoard::bit_on_bit_board(bit, H_FILE) {
            return H_FILE;
        }
        return 0;
    }

    pub fn get_row_for_bit(bit: u64) -> u64 {
        if BitBoard::bit_on_bit_board(bit, ROW_1) {
            return ROW_1;
        }
        if BitBoard::bit_on_bit_board(bit, ROW_2) {
            return ROW_2;
        }
        if BitBoard::bit_on_bit_board(bit, ROW_3) {
            return ROW_3;
        }
        if BitBoard::bit_on_bit_board(bit, ROW_4) {
            return ROW_4;
        }
        if BitBoard::bit_on_bit_board(bit, ROW_5) {
            return ROW_5;
        }
        if BitBoard::bit_on_bit_board(bit, ROW_6) {
            return ROW_6;
        }
        if BitBoard::bit_on_bit_board(bit, ROW_7) {
            return ROW_7;
        }
        if BitBoard::bit_on_bit_board(bit, ROW_8) {
            return ROW_8;
        }
        return 0;
    }

    pub fn get_white_pieces(&self) -> u64 {
        self.white_pieces
    }

    pub fn get_black_pieces(&self) -> u64 {
        self.black_pieces
    }

    //@todo: test
    pub fn get_piece_count(&self) -> u64 {
        u64::count_ones(self.pieces) as u64
    }

    //@todo: test
    pub fn get_white_piece_count(&self) -> u64 {
        u64::count_ones(self.white_pieces) as u64
    }

    //@todo: test
    pub fn get_black_piece_count(&self) -> u64 {
        u64::count_ones(self.black_pieces) as u64
    }

    pub fn get_piece_types_idx(&self, piece_type: PieceType) -> Vec<u64> {
        let piece_type_board = match piece_type {
            PieceType::King => self.kings,
            PieceType::Queen => self.queens,
            PieceType::Bishop => self.bishops,
            PieceType::Knight => self.knights,
            PieceType::Rook => self.rooks,
            PieceType::Pawn => self.pawns,
        };
        return BitBoard::get_indices_of_bit_board(piece_type_board);
    }

    pub fn get_piece_types_by_color_idx(&self, piece_type: PieceType, color: Color) -> Vec<u64> {
        let color_board = match color {
            Color::White => self.white_pieces,
            Color::Black => self.black_pieces,
        };
        let piece_type_board = match piece_type {
            PieceType::King => self.kings,
            PieceType::Queen => self.queens,
            PieceType::Bishop => self.bishops,
            PieceType::Knight => self.knights,
            PieceType::Rook => self.rooks,
            PieceType::Pawn => self.pawns,
        };
        return BitBoard::get_indices_of_bit_board(piece_type_board & color_board);
    }

    //@todo : remove, use game_state.get_all_pieces()
    //preferably don't call this
    pub fn get_all_pieces(&self, color: Color) -> Vec<Piece> {
        let pieces: Vec<Piece> = vec![];
        let mut all_pieces = self.pieces;
        while all_pieces > 0 {
            let bit = BitBoard::pop_bit(&mut all_pieces);
        }
        return pieces;
    }

    //@todo : remove, use game_state.get_pieces()
    //preferably don't call this
    pub fn get_pieces(&self, color: Color, piece_type: PieceType) -> Vec<Piece> {
        let mut pieces: Vec<Piece> = vec![];
        let color_board = match color {
            Color::White => self.white_pieces,
            Color::Black => self.black_pieces,
        };
        let piece_type_board = match piece_type {
            PieceType::King => self.kings,
            PieceType::Queen => self.queens,
            PieceType::Bishop => self.bishops,
            PieceType::Knight => self.knights,
            PieceType::Rook => self.rooks,
            PieceType::Pawn => self.pawns,
        };
        let mut all_pieces = color_board & piece_type_board;
        while all_pieces > 0 {
            let bit = BitBoard::pop_bit(&mut all_pieces);
            pieces.push(Piece::new(
                color,
                piece_type,
                Some(BitBoard::bit_to_coordinate(bit)),
            ));
        }
        return pieces;
    }

    //@todo : test
    pub fn get_piece_at(&self, at: &Coordinate) -> Option<Piece> {
        // check pieces
        let idx = BitBoard::coordinate_to_idx(*at);
        // check all pieces
        if BitBoard::get_bit(self.pieces, idx) {
            // check color
            let mut color: Option<Color> = None;
            if BitBoard::get_bit(self.white_pieces, idx) {
                color = Some(Color::White);
            } else if BitBoard::get_bit(self.black_pieces, idx) {
                color = Some(Color::Black);
            } else {
                return None;
            }
            let mut piece_type: Option<PieceType> = None;
            if BitBoard::get_bit(self.kings, idx) {
                piece_type = Some(PieceType::King);
            } else if BitBoard::get_bit(self.queens, idx) {
                piece_type = Some(PieceType::Queen);
            } else if BitBoard::get_bit(self.bishops, idx) {
                piece_type = Some(PieceType::Bishop);
            } else if BitBoard::get_bit(self.knights, idx) {
                piece_type = Some(PieceType::Knight);
            } else if BitBoard::get_bit(self.rooks, idx) {
                piece_type = Some(PieceType::Rook);
            } else if BitBoard::get_bit(self.pawns, idx) {
                piece_type = Some(PieceType::Pawn);
            } else {
                return None;
            }
            return Some(Piece::new(color.unwrap(), piece_type.unwrap(), Some(*at)));
        } else {
            return None;
        }
        return None;
    }

    //@todo: test
    pub fn get_piece_type_count(&self, piece_type: PieceType, color: Color) -> u64 {
        let color_board = match color {
            Color::White => self.white_pieces,
            Color::Black => self.black_pieces,
        };
        let piece_type_board = match piece_type {
            PieceType::King => self.kings,
            PieceType::Queen => self.queens,
            PieceType::Bishop => self.bishops,
            PieceType::Knight => self.knights,
            PieceType::Rook => self.rooks,
            PieceType::Pawn => self.pawns,
        };
        return color_board & piece_type_board;
    }

    fn compliment(bit_board: u64) -> u64 {
        !bit_board + 1
    }

    pub fn msb(bit_board: u64) -> u64 {
        if bit_board == 0 {
            return 0u64;
        }
        let go_right = u64::leading_zeros(bit_board);
        let high_bit = 1u64 << 63;
        return high_bit >> go_right;
    }

    //@todo ::
    pub fn lsb(bit_board: u64) -> u64 {
        if bit_board == 0 {
            return 0u64;
        }
        bit_board & (!bit_board + 1)
    }

    pub fn get_indices_of_bit_board(mut bit_board: u64) -> Vec<u64> {
        let mut indices: Vec<u64> = Vec::new();
        while bit_board > 0 {
            let lsb = BitBoard::pop_bit(&mut bit_board);
            indices.push((u64::trailing_zeros(lsb) + 1) as u64);
        }
        return indices;
    }

    //@todo : get idx of bit
    pub fn get_index_of_bit(bit: u64) -> u64 {
        return (u64::trailing_zeros(bit) + 1) as u64;
    }

    // for some bit board, give me the lsb and remove it
    //@todo ::
    pub fn pop_bit(bit_board: &mut u64) -> u64 {
        let lsb = BitBoard::lsb(*bit_board);
        if lsb == 0 {
            return lsb;
        }
        // remove lsb from bit board
        *bit_board = *bit_board ^ lsb;
        return lsb;
    }

    pub fn bit_on_bit_board(bit: u64, bit_board: u64) -> bool {
        return (bit_board & bit) != 0u64;
    }

    pub fn on_row(bit_board: u64, row_board: u64) -> bool {
        return (bit_board & row_board) != 0u64;
    }
    pub fn on_file(bit_board: u64, file_board: u64) -> bool {
        return (bit_board & file_board) != 0u64;
    }
    // idx 1..64
    pub fn get_bit(bit_board: u64, idx: u64) -> bool {
        // idx 1..8 => row_idx 1
        // idx 9..18 => row_idx 2
        // get row
        //1..8
        let row_idx = ((idx - 1) / 8) + 1;
        let row = match row_idx {
            1 => a1,
            2 => a2,
            3 => a3,
            4 => a4,
            5 => a5,
            6 => a6,
            7 => a7,
            8 => a8,
            _ => a1,
        };
        // idx 1..8 => 1..8
        // idx 9..18 => 1..8
        //1..8
        let col_idx: u64 = ((idx - 1) % 8) + 1;
        let mask: u64 = row << (col_idx - 1);

        return (mask & bit_board) > 0;
    }

    pub fn set_bit_to(bit_board: u64, idx: u64, bit: bool) -> u64 {
        if bit {
            return BitBoard::set_bit(bit_board, idx);
        } else {
            return BitBoard::unset_bit(bit_board, idx);
        }
    }

    pub fn set_bit(bit_board: u64, idx: u64) -> u64 {
        let row_idx = ((idx - 1) / 8) + 1;
        let row = match row_idx {
            1 => a1,
            2 => a2,
            3 => a3,
            4 => a4,
            5 => a5,
            6 => a6,
            7 => a7,
            8 => a8,
            _ => a1,
        };
        // idx 1..8 => 1..8
        // idx 9..18 => 1..8
        //1..8
        let col_idx: u64 = ((idx - 1) % 8) + 1;
        let mask: u64 = row << (col_idx - 1);
        return mask | bit_board;
    }

    pub fn unset_bit(bit_board: u64, idx: u64) -> u64 {
        let row_idx = ((idx - 1) / 8) + 1;
        let row = match row_idx {
            1 => a1,
            2 => a2,
            3 => a3,
            4 => a4,
            5 => a5,
            6 => a6,
            7 => a7,
            8 => a8,
            _ => a1,
        };
        // idx 1..8 => 1..8
        // idx 9..18 => 1..8
        //1..8
        let col_idx: u64 = ((idx - 1) % 8) + 1;
        let mask: u64 = row << (col_idx - 1);
        // flip it
        return (!mask) & bit_board;
    }

    pub fn print(&self) {
        BitBoard::print_board(self);
    }

    pub fn print_board(board: &BitBoard) {
        println!("=======PIECES========");
        BitBoard::print_bitboard(board.pieces);
        println!("========WHITE PIECES=======");
        BitBoard::print_bitboard(board.white_pieces);
        println!("======BLACK PIECES=========");
        BitBoard::print_bitboard(board.black_pieces);
        println!("=======PAWNS========");
        BitBoard::print_bitboard(board.pawns);
        println!("=======KNIGHTS========");
        BitBoard::print_bitboard(board.knights);
        println!("=======BISHOPS========");
        BitBoard::print_bitboard(board.bishops);
        println!("========ROOKS=======");
        BitBoard::print_bitboard(board.rooks);
        println!("=======QUEENS========");
        BitBoard::print_bitboard(board.queens);
        println!("=======KINGS========");
        BitBoard::print_bitboard(board.kings);
    }

    pub fn union(board_a: u64, board_b: u64) -> u64 {
        board_a & board_b
    }

    pub fn attack_map_to_coordinates(attack_map: u64) -> Vec<Coordinate> {
        let mut attack_map = attack_map;
        let mut coordinates: Vec<Coordinate> = vec![];
        while attack_map > 0 {
            let lsb = BitBoard::pop_bit(&mut attack_map);
            coordinates.push(BitBoard::bit_to_coordinate(lsb));
        }
        return coordinates;
    }

    pub fn bit_to_coordinate(bit: u64) -> Coordinate {
        let idx = BitBoard::get_index_of_bit(bit);
        return Coordinate::new((((idx - 1) % 8) + 1) as u8, (((idx - 1) / 8) + 1) as u8);
    }

    pub fn coordinate_to_bit(coordinate: Coordinate) -> u64 {
        let idx = BitBoard::coordinate_to_idx(coordinate);
        return 1u64 << (idx - 1);
    }

    //@todo : test
    pub fn coordinate_to_idx(c: Coordinate) -> u64 {
        return ((c.y() - 1) * 8 + c.x()) as u64;
    }

    //@todo
    pub fn idx_to_coordinate(idx: u64) -> Coordinate {
        BitBoard::bit_to_coordinate(BitBoard::idx_to_bit(idx))
    }

    pub fn idx_to_bit(idx: u64) -> u64 {
        1u64 << (idx - 1)
    }

    //@todo : test
    pub fn is_piece_at_coordinate(self, at: &Coordinate) -> bool {
        let idx = BitBoard::coordinate_to_idx(*at);
        return BitBoard::get_bit(self.pieces, idx);
    }

    //@todo : test
    pub fn is_piece_at(self, piece_type: PieceType, color: Color, at: Coordinate) -> bool {
        let idx = BitBoard::coordinate_to_idx(at);
        // check all pieces
        if BitBoard::get_bit(self.pieces, idx) {
            // check color
            let color_check = match color {
                Color::White => BitBoard::get_bit(self.white_pieces, idx),
                Color::Black => BitBoard::get_bit(self.black_pieces, idx),
            };
            // check piece type
            if color_check {
                return match piece_type {
                    PieceType::King => BitBoard::get_bit(self.kings, idx),
                    PieceType::Queen => BitBoard::get_bit(self.queens, idx),
                    PieceType::Bishop => BitBoard::get_bit(self.bishops, idx),
                    PieceType::Knight => BitBoard::get_bit(self.knights, idx),
                    PieceType::Rook => BitBoard::get_bit(self.rooks, idx),
                    PieceType::Pawn => BitBoard::get_bit(self.pawns, idx),
                };
            }
        }
        return false;
    }

    /* functionality to rework and add ?
    fn get_piece_at(&self, at: &Coordinate) -> Option<&Piece>
    fn get_kings(&self) -> Vec<&Piece>
    fn get_pieces(&self, color: Color, piece_type: PieceType) -> Vec<&Piece>
    fn get_all_pieces(&self, color: Color) -> Vec<&Piece>
    **/
    pub fn has_piece_at(&self, bit_board: u64) -> bool {
        (self.pieces & bit_board) > 0u64
    }

    //@todo : test
    // should it remove any old pieces? that have been captured ?
    // nah
    pub fn move_piece(
        &mut self,
        piece_type: PieceType,
        color: Color,
        at: Coordinate,
        to: Coordinate,
    ) {
        self.remove_piece(piece_type, color, at);
        self.set_piece(piece_type, color, to);
    }

    //@todo : test
    pub fn set_piece(&mut self, piece_type: PieceType, color: Color, at: Coordinate) {
        let idx = BitBoard::coordinate_to_idx(at);
        // remove from pieces
        self.pieces = BitBoard::set_bit(self.pieces, idx);
        // remove from color bit board
        match color {
            Color::White => self.white_pieces = BitBoard::set_bit(self.white_pieces, idx),
            Color::Black => self.black_pieces = BitBoard::set_bit(self.black_pieces, idx),
        }
        // remove from piece_type bit board
        match piece_type {
            PieceType::King => self.kings = BitBoard::set_bit(self.kings, idx),
            PieceType::Queen => self.queens = BitBoard::set_bit(self.queens, idx),
            PieceType::Bishop => self.bishops = BitBoard::set_bit(self.bishops, idx),
            PieceType::Knight => self.knights = BitBoard::set_bit(self.knights, idx),
            PieceType::Rook => self.rooks = BitBoard::set_bit(self.rooks, idx),
            PieceType::Pawn => self.pawns = BitBoard::set_bit(self.pawns, idx),
        }
    }

    //@todo : test
    pub fn remove_piece(&mut self, piece_type: PieceType, color: Color, at: Coordinate) {
        let idx = BitBoard::coordinate_to_idx(at);
        // remove from pieces
        self.pieces = BitBoard::unset_bit(self.pieces, idx);
        // remove from color bit board
        match color {
            Color::White => self.white_pieces = BitBoard::unset_bit(self.white_pieces, idx),
            Color::Black => self.black_pieces = BitBoard::unset_bit(self.black_pieces, idx),
        }
        // remove from piece_type bit board
        match piece_type {
            PieceType::King => self.kings = BitBoard::unset_bit(self.kings, idx),
            PieceType::Queen => self.queens = BitBoard::unset_bit(self.queens, idx),
            PieceType::Bishop => self.bishops = BitBoard::unset_bit(self.bishops, idx),
            PieceType::Knight => self.knights = BitBoard::unset_bit(self.knights, idx),
            PieceType::Rook => self.rooks = BitBoard::unset_bit(self.rooks, idx),
            PieceType::Pawn => self.pawns = BitBoard::unset_bit(self.pawns, idx),
        }
    }

    // range a1..h8
    // maps to bit 1..bit 64
    pub fn print_bitboard(bitboard: u64) {
        println!("================================");
        const LAST_BIT: u64 = 63;
        for rank in 0..8 {
            for file in (0..8).rev() {
                let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
                let char = if bitboard & mask != 0 { '1' } else { '0' };
                print!("{:2} ", char);
            }
            println!();
        }
        println!("================================");
    }

    // empty the board
    pub fn clear(&mut self) {
        self.pieces = 0u64;
        self.white_pieces = 0u64;
        self.black_pieces = 0u64;
        self.pawns = 0u64;
        self.knights = 0u64;
        self.bishops = 0u64;
        self.rooks = 0u64;
        self.queens = 0u64;
        self.kings = 0u64;
    }

    fn init_pieces() -> BitBoard {
        // all_pieces
        let mut bit_board = 0u64;
        for i in 1..=16 {
            bit_board = BitBoard::set_bit(bit_board, i);
        }
        for i in 49..=64 {
            bit_board = BitBoard::set_bit(bit_board, i);
        }
        let pieces = bit_board;

        // white_pieces
        bit_board = 0u64;
        for i in 1..=16 {
            bit_board = BitBoard::set_bit(bit_board, i);
        }

        let white_pieces = bit_board;

        // black_pieces
        bit_board = 0u64;
        for i in 49..=64 {
            bit_board = BitBoard::set_bit(bit_board, i);
        }

        let black_pieces = bit_board;

        // pawns
        bit_board = 0u64;
        for i in 9..=16 {
            bit_board = BitBoard::set_bit(bit_board, i);
        }
        for i in 49..=56 {
            bit_board = BitBoard::set_bit(bit_board, i);
        }

        let pawns = bit_board;

        // knights
        bit_board = 0u64;
        bit_board = BitBoard::set_bit(bit_board, 2);
        bit_board = BitBoard::set_bit(bit_board, 7);
        bit_board = BitBoard::set_bit(bit_board, 58);
        bit_board = BitBoard::set_bit(bit_board, 63);
        let knights = bit_board;

        // bishops
        bit_board = 0u64;
        bit_board = BitBoard::set_bit(bit_board, 3);
        bit_board = BitBoard::set_bit(bit_board, 6);
        bit_board = BitBoard::set_bit(bit_board, 59);
        bit_board = BitBoard::set_bit(bit_board, 62);
        let bishops = bit_board;

        // rooks
        bit_board = 0u64;
        bit_board = BitBoard::set_bit(bit_board, 1);
        bit_board = BitBoard::set_bit(bit_board, 8);
        bit_board = BitBoard::set_bit(bit_board, 57);
        bit_board = BitBoard::set_bit(bit_board, 64);
        let rooks = bit_board;

        // queens
        bit_board = 0u64;
        bit_board = BitBoard::set_bit(bit_board, 4);
        bit_board = BitBoard::set_bit(bit_board, 60);
        let queens = bit_board;

        // kings
        bit_board = 0u64;
        bit_board = BitBoard::set_bit(bit_board, 5);
        bit_board = BitBoard::set_bit(bit_board, 61);
        let kings = bit_board;
        let board = BitBoard {
            pieces,
            white_pieces,
            black_pieces,
            pawns,
            knights,
            bishops,
            rooks,
            queens,
            kings,
        };

        return board;
    }
}

/*
 * board indices
57 58 59 60 61 62 63 64
49 50 51 52 53 54 55 56
41 42 43 44 45 46 47 48
33 34 35 36 37 38 39 40
25 26 27 28 29 30 31 32
17 18 19 20 21 22 23 24
 9 10 11 12 13 14 15 16
 1  2  3  4  5  6  7  8
 */

fn print_bitboard_indices() {
    println!("================================");
    for rank in 0..8 {
        for file in (0..8).rev() {
            let idx = 64 - (rank * 8) - file;
            print!("{:2} ", idx);
        }
        println!();
    }
    println!("================================");
}

pub fn test() {
    for y in (1..=8) {
        for x in (1..=8) {
            let c = Coordinate::new(x, y);
            let bit = BitBoard::coordinate_to_bit(c);
            let color = BitBoard::get_square_color_at(c);
            println!("{} at {} ", color, c);
            // BitBoard::print_bitboard(bit);
        }
    }

    return;

    init_gen_file_boards();
    return;
    // testing

    // let mut bit_board = 1u64 << 1;

    // println!("{}", u64::count_ones(bit_board));
    // println!("{}", u64::count_zeros(bit_board));

    // BitBoard::print_bitboard(bit_board | 1u64 << 8);
    // BitBoard::print_bitboard(BitBoard::lsb(bit_board));
    // return;
    // let start_bit = BitBoard::set_bit(0u64, 19);
    // let diagonals = BitBoard::get_diagonals_for_bit(start_bit);
    // BitBoard::print_bitboard(diagonals);
    // return;

    // println!("up right");
    // for diagonal in DARK_ARRAY_UP_RIGHT {
    //     println!("{}", diagonal);
    //     BitBoard::print_bitboard(diagonal);
    // }
    // println!("up left");
    // for diagonal in DARK_ARRAY_UP_LEFT {
    //     println!("{}", diagonal);
    //     BitBoard::print_bitboard(diagonal);
    // }
    // println!("up right");
    // for diagonal in LIGHT_ARRAY_UP_RIGHT {
    //     println!("{}", diagonal);
    //     BitBoard::print_bitboard(diagonal);
    // }
    // println!("up left");
    // for diagonal in LIGHT_ARRAY_UP_LEFT {
    //     println!("{}", diagonal);
    //     BitBoard::print_bitboard(diagonal);
    // }
    // return;

    plmg::test();
    return;
    let t: u64 = 0;
    let mut t2 = 1u64;
    // let a1:u64 = 1;
    // let a2:u64 = 1 << 8;
    // let a3:u64 = 1 << 16;
    // let a4:u64 = 1 << 24;
    // let a5:u64 = 1 << 32;
    // let a6:u64 = 1 << 40;
    // let a7:u64 = 1 << 48;
    // let a8:u64 = 1 << 56;
    // print_bitboard(1u64);
    // print_bitboard(!1u64);
    let mut board: BitBoard = BitBoard::new();
    board.print();
    return;
    BitBoard::print_bitboard(BitBoard::set_bit(1u64, 64));
    BitBoard::print_bitboard(BitBoard::set_bit(1u64, 16));

    println!("{:?}", BitBoard::get_bit(1u64, 1));
    println!("{:?}", BitBoard::get_bit(1u64, 2));
    println!("{:?}", BitBoard::get_bit(1u64 << 1, 2));
    println!("{:?}", BitBoard::get_bit(1u64 << 2, 3));
    println!("{:?}", BitBoard::get_bit(1u64 << 7, 8));
    println!("{:?}", BitBoard::get_bit(1u64 << 8, 9));
    println!("{:?}", BitBoard::get_bit(1u64 << 8, 10));
    println!("{:?}", BitBoard::get_bit(1u64 << 63, 64));
    // println!("{:?}", get_bit(1u64, 2));
    // println!("{:?}", get_bit(1u64 << 1, 2));
    // println!("{:?}", get_bit(1u64 << 2, 3));
    return;

    BitBoard::print_bitboard(a1);
    BitBoard::print_bitboard(a2);
    BitBoard::print_bitboard(a3);
    BitBoard::print_bitboard(a4);
    BitBoard::print_bitboard(a5);
    BitBoard::print_bitboard(a6);
    BitBoard::print_bitboard(a7);
    BitBoard::print_bitboard(a8);

    return;
}

/*  INITIAL GENERATOR FUNCTIONS */

fn init_gen_file_boards() {
    /* Castling Blockers */

    /*
     * board indices
    57 58 59 60 61 62 63 64
    49 50 51 52 53 54 55 56
    41 42 43 44 45 46 47 48
    33 34 35 36 37 38 39 40
    25 26 27 28 29 30 31 32
    17 18 19 20 21 22 23 24
     9 10 11 12 13 14 15 16
     1  2  3  4  5  6  7  8
     */
    let mut w_k_blockers = 0u64;
    w_k_blockers = BitBoard::set_bit(w_k_blockers, 6);
    w_k_blockers = BitBoard::set_bit(w_k_blockers, 7);
    let mut w_q_blockers = 0u64;
    w_q_blockers = BitBoard::set_bit(w_q_blockers, 2);
    w_q_blockers = BitBoard::set_bit(w_q_blockers, 3);
    w_q_blockers = BitBoard::set_bit(w_q_blockers, 4);
    let mut b_k_blockers = 0u64;
    b_k_blockers = BitBoard::set_bit(b_k_blockers, 62);
    b_k_blockers = BitBoard::set_bit(b_k_blockers, 63);
    let mut b_q_blockers = 0u64;
    b_q_blockers = BitBoard::set_bit(b_q_blockers, 58);
    b_q_blockers = BitBoard::set_bit(b_q_blockers, 59);
    b_q_blockers = BitBoard::set_bit(b_q_blockers, 60);
    BitBoard::print_bitboard(w_k_blockers);
    BitBoard::print_bitboard(w_q_blockers);
    BitBoard::print_bitboard(b_k_blockers);
    BitBoard::print_bitboard(b_q_blockers);
    println!(
        "{} {} {} {}",
        w_k_blockers, w_q_blockers, b_k_blockers, b_q_blockers
    );

    /*   DIAGONALS  */
    //b8
    //d8
    //
    let d = LIGHT_DIAGONAL_1
        | LIGHT_DIAGONAL_2
        | LIGHT_DIAGONAL_3
        | LIGHT_DIAGONAL_4
        | LIGHT_DIAGONAL_5
        | LIGHT_DIAGONAL_6;
    println!("{}", d);

    let d = LIGHT_DIAGONAL_A
        | LIGHT_DIAGONAL_B
        | LIGHT_DIAGONAL_C
        | LIGHT_DIAGONAL_D
        | LIGHT_DIAGONAL_E
        | LIGHT_DIAGONAL_F
        | LIGHT_DIAGONAL_G;
    println!("{}", d);

    // sanity check , confirmed!
    println!(
        "{} {}",
        DARK_DIAGONALS_UP_LEFT | DARK_DIAGONALS_UP_RIGHT,
        DARK_SQUARES
    );
    println!(
        "{} {}",
        LIGHT_DIAGONALS_UP_LEFT | LIGHT_DIAGONALS_UP_RIGHT,
        LIGHT_SQUARES
    );

    let mut bit_board = A_FILE;
    let a = BitBoard::lsb(A_FILE);
    println!("{}", a);

    let mut lsb = BitBoard::pop_bit(&mut bit_board);
    BitBoard::print_bitboard(lsb);
    BitBoard::print_bitboard(bit_board);
    lsb = BitBoard::pop_bit(&mut bit_board);
    BitBoard::print_bitboard(lsb);
    BitBoard::print_bitboard(bit_board);

    // ON ROW 1 , GOING UP RIGHT ON DARK SQUARES
    let mut bit_board = ROW_1;
    let mut diagonals: Vec<u64> = Vec::new();
    while bit_board > 0 {
        let mut lsb = BitBoard::pop_bit(&mut bit_board);
        if BitBoard::bit_on_bit_board(lsb, DARK_SQUARES) {
            let mut diagonal = 0u64;
            while BitBoard::bit_on_bit_board(lsb, DARK_SQUARES | LIGHT_SQUARES) {
                // going up right
                diagonal = diagonal | lsb;
                // if on h file exit
                if BitBoard::bit_on_bit_board(lsb, H_FILE) {
                    break;
                }
                lsb = lsb << 9;
            }
            println!("made diagonal");
            BitBoard::print_bitboard(diagonal);
            diagonals.push(diagonal);
        }
    }
    // ON A_FILE , GOING UP RIGHT ON DARK SQUARES
    let mut bit_board = A_FILE;
    while bit_board > 0 {
        let mut lsb = BitBoard::pop_bit(&mut bit_board);
        if BitBoard::bit_on_bit_board(lsb, DARK_SQUARES) {
            let mut diagonal = 0u64;
            while BitBoard::bit_on_bit_board(lsb, DARK_SQUARES | LIGHT_SQUARES) {
                // going up right
                diagonal = diagonal | lsb;
                // if on h file exit
                if BitBoard::bit_on_bit_board(lsb, ROW_8) {
                    break;
                }
                lsb = lsb << 9;
            }
            println!("made diagonal");
            BitBoard::print_bitboard(diagonal);
            diagonals.push(diagonal);
        }
    }
    // ON ROW 1 , GOING UP LEFT ON DARK SQUARES
    let mut bit_board = ROW_1;
    while bit_board > 0 {
        let mut lsb = BitBoard::pop_bit(&mut bit_board);
        if BitBoard::bit_on_bit_board(lsb, DARK_SQUARES) {
            let mut diagonal = 0u64;
            while BitBoard::bit_on_bit_board(lsb, DARK_SQUARES | LIGHT_SQUARES) {
                // going up right
                diagonal = diagonal | lsb;
                // if on h file exit
                if BitBoard::bit_on_bit_board(lsb, A_FILE) {
                    break;
                }
                lsb = lsb << 7;
            }
            println!("made diagonal");
            BitBoard::print_bitboard(diagonal);
            diagonals.push(diagonal);
        }
    }
    // ON H_FILE , GOING UP LEFT ON DARK SQUARES
    let mut bit_board = H_FILE;
    while bit_board > 0 {
        let mut lsb = BitBoard::pop_bit(&mut bit_board);
        if BitBoard::bit_on_bit_board(lsb, DARK_SQUARES) {
            let mut diagonal = 0u64;
            while BitBoard::bit_on_bit_board(lsb, DARK_SQUARES | LIGHT_SQUARES) {
                // going up right
                diagonal = diagonal | lsb;
                // if on h file exit
                if BitBoard::bit_on_bit_board(lsb, ROW_8) {
                    break;
                }
                lsb = lsb << 7;
            }
            println!("made diagonal");
            BitBoard::print_bitboard(diagonal);
            diagonals.push(diagonal);
        }
    }

    // printing all dark_square diagonals
    for diagonal in diagonals {
        println!("{}", diagonal);
        BitBoard::print_bitboard(diagonal);
    }

    // MAKING LIGHT SQUARE DIAGONALS
    println!("==============LIGHT SQUARES===================");
    // ON ROW 1 , GOING UP RIGHT ON DARK SQUARES
    let mut bit_board = ROW_1;
    let mut diagonals: Vec<u64> = Vec::new();
    while bit_board > 0 {
        let mut lsb = BitBoard::pop_bit(&mut bit_board);
        if BitBoard::bit_on_bit_board(lsb, LIGHT_SQUARES) {
            let mut diagonal = 0u64;
            while BitBoard::bit_on_bit_board(lsb, LIGHT_SQUARES | DARK_SQUARES) {
                // going up right
                diagonal = diagonal | lsb;
                // if on h file exit
                if BitBoard::bit_on_bit_board(lsb, H_FILE) {
                    break;
                }
                lsb = lsb << 9;
            }
            println!("made diagonal");
            BitBoard::print_bitboard(diagonal);
            diagonals.push(diagonal);
        }
    }
    // ON A_FILE , GOING UP RIGHT ON DARK SQUARES
    let mut bit_board = A_FILE;
    while bit_board > 0 {
        let mut lsb = BitBoard::pop_bit(&mut bit_board);
        if BitBoard::bit_on_bit_board(lsb, LIGHT_SQUARES) {
            let mut diagonal = 0u64;
            while BitBoard::bit_on_bit_board(lsb, LIGHT_SQUARES | DARK_SQUARES) {
                // going up right
                diagonal = diagonal | lsb;
                // if on h file exit
                if BitBoard::bit_on_bit_board(lsb, ROW_8) {
                    break;
                }
                lsb = lsb << 9;
            }
            println!("made diagonal");
            BitBoard::print_bitboard(diagonal);
            diagonals.push(diagonal);
        }
    }
    // ON ROW 1 , GOING UP LEFT ON DARK SQUARES
    let mut bit_board = ROW_1;
    while bit_board > 0 {
        let mut lsb = BitBoard::pop_bit(&mut bit_board);
        if BitBoard::bit_on_bit_board(lsb, LIGHT_SQUARES) {
            let mut diagonal = 0u64;
            while BitBoard::bit_on_bit_board(lsb, LIGHT_SQUARES | DARK_SQUARES) {
                // going up right
                diagonal = diagonal | lsb;
                // if on h file exit
                if BitBoard::bit_on_bit_board(lsb, A_FILE) {
                    break;
                }
                lsb = lsb << 7;
            }
            println!("made diagonal");
            BitBoard::print_bitboard(diagonal);
            diagonals.push(diagonal);
        }
    }
    // ON H_FILE , GOING UP LEFT ON DARK SQUARES
    let mut bit_board = H_FILE;
    while bit_board > 0 {
        let mut lsb = BitBoard::pop_bit(&mut bit_board);
        if BitBoard::bit_on_bit_board(lsb, LIGHT_SQUARES) {
            let mut diagonal = 0u64;
            while BitBoard::bit_on_bit_board(lsb, LIGHT_SQUARES | DARK_SQUARES) {
                // going up right
                diagonal = diagonal | lsb;
                // if on h file exit
                if BitBoard::bit_on_bit_board(lsb, ROW_8) {
                    break;
                }
                lsb = lsb << 7;
            }
            println!("made diagonal");
            BitBoard::print_bitboard(diagonal);
            diagonals.push(diagonal);
        }
    }

    // printing all dark_square diagonals
    for diagonal in diagonals {
        println!("{}", diagonal);
        BitBoard::print_bitboard(diagonal);
    }

    /*         SQUARE COLORS        */
    //light squares
    let mut light_squares = 0u64;
    let mut dark_squares = 0u64;
    // odd rows then even cols
    // even rows then odd cols
    for i in (1..=64) {
        let row = ((i - 1) / 8) + 1;
        let col = ((i - 1) % 8) + 1;
        println!("{} {}", row, col);
        if row % 2 == 0 {
            //even row
            if col % 2 != 0 {
                light_squares = light_squares | BitBoard::set_bit(0u64, i as u64);
            } else {
                dark_squares = dark_squares | BitBoard::set_bit(0u64, i as u64)
            }
        } else {
            // odd row
            if col % 2 == 0 {
                light_squares = light_squares | BitBoard::set_bit(0u64, i as u64);
            } else {
                dark_squares = dark_squares | BitBoard::set_bit(0u64, i as u64)
            }
        }
    }
    println!("{}", light_squares);
    BitBoard::print_bitboard(light_squares);
    println!("{}", dark_squares);
    BitBoard::print_bitboard(dark_squares);

    /*   ROWS  */
    let mut row1 = 0u64;
    for idx in (1u64..=8u64) {
        row1 = BitBoard::set_bit(row1, idx);
    }
    println!("{}", row1);
    BitBoard::print_bitboard(row1);

    let row2 = row1 << 8;
    println!("{}", row2);
    BitBoard::print_bitboard(row2);

    let row3 = row1 << 16;
    println!("{}", row3);
    BitBoard::print_bitboard(row3);

    let row4 = row1 << 24;
    println!("{}", row4);
    BitBoard::print_bitboard(row4);

    let row5 = row1 << 32;
    println!("{}", row5);
    BitBoard::print_bitboard(row5);

    let row6 = row1 << 40;
    println!("{}", row6);
    BitBoard::print_bitboard(row6);

    let row7 = row1 << 48;
    println!("{}", row7);
    BitBoard::print_bitboard(row7);

    let row8 = row1 << 56;
    println!("{}", row8);
    BitBoard::print_bitboard(row8);

    /*  FILES  */
    let mut a_file = 0u64;
    for idx in (1u64..=8u64) {
        a_file = BitBoard::set_bit(a_file, 1 + ((idx - 1) * 8));
    }
    println!("{}", a_file);
    BitBoard::print_bitboard(a_file);

    let b_file = a_file << 1;
    println!("{}", b_file);
    BitBoard::print_bitboard(b_file);

    let c_file = a_file << 2;
    println!("{}", c_file);
    BitBoard::print_bitboard(c_file);

    let d_file = a_file << 3;
    println!("{}", d_file);
    BitBoard::print_bitboard(d_file);

    let e_file = a_file << 4;
    println!("{}", e_file);
    BitBoard::print_bitboard(e_file);

    let f_file = a_file << 5;
    println!("{}", f_file);
    BitBoard::print_bitboard(f_file);

    let g_file = a_file << 6;
    println!("{}", g_file);
    BitBoard::print_bitboard(g_file);

    let h_file = a_file << 7;
    println!("{}", h_file);
    BitBoard::print_bitboard(h_file);

    println!("================FILES================");
    BitBoard::print_bitboard(A_FILE);
    BitBoard::print_bitboard(B_FILE);
    BitBoard::print_bitboard(C_FILE);
    BitBoard::print_bitboard(D_FILE);
    BitBoard::print_bitboard(E_FILE);
    BitBoard::print_bitboard(F_FILE);
    BitBoard::print_bitboard(G_FILE);
    BitBoard::print_bitboard(H_FILE);
}

#[cfg(test)]
mod test {
    use crate::{bit_board::BitBoard, board::*};

    // pub fn bit_to_coordinate(bit: u64) -> Coordinate {
    //     let idx = BitBoard::get_index_of_bit(bit);
    //     return Coordinate::new((idx % 8) as u8, ((idx / 8) + 1) as u8);
    // }

    // pub fn coordinate_to_bit(coordinate: Coordinate) -> u64 {
    //     let idx = BitBoard::coordinate_to_idx(coordinate);
    //     return 1u64 << (idx - 1);
    // }

    // //@todo : test
    // pub fn coordinate_to_idx(c: Coordinate) -> u64 {
    //     return ((c.y() - 1) * 8 + c.x()) as u64;
    // }

    #[test]
    fn test_msb() {
        let indices: Vec<u64> = vec![1, 12, 8, 24, 57, 64];
        let bits: Vec<u64> = indices
            .iter()
            .map(|idx| BitBoard::idx_to_bit(*idx))
            .collect();

        for (idx, &bit) in bits.iter().enumerate() {
            assert_eq!(BitBoard::msb(bit), bit);
            let less_and = (bit - 1) | bit;
            assert_eq!(BitBoard::msb(less_and), bit);
        }
    }

    #[test]
    fn test_bit_to_coordinate() {
        let coordinates = vec![
            Coordinate::new(1, 1),
            Coordinate::new(3, 3),
            Coordinate::new(8, 1),
            Coordinate::new(8, 2),
        ];
        let bits: Vec<u64> = vec![1u64, 1u64 << 18, 1u64 << 7, 1u64 << 15];

        for (i, c) in bits.iter().enumerate() {
            assert_eq!(BitBoard::bit_to_coordinate(*c), coordinates[i]);
        }
    }

    #[test]
    fn test_coordinate_to_bit() {
        let coordinates = vec![
            Coordinate::new(1, 1),
            Coordinate::new(3, 3),
            Coordinate::new(8, 1),
        ];
        let indices: Vec<u64> = vec![1u64, 1u64 << 18, 1u64 << 7];

        for (i, c) in coordinates.iter().enumerate() {
            assert_eq!(BitBoard::coordinate_to_bit(*c), indices[i]);
        }
    }
    #[test]
    fn test_coordinate_to_idx() {
        let coordinates = vec![
            Coordinate::new(1, 1),
            Coordinate::new(3, 3),
            Coordinate::new(8, 1),
        ];
        let indices: Vec<u64> = vec![1, 19, 8];

        for (i, c) in coordinates.iter().enumerate() {
            assert_eq!(BitBoard::coordinate_to_idx(*c), indices[i]);
        }
    }
}
