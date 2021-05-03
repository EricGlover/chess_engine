pub mod evaluator;
use crate::ai::evaluator::Evaluation;
use crate::board::*;
use crate::move_generator::*;
use rand::prelude::ThreadRng;
use rand::Rng;
use std::time::{Duration, Instant};
use crate::hash::Zobrist;
use std::iter::Map;
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess_notation::fen_reader::{make_board, make_initial_board};
    use ai;
    use std::time::{Duration, Instant};

    #[test]
    fn bug_alpha_beta() {
        let fen = "rnb1kbnr/pppp1p1p/4pp2/8/8/3BP3/PPPP1PPP/RNB1K1NR b KQkq - 3 4";
        let board = make_board(fen);
        let mut ai = ai::new(Color::Black);
        ai.make_move(&board, Some(4));
    }

    #[test]
    fn search() {
        //@todo :  don't search past a checkmate
    }

    #[test]
    fn test_alpha_beta() {
        //@todo : test more boards.... use pgn ????
        fn test_initial_board_at_depth(depth: u8) {
            let mut ai = ai::new(Color::White);
            let mut board = make_initial_board();
            let (eval, best_move) =
                ai.alpha_beta(&mut *board.clone(), Color::White, depth, None, None);
            let (expected_eval, expected_best_move) = ai.minimax(&mut board, Color::White, depth);

            assert!(best_move.is_some(), "there is a best move");
            assert_eq!(
                best_move.unwrap(),
                expected_best_move.unwrap(),
                "alpha beta and minimax find the same move"
            );
            assert_eq!(
                eval.score, expected_eval.score,
                "alpha beta and minimax evaluate the same"
            );
        }
        test_initial_board_at_depth(1);
        test_initial_board_at_depth(2);
        test_initial_board_at_depth(3);
        // this takes forever...
        // test_initial_board_at_depth(4);
    }
}

pub enum AiSearch {
    AlphaBeta,
    Minimax,
    Random,
}

struct SearchResultCache {
    cache: HashMap<u64, (evaluator::Evaluation, Option<Move>)>
}

//@todo : pass in an Evaluator struct, or Evaluation function
// need to understand Box<> or something first
pub struct ai {
    rng: ThreadRng,
    color: Color,
    default_search_depth: u8,
    started_at: Instant,
    time_elapsed_during_search: Option<Duration>,
    minimax_calls: i64,
    ai_search_function: AiSearch,
    hasher: Zobrist,
    transposition_table: HashMap<u64, (evaluator::Evaluation, Option<Move>)>
}

impl ai {
    pub fn new(color: Color) -> ai {
        ai {
            rng: rand::thread_rng(),
            color,
            default_search_depth: 4,
            started_at: Instant::now(),
            time_elapsed_during_search: None,
            minimax_calls: 0,
            ai_search_function: AiSearch::AlphaBeta,
            hasher: Zobrist::new(),
            transposition_table: HashMap::new(),
        }
    }

