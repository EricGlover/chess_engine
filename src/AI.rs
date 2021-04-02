pub mod evaluator;

use crate::board::*;
use crate::fen_reader;
use crate::move_generator::*;
use rand::prelude::ThreadRng;
use rand::Rng;

pub struct AI {
    rng: ThreadRng,
    color: Color,
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
            let (white_eval, black_eval) = evaluator::evaluate(&new_board);
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
            let (white_eval, black_eval) = evaluator::evaluate(&new_board);
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

    pub fn make_move(&self, board: &Board) -> Move {
        self.search(board, 1)
    }
}
