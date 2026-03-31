// works with bitboards
use crate::bit_board::{self, BitBoard, A_FILE, H_FILE, ROW_1, ROW_8};
use crate::board::{Color, Coordinate, Piece, PieceType};
use crate::board::{HIGH_X, HIGH_Y, LOW_X, LOW_Y};
use crate::move_generator::Move;

// array of bitboards with the attack from the corresponding index
const BLACK_PAWN_ATTACKS: [u64; 64] = [
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    2,
    5,
    10,
    20,
    40,
    80,
    160,
    64,
    512,
    1280,
    2560,
    5120,
    10240,
    20480,
    40960,
    16384,
    131072,
    327680,
    655360,
    1310720,
    2621440,
    5242880,
    10485760,
    4194304,
    33554432,
    83886080,
    167772160,
    335544320,
    671088640,
    1342177280,
    2684354560,
    1073741824,
    8589934592,
    21474836480,
    42949672960,
    85899345920,
    171798691840,
    343597383680,
    687194767360,
    274877906944,
    2199023255552,
    5497558138880,
    10995116277760,
    21990232555520,
    43980465111040,
    87960930222080,
    175921860444160,
    70368744177664,
    562949953421312,
    1407374883553280,
    2814749767106560,
    5629499534213120,
    11258999068426240,
    22517998136852480,
    45035996273704960,
    18014398509481984,
];

const WHITE_PAWN_ATTACKS: [u64; 64] = [
    512,
    1280,
    2560,
    5120,
    10240,
    20480,
    40960,
    16384,
    131072,
    327680,
    655360,
    1310720,
    2621440,
    5242880,
    10485760,
    4194304,
    33554432,
    83886080,
    167772160,
    335544320,
    671088640,
    1342177280,
    2684354560,
    1073741824,
    8589934592,
    21474836480,
    42949672960,
    85899345920,
    171798691840,
    343597383680,
    687194767360,
    274877906944,
    2199023255552,
    5497558138880,
    10995116277760,
    21990232555520,
    43980465111040,
    87960930222080,
    175921860444160,
    70368744177664,
    562949953421312,
    1407374883553280,
    2814749767106560,
    5629499534213120,
    11258999068426240,
    22517998136852480,
    45035996273704960,
    18014398509481984,
    144115188075855872,
    360287970189639680,
    720575940379279360,
    1441151880758558720,
    2882303761517117440,
    5764607523034234880,
    11529215046068469760,
    4611686018427387904,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];

pub fn test() {
    init_gen_pawn_attacks();
}

fn init_gen_pawn_attacks() {
    let mut wpa: [u64; 64] = [0; 64];
    let mut bpa: [u64; 64] = [0; 64];

    // interesting rust tid bit
    // for(x, idx ) in wpa.iter_mut().zip((0..).into_iter()) {
    //     println!("{x} {idx}");
    // }
    // WHITE PAWN ATTACKS
    let gen_white = false;
    if gen_white {
        for (idx, bit_board) in wpa.iter_mut().enumerate() {
            let mut start_bit = BitBoard::set_bit(0u64, (idx + 1) as u64);
            if BitBoard::on_row(start_bit, ROW_8) {
                continue;
            }
            let on_a_file = BitBoard::on_file(start_bit, A_FILE);
            let on_h_file = BitBoard::on_file(start_bit, H_FILE);

            let mut res = 0u64;
            if !on_a_file {
                res = start_bit << 7;
            }
            if !on_h_file {
                res = res | start_bit << 9;
            }
            *bit_board = res;
            println!("{bit_board} {idx}, {} {}", on_a_file, on_h_file);
        }

        //printing results
        for (idx, bit_board) in wpa.iter().enumerate() {
            println!("{bit_board} {idx}");
            BitBoard::print_bitboard(*bit_board);
        }
        for idx in wpa.iter() {
            println!("{}", idx);
        }
    }
    let mut b = BitBoard::set_bit(0u64, 1);
    println!("{}", 1);
    BitBoard::print_bitboard(b >> 7);

    b = BitBoard::set_bit(0u64, 2);
    println!("{}", 2);
    BitBoard::print_bitboard(b >> 7);

    b = BitBoard::set_bit(0u64, 7);
    println!("===================");
    BitBoard::print_bitboard(b);
    println!("{}", 7);
    BitBoard::print_bitboard(b >> 7);

    b = BitBoard::set_bit(0u64, 8);
    println!("===================");
    BitBoard::print_bitboard(b);
    println!("{}", 8);
    BitBoard::print_bitboard(b >> 7);

    // b = BitBoard::set_bit(0u64, 9);
    // println!("{1}");
    // BitBoard::print_bitboard(b >> 7);

    // b = BitBoard::set_bit(0u64, 15);
    // println!("{1}");
    // BitBoard::print_bitboard(b >> 7);

    //BLACK PAWN ATTACKS
    for (idx, bit_board) in bpa.iter_mut().enumerate() {
        let mut start_bit = BitBoard::set_bit(0u64, (idx + 1) as u64);

        let on_a_file = BitBoard::on_file(start_bit, A_FILE);
        let on_h_file = BitBoard::on_file(start_bit, H_FILE);

        if BitBoard::on_row(start_bit, ROW_1) {
            continue;
        }

        let mut res = 0u64;
        if !on_a_file {
            res = start_bit >> 9;
        }
        if !on_h_file {
            res = res | start_bit >> 7;
        }
        *bit_board = res;
        println!("{bit_board} {idx}, {} {}", on_a_file, on_h_file);
    }

    //printing results
    for (idx, bit_board) in bpa.iter().enumerate() {
        println!("{bit_board} {idx}");
        BitBoard::print_bitboard(*bit_board);
    }
    for idx in bpa.iter() {
        println!("{}", idx);
    }
}

/**
 * @todo
one square move, two square move, capturing diagonally forward, pawn promotion, en passant
**/
pub fn gen_pawn_moves(board: &BitBoard, piece: &Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    return moves;
}

/** HELPER FUNCTIONS  */
fn square_is_empty(board: &BitBoard, at: &Coordinate) -> bool {
    board.is_piece_at_coordinate(at)
}

// if square is off board || square has friendly price => false
fn square_occupiable_by(board: &BitBoard, at: &Coordinate, color: Color) -> bool {
    if !is_on_board(at) {
        return false;
    }
    board.get_piece_at(at).map_or(true, |p| p.color != color)
}

fn has_enemy_piece(board: &BitBoard, at: &Coordinate, own_color: Color) -> bool {
    if !is_on_board(at) {
        return false;
    }
    board
        .get_piece_at(at)
        .map_or(false, |piece| piece.color == own_color.opposite())
}

fn is_on_board(c: &Coordinate) -> bool {
    c.x() >= LOW_X && c.x() <= HIGH_X && c.y() >= LOW_Y && c.y() <= HIGH_Y
}
