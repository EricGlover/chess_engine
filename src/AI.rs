use crate::board::*;
use crate::fen_reader;
use crate::move_generator::*;
use rand::prelude::ThreadRng;
use rand::Rng;

pub struct AI {
    rng: ThreadRng,
    color: Color,
}

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
    pub fn new(board: &Board) -> PieceCount {
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
        for square in board.squares() {
            if square.piece.is_some() {
                let piece = &square.piece.unwrap();
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

#[test]
fn test_eval() {
    let board = fen_reader::read(fen_reader::INITIAL_BOARD);
    assert_eq!(AI::evaluate(&board), (0.0, 0.0));
    let board = fen_reader::read(fen_reader::WHITE_IN_CHECK);
    println!("{:?}", AI::evaluate(&board));
}

impl AI {
    pub fn new(color: Color) -> AI {
        AI {
            rng: rand::thread_rng(),
            color,
        }
    }

    fn choose_random_move(&mut self, board: &Board) -> Move {
        let mut moves = gen_moves(&board, self.color);
        let moveCount = moves.iter().len();
        let i = self.rng.gen_range(0..moveCount);
        moves.remove(i)
    }

    fn old_search(&self, board: &Board, depth: u8) -> Move {
        if depth < 1 {
            panic!("can not search for depth less than one");
        }
        let mut moves = gen_moves(&board, self.color);

        // @todo:: add evaluations
        // map and sort ?, search & find highest eval ?
        let mut best_move: Option<Move> = None;
        let mut best_eval: Option<f32> = None;
        for m in moves.into_iter() {
            let new_board = board.make_move(&m);
            let (white_eval, black_eval) = AI::evaluate(&new_board);
            let eval = if self.color == Color::White {
                white_eval
            } else {
                black_eval
            };
            if best_eval.is_none() {
                best_eval = Some(eval);
                best_move = Some(m)
            }
            if best_eval.unwrap() < eval {
                best_eval = Some(eval);
                best_move = Some(m)
            }
        }
        if best_move.is_some() {
            if depth == 1 {
                return best_move.unwrap();
            } else {
                return self.search(board, depth - 1);
            }
        } else {
            panic!("no moves");
        }

        // let i = self.rng.gen_range((0..moveCount));
        // moves.remove(i)
    }

    // do an exhaustive search , depth-first search
    fn search(&self, board: &Board, depth: u8) -> Move {
        if depth < 1 {
            panic!("can not search for depth less than one");
        }
        let mut moves = gen_moves(&board, self.color);
        let moveCount = moves.iter().len();

        // @todo:: add evaluations
        // map and sort ?, search & find highest eval ?
        let mut best_move: Option<Move> = None;
        let mut best_eval: Option<f32> = None;
        for m in moves.into_iter() {
            let new_board = board.make_move(&m);
            let (white_eval, black_eval) = AI::evaluate(&new_board);
            let eval = if self.color == Color::White {
                white_eval
            } else {
                black_eval
            };
            if best_eval.is_none() {
                best_eval = Some(eval);
                best_move = Some(m)
            }
            if best_eval.unwrap() < eval {
                best_eval = Some(eval);
                best_move = Some(m)
            }
        }
        if best_move.is_some() {
            if depth == 1 {
                return best_move.unwrap();
            } else {
                return self.search(board, depth - 1);
            }
        } else {
            panic!("no moves");
        }

        // let i = self.rng.gen_range((0..moveCount));
        // moves.remove(i)
    }

    // returns (black eval, white eval)
    // maybe I should make eval structs ?

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

    fn count_doubled_pawns(board: &Board, color: Color) -> u8 {
        0
    }

    fn count_blocked_pawns(board: &Board, color: Color) -> u8 {
        0
    }

    fn count_isolated_pawns(board: &Board, color: Color) -> u8 {
        0
    }

    pub fn evaluate(board: &Board) -> (f32, f32) {
        let c = PieceCount::new(board);
        let k: i32 = 200 * (c.white_king as i32 - c.black_king as i32);
        let q: i32 = 9 * (c.white_queen as i32 - c.black_queen as i32);
        let r: i32 = 5 * (c.white_rook as i32 - c.black_rook as i32);
        let b: i32 = 3
            * (c.white_bishop as i32 - c.black_bishop as i32 + c.white_knight as i32
                - c.black_knight as i32);
        let p: i32 = 1 * (c.white_pawn as i32 - c.black_pawn as i32);
        let doubled = AI::count_doubled_pawns(board, Color::White)
            - AI::count_doubled_pawns(board, Color::Black);
        let isolated = AI::count_isolated_pawns(board, Color::White)
            - AI::count_isolated_pawns(board, Color::Black);
        let blocked = AI::count_blocked_pawns(board, Color::White)
            - AI::count_blocked_pawns(board, Color::Black);
        let pawn_structure = 0.5 * (doubled + isolated + blocked) as f32;
        // @bugs be here
        let white_moves = gen_moves(board, Color::White);
        let black_moves = gen_moves(board, Color::Black);
        println!("{} {} ", white_moves.iter().len(), black_moves.iter().len());
        let mobility =
            0.1 * (white_moves.iter().len() as i32 - black_moves.iter().len() as i32) as f32;
        let eval = (k + q + r + b + p) as f32 + mobility + pawn_structure;
        //@todo : pawn structures, mobility
        (eval, eval * -1.0)
    }

    pub fn make_move(&self, board: &Board) -> Move {
        self.search(board, 1)
    }
}
