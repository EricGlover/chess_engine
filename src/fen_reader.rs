use crate::board::new_board::*;
use crate::board::*;
use crate::board_console_printer;
use crate::board_console_printer::print_board;
use std::path::Prefix::Verbatim;

// @todo : board -> fen string

/**
FEN SPEC
    1. Piece placement (from White's perspective). Each rank is described, starting with rank 8 and ending with rank 1;
        within each rank, the contents of each square are described from file "a" through file "h".
        Following the Standard Algebraic Notation (SAN), each piece is identified by a single letter taken from the
        standard English names (pawn = "P", knight = "N", bishop = "B", rook = "R", queen = "Q" and king = "K").
        White pieces are designated using upper-case letters ("PNBRQK") while black pieces use lowercase ("pnbrqk").
        Empty squares are noted using digits 1 through 8 (the number of empty squares), and "/" separates ranks.
    2. Active color. "w" means White moves next, "b" means Black moves next.
    3. Castling availability. If neither side can castle, this is "-".
        Otherwise, this has one or more letters: "K" (White can castle kingside),
        "Q" (White can castle queenside), "k" (Black can castle kingside), and/or "q" (Black can castle queenside).
        A move that temporarily prevents castling does not negate this notation.
    4. En passant target square in algebraic notation.
        If there's no en passant target square, this is "-".
        If a pawn has just made a two-square move, this is the position "behind" the pawn.
        This is recorded regardless of whether there is a pawn in position to make an en passant capture.
    5. Halfmove clock: This is the number of halfmoves since the last capture or pawn advance.
        The reason for this field is that the value is used in the fifty-move rule.
    6. Fullmove number: The number of the full move. It starts at 1, and is incremented after Black's move.
**/

pub const INITIAL_BOARD: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
pub const TEST_BOARD_1: &str =
    "r1bqkb1r/ppp2p1p/2n2np1/1B1pp3/3PPB2/5N2/PPP2PPP/RN1Q1RK1 b kq - 1 6";
pub const TEST_BOARD_2: &str = "2kr1b1r/1bp4p/ppn3p1/1B1pNp2/P2PnBQq/N1P5/1P3PPP/4RRK1 w - - 1 13";
pub const BLACK_IN_CHECK: &str =
    "rn1qk2r/pppB1ppp/4pn2/3p4/1b1P4/2N1PN2/PPP2PPP/R1BQK2R b KQkq - 0 6";
pub const WHITE_IN_CHECK: &str =
    "rn2k2r/ppp2ppp/5n2/3Pp1q1/1b1PK3/P1N2N2/1PP2PPP/R1BQ3R w kq - 3 13";

fn piece_to_fen(piece: &Piece) -> String {
    let str = match piece.piece_type {
        PieceType::King => "k",
        PieceType::Queen => "q",
        PieceType::Bishop => "b",
        PieceType::Knight => "n",
        PieceType::Rook => "r",
        PieceType::Pawn => "p",
    };
    if piece.color == Color::White {
        return str.to_uppercase();
    } else {
        return String::from(str);
    }
}

// @todo : print timestamp+output.txt for games
// @todo : run ai v ai
// @todo : use fen to debug the board.make_move for the king castling bug
fn make_fen_pieces(board: &dyn BoardTrait) -> String {
    (1u8..9)
        .into_iter()
        .map(|i| {
            let mut pieces: Vec<String> = vec![];
            let mut empty = 0;
            board.get_rank(i).iter().for_each(|&square| {
                // insert empty
                if empty > 0 && square.piece().is_some() {
                    pieces.push(empty.to_string());
                    empty = 0;
                }
                if square.piece().is_some() {
                    pieces.push(piece_to_fen(square.piece().unwrap()));
                } else {
                    empty = empty + 1;
                }
            });
            // insert empty
            if empty > 0 {
                pieces.push(empty.to_string());
                empty = 0;
            }
            pieces.concat()
        })
        .rev()
        .collect::<Vec<String>>()
        .join("/")
}

pub fn make_fen(board: &dyn BoardTrait) -> String {
    // piece location string
    let piece_string = make_fen_pieces(board);
    // player to move
    let to_move = board.player_to_move().to_char();
    // castling rights
    let mut castle_rights = String::new();
    if board.can_castle_king_side(Color::White) {
        castle_rights.push('K');
    }
    if board.can_castle_queen_side(Color::White) {
        castle_rights.push('Q');
    }
    if board.can_castle_king_side(Color::Black) {
        castle_rights.push('k');
    }
    if board.can_castle_queen_side(Color::Black) {
        castle_rights.push('q');
    }
    if castle_rights.is_empty() {
        castle_rights.push('-');
    }
    // en passant square
    let en_passant = if board.en_passant_target().is_some() {
        board.en_passant_target().unwrap().clone().to_string()
    } else {
        String::from('-')
    };
    // half move
    let half_move = board.half_move_clock().to_string();
    // full move
    let full_move = board.full_move_number().to_string();
    format!(
        "{} {} {} {} {} {}",
        piece_string, to_move, castle_rights, en_passant, half_move, full_move
    )
}

fn read_piece(char: &str) -> Piece {
    let color = if char.to_lowercase() == char {
        Color::Black
    } else {
        Color::White
    };
    let piece_type = PieceType::from(char.to_lowercase().as_str()).unwrap();
    Piece::new(color, piece_type, None)
}

