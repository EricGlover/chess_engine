use crate::board::{BoardTrait, Coordinate, PieceType};
use crate::chess_notation::fen_reader;
use crate::chess_notation::fen_reader::make_board;
use crate::game::Game as chess_game;
use crate::move_generator::{gen_legal_moves, Move, MoveType};
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
    let piece = board.get_piece_at(&m.from).unwrap();
    let mover_color = piece.color;
    let mut moves = gen_legal_moves(board, mover_color);
    let similar_moves: Vec<Move> = moves
        .drain(..)
        .filter(|m2| m2.piece == piece.piece_type && m2.to == m.to)
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
    if m.is_king_side_castle() {
        return String::from("O-O"); // O not 0
    }
    if m.is_queen_side_castle() {
        return String::from("O-O-O"); // O not 0
    }

    let piece_specifier = get_piece_specifier(m, board);

    // =Q or =B etc
    let mut pawn_promotion = String::new();
    if let MoveType::Promotion(promoted_to) = m.move_type() {
        pawn_promotion = format!("={}", promoted_to.to().to_uppercase());
    }
    //@todo:: en passant

    let mut check = "";
    if m.is_check {
        check = "+";
    }
    if m.is_checkmate {
        check = "#";
    }

    let piece = if m.piece == PieceType::Pawn {
        if m.captured.is_none() {
            String::new()
        } else {
            String::from(m.from.x_to())
        }
    } else {
        m.piece.to().to_uppercase()
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
        // partition and step_by work here ?
        // try step_by and zip
        // zip together moves
        fn make_move_pairs(mut moves: Vec<String>) -> Vec<(String, Option<String>)> {
            let mut pairs = Vec::new();
            while moves.len() > 0 {
                let first = moves.remove(0);
                let second = if moves.len() > 0 {
                    Some(moves.remove(0))
                } else {
                    None
                };
                pairs.push((first, second));
            }
            pairs
        }
        let pairs = make_move_pairs(game.moves());

        // @todo : add result
        let move_text = pairs.into_iter().enumerate().fold(String::new(), |mut acc, (idx, (white, black))| {
            let black = black.unwrap_or(String::new());
            let move_string = format!("{}. {} {} ", idx, white, black);
            acc.push_str(move_string.as_str());
            acc
        });

        Game {
            event: String::from(r#""""#),
            site: String::from(r#""""#),
            date: String::from(r#""""#),
            round: String::from(r#""""#),
            white: String::from(r#""""#),
            black: String::from(r#""""#),
            result: String::from(r#""""#),
            move_text,
        }
    }
    fn print_tag(&self, name: &str, value: &String) -> String {
        format!("[{} {}]", name, value)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let tags = vec![
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
    use crate::board::CastlingRights;

    #[test]
    fn pawn_move() {
        let board = fen_reader::make_initial_board();
        let m = Move::new(
            Coordinate::new(1, 2),
            Coordinate::new(1, 3),
            PieceType::Pawn,
            MoveType::Move,
            None,
            None,
        );
        let log = make_move_log(&m, &board);
        assert_eq!(log, String::from("a3"));
    }

    #[test]
    fn non_pawn_move() {
        let board = fen_reader::make_initial_board();
        let m = Move::new(
            Coordinate::new(2, 1),
            Coordinate::new(1, 3),
            PieceType::Knight,
            MoveType::Move,
            None,
            None,
        );
        let log = make_move_log(&m, &board);
        assert_eq!(log, String::from("Na3"));

        // file specified
        let fen = "rnbqkbnr/1ppppppp/8/8/8/p1N1N3/PPPPPPPP/R1BQKB1R b KQkq - 1 5";
        let board = make_board(fen);
        let m = Move::new(
            Coordinate::new(3, 3),
            Coordinate::new(4, 5),
            PieceType::Knight,
            MoveType::Move,
            None,
            None,
        );
        let log = make_move_log(&m, &board);
        assert_eq!(
            log,
            String::from("Ncd5"),
            "Two Knights can reach d5, c file knight needs to be specified."
        );

        let m = Move::new(
            Coordinate::new(5, 3),
            Coordinate::new(4, 5),
            PieceType::Knight,
            MoveType::Move,
            None,
            None,
        );
        let log = make_move_log(&m, &board);
        assert_eq!(
            log,
            String::from("Ned5"),
            "Two Knights can reach d5, e file knight needs to be specified."
        );

        //rank specified
        let fen = "rnbqkbnr/1ppppppp/8/3N4/8/1nP5/P1QPPPPP/2BNKB1R w Kkq - 5 10";
        let board = make_board(fen);
        let m = Move::new(
            Coordinate::new(4, 1),
            Coordinate::new(5, 3),
            PieceType::Knight,
            MoveType::Move,
            None,
            None,
        );
        let log = make_move_log(&m, &board);
        assert_eq!(
            log,
            String::from("N1e3"),
            "Two Knights can reach d5, 1 rank knight needs to be specified."
        );

        let m = Move::new(
            Coordinate::new(4, 5),
            Coordinate::new(5, 3),
            PieceType::Knight,
            MoveType::Move,
            None,
            None,
        );
        let log = make_move_log(&m, &board);
        assert_eq!(
            log,
            String::from("N5e3"),
            "Two Knights can reach d5, 5th rank knight needs to be specified."
        );
    }

    #[test]
    fn captures() {
        let fen = "rnbqkbnr/1ppppppp/8/3N4/8/1nP5/P1QPPPPP/2BNKB1R w Kkq - 5 10";
        let board = make_board(fen);
        let m = Move::new(
            Coordinate::new(1, 2),
            Coordinate::new(2, 3),
            PieceType::Pawn,
            MoveType::Move,
            Some(PieceType::Knight),
            None,
        );
        let log = make_move_log(&m, &board);
        assert_eq!(log, String::from("axb3"), "A Pawn takes knight.");
    }

    #[test]
    fn pawn_promotion() {
        let fen = "rnbqkbnr/1ppppppp/8/8/2N5/2N5/PpPPPPPP/R1BQKB1R b KQkq - 1 6";
        let board = make_board(fen);
        let m = Move::new(
            Coordinate::new(2, 2),
            Coordinate::new(1, 1),
            PieceType::Pawn,
            MoveType::Promotion(PieceType::Knight),
            Some(PieceType::Rook),
            None,
        );
        let log = make_move_log(&m, &board);
        assert_eq!(
            log,
            String::from("bxa1=N"),
            "Pawn promotes and captures rook"
        );

        let m = Move::new(
            Coordinate::new(2, 2),
            Coordinate::new(1, 1),
            PieceType::Pawn,
            MoveType::Promotion(PieceType::Bishop),
            Some(PieceType::Rook),
            None,
        );
        let log = make_move_log(&m, &board);
        assert_eq!(
            log,
            String::from("bxa1=B"),
            "Pawn promotes and captures rook"
        );


        let m = Move::new(
            Coordinate::new(2, 2),
            Coordinate::new(1, 1),
            PieceType::Pawn,
            MoveType::Promotion(PieceType::Rook),
            Some(PieceType::Rook),
            None,
        );
        let log = make_move_log(&m, &board);
        assert_eq!(
            log,
            String::from("bxa1=R"),
            "Pawn promotes and captures rook"
        );
        let m = Move::new(
            Coordinate::new(2, 2),
            Coordinate::new(1, 1),
            PieceType::Pawn,
            MoveType::Promotion(PieceType::Queen),
            Some(PieceType::Rook),
            None,
        );
        let log = make_move_log(&m, &board);
        assert_eq!(
            log,
            String::from("bxa1=Q"),
            "Pawn promotes and captures rook"
        );
    }

    #[test]
    fn castling() {
        let fen = "rnbqkbnr/1pp4p/4pp2/3p2p1/3P4/2NQN1PB/PBP1PP1P/R3K2R b KQkq - 1 10";
        let board = make_board(fen);
        let m = Move::new(
            Coordinate::new(5, 1),
            Coordinate::new(7, 1),
            PieceType::King,
            MoveType::Castling {
                rook_from: Coordinate::new(8,1),
                rook_to: Coordinate::new(6, 1),
            },
            None,
            Some(CastlingRights::new(true, true)),
        );
        let log = make_move_log(&m, &board);
        assert_eq!(
            log,
            String::from("O-O"),
            "short castles"
        );

        let m = Move::new(
            Coordinate::new(5, 1),
            Coordinate::new(3, 1),
            PieceType::King,
            MoveType::Castling {
                rook_from: Coordinate::new(1,1),
                rook_to: Coordinate::new(4, 1),
            },
            None,
            Some(CastlingRights::new(true, true)),
        );
        let log = make_move_log(&m, &board);
        assert_eq!(
            log,
            String::from("O-O-O"),
            "long castles"
        );
    }

    #[test]
    fn en_passant() {
        // unimplemented!("");
        let fen = "rnbqkbnr/1pp4p/4pp2/3p4/3P1Pp1/2NQN1PB/PBP1P2P/R3K2R b KQkq f3 0 11";
        let board = make_board(fen);
        let m = Move::new(
            Coordinate::new(7, 4),
            Coordinate::new(6, 3),
            PieceType::Pawn,
            MoveType::EnPassant,
            Some(PieceType::Pawn),
            None,
        );
        let log = make_move_log(&m, &board);
        assert_eq!(
            log,
            String::from("gxf3"),
            "Pawn en passant captures"
        );
    }

    #[test]
    fn test_make_move_log() {
        // double capture
        let double_capture = "rnbqkbnr/ppppp1pp/8/5p2/4P1P1/8/PPPP1P1P/RNBQKBNR b KQkq g3 0 2";
        let board = make_board(double_capture);
        // let m = Move::new()
    }

    #[test]
    fn move_from_game() {
        let game = chess_game::new();

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
        let str = String::from(TEST_PGN);
        assert_eq!(str.to_string(), format!("{}", game).to_string())
    }
}