    pub fn new_with_search(color: Color, search_fn: AiSearch) -> ai {
        ai {
            rng: rand::thread_rng(),
            color,
            default_search_depth: 4,
            started_at: Instant::now(),
            time_elapsed_during_search: None,
            minimax_calls: 0,
            ai_search_function: search_fn,
            hasher: Zobrist::new(),
            transposition_table: HashMap::new(),
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
    //
    fn alpha_beta(
        &mut self,
        board: &mut dyn BoardTrait,
        player_moving: Color,
        depth_to_go: u8,
        mut lower_bound: Option<evaluator::Evaluation>,
        mut upper_bound: Option<evaluator::Evaluation>,
    ) -> (evaluator::Evaluation, Option<Move>) {
        // transposition table
        let hash = self.hasher.hash_board(board);
        if self.transposition_table.contains_key(&hash) {
            return self.transposition_table.get(&hash).unwrap().clone();
        }

        // end of recursion, depth_to_go = 0 so eval the board
        if depth_to_go == 0 {
            self.minimax_calls = self.minimax_calls + 1;
            return (evaluator::evaluate(board, None, None), None);
        }
        // also end recursion if someone lost a king
        let kings = board.get_kings();
        if kings.len() < 2 {
            return (evaluator::evaluate(board, None, None), None);
        }
        // search moves
        let moves_to_try = gen_legal_moves(board, player_moving);
        // this should cover checkmates so we don't try to search further,

        // attempting to pass moves to evaluator
        let white_moves = match player_moving {
            Color::White => Some(&moves_to_try),
            Color::Black => None,
        };
        let black_moves = match player_moving {
            Color::White => None,
            Color::Black => Some(&moves_to_try),
        };

        if moves_to_try.len() == 0 {
            return (evaluator::evaluate(board, white_moves, black_moves), None);
        }

        // dfs with bounds
        // find the best move out of the legal moves
        let mut best_move: Option<Move> = None;
        let mut best_eval: Option<Evaluation> = None;
        for a_move in moves_to_try {
            // player takes move , examine this board
            // assuming this player and the opponent make optimal moves
            // what's the evaluation of the best board state starting from here ?
            board.make_move_mut(&a_move);
            let (eval, _m) = self.alpha_beta(
                board,
                player_moving.opposite(),
                depth_to_go - 1,
                lower_bound,
                upper_bound,
            );
            board.unmake_move_mut(&a_move);

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
            if Color::Black == player_moving
                && lower_bound.is_some()
                && eval.score < lower_bound.as_ref().unwrap().score
            {
                return (best_eval.unwrap(), best_move);
            }
            // if score is higher than higher bound then black will object
            if Color::White == player_moving
                && upper_bound.is_some()
                && eval.score > upper_bound.as_ref().unwrap().score
            {
                return (best_eval.unwrap(), best_move);
            }

            // set bounds
            // if no bounds set, then set them
            if Color::White == player_moving && lower_bound.is_none() {
                lower_bound = Some(eval.clone());
                best_move = Some(a_move);
            }
            if Color::Black == player_moving && upper_bound.is_none() {
                upper_bound = Some(eval.clone());
                best_move = Some(a_move);
            }
        }
        let result = (best_eval.unwrap(), best_move);
        self.transposition_table.insert(hash, result.clone());
        return result;
    }

    fn choose_random_move(
        &mut self,
        board: &dyn BoardTrait,
    ) -> (evaluator::Evaluation, Option<Move>) {
        let mut moves = gen_legal_moves(board, self.color);
        if moves.len() == 0 {
            return (evaluator::evaluate(board, None, None), None);
        }
        let move_count = moves.iter().len();
        let i = self.rng.gen_range(0..move_count);
        let chosen_move = moves.remove(i);
        (evaluator::evaluate(board, None, None), Some(chosen_move))
    }

    fn minimax(
        &mut self,
        board: &mut dyn BoardTrait,
        color: Color,
        depth: u8,
    ) -> (evaluator::Evaluation, Option<Move>) {
        // end of recursion
        if depth == 0 {
            self.minimax_calls = self.minimax_calls + 1;
            return (evaluator::evaluate(board, None, None), None);
        }
        // also end recursion if someone lost a king
        let kings = board.get_kings();
        if kings.len() < 2 {
            return (evaluator::evaluate(board, None, None), None);
        }
        // search moves
        let moves_to_try = gen_legal_moves(board, color);
        if moves_to_try.len() == 0 {
            return (evaluator::evaluate(board, None, None), None);
        }

        // dfs with recursion time
        let best = moves_to_try.into_iter().fold(None, |acc, move_to_try| {
            // @todo : if this move takes the king return
            let piece_captured = board.get_piece_at(&move_to_try.to);
            if piece_captured.is_some() && piece_captured.unwrap().piece_type == PieceType::King {
                // hmmm....
                panic!("king captured in search, we never should've gotten here.");
            }

            // player takes move , examine this board
            // assuming this player and the opponent make optimal moves
            // what's the evaluation of the best board state starting from here ?
            board.make_move_mut(&move_to_try);
            let (eval, _) = self.minimax(board, color.opposite(), depth - 1);
            board.unmake_move_mut(&move_to_try);

            if acc.is_none() {
                return Some((eval, move_to_try));
            }

            // choose the best move for this player, white wants high eval scores , black wants low eval scores
            let (best_eval_so_far, _best_move_so_far) = acc.as_ref().unwrap();
            if (Color::White == color && eval.score > best_eval_so_far.score)
                || (Color::Black == color && eval.score < best_eval_so_far.score)
            {
                return Some((eval, move_to_try));
            }
            return acc;
        });
        if best.is_none() {
            return (evaluator::evaluate(board, None, None), None);
        } else {

            let (eval, m) = best.unwrap();
            return (eval, Some(m));
        }
    }

    // do an exhaustive search , depth-first search
    // should return an eval, board, and move list to reach that board
    fn search(
        &mut self,
        board: &dyn BoardTrait,
        depth: u8,
        color: Color,
    ) -> Option<(evaluator::Evaluation, Option<Move>)> {
        self.minimax_calls = 0;
        self.started_at = Instant::now();

        let (eval, best_move): (Evaluation, Option<Move>) = match self.ai_search_function {
            AiSearch::AlphaBeta => self.alpha_beta(&mut *board.clone(), color, depth, None, None),
            AiSearch::Minimax => self.minimax(&mut *board.clone(), color, depth),
            AiSearch::Random => self.choose_random_move(board),
        };
        // check move
        // if best_move.is_some() {
        //     let best_move = best_move.unwrap();
        //     let all_moves = gen_legal_moves(board, self.color);
        //     let best_move_is_legal = all_moves.iter().any(|m| m == &best_move);
        //     if !best_move_is_legal {
        //         println!("best move \n{}", best_move);
        //         println!(
        //             "all moves \n{}",
        //             all_moves
        //                 .iter()
        //                 .map(|m| m.to_string())
        //                 .collect::<Vec<String>>()
        //                 .join("\n")
        //         );
        //         panic!("AI SEARCH TRYING ILLEGAL MOVE");
        //     }
        // }

        // print stuff here
        self.time_elapsed_during_search = Some(self.started_at.elapsed());
        return Some((eval, best_move));
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
            Some((_eval, m)) => m,
        }
    }
}
