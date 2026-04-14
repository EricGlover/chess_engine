use crate::board::*;
use crate::game_state::GameState;
use crate::move_generator::Move;
use crate::move_generator::{self, plmg};

/*
previously with Board
test ai::evaluator::tests::bench_evaluate_board                        ... bench:      24,675 ns/iter (+/- 855)
 ================================   60x faster  ==========================================
with Bit Boards and eval improvements
test ai::evaluator::tests::bench_evaluate_board                        ... bench:         409 ns/iter (+/- 11)
*/

#[derive(Debug)]
pub struct PawnCountByFile {
    pub files: [u8; 8],
}

#[derive(Debug)]
struct PieceCount {
    pub white_king: u8,
    pub white_queen: u8,
    pub white_bishop: u8,
    pub white_knight: u8,
    pub white_rook: u8,
    pub white_pawn: u8,
    pub black_king: u8,
    pub black_queen: u8,
    pub black_bishop: u8,
    pub black_knight: u8,
    pub black_rook: u8,
    pub black_pawn: u8,
}

impl PieceCount {
    pub fn new(game_state: &GameState) -> PieceCount {
        let board = game_state.get_board();
        return PieceCount {
            white_king: board.get_piece_type_count(PieceType::King, Color::White),
            white_queen: board.get_piece_type_count(PieceType::Queen, Color::White),
            white_bishop: board.get_piece_type_count(PieceType::Bishop, Color::White),
            white_knight: board.get_piece_type_count(PieceType::Knight, Color::White),
            white_rook: board.get_piece_type_count(PieceType::Rook, Color::White),
            white_pawn: board.get_piece_type_count(PieceType::Pawn, Color::White),
            black_king: board.get_piece_type_count(PieceType::King, Color::Black),
            black_queen: board.get_piece_type_count(PieceType::Queen, Color::Black),
            black_bishop: board.get_piece_type_count(PieceType::Bishop, Color::Black),
            black_knight: board.get_piece_type_count(PieceType::Knight, Color::Black),
            black_rook: board.get_piece_type_count(PieceType::Rook, Color::Black),
            black_pawn: board.get_piece_type_count(PieceType::Pawn, Color::Black),
        };
    }
}

fn count_doubled_pawns(white: &PawnCountByFile, black: &PawnCountByFile) -> (u8, u8) {
    let mut white_doubled: u8 = 0;
    let mut black_doubled: u8 = 0;
    for &file in white.files.iter() {
        if file >= 2 {
            white_doubled += file - 1;
        }
    }
    for &file in black.files.iter() {
        if file >= 2 {
            black_doubled += file - 1;
        }
    }
    (white_doubled, black_doubled)
}

fn count_blocked_pawns(board: &GameState) -> (u8, u8) {
    let files = board.get_files();
    let mut white_blocked: u8 = 0;
    let mut black_blocked: u8 = 0;
    files.iter().for_each(|file| {
        file.iter().for_each(|&square| {
            let piece = square.piece();
            if piece.is_none() {
                return;
            }
            let piece = piece.unwrap();
            if piece.piece_type != PieceType::Pawn {
                return;
            }
            let direction = match piece.color {
                Color::White => 1,
                Color::Black => -1,
            };
            let next_square = square.coordinate().add(0, direction);
            if board.has_piece(&next_square) {
                match piece.color {
                    Color::White => white_blocked = white_blocked + 1,
                    Color::Black => black_blocked = black_blocked + 1,
                }
            }
        })
    });
    (white_blocked, black_blocked)
}

// test ai::evaluator::tests::bench_make_pawn_count_by_file               ... bench:      25,077 ns/iter (+/- 767)
// after rewriting the get_pawn_count_by_file function
// test ai::evaluator::tests::bench_make_pawn_count_by_file               ... bench:       7,547 ns/iter (+/- 233)
// after the second rewrite
// test ai::evaluator::tests::bench_make_pawn_count_by_file               ... bench:         242 ns/iter (+/- 8)
fn make_pawn_count_by_file(game_state: &GameState) -> (PawnCountByFile, PawnCountByFile) {
    return game_state.get_board_ref().get_pawn_count_by_file();
}

