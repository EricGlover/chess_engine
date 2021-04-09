pub mod evaluator;
use crate::board::*;
use crate::move_generator::*;
use rand::prelude::ThreadRng;
use rand::Rng;

pub struct AI {
    rng: ThreadRng,
    color: Color,
    search_depth: u8,
}

impl AI {
    pub fn new(color: Color) -> AI {
        AI {
            rng: rand::thread_rng(),
            color,
            search_depth: 4,
        }
    }

    fn choose_random_move(&mut self, board: &Board) -> Move {
        let mut moves = gen_legal_moves(&board, self.color);
        let moveCount = moves.iter().len();
        let i = self.rng.gen_range(0..moveCount);
        moves.remove(i)
    }

    fn minimax(
        &self,
        board: Board,
        color: Color,
        depth: u8,
        moves: Vec<Move>,
    ) -> (evaluator::Evaluation, Board, Vec<Move>) {
        // end of recursion
        if depth == 0 {
            return (evaluator::evaluate(&board), board, moves);
        }
        // search moves
        let mut moves_to_try = gen_legal_moves(&board, color);
        if moves_to_try.len() == 0 {
            return (evaluator::evaluate(&board), board, moves);
        }
        let best = moves_to_try.into_iter().fold(None, |acc, m| {
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
    fn search(&self, board: &Board, depth: u8) -> Option<(evaluator::Evaluation, Move)> {
        if depth < 1 {
            return None;
        }
        let (eval, best_board, moves) = self.minimax(board._clone(), self.color, depth, vec![]);

        // print stuff here

        return Some((eval, moves[0]));
    }

    pub fn make_move(&self, board: &Board) -> Option<Move> {
        let m = match self.color {
            Color::White => self.search(board, self.search_depth),
            Color::Black => self.search(board, self.search_depth),
        };

        match m {
            None => None,
            Some((_eval, m)) => Some(m),
        }
    }
}
