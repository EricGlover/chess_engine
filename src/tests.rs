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