//test ai::evaluator::tests::bench_make_pawn_count_by_file               ... bench:         242 ns/iter (+/- 24)
// fn make_pawn_count_by_file(game_state: &GameState) -> (PawnCountByFile, PawnCountByFile) {
//     let board = game_state.get_board_ref();
//     let mut white_p = PawnCountByFile { files: [0; 8] };
//     let mut black_p = PawnCountByFile { files: [0; 8] };
//     for idx in 0..=7u8 {
//         white_p.files[idx as usize] =
//             board.get_piece_count_by_file(PieceType::Pawn, Color::White, idx);
//         black_p.files[idx as usize] =
//             board.get_piece_count_by_file(PieceType::Pawn, Color::Black, idx);
//     }
//     (white_p, black_p)
// }

/*
before rewrite
test ai::evaluator::tests::bench_make_pawn_count_isolated_pawns        ... bench:       4,711 ns/iter (+/- 280)
after
test ai::evaluator::tests::bench_make_pawn_count_isolated_pawns        ... bench:         251 ns/iter (+/- 12)
 */
fn count_isolated_pawns(white: &PawnCountByFile, black: &PawnCountByFile) -> (u8, u8) {
    let mut white_p: u8 = 0;
    let mut black_p: u8 = 0;
    for i in 0..=7usize {
        let mut left_empty_w = true;
        let mut left_empty_b = true;
        if i > 0 {
            left_empty_w = white.files[i - 1] == 0;
            left_empty_b = black.files[i - 1] == 0;
        }
        let mut right_empty_w = true;
        let mut right_empty_b = true;
        if i < 7 {
            right_empty_w = white.files[i + 1] == 0;
            right_empty_b = black.files[i + 1] == 0;
        }
        if left_empty_w && right_empty_w {
            white_p += white.files[i];
        }
        if left_empty_b && right_empty_b {
            black_p += black.files[i];
        }
    }
    (white_p, black_p)
}

#[derive(Debug, Copy, Clone)]
pub struct Evaluation {
    pub score: f32,
    pub mated_player: Option<Color>,
}

impl Evaluation {
    pub fn is_checkmate(&self) -> bool {
        self.mated_player.is_some()
    }
}

// Basic evaluation algorithm
//     f(p) = 200(K-K')
//        + 9(Q-Q')
//        + 5(R-R')
//        + 3(B-B' + N-N')
//        + 1(P-P')
//        - 0.5(D-D' + S-S' + I-I')
//        + 0.1(M-M') + ...
//
// KQRBNP = number of kings, queens, rooks, bishops, knights and pawns
// D,S,I = doubled, blocked and isolated pawns
// M = Mobility (the number of legal moves)

