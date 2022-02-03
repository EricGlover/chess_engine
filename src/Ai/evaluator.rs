use crate::board::*;
use crate::move_generator;
use crate::move_generator::Move;


#[cfg(test)]
mod piece_count {
    use super::*;
    use crate::board_console_printer::print_board;
    use crate::chess_notation::fen_reader;

    #[test]
    fn piece_count_initial_board() {
        let board = fen_reader::make_initial_board();
        let piece_count = PieceCount::new(&board);
        let expected = PieceCount {
             white_king: 1,
             white_queen: 1,
             white_bishop: 2,
             white_knight: 2,
             white_rook: 2,
             white_pawn: 8,
             black_king: 1,
             black_queen: 1,
             black_bishop: 2,
             black_knight: 2,
             black_rook: 2,
             black_pawn: 8,
        };
        assert_eq!(expected.white_king, piece_count.white_king);
        assert_eq!(expected.white_queen, piece_count.white_queen);
        assert_eq!(expected.white_bishop, piece_count.white_bishop);
        assert_eq!(expected.white_knight, piece_count.white_knight);
        assert_eq!(expected.white_rook, piece_count.white_rook);
        assert_eq!(expected.white_pawn, piece_count.white_pawn);
        assert_eq!(expected.black_king, piece_count.black_king);
        assert_eq!(expected.black_queen, piece_count.black_queen);
        assert_eq!(expected.black_bishop, piece_count.black_bishop);
        assert_eq!(expected.black_knight, piece_count.black_knight);
        assert_eq!(expected.black_rook, piece_count.black_rook);
        assert_eq!(expected.black_pawn, piece_count.black_pawn);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_console_printer::print_board;
    use crate::chess_notation::fen_reader;
    use test::Bencher;

    #[test]
    fn test_pawn_count() {
        let fen = "rnb1kr2/pp1p1p1p/1qB2n2/7Q/1P1pPP1p/b4N1R/P1P3P1/RNB1K3 b Qq - 4 10";
        let board = fen_reader::make_board(fen);
        let (w_count, b_count) = make_pawn_count_by_file(&board);
        let white_file: [u8; 8] = [1, 1, 1, 0, 1, 1, 1, 0];
        let black_file: [u8; 8] = [1, 1, 0, 2, 0, 1, 0, 2];
        assert_eq!(w_count.files, white_file);
        assert_eq!(b_count.files, black_file);
    }

    #[test]
    fn test_count_pawn_structure() {
        let fen = "rnb1kr2/pp1p1p1p/1qB2n2/7Q/1P1pPP1p/b4N1R/P1P3P1/RNB1K3 b Qq - 4 10";
        let board = fen_reader::make_board(fen);
        let (w, b) = count_blocked_pawns(&board);
        assert_eq!(3, b);
        assert_eq!(1, w);
        // print_board(&board);
        let (w_count, b_count) = make_pawn_count_by_file(&board);
        println!("{:?}", w_count);
        let (w, b) = count_doubled_pawns(&w_count, &b_count);
        assert_eq!(0, w);
        assert_eq!(4, b);
        let (w, b) = count_isolated_pawns(&w_count, &b_count);
        assert_eq!(5, b);
        assert_eq!(0, w);
    }

    #[bench]
    fn bench_evaluate_board(b: &mut Bencher) {
        let fen = "rnb1kr2/pp1p1p1p/1qB2n2/7Q/1P1pPP1p/b4N1R/P1P3P1/RNB1K3 b Qq - 4 10";
        let board = fen_reader::make_board(fen);
        b.iter(|| evaluate(&board, None, None))
    }
}

#[derive(Debug)]
struct PawnCountByFile {
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
    pub fn new(board: &dyn BoardTrait) -> PieceCount {
        let mut piece_count = PieceCount {
            white_king: 0,
            white_queen: 0,
            white_bishop: 0,
            white_knight: 0,
            white_rook: 0,
            white_pawn: 0,
            black_king: 0,
            black_queen: 0,
            black_bishop: 0,
            black_knight: 0,
            black_rook: 0,
            black_pawn: 0,
        };
        for square in board.get_squares_iter() {
            if square.piece().is_some() {
                let piece = square.piece().unwrap();
                match piece.piece_type {
                    PieceType::King => match piece.color {
                        Color::White => piece_count.white_king = piece_count.white_king + 1,
                        Color::Black => piece_count.black_king = piece_count.black_king + 1,
                    },
                    PieceType::Queen => match piece.color {
                        Color::White => piece_count.white_queen = piece_count.white_queen + 1,
                        Color::Black => piece_count.black_queen = piece_count.black_queen + 1,
                    },
                    PieceType::Bishop => match piece.color {
                        Color::White => piece_count.white_bishop = piece_count.white_bishop + 1,
                        Color::Black => piece_count.black_bishop = piece_count.black_bishop + 1,
                    },
                    PieceType::Knight => match piece.color {
                        Color::White => piece_count.white_knight = piece_count.white_knight + 1,
                        Color::Black => piece_count.black_knight = piece_count.black_knight + 1,
                    },
                    PieceType::Rook => match piece.color {
                        Color::White => piece_count.white_rook = piece_count.white_rook + 1,
                        Color::Black => piece_count.black_rook = piece_count.black_rook + 1,
                    },
                    PieceType::Pawn => match piece.color {
                        Color::White => piece_count.white_pawn = piece_count.white_pawn + 1,
                        Color::Black => piece_count.black_pawn = piece_count.black_pawn + 1,
                    },
                }
            }
        }
        piece_count
    }
}

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

fn count_doubled_pawns(white: &PawnCountByFile, black: &PawnCountByFile) -> (u8, u8) {
    let mut white_doubled: u8 = 0;
    let mut black_doubled: u8 = 0;
    for file in white.files.iter() {
        if *file >= 2 {
            white_doubled = white_doubled + *file;
        }
    }
    for file in black.files.iter() {
        if *file >= 2 {
            black_doubled = black_doubled + *file;
        }
    }
    (white_doubled, black_doubled)
}

fn count_blocked_pawns(board: &dyn BoardTrait) -> (u8, u8) {
    let files = board.get_files();
    let mut white_blocked: u8 = 0;
    let mut black_blocked: u8 = 0;
    files.for_each(|file| {
        file.for_each(|square| {
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

fn make_pawn_count_by_file(board: &dyn BoardTrait) -> (PawnCountByFile, PawnCountByFile) {
    let files = board.get_files();
    let mut white_p = PawnCountByFile { files: [0; 8] };
    let mut black_p = PawnCountByFile { files: [0; 8] };
    files.enumerate().for_each(|(x, file)| {
        file.for_each(|square| {
            if square.piece().is_some() {
                let piece = square.piece().unwrap();
                if piece.piece_type == PieceType::Pawn {
                    match piece.color {
                        Color::White => white_p.files[x] = white_p.files[x] + 1,
                        Color::Black => black_p.files[x] = black_p.files[x] + 1,
                    }
                }
            }
        });
    });
    (white_p, black_p)
}

fn count_isolated_pawns(white: &PawnCountByFile, black: &PawnCountByFile) -> (u8, u8) {
    let mut white_p: u8 = 0;
    let mut black_p: u8 = 0;
    for (i, file) in white.files.iter().enumerate() {
        let mut left_empty_or_none = false;
        if i > 0 {
            let left = white.files.get(i - 1);
            left_empty_or_none = (left.is_some() && *left.unwrap() == 0) || left.is_none();
        }
        let right = white.files.get(i + 1);
        let right_empty_or_none = (right.is_some() && *right.unwrap() == 0) || right.is_none();

        if left_empty_or_none && right_empty_or_none {
            white_p = white_p + *file;
        }
    }
    for (i, file) in black.files.iter().enumerate() {
        let mut left_empty_or_none = false;
        if i > 0 {
            let left = black.files.get(i - 1);
            left_empty_or_none = (left.is_some() && *left.unwrap() == 0) || left.is_none();
        }
        let right = black.files.get(i + 1);
        let right_empty_or_none = (right.is_some() && *right.unwrap() == 0) || right.is_none();

        if left_empty_or_none && right_empty_or_none {
            black_p = black_p + *file;
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
//@todo:  https://www.chessprogramming.org/Simplified_Evaluation_Function

//
// pub trait Evaluator {
//     fn evaluate(board: &Board)-> Evaluation;
// }
//
// pub struct BasicEvaluator {
//
// }
//
// impl Evaluator for BasicEvaluator {
//     pub fn evaluate(board: &Board) -> Evaluation {
//         let c = PieceCount::new(board);
//         let k: i32 = 200 * (c.white_king as i32 - c.black_king as i32);
//         let q: i32 = 9 * (c.white_queen as i32 - c.black_queen as i32);
//         let r: i32 = 5 * (c.white_rook as i32 - c.black_rook as i32);
//         let b: i32 = 3
//             * (c.white_bishop as i32 - c.black_bishop as i32 + c.white_knight as i32
//             - c.black_knight as i32);
//         let p: i32 = 1 * (c.white_pawn as i32 - c.black_pawn as i32);
//
//         // pawn structure evaluation
//         let (white_pawn_file, black_pawn_file) = make_pawn_count_by_file(board);
//         let (white_doubled_pawns, black_doubled_pawns) =
//             count_doubled_pawns(&white_pawn_file, &black_pawn_file);
//         let doubled: i32 = white_doubled_pawns as i32 - black_doubled_pawns as i32;
//         let (white_isolated_pawns, black_isolated_pawns) =
//             count_isolated_pawns(&white_pawn_file, &black_pawn_file);
//         let isolated: i32 = white_isolated_pawns as i32 - black_isolated_pawns as i32;
//         let (white_blocked_pawns, black_blocked_pawns) = count_blocked_pawns(board);
//         let blocked: i32 = (white_blocked_pawns as i32) - (black_blocked_pawns as i32);
//         let pawn_structure = 0.5 * (doubled + isolated + blocked) as f32;
//
//         // mobility
//         let white_moves = move_generator::gen_legal_moves(board, Color::White);
//         let black_moves = move_generator::gen_legal_moves(board, Color::Black);
//
//         // checkmate
//         let mated_player = if board.player_to_move == Color::White && white_moves.len() == 0 {
//             Some(Color::White)
//         } else if board.player_to_move == Color::Black && black_moves.len() == 0 {
//             Some(Color::Black)
//         } else {
//             None
//         };
//
//         let mobility = 0.1 * (white_moves.iter().len() as i32 - black_moves.iter().len() as i32) as f32;
//
//         Evaluation {
//             score: (k + q + r + b + p) as f32 + mobility + pawn_structure,
//             mated_player,
//         }
//     }
// }

pub fn evaluate(
    board: &dyn BoardTrait,
    white_moves_ref: Option<&Vec<Move>>,
    black_moves_ref: Option<&Vec<Move>>,
) -> Evaluation {
    let c = PieceCount::new(board);
    let k: i32 = 20 * (c.white_king as i32 - c.black_king as i32);
    let q: i32 = 9 * (c.white_queen as i32 - c.black_queen as i32);
    let r: i32 = 5 * (c.white_rook as i32 - c.black_rook as i32);
    let b: i32 = 3
        * (c.white_bishop as i32 - c.black_bishop as i32);
    let n: i32 = 3 * (c.white_knight as i32 - c.black_knight as i32);
    let p: i32 = 1 * (c.white_pawn as i32 - c.black_pawn as i32);

    // pawn structure evaluation
    let (white_pawn_file, black_pawn_file) = make_pawn_count_by_file(board);
    let (white_doubled_pawns, black_doubled_pawns) =
        count_doubled_pawns(&white_pawn_file, &black_pawn_file);
    let doubled: i32 = white_doubled_pawns as i32 - black_doubled_pawns as i32;
    let (white_isolated_pawns, black_isolated_pawns) =
        count_isolated_pawns(&white_pawn_file, &black_pawn_file);
    let isolated: i32 = white_isolated_pawns as i32 - black_isolated_pawns as i32;
    let (white_blocked_pawns, black_blocked_pawns) = count_blocked_pawns(board);
    let blocked: i32 = (white_blocked_pawns as i32) - (black_blocked_pawns as i32);
    let pawn_structure = 0.5 * ((doubled + isolated + blocked) as f32);

    // mobility
    let white_move_count: i32 = white_moves_ref.map_or_else(
        || move_generator::gen_pseudo_legal_moves(board, Color::White).len() as i32,
        // || 0,
        |moves| moves.len() as i32,
    );
    let black_move_count: i32 = black_moves_ref.map_or_else(
        || move_generator::gen_pseudo_legal_moves(board, Color::Black).len() as i32,
        // || 0,
        |moves| moves.len() as i32,
    );

    // checkmate
    let mated_player = if board.player_to_move() == Color::White && white_move_count == 0 {
        Some(Color::White)
    } else if board.player_to_move() == Color::Black && black_move_count == 0 {
        Some(Color::Black)
    } else {
        None
    };

    let mobility = 0.1 * ((white_move_count - black_move_count) as f32);

    Evaluation {
        score: (k + q + r + b + n + p) as f32 + mobility + pawn_structure,
        mated_player,
    }
}
