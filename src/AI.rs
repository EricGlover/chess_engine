pub mod evaluator;
use crate::board::*;
use crate::move_generator::*;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::time::{Duration, Instant};

pub struct AI {
    rng: ThreadRng,
    color: Color,
    default_search_depth: u8,
    started_at: Instant,
    time_elapsed_during_search: Option<Duration>,
    minimax_calls: i64,
}

impl AI {
    pub fn new(color: Color) -> AI {
        AI {
            rng: rand::thread_rng(),
            color,
            default_search_depth: 4,
            started_at: Instant::now(),
            time_elapsed_during_search: None,
            minimax_calls: 0,
        }
    }

    pub fn minimax_calls(&self) -> i64 {
        self.minimax_calls
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn time_elapsed(&self) -> Option<Duration> {
        self.time_elapsed_during_search
    }

    fn choose_random_move(&mut self, board: &Board) -> Move {
        let mut moves = gen_legal_moves(&board, self.color);
        let moveCount = moves.iter().len();
        let i = self.rng.gen_range(0..moveCount);
        moves.remove(i)
    }

    fn minimax(
        &mut self,
        board: Board,
        color: Color,
        depth: u8,
        moves: Vec<Move>,
    ) -> (evaluator::Evaluation, Board, Vec<Move>) {
        // end of recursion
        if depth == 0 {
            self.minimax_calls = self.minimax_calls + 1;
            return (evaluator::evaluate(&board), board, moves);
        }
        // also end recursion if someone lost a king
        let kings = board.get_kings();
        if kings.len() < 2 {
            return (evaluator::evaluate(&board), board, moves);
        }
        // search moves
        let mut moves_to_try = gen_legal_moves(&board, color);
        if moves_to_try.len() == 0 {
            return (evaluator::evaluate(&board), board, moves);
        }
        let best = moves_to_try.into_iter().fold(None, |acc, m| {
            // @todo : if this move takes the king return
            let piece_captured = board.get_piece_at(&m.to);
            if piece_captured.is_some() && piece_captured.unwrap().piece_type == PieceType::King {

            }
            // add this move to the move list
            let mut move_list = moves.clone();
            move_list.push(m);

            // player takes move , examine this board
            // assuming this player and the opponent make optimal moves
            // what's the evaluation of the best board state starting from here ?
            let (eval, final_board, move_list) =
                self.minimax(board.make_move(&m), color.opposite(), depth - 1, move_list);
            if acc.is_none() {
                return Some((eval, final_board, move_list));
            }
            let (best_eval, best_board, best_moves) = acc.unwrap();
            if (Color::White == color && eval.score > best_eval.score)
                || (Color::Black == color && eval.score < best_eval.score)
            {
                return Some((eval, final_board, move_list));
            }
            Some((best_eval, best_board, best_moves))
        });
        return best.unwrap();
    }

    // do an exhaustive search , depth-first search
    // should return an eval, board, and move list to reach that board
    fn search(&mut self, board: &Board, depth: u8, color: Color) -> Option<(evaluator::Evaluation, Move)> {
        self.minimax_calls = 0;
        self.started_at = Instant::now();
        // if depth < 1 {
        //     return None;
        // }
        let (eval, best_board, moves) = self.minimax(board._clone(), color, depth, vec![]);
        if moves.len() == 0 {
            return None;
        }
        // print stuff here
        self.time_elapsed_during_search = Some(self.started_at.elapsed());
        return Some((eval, moves[0]));
    }

    pub fn make_move(&mut self, board: &Board, depth: Option<u8>) -> Option<Move> {
        let search_depth = if depth.is_some() {
            depth.unwrap()
        } else {
            self.default_search_depth
        };
        let m = self.search(board, search_depth, self.color);
        match m {
            None => None,
            Some((_eval, m)) => Some(m),
        }
    }
}
