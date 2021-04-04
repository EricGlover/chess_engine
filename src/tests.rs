#[cfg(test)]
mod eval_tester {
    use crate::fen_reader;
    use crate::AI::evaluator;
    use crate::AI::AI;

    fn test_evaluate() {}

    #[test]
    fn test_eval() {
        let board = fen_reader::make_board(fen_reader::INITIAL_BOARD);
        assert_eq!(evaluator::evaluate(&board), 0.0);
        let board = fen_reader::make_board(fen_reader::WHITE_IN_CHECK);
        println!("{:?}", evaluator::evaluate(&board));
    }
}

#[cfg(test)]
mod move_gen_tester {
    use crate::board::*;
    use crate::fen_reader;
    use crate::move_generator::*;

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
        let white_moves = gen_legal_moves(&board, Color::White);
        let black_moves = gen_legal_moves(&board, Color::Black);
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

        let m1 = Move::new(Coordinate::new(1, 1), Coordinate::new(1, 1), pawn, false);
        let m2 = Move::new(Coordinate::new(2, 1), Coordinate::new(1, 1), pawn, false);
        let m3 = Move::new(Coordinate::new(1, 1), Coordinate::new(1, 1), pawn_2, false);

        let ml: Vec<Move> = vec![m1.clone(), m2.clone()];
        let ml2: Vec<Move> = vec![m1.clone(), m2.clone()];
        let ml3: Vec<Move> = vec![m1.clone(), m3.clone()];
        assert!(move_list_eq(&ml, &ml2));
        assert!(!move_list_eq(&ml, &ml3));
    }
}
