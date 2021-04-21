use crate::board::*;
use crate::fen_reader;
use crate::move_generator::*;

pub mod pgn {
    use crate::board::{Board, BoardTrait, Coordinate, PieceType};
    use crate::fen_reader::make_board;
    use crate::game::Game as chess_game;
    use crate::move_generator::{gen_legal_moves, Move};
    use std::fmt;
    use std::fmt::Formatter;

    const TEST_PGN: &str = r#"[Event "F/S Return Match"]
[Site "Belgrade, Serbia JUG"]
[Date "1992.11.04"]
[Round "29"]
[White "Fischer, Robert J."]
[Black "Spassky, Boris V."]
[Result "1/2-1/2"]

1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 {This opening is called the Ruy Lopez.}
4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
Nf2 42. g4 Bd3 43. Re6 1/2-1/2"#;

    enum Termination {
        Abandoned,
        Adjudication,
        Death,
        Emergency,
        Normal,
        Time,
        Forfeit,
        RulesInfraction,
        Unterminated,
    }
    impl fmt::Display for Termination {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let str = match self {
                Termination::Abandoned => "abandoned",
                Termination::Adjudication => "adjudication",
                Termination::Death => "death",
                Termination::Emergency => "emergency",
                Termination::Normal => "normal",
                Termination::Time => "time",
                Termination::Forfeit => "forfeit",
                Termination::RulesInfraction => "rulesInfraction",
                Termination::Unterminated => "unterminated",
            };
            write!(f, "{}", str)
        }
    }
    enum Mode {
        OverTheBoard,
        InternetChessServer,
    }
    impl fmt::Display for Mode {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let str = match self {
                Mode::OverTheBoard => "OTB",
                Mode::InternetChessServer => "ICS",
            };
            write!(f, "{}", str)
        }
    }

    /**
                                                                                                                                                    <piece_specifier><piece_file | piece_rank | piece_file && piece_rank><captures><file><rank>
                                                                                                                                                    piece_specifier = ['R', 'B', 'N', 'Q', 'K']
                                                                                                                                                    piece_file = [a-h][1-8]
                                                                                                                                                    captures = 'x'
                                                                                                                                                    file = [a-h]
                                                                                                                                                    rank = [1-8]
                                                                                                                                                    **/
    /**
                                                                                                                                                    When two (or more) identical pieces can move to the same square, the moving piece is uniquely
                                                                                                                                                    identified by specifying the piece's letter, followed by (in descending order of preference):

                                                                                                                                                    1. the file of departure (if they differ); or
                                                                                                                                                    2. the rank of departure (if the files are the same but the ranks differ); or
                                                                                                                                                    3. both the file and rank of departure (if neither alone is sufficient to
                                                                                                                                                    identify the piece—which occurs only in rare cases where a player has three or more identical
                                                                                                                                                    pieces able to reach the same square, as a result of one or more pawns having promoted).
                                                                                                                                                    **/

    fn get_piece_specifier(m: &Move, board: &dyn BoardTrait) -> String {
        // search for other moves , if similar moves we have to get specific about what piece is moving
        let mover_color = m.piece.color;
        let mut moves = gen_legal_moves(board, mover_color);
        let similar_moves: Vec<Move> = moves
            .drain(..)
            .filter(|m2| m2.piece.piece_type == m.piece.piece_type && m2.to == m.to)
            .into_iter()
            .collect::<Vec<Move>>();

        let mut has_similar_moves = false;
        let mut piece_specifier = String::new();
        if similar_moves.len() > 1 {
            let mut has_same_file = false;
            let mut has_same_rank = false;
            has_similar_moves = true;

            // check file
            let mut same_file_moves: Vec<&Move> = vec![];
            for m2 in similar_moves.iter() {
                if m2.from.x() == m.from.x() {
                    same_file_moves.push(&m2);
                }
            }
            if same_file_moves.len() > 1 {
                has_same_file = true;
            }

            // check rank
            let mut same_rank_moves: Vec<&Move> = vec![];
            for m2 in similar_moves.iter() {
                if m2.from.y() == m.from.y() {
                    same_rank_moves.push(&m2);
                }
            }

            if same_rank_moves.len() > 1 {
                has_same_rank = true;
            }
            if !has_same_file {
                piece_specifier = String::from(m.from.x_to());
            } else if !has_same_rank {
                piece_specifier = String::from(m.from.y_to());
            } else {
                piece_specifier = format!("{}{}", m.from.x_to(), m.from.y_to().as_str());
            }
        }
        piece_specifier
    }

    pub fn make_move_log(m: &Move, board: &dyn BoardTrait) -> String {
        // if m.is_king_side_castle() {
        //     return String::from("O-O"); // O not 0
        // }
        // if m.is_queen_side_castle() {
        //     return String::from("O-O-O"); // O not 0
        // }

        let piece_specifier = get_piece_specifier(m, board);

        // =Q or =B etc
        let mut pawn_promotion = String::new();
        if m.promoted_to.is_some() {
            pawn_promotion = format!("={}", m.promoted_to.unwrap().to().to_uppercase());
        }
        //@todo:: en passant

        let mut check = "";
        if m.is_check {
            check = "+";
        }
        if m.is_checkmate {
            check = "#";
        }

        let piece = if m.piece.piece_type == PieceType::Pawn {
            String::new()
        } else {
            m.piece.piece_type.to().to_uppercase()
        };
        let captures = if m.captured.is_some() { "x" } else { "" };
        let capture_file = m.to.x_to();
        let capture_rank = m.to.y_to();
        format!(
            "{}{}{}{}{}{}{}",
            piece, piece_specifier, captures, capture_file, capture_rank, pawn_promotion, check
        )
    }

    pub struct Game {
        pub event: String,
        pub site: String,
        pub date: String,
        pub round: String,
        pub white: String,
        pub black: String,
        pub result: String,
        pub move_text: String,
    }

    impl Game {
        pub fn new_from_game(game: chess_game) -> Game {
            //  @todo:
            // let mut moves:Vec<String> = game.moves();
            // let mut turns = vec![];
            // while moves.len() > 1 {
            //     let m1 = moves.remove(0);
            // }
            //
            // // check for one move
            // for (i, &turn) in moves.into_iter().enumerate() {
            //
            // }
            Game {
                event: String::from(r#""""#),
                site: String::from(r#""""#),
                date: String::from(r#""""#),
                round: String::from(r#""""#),
                white: String::from(r#""""#),
                black: String::from(r#""""#),
                result: String::from(r#""""#),
                move_text: String::from(
                    "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 {This opening is called the Ruy Lopez.}
4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
Nf2 42. g4 Bd3 43. Re6 1/2-1/2",
                ),
            }
        }
        fn print_tag(&self, name: &str, value: &String) -> String {
            format!("[{} {}]", name, value)
        }
    }

    impl fmt::Display for Game {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            let mut tags = vec![
                self.print_tag("Event", &self.event),
                self.print_tag("Site", &self.site),
                self.print_tag("Date", &self.date),
                self.print_tag("Round", &self.round),
                self.print_tag("White", &self.white),
                self.print_tag("Black", &self.black),
                self.print_tag("Result", &self.result),
            ];
            let str = format!("{}\n\n{}", tags.join("\n"), self.move_text);
            write!(f, "{}", str)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_make_move_log() {
            // double capture
            let double_capture = "rnbqkbnr/ppppp1pp/8/5p2/4P1P1/8/PPPP1P1P/RNBQKBNR b KQkq g3 0 2";
            let board = make_board(double_capture);
            // let m = Move::new()
        }

        #[test]
        fn test_pgn() {
            let game = Game {
                event: String::from(r#""F/S Return Match""#),
                site: String::from(r#""Belgrade, Serbia JUG""#),
                date: String::from(r#""1992.11.04""#),
                round: String::from(r#""29""#),
                white: String::from(r#""Fischer, Robert J.""#),
                black: String::from(r#""Spassky, Boris V.""#),
                result: String::from(r#""1/2-1/2""#),
                move_text: String::from(
                    "1. e4 e5 2. Nf3 Nc6 3. Bb5 a6 {This opening is called the Ruy Lopez.}
4. Ba4 Nf6 5. O-O Be7 6. Re1 b5 7. Bb3 d6 8. c3 O-O 9. h3 Nb8 10. d4 Nbd7
11. c4 c6 12. cxb5 axb5 13. Nc3 Bb7 14. Bg5 b4 15. Nb1 h6 16. Bh4 c5 17. dxe5
Nxe4 18. Bxe7 Qxe7 19. exd6 Qf6 20. Nbd2 Nxd6 21. Nc4 Nxc4 22. Bxc4 Nb6
23. Ne5 Rae8 24. Bxf7+ Rxf7 25. Nxf7 Rxe1+ 26. Qxe1 Kxf7 27. Qe3 Qg5 28. Qxg5
hxg5 29. b3 Ke6 30. a3 Kd6 31. axb4 cxb4 32. Ra5 Nd5 33. f3 Bc8 34. Kf2 Bf5
35. Ra7 g6 36. Ra6+ Kc5 37. Ke1 Nf4 38. g3 Nxh3 39. Kd2 Kb5 40. Rd6 Kc5 41. Ra6
Nf2 42. g4 Bd3 43. Re6 1/2-1/2",
                ),
            };
            // println!("{}", game);
            let str = String::from(TEST_PGN);
            // println!("original === \n{}", str.to_string());
            // println!("mine ===\n{}", format!("{}", game).to_string());
            assert_eq!(str.to_string(), format!("{}", game).to_string())
        }
    }
}
/**
chess move reader

<piece_specifier><piece_file | piece_rank | piece_file && piece_rank><captures><file><rank>
piece_specifier = ['R', 'B', 'N', 'Q', 'K']
piece_file = [a-h][1-8]
captures = 'x'
file = [a-h]
rank = [1-8]
**/

// algebraic moves and move generator moves are different because they're board dependent

pub fn parse_move(str: &str) -> (PieceType, Coordinate) {
    // need to generate moves to determine which piece can move there
    // piece specifier is uppercase
    let mut chars = str.chars().collect::<Vec<char>>();
    let first = chars.get(0).unwrap();

    let piece_type: PieceType = if first.to_lowercase().to_string() != first.to_string() {
        let t = PieceType::from(first.to_lowercase().to_string().as_str()).unwrap();
        chars.remove(0);
        t
    } else {
        PieceType::Pawn
    };
    let s: String = chars.splice(0..2, std::iter::empty()).collect();
    let to = Coordinate::from(s.as_str());
    (piece_type, to)
}

// change this to result error ?
// doesn't return illegal moves, return None if not possible
pub fn read_move<'a>(str: &str, board: &'a dyn BoardTrait, color: Color) -> Option<Move<'a>> {
    // figure out what they're trying to move and where
    let (piece_type, to) = parse_move(str);

    // find what piece they're talking about by looking through the possible moves
    let mut moves = gen_legal_moves(board, color);
    moves
        .into_iter()
        .find(|m| m.piece.piece_type == piece_type && m.to == to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_move_test() {
        let board = fen_reader::make_board(fen_reader::INITIAL_BOARD);
        let s = "Ra2";
        let s2 = "a4";
        let m = read_move(s, &board, Color::White);
        let m2 = read_move(s2, &board, Color::White).unwrap();
        let a1 = Coordinate::from("a1");
        let a2 = Coordinate::from("a2");
        let a4 = Coordinate::from("a4");
        let rook = Piece::new(Color::White, PieceType::Rook, Some(a1.clone()));
        let pawn = Piece::new(Color::White, PieceType::Pawn, Some(a2.clone()));
        assert!(m.is_none());
        assert_eq!(m2, Move::new(a2.clone(), a4.clone(), &pawn, false));
    }
}
