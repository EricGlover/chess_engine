#[cfg(test)]
mod eval_tester {
    use crate::ai::evaluator;
    use crate::ai::*;
    use crate::chess_notation::fen_reader;
    use crate::game_state::GameState;

    fn test_evaluate() {}

    #[test]
    fn test_eval() {
        let board = fen_reader::make_board(fen_reader::INITIAL_BOARD);
        let game_state = GameState::starting_game();
        assert_eq!(evaluator::evaluate(&game_state, None, None).score, 0.0);
        let board = fen_reader::make_board(fen_reader::WHITE_IN_CHECK);
        println!("{:?}", evaluator::evaluate(&game_state, None, None));
    }
}

#[cfg(test)]
mod test {
    use crate::ai::Ai;
    use crate::{board::*, game_state};
    use crate::board::{Color};
    use crate::chess_notation::fen_reader;
    use crate::move_generator::*;
    use crate::game_state::GameState;

    /*
    black player choose move
Qe6
Game time elasped : 225.456525662s
white to move

(15. Bxd7+ is white's move)

    thread 'main' panicked at 'tried to remove piece
piece not found at idx 57', src/game_state.rs:524:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/4d6d601c8a83284d6b23c253a3e2a060fd197316/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/4d6d601c8a83284d6b23c253a3e2a060fd197316/library/core/src/panicking.rs:142:14
   2: chess_engine::game_state::GameState::remove_piece_at
             at ./src/game_state.rs:524:13
   3: <chess_engine::game_state::GameState as chess_engine::board::board_trait::BoardTrait>::make_move_mut
             at ./src/game_state.rs:152:44
   4: chess_engine::ai::Ai::alpha_beta
             at ./src/ai.rs:280:13
   5: chess_engine::ai::Ai::alpha_beta
             at ./src/ai.rs:281:30
   6: chess_engine::ai::Ai::alpha_beta
             at ./src/ai.rs:281:30
   7: chess_engine::ai::Ai::alpha_beta
             at ./src/ai.rs:281:30
   8: chess_engine::ai::Ai::search
             at ./src/ai.rs:437:36
   9: chess_engine::ai::Ai::make_move
             at ./src/ai.rs:477:17
  10: chess_engine::game::Game::run_sim_game
             at ./src/game.rs:212:25
  11: chess_engine::main
             at ./src/main.rs:94:13
  12: core::ops::function::FnOnce::call_once
             at /rustc/4d6d601c8a83284d6b23c253a3e2a060fd197316/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
     */



    #[test]
    fn test_opera_game() {
        let fail_fen = "4kb1r/p2r1ppp/4qn2/1B2p1B1/4P3/1Q6/PPP2PPP/2KR4 w k - 10 20";
        let mut game_state = fen_reader::make_game_state(fail_fen);
        // let moves = gen_legal_moves(&game_state, Color::White);
        let mut ai = Ai::new(Color::White);
        let chosen_move = ai.make_move(&mut game_state, None);
    }
}




#[cfg(test)]
mod move_gen_tester {
    use crate::board::*;
    use crate::chess_notation::fen_reader;
    use crate::move_generator::*;
    use crate::game_state::GameState;

    fn move_list_eq(m: &Vec<Move>, m2: &Vec<Move>) -> bool {
        if m.len() != m2.len() {
            return false;
        }
        for mov in m.iter() {
            let mut found = false;
            for mov_2 in m2.iter() {
                if mov == mov_2 {
                    found = true;
                    break;
                }
            }
            if !found {
                return false;
            }
        }
        return true;
    }

    #[test]
    fn move_gen() {
        let board = fen_reader::make_board(fen_reader::INITIAL_BOARD);
        let game_state = GameState::starting_game();
        let white_moves = gen_legal_moves(&game_state, Color::White);
        let black_moves = gen_legal_moves(&game_state, Color::Black);
        println!("White moves");
        for m in white_moves.iter() {
            println!("{}", m);
        }
        println!("Black moves");
        for m in black_moves.iter() {
            println!("{}", m);
        }
        assert_eq!(white_moves.iter().len(), black_moves.iter().len());
    }

    #[test]
    fn move_list_is_same() {
        let pawn = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(1, 1)));
        let pawn_2 = Piece::new(Color::White, PieceType::Pawn, Some(Coordinate::new(1, 1)));

        let m1 = Move::new(
            Coordinate::new(1, 1),
            Coordinate::new(1, 1),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
            None,
        );
        let m2 = Move::new(
            Coordinate::new(2, 1),
            Coordinate::new(1, 1),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
            None,
        );
        let m3 = Move::new(
            Coordinate::new(1, 1),
            Coordinate::new(1, 1),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
            None,
        );

        let ml: Vec<Move> = vec![m1.clone(), m2.clone()];
        let ml2: Vec<Move> = vec![m1.clone(), m2.clone()];
        let ml3: Vec<Move> = vec![m1.clone(), m3.clone()];
        assert!(move_list_eq(&ml, &ml2));
        assert!(!move_list_eq(&ml, &ml3));
    }
}