pub fn evaluate(
    game_state: &GameState,
    white_moves_ref: Option<&Vec<Move>>,
    black_moves_ref: Option<&Vec<Move>>,
) -> Evaluation {
    let board = game_state.get_board_ref();
    let c = PieceCount::new(game_state);
    // let k: i32 = 200 * (c.white_king as i32 - c.black_king as i32);
    let queen_val: i32 = 900 * (c.white_queen as i32 - c.black_queen as i32);
    let rook_val: i32 = 500 * (c.white_rook as i32 - c.black_rook as i32);
    let bishop_val: i32 = 300 * (c.white_bishop as i32 - c.black_bishop as i32);
    let knight_val: i32 = 285 * (c.white_knight as i32 - c.black_knight as i32);
    let pawn_val: i32 = 100 * (c.white_pawn as i32 - c.black_pawn as i32);
    let material_value =  (queen_val + rook_val + bishop_val + pawn_val) as f32;

    // pawn structure evaluation
    let (white_pawn_file, black_pawn_file) = make_pawn_count_by_file(game_state);
    let (white_doubled_pawns, black_doubled_pawns) =
        count_doubled_pawns(&white_pawn_file, &black_pawn_file);
    let doubled: i32 = white_doubled_pawns as i32 - black_doubled_pawns as i32;
    let (white_isolated_pawns, black_isolated_pawns) =
        count_isolated_pawns(&white_pawn_file, &black_pawn_file);
    let isolated: i32 = white_isolated_pawns as i32 - black_isolated_pawns as i32;
    let pawn_structure:f32 = -1.0 * (doubled + isolated) as f32;

    // mobility
    let white_move_count: i32 = plmg::get_attack_mobility_count(board, Color::White) as i32;
    let black_move_count: i32 = plmg::get_attack_mobility_count(board, Color::Black) as i32;

    // let white_move_count: i32 = white_moves_ref.map_or_else(
    //     || plmg::get_attack_mobility_count(board, Color::White) as i32,
    //     // || 0,
    //     |moves| moves.len() as i32,
    // );
    // let black_move_count: i32 = black_moves_ref.map_or_else(
    //     || plmg::get_attack_mobility_count(board, Color::Black) as i32,
    //     // || 0,
    //     |moves| moves.len() as i32,
    // );

    // checkmate
    let mated_player = if game_state.player_to_move() == Color::White && white_move_count == 0 {
        Some(Color::White)
    } else if game_state.player_to_move() == Color::Black && black_move_count == 0 {
        Some(Color::Black)
    } else {
        None
    };

    let mobility:f32 = 1.0 * (white_move_count - black_move_count) as f32;
    // let mobility: f32 = 0.0;

    Evaluation {
        score: material_value + mobility + pawn_structure,
        mated_player: mated_player,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_console_printer::print_board;
    use crate::chess_notation::fen_reader;
    use crate::game_state;
    use test::{black_box, Bencher};

    #[test]
    fn test_make_pawn_count_by_file() {
        // initial position
        let game_state = GameState::starting_game();
        let (w, b) = make_pawn_count_by_file(&game_state);
        for idx in 0..7u8 {
            assert_eq!(w.files[idx as usize], 1);
            assert_eq!(b.files[idx as usize], 1);
        }
        // position 2
        let fen = "3rkr2/pp3p1p/4b3/3PP2n/1P1q3p/3R4/P1P3P1/2R1K3 b - - 0 19";
        let game_state = fen_reader::make_game_state(fen);
        let (w, b) = make_pawn_count_by_file(&game_state);
        assert_eq!(w.files[0], 1);
        assert_eq!(w.files[1], 1);
        assert_eq!(w.files[2], 1);
        assert_eq!(w.files[3], 1);
        assert_eq!(w.files[4], 1);
        assert_eq!(w.files[5], 0);
        assert_eq!(w.files[6], 1);
        assert_eq!(w.files[7], 0);
        ////
        assert_eq!(b.files[0], 1);
        assert_eq!(b.files[1], 1);
        assert_eq!(b.files[2], 0);
        assert_eq!(b.files[3], 0);
        assert_eq!(b.files[4], 0);
        assert_eq!(b.files[5], 1);
        assert_eq!(b.files[6], 0);
        assert_eq!(b.files[7], 2);

        // pos 3
        let fen = "rnbqkbnr/3pppp1/2pP2p1/p6P/p1P5/8/P1PP1P1P/RNBQKBNR w KQkq - 0 1";
        let game_state = fen_reader::make_game_state(fen);
        let (w, b) = make_pawn_count_by_file(&game_state);
        assert_eq!(w.files[0], 1);
        assert_eq!(w.files[1], 0);
        assert_eq!(w.files[2], 2);
        assert_eq!(w.files[3], 2);
        assert_eq!(w.files[4], 0);
        assert_eq!(w.files[5], 1);
        assert_eq!(w.files[6], 0);
        assert_eq!(w.files[7], 2);
        ////
        assert_eq!(b.files[0], 2);
        assert_eq!(b.files[1], 0);
        assert_eq!(b.files[2], 1);
        assert_eq!(b.files[3], 1);
        assert_eq!(b.files[4], 1);
        assert_eq!(b.files[5], 1);
        assert_eq!(b.files[6], 2);
        assert_eq!(b.files[7], 0);
    }
    #[test]
    fn test_count_doubled_pawns() {
        // rows with more than one pawn

        // starting pos
        let game_state = GameState::starting_game();
        let (wf, bf) = make_pawn_count_by_file(&game_state);
        let (w, b) = count_doubled_pawns(&wf, &bf);
        assert_eq!(w, 0);
        assert_eq!(b, 0);

        // pos 2
        let fen2 = "3rkr2/pp3p1p/4b3/3PP2n/1P1q3p/3R4/P1P3P1/2R1K3 b - - 0 19";
        let game_state2 = fen_reader::make_game_state(fen2);
        let (w, b) = make_pawn_count_by_file(&game_state2);
        let (w, b) = count_doubled_pawns(&w, &b);
        assert_eq!(w, 0);
        assert_eq!(b, 1);

        // pos 3
        let fen3 = "rnbqkbnr/3pppp1/2pP2p1/p6P/p1P5/8/P1PP1P1P/RNBQKBNR w KQkq - 0 1";
        let game_state3 = fen_reader::make_game_state(fen3);
        let (w, b) = make_pawn_count_by_file(&game_state3);
        let (w, b) = count_doubled_pawns(&w, &b);
        assert_eq!(w, 3);
        assert_eq!(b, 2);
    }
    #[test]
    fn test_count_isolated_pawns() {
        // starting pos
        let game_state = GameState::starting_game();
        let (wf, bf) = make_pawn_count_by_file(&game_state);
        let (w, b) = count_isolated_pawns(&wf, &bf);
        assert_eq!(w, 0);
        assert_eq!(b, 0);

        // pos 2
        let fen = "2r2rk1/1b4bp/1q1pp1p1/2p1np2/1p2P3/pNnPBPP1/PPPQN1BP/3R1RK1 w - - 0 1";
        let game_state = fen_reader::make_game_state(fen);
        let (wf, bf) = make_pawn_count_by_file(&game_state);
        let (w, b) = count_isolated_pawns(&wf, &bf);
        assert_eq!(w, 0);
        assert_eq!(b, 0);

        // pos 3, edge pawns
        let fen = "2r2rk1/1b4bp/1q1pp1p1/4n3/1p2P3/1NnPBP2/P1PQN1BP/3R1RK1 w - - 0 1";
        let game_state = fen_reader::make_game_state(fen);
        let (wf, bf) = make_pawn_count_by_file(&game_state);
        let (w, b) = count_isolated_pawns(&wf, &bf);
        assert_eq!(w, 2);
        assert_eq!(b, 1);
    }

    #[test]
    fn test_piece_count() {
        // starting position
        let game_state = GameState::starting_game();
        let piece_count = PieceCount::new(&game_state);
        // black pieces
        assert_eq!(piece_count.black_rook, 2);
        assert_eq!(piece_count.black_bishop, 2);
        assert_eq!(piece_count.black_king, 1);
        assert_eq!(piece_count.black_knight, 2);
        assert_eq!(piece_count.black_queen, 1);
        assert_eq!(piece_count.black_pawn, 8);
        // white pieces
        assert_eq!(piece_count.white_rook, 2);
        assert_eq!(piece_count.white_bishop, 2);
        assert_eq!(piece_count.white_king, 1);
        assert_eq!(piece_count.white_knight, 2);
        assert_eq!(piece_count.white_queen, 1);
        assert_eq!(piece_count.white_pawn, 8);

        // position 2
        let fen = "3rkr2/pp3p1p/4b3/3PP2n/1P1q3p/3R4/P1P3P1/2R1K3 b - - 0 19";
        let game_state = fen_reader::make_game_state(fen);
        let piece_count = PieceCount::new(&game_state);
        // black pieces
        assert_eq!(piece_count.black_rook, 2);
        assert_eq!(piece_count.black_bishop, 1);
        assert_eq!(piece_count.black_king, 1);
        assert_eq!(piece_count.black_knight, 1);
        assert_eq!(piece_count.black_queen, 1);
        assert_eq!(piece_count.black_pawn, 5);
        // white pieces
        assert_eq!(piece_count.white_rook, 2);
        assert_eq!(piece_count.white_bishop, 0);
        assert_eq!(piece_count.white_king, 1);
        assert_eq!(piece_count.white_knight, 0);
        assert_eq!(piece_count.white_queen, 0);
        assert_eq!(piece_count.white_pawn, 6);
    }

    #[test]
    fn test_pawn_count() {
        let fen = "rnb1kr2/pp1p1p1p/1qB2n2/7Q/1P1pPP1p/b4N1R/P1P3P1/RNB1K3 b Qq - 4 10";
        let game_state = fen_reader::make_game_state(fen);
        let (w_count, b_count) = make_pawn_count_by_file(&game_state);
        let white_file: [u8; 8] = [1, 1, 1, 0, 1, 1, 1, 0];
        let black_file: [u8; 8] = [1, 1, 0, 2, 0, 1, 0, 2];
        assert_eq!(w_count.files, white_file);
        assert_eq!(b_count.files, black_file);
    }

    #[test]
    fn test_count_pawn_structure() {
        let fen = "rnb1kr2/pp1p1p1p/1qB2n2/7Q/1P1pPP1p/b4N1R/P1P3P1/RNB1K3 b Qq - 4 10";
        let game_state = fen_reader::make_game_state(fen);
        // let (w, b) = count_blocked_pawns(&game_state);
        // assert_eq!(3, b);
        // assert_eq!(1, w);
        // print_board(&board);
        let (w_count, b_count) = make_pawn_count_by_file(&game_state);
        println!("{:?}", w_count);
        let (w, b) = count_doubled_pawns(&w_count, &b_count);
        assert_eq!(0, w);
        assert_eq!(2, b);
        let (w, b) = count_isolated_pawns(&w_count, &b_count);
        assert_eq!(5, b);
        assert_eq!(0, w);
    }

    #[bench]
    fn bench_evaluate_board(b: &mut Bencher) {
        let fen = "rnb1kr2/pp1p1p1p/1qB2n2/7Q/1P1pPP1p/b4N1R/P1P3P1/RNB1K3 b Qq - 4 10";
        let game_state = fen_reader::make_game_state(fen);
        b.iter(|| {
            for i in 0..100 {
                black_box({
                    evaluate(&game_state, None, None);
                })
            }
        });
    }

    #[bench]
    fn bench_make_pawn_count_by_file(b: &mut Bencher) {
        let fen = "3rkr2/pp3p1p/4b3/3PP2n/1P1q3p/3R4/P1P3P1/2R1K3 b - - 0 19";
        let game_state = fen_reader::make_game_state(fen);
        b.iter(|| {
            for i in 0..1000 {
                black_box({
                    let (w, b) = make_pawn_count_by_file(&game_state);
                })
            }
        });
    }
    #[bench]
    fn bench_make_pawn_count_isolated_pawns(b: &mut Bencher) {
        let fen = "2r2rk1/1b4bp/1q1pp1p1/4n3/1p2P3/1NnPBP2/P1PQN1BP/3R1RK1 w - - 0 1";
        let game_state = fen_reader::make_game_state(fen);

        b.iter(|| {
            let (w, b) = make_pawn_count_by_file(&game_state);
            for i in 0..1000 {
                black_box({
                    let (wi, bi) = count_isolated_pawns(&w, &b);
                })
            }
        });

        let game_state = GameState::starting_game();
        b.iter(|| {
            let (w, b) = make_pawn_count_by_file(&game_state);
            for i in 0..1000 {
                black_box({
                    let (wi, bi) = count_isolated_pawns(&w, &b);
                })
            }
        });
    }
}
