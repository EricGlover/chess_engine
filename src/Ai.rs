pub mod evaluator;
use crate::board::*;
use crate::move_generator::*;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::time::{Duration, Instant};
use crate::Ai::evaluator::{Evaluation};

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, Instant};
    use crate::fen_reader::make_initial_board;

    #[test]
    fn search() {
        //@todo :  don't search past a checkmate
    }

    #[test]
    fn test_alpha_beta() {
        //@todo : test more boards.... use pgn ????
        fn test_initial_board_at_depth( depth: u8) {
            let mut ai = AI::new(Color::White);
            let board =  make_initial_board();
            let (eval, best_move) = ai.alpha_beta(board._clone(), Color::White, depth, None, None);
            let (expected_eval, final_board, expected_best_move) = ai.minimax(board._clone(), Color::White, depth, vec![]);

            assert!(best_move.is_some(), "there is a best move");
            assert_eq!(best_move.unwrap(), expected_best_move[0], "alpha beta and minimax find the same move");
            assert_eq!(eval.score, expected_eval.score, "alpha beta and minimax evaluate the same");
        }
        test_initial_board_at_depth(1);
        test_initial_board_at_depth(2);
        test_initial_board_at_depth(3);
        // this takes forever...
        // test_initial_board_at_depth(4);
    }
}


enum AiSearch {
    AlphaBeta,
    Minimax,
}

//@todo : pass in an Evaluator struct, or Evaluation function
// need to understand Box<> or something first
pub struct AI {
    rng: ThreadRng,
    color: Color,
    default_search_depth: u8,
    started_at: Instant,
    time_elapsed_during_search: Option<Duration>,
    minimax_calls: i64,
    ai_search_function: AiSearch,
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
            ai_search_function: AiSearch::AlphaBeta,
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

    // returns  evaluation, final board, lower_bound, upper_bound
    // white sets lower bound , and will accept no branch evaluated lower than that
    // black sets upper bound , and will accept no branch evaluated higher than that
    fn alpha_beta(
        &mut self,
        board: Board,
        player_moving: Color,
        depth_to_go: u8,
        mut lower_bound: Option<evaluator::Evaluation>,
        mut upper_bound: Option<evaluator::Evaluation>,

    ) -> (evaluator::Evaluation, Option<Move>) {
        // end of recursion, depth_to_go = 0 so eval the board
        if depth_to_go == 0 {
            self.minimax_calls = self.minimax_calls + 1;
            return (evaluator::evaluate(&board), None);
        }
        // also end recursion if someone lost a king
        let kings = board.get_kings();
        if kings.len() < 2 {
            return (evaluator::evaluate(&board), None);
        }
        // search moves
        let mut moves_to_try = gen_legal_moves(&board, player_moving);
        // this should cover checkmates so we don't try to search further,
        // @todo: draws
        if moves_to_try.len() == 0 {
            return (evaluator::evaluate(&board), None);
        }

        // dfs with bounds
        // find the best move out of the legal moves
        let mut best_move:Option<Move> = None;
        let mut best_eval:Option<Evaluation> = None;
        for a_move in moves_to_try {
            // player takes move , examine this board
            // assuming this player and the opponent make optimal moves
            // what's the evaluation of the best board state starting from here ?
            let (eval, m) = self.alpha_beta(
                board.make_move(&a_move),
                player_moving.opposite(),
                depth_to_go - 1,
                lower_bound,
                upper_bound
            );
            // println!("{:?}, depth = {} move = {:?} ", eval, depth_to_go, a_move);

            // set best_move and best eval if they're not set
            if best_move.is_none() {
                best_move = Some(a_move);
            }
            if best_eval.is_none() {
                best_eval = Some(eval);
            }

            // look for best move at this depth, like in minimax
            if Color::White == player_moving && eval.score > best_eval.unwrap().score {
                // println!("setting lower bound {:?} , {:?}", eval, a_move);
                best_move = Some(a_move);
                best_eval = Some(eval.clone());
            }
            if Color::Black == player_moving && eval.score < best_eval.unwrap().score {
                // println!("setting upper bound {:?} , {:?}", eval, a_move);
                best_move = Some(a_move);
                best_eval = Some(eval.clone());
            }

            // alpha - beta bound checking
            // check our bounds here
            // if score is lower than lower bound then white will object
            if Color::Black == player_moving && lower_bound.is_some() && eval.score < lower_bound.as_ref().unwrap().score {
                // println!("{:?}, depth = {} move = {:?} ", eval, depth_to_go, a_move);
                // println!("returning early");
                return (best_eval.unwrap(), best_move);
            }
            // if score is higher than higher bound then black will object
            if Color::White == player_moving && upper_bound.is_some() && eval.score > upper_bound.as_ref().unwrap().score {
                // println!("{:?}, depth = {} move = {:?} ", eval, depth_to_go, a_move);
                // println!("returning early");
                return (best_eval.unwrap(), best_move);
            }

            // set bounds
            // if no bounds set, then set them
            if Color::White == player_moving && lower_bound.is_none() { // ????
                lower_bound = Some(eval.clone());
                best_move = Some(a_move);
            }
            if Color::Black == player_moving && upper_bound.is_none() { // ????
                upper_bound = Some(eval.clone());
                best_move = Some(a_move);
            }
        }
        return (best_eval.unwrap(), best_move);
    }

    fn choose_random_move(&mut self, board: &Board) -> Move {
        let mut moves = gen_legal_moves(&board, self.color);
        let move_count = moves.iter().len();
        let i = self.rng.gen_range(0..move_count);
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

        // dfs with recursion time
        let best = moves_to_try.into_iter().fold(None, |acc, m| {
            // @todo : if this move takes the king return
            let piece_captured = board.get_piece_at(&m.to);
            if piece_captured.is_some() && piece_captured.unwrap().piece_type == PieceType::King {
                // hmmm....
                panic!("king captured in search, we never should've gotten here.");
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

            // choose the best move for this player, white wants high eval scores , black wants low eval scores
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
    fn search(
        &mut self,
        board: &Board,
        depth: u8,
        color: Color,
    ) -> Option<(evaluator::Evaluation, Move)> {
        self.minimax_calls = 0;
        self.started_at = Instant::now();
        let (eval, best_move) : (Evaluation, Option<Move>) = match self.ai_search_function {
            AiSearch::AlphaBeta => self.alpha_beta(board._clone(), color, depth, None, None),
            _ => panic!("don't use minimax you fool"),
            // AiSearch::Minimax => {
            //     let (eval, best_board, mut moves) = self.minimax(board._clone(), color, depth, vec![]);
            //     if moves.len() == 0 {
            //         (eval, None)
            //     } else {
            //         (eval, Some(moves.remove(0)))
            //     }
            // }
        };
        // print stuff here
        self.time_elapsed_during_search = Some(self.started_at.elapsed());
        return Some((eval, best_move.unwrap()));
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