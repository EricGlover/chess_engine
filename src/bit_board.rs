use crate::board::{Color, Coordinate, Piece, PieceType};
use crate::board_console_printer::print_board;
use std::fmt;
use std::fmt::{Error, Formatter};

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

impl BitBoard {
    pub fn new() -> BitBoard {
        return BitBoard::init_pieces();
    }
    // idx 1..64
    fn get_bit(bit_board: u64, idx: u64) -> bool {
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

    fn set_bit_to(bit_board: u64, idx: u64, bit: bool) -> u64 {
        if bit {
            return BitBoard::set_bit(bit_board, idx);
        } else {
            return BitBoard::unset_bit(bit_board, idx);
        }
    }

    fn set_bit(bit_board: u64, idx: u64) -> u64 {
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

    fn unset_bit(bit_board: u64, idx: u64) -> u64 {
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

    //@todo : test
    fn coordinate_to_idx(c: Coordinate) -> u64 {
        return ((c.y() - 1) * 8 + c.x()) as u64;
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

    /** functionality to rework and add ?
    fn get_piece_at(&self, at: &Coordinate) -> Option<&Piece>
    fn get_kings(&self) -> Vec<&Piece>
    fn get_pieces(&self, color: Color, piece_type: PieceType) -> Vec<&Piece>
    fn get_all_pieces(&self, color: Color) -> Vec<&Piece>
    **/

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
        let mut board = BitBoard {
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

/**
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
