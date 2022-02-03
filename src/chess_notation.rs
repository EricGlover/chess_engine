pub mod fen_reader;
pub mod pgn;

use crate::board::*;
use crate::move_generator::get_path_to;
use crate::move_generator::*;

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

pub fn print_move(m: &Move, board: &dyn BoardTrait) -> String {
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

pub fn get_path(from: &str, to: &str) -> Option<Vec<Coordinate>> {
    get_path_to(&Coordinate::from(from), &Coordinate::from(to))
}

// change this to result error ?
// doesn't return illegal moves, return None if not possible
pub fn parse_move(str: &str, board: &dyn BoardTrait, color: Color) -> Option<Move> {
    // find what piece they're talking about by looking through the possible moves
    let their_move = String::from(str);
    gen_legal_moves(board, color)
        .into_iter()
        .find(|m| their_move == print_move(m, board))
}

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

// algebraic moves and move generator moves are different because they're board dependent
fn read_move(str: &str) -> (PieceType, Coordinate) {
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

#[cfg(test)]
mod print_move_test {
    use super::*;
    use crate::board::CastlingRights;
    use crate::chess_notation::fen_reader::*;

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
            None,
        );
        let log = print_move(&m, &board);
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
            None,
        );
        let log = print_move(&m, &board);
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
            None,
        );
        let log = print_move(&m, &board);
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
            None,
        );
        let log = print_move(&m, &board);
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
            None,
        );
        let log = print_move(&m, &board);
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
            None,
        );
        let log = print_move(&m, &board);
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
            None,
        );
        let log = print_move(&m, &board);
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
            None,
        );
        let log = print_move(&m, &board);
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
            None,
        );
        let log = print_move(&m, &board);
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
            None,
        );
        let log = print_move(&m, &board);
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
            None,
        );
        let log = print_move(&m, &board);
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
                rook_from: Coordinate::new(8, 1),
                rook_to: Coordinate::new(6, 1),
            },
            None,
            Some(CastlingRights::new(true, true)),
            None,
        );
        let log = print_move(&m, &board);
        assert_eq!(log, String::from("O-O"), "short castles");

        let m = Move::new(
            Coordinate::new(5, 1),
            Coordinate::new(3, 1),
            PieceType::King,
            MoveType::Castling {
                rook_from: Coordinate::new(1, 1),
                rook_to: Coordinate::new(4, 1),
            },
            None,
            Some(CastlingRights::new(true, true)),
            None,
        );
        let log = print_move(&m, &board);
        assert_eq!(log, String::from("O-O-O"), "long castles");
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
            None,
        );
        let log = print_move(&m, &board);
        assert_eq!(log, String::from("gxf3"), "Pawn en passant captures");
    }

    #[test]
    fn test_make_move_log() {
        // double capture
        let double_capture = "rnbqkbnr/ppppp1pp/8/5p2/4P1P1/8/PPPP1P1P/RNBQKBNR b KQkq g3 0 2";
        let board = make_board(double_capture);
        // let m = Move::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::CastlingRights;
    use crate::chess_notation::fen_reader::*;

    #[test]
    fn read_move_test() {
        let board = fen_reader::make_board(fen_reader::INITIAL_BOARD);
        let s = "Ra2";
        let s2 = "a4";
        let m = parse_move(s, &board, Color::White);
        let m2 = parse_move(s2, &board, Color::White).unwrap();
        let a1 = Coordinate::from("a1");
        let a2 = Coordinate::from("a2");
        let a4 = Coordinate::from("a4");
        let rook = Piece::new(Color::White, PieceType::Rook, Some(a1.clone()));
        let pawn = Piece::new(Color::White, PieceType::Pawn, Some(a2.clone()));
        assert!(m.is_none());
        assert_eq!(
            m2,
            Move::new(
                a2.clone(),
                a4.clone(),
                pawn.piece_type,
                MoveType::Move,
                None,
                None,
                None,
            )
        );
    }
}
