use crate::board::*;
use crate::board_console_printer;

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

fn read_piece(char: &str) -> Piece {
    let color = if char.to_lowercase() == char {
        Color::Black
    } else {
        Color::White
    };
    let piece_type = PieceType::from(char.to_lowercase().as_str()).unwrap();
    Piece::new(color, piece_type, None)
}

fn read_pieces(piece_string: &str, board: &mut Board) {
    // tokenize by row
    let piece_chars = "PNBRQKpnbrqk";
    let numbers = "123456789";
    let rows = piece_string.split("/");
    for (i, row) in rows.enumerate() {
        let y = 8 - (i as u8);
        let mut x: u8 = 1;
        // read each character of the string
        for (_, char) in row.chars().enumerate() {
            let coordinate = Coordinate { y, x };
            if numbers.contains(char) {
                x += char.to_string().parse::<u8>().unwrap();
            } else if piece_chars.contains(char) {
                let mut piece = read_piece(char.to_string().as_str());
                board.place_piece(piece, coordinate);
                x += 1;
            } else {
                panic!("{} char not recognized", char);
            }
        }
    }
}

pub fn make_initial_board() -> Board {
    read(INITIAL_BOARD)
}

pub fn read(fen_string: &str) -> Board {
    let mut board = Board::new();
    let parts = fen_string.split(" ").collect::<Vec<&str>>();
    read_pieces(parts[0], &mut board);
    board.white_to_move = parts[1] == "w";
    board.white_can_castle_king_side = parts[2].contains("K");
    board.white_can_castle_queen_side = parts[2].contains("Q");
    board.black_can_castle_king_side = parts[2].contains("k");
    board.black_can_castle_queen_side = parts[2].contains("q");
    board.en_passant_target = if parts[3] == "-" {
        None
    } else {
        Some(Coordinate::from(parts[3]))
    };
    board.half_move_clock = parts[4].parse::<u8>().unwrap();
    board.full_move_number = parts[5].parse::<u8>().unwrap();

    println!("{:?}", board.get_pieces(Color::White));
    return board;
}

#[test]
fn test_initial_board() {
    let board = read(INITIAL_BOARD);
    let white_pieces = board.get_pieces(Color::White);
    let black_pieces = board.get_pieces(Color::Black);
    assert_eq!(white_pieces.len(), 16);
    assert_eq!(black_pieces.len(), 16);

    for piece in white_pieces.iter() {
        match piece.piece_type {
            PieceType::King => {
                assert_eq!(piece.at().unwrap(), Coordinate { x: 5, y: 1 });
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
    let board = read(TEST_BOARD_2);
    fn has_piece(board: &Board, at: &Coordinate) -> bool {
        board.has_piece(at)
    }
    board_console_printer::print_board(&board);

    // test if pieces are in the correct spot
    // todo: test if pieces themselves are the correct kind

    // row 8
    assert_eq!(board.has_piece(&Coordinate { x: 1, y: 8 }), false);
    assert_eq!(board.has_piece(&Coordinate { x: 2, y: 8 }), false);
    assert_eq!(board.has_piece(&Coordinate { x: 3, y: 8 }), true);
    assert_eq!(board.has_piece(&Coordinate { x: 4, y: 8 }), true);
    assert_eq!(board.has_piece(&Coordinate { x: 5, y: 8 }), false);
    assert_eq!(board.has_piece(&Coordinate { x: 6, y: 8 }), true);
    assert_eq!(board.has_piece(&Coordinate { x: 7, y: 8 }), false);
    assert_eq!(board.has_piece(&Coordinate { x: 8, y: 8 }), true);

    // row 7
    assert_eq!(has_piece(&board, &Coordinate { x: 1, y: 7 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 2, y: 7 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 3, y: 7 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 4, y: 7 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 5, y: 7 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 6, y: 7 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 7, y: 7 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 8, y: 7 }), true);
    // row 6
    assert_eq!(has_piece(&board, &Coordinate { x: 1, y: 6 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 2, y: 6 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 3, y: 6 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 4, y: 6 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 5, y: 6 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 6, y: 6 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 7, y: 6 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 8, y: 6 }), false);
    // row 5
    assert_eq!(has_piece(&board, &Coordinate { x: 1, y: 5 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 2, y: 5 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 3, y: 5 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 4, y: 5 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 5, y: 5 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 6, y: 5 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 7, y: 5 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 8, y: 5 }), false);

    // row 4
    assert_eq!(has_piece(&board, &Coordinate { x: 1, y: 4 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 2, y: 4 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 3, y: 4 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 4, y: 4 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 5, y: 4 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 6, y: 4 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 7, y: 4 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 8, y: 4 }), true);
    // row 3
    assert_eq!(has_piece(&board, &Coordinate { x: 1, y: 3 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 2, y: 3 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 3, y: 3 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 4, y: 3 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 5, y: 3 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 6, y: 3 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 7, y: 3 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 8, y: 3 }), false);
    // row 2
    assert_eq!(has_piece(&board, &Coordinate { x: 1, y: 2 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 2, y: 2 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 3, y: 2 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 4, y: 2 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 5, y: 2 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 6, y: 2 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 7, y: 2 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 8, y: 2 }), true);
    // row 1
    assert_eq!(has_piece(&board, &Coordinate { x: 1, y: 1 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 2, y: 1 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 3, y: 1 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 4, y: 1 }), false);
    assert_eq!(has_piece(&board, &Coordinate { x: 5, y: 1 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 6, y: 1 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 7, y: 1 }), true);
    assert_eq!(has_piece(&board, &Coordinate { x: 8, y: 1 }), false);
}