fn read_pieces(piece_string: &str, board: &mut dyn BoardTrait) {
    // tokenize by row
    let piece_chars = "PNBRQKpnbrqk";
    let numbers = "123456789";
    let rows = piece_string.split("/");
    for (i, row) in rows.enumerate() {
        let y = 8 - (i as u8);
        let mut x: u8 = 1;
        // read each character of the string
        for (_, char) in row.chars().enumerate() {
            let coordinate = Coordinate::new(x, y);
            if numbers.contains(char) {
                x += char.to_string().parse::<u8>().unwrap();
            } else if piece_chars.contains(char) {
                let piece = read_piece(char.to_string().as_str());
                board.place_piece(piece, &coordinate);
                x += 1;
            } else {
                panic!("{} char not recognized", char);
            }
        }
    }
}

pub fn make_initial_board() -> BoardRef {
    make_board(INITIAL_BOARD)
}

pub fn make_board(fen_string: &str) -> BoardRef {
    let parts = fen_string.split(" ").collect::<Vec<&str>>();
    let player_to_move = if parts[1] == "w" {
        Color::White
    } else {
        Color::Black
    };
    let white_can_castle_king_side = parts[2].contains("K");
    let white_can_castle_queen_side = parts[2].contains("Q");
    let black_can_castle_king_side = parts[2].contains("k");
    let black_can_castle_queen_side = parts[2].contains("q");
    let en_passant_target = if parts[3] == "-" {
        None
    } else {
        Some(Coordinate::from(parts[3]))
    };
    let half_move_clock = parts[4].parse::<u8>().unwrap();
    let full_move_number = parts[5].parse::<u8>().unwrap();

    let mut board = BoardRef::make_board(
        player_to_move,
        white_can_castle_king_side,
        white_can_castle_queen_side,
        black_can_castle_king_side,
        black_can_castle_queen_side,
        en_passant_target,
        half_move_clock,
        full_move_number,
    );

    read_pieces(parts[0], &mut board);
    return board;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_fen_pieces() {
        let expected = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
        let board = make_initial_board();
        let result = make_fen_pieces(&board);
        assert_eq!(expected, result, "reads pieces");
    }

    #[test]
    fn test_make_fen() {
        let board = make_initial_board();
        let fen_result = make_fen(&board);
        // "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let expected = self::INITIAL_BOARD;
        assert_eq!(expected, fen_result, "initial board fen string is correct");
        println!("{}", fen_result.as_str());
    }

    #[test]
    fn test_initial_board() {
        let board = make_board(INITIAL_BOARD);
        // print_board(&board);
        let white_pieces = board.get_all_pieces(Color::White);
        let black_pieces = board.get_all_pieces(Color::Black);

        assert_eq!(white_pieces.len(), 16);
        assert_eq!(black_pieces.len(), 16);

        for piece in white_pieces.iter() {
            match piece.piece_type {
                PieceType::King => {
                    assert_eq!(piece.at().unwrap(), Coordinate::new(5, 1));
                }
                PieceType::Queen => {}
                PieceType::Bishop => {}
                PieceType::Knight => {}
                PieceType::Rook => {}
                PieceType::Pawn => {}
            }
        }
    }

    #[test]
    fn test_board_2() {
        let board = make_board(TEST_BOARD_2);
        fn has_piece(board: &BoardTrait, at: &Coordinate) -> bool {
            board.has_piece(at)
        }
        // board_console_printer::print_board(&board);

        // test if pieces are in the correct spot
        // todo: test if pieces themselves are the correct kind

        // row 8
        assert_eq!(board.has_piece(&Coordinate::new(1, 8)), false);
        assert_eq!(board.has_piece(&Coordinate::new(2, 8)), false);
        assert_eq!(board.has_piece(&Coordinate::new(3, 8)), true);
        assert_eq!(board.has_piece(&Coordinate::new(4, 8)), true);
        assert_eq!(board.has_piece(&Coordinate::new(5, 8)), false);
        assert_eq!(board.has_piece(&Coordinate::new(6, 8)), true);
        assert_eq!(board.has_piece(&Coordinate::new(7, 8)), false);
        assert_eq!(board.has_piece(&Coordinate::new(8, 8)), true);

        // row 7
        assert_eq!(has_piece(&board, &Coordinate::new(1, 7)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(2, 7)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(3, 7)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(4, 7)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(5, 7)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(6, 7)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(7, 7)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(8, 7)), true);
        // row 6
        assert_eq!(has_piece(&board, &Coordinate::new(1, 6)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(2, 6)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(3, 6)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(4, 6)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(5, 6)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(6, 6)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(7, 6)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(8, 6)), false);
        // row 5
        assert_eq!(has_piece(&board, &Coordinate::new(1, 5)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(2, 5)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(3, 5)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(4, 5)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(5, 5)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(6, 5)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(7, 5)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(8, 5)), false);

        // row 4
        assert_eq!(has_piece(&board, &Coordinate::new(1, 4)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(2, 4)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(3, 4)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(4, 4)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(5, 4)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(6, 4)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(7, 4)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(8, 4)), true);
        // row 3
        assert_eq!(has_piece(&board, &Coordinate::new(1, 3)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(2, 3)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(3, 3)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(4, 3)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(5, 3)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(6, 3)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(7, 3)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(8, 3)), false);
        // row 2
        assert_eq!(has_piece(&board, &Coordinate::new(1, 2)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(2, 2)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(3, 2)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(4, 2)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(5, 2)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(6, 2)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(7, 2)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(8, 2)), true);
        // row 1
        assert_eq!(has_piece(&board, &Coordinate::new(1, 1)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(2, 1)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(3, 1)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(4, 1)), false);
        assert_eq!(has_piece(&board, &Coordinate::new(5, 1)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(6, 1)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(7, 1)), true);
        assert_eq!(has_piece(&board, &Coordinate::new(8, 1)), false);
    }
}
