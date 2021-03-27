use crate::board::*;
use crate::board_console_printer;

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

#[test]
fn testing() {
    assert_eq!(1 + 3, 4);
}

#[test]
fn test_board_2() {
    let board = read(TEST_BOARD_2);
    fn has_piece(board: &Board, at: &Coordinate) -> bool {
        board.has_piece(at)
    }
    board_console_printer::print_board(&board);
    // println!("{:?}", board);

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
    assert_eq!(has_piece(&board,  &Coordinate {x: 1, y: 7}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 2, y: 7}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 3, y: 7}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 4, y: 7}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 5, y: 7}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 6, y: 7}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 7, y: 7}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 8, y: 7}), true);
    // row 6
    assert_eq!(has_piece(&board,  &Coordinate {x: 1, y: 6}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 2, y: 6}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 3, y: 6}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 4, y: 6}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 5, y: 6}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 6, y: 6}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 7, y: 6}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 8, y: 6}), false);
    // row 5
    assert_eq!(has_piece(&board,  &Coordinate {x: 1, y: 5}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 2, y: 5}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 3, y: 5}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 4, y: 5}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 5, y: 5}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 6, y: 5}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 7, y: 5}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 8, y: 5}), false);

    // row 4
    assert_eq!(has_piece(&board,  &Coordinate {x: 1, y: 4}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 2, y: 4}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 3, y: 4}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 4, y: 4}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 5, y: 4}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 6, y: 4}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 7, y: 4}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 8, y: 4}), true);
    // row 3
    assert_eq!(has_piece(&board,  &Coordinate {x: 1, y: 3}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 2, y: 3}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 3, y: 3}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 4, y: 3}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 5, y: 3}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 6, y: 3}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 7, y: 3}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 8, y: 3}), false);
    // row 2
    assert_eq!(has_piece(&board,  &Coordinate {x: 1, y: 2}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 2, y: 2}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 3, y: 2}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 4, y: 2}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 5, y: 2}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 6, y: 2}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 7, y: 2}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 8, y: 2}), true);
    // row 1
    assert_eq!(has_piece(&board,  &Coordinate {x: 1, y: 1}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 2, y: 1}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 3, y: 1}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 4, y: 1}), false);
    assert_eq!(has_piece(&board,  &Coordinate {x: 5, y: 1}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 6, y: 1}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 7, y: 1}), true);
    assert_eq!(has_piece(&board,  &Coordinate {x: 8, y: 1}), false);

}

fn read_piece(char: &str) -> Piece {
    let color = if char.to_lowercase() == char {
        Color::White
    } else {
        Color::Black
    };
    let piece: Piece = match char.to_lowercase().as_str() {
        "p" => Piece {
            color,
            piece_type: PieceType::Pawn,
        },
        "n" => Piece {
            color,
            piece_type: PieceType::Knight,
        },
        "b" => Piece {
            color,
            piece_type: PieceType::Bishop,
        },
        "r" => Piece {
            color,
            piece_type: PieceType::Rook,
        },
        "q" => Piece {
            color,
            piece_type: PieceType::Queen,
        },
        "k" => Piece {
            color,
            piece_type: PieceType::King,
        },
        _ => panic!("can not read {}", char),
    };
    piece
}

fn read_pieces(piece_string: &str, board: &mut Board) {
    // tokenize by row
    let piece_chars = "PNBRQKpnbrqk";
    let numbers = "123456789";
    let rows = piece_string.split("/");
    // println!("reading pieces ");
    for (i, row) in rows.enumerate() {
        let y = 8 - (i as u8);
        let mut x: u8 = 1;
        // read each character of the string
        for (_, char) in row.chars().enumerate() {
            let coordinate = Coordinate { y, x };
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
    // board
}

pub fn read(fen_string: &str) -> Board {
    let mut board = Board::new();
    let parts = fen_string.split(" ").collect::<Vec<&str>>();
    println!("parts = {:?}", parts);
    read_pieces(parts[0], &mut board);
    return board;
    // board_console_printer::print_board(board.get_squares());
    //
    // // let parts = string.split(" ").collect()
    // println!("{:?}", fen_string.split(" ").collect::<Vec<&str>>());
    // for str in fen_string.split(" ") {
    //     println!("doing stuff {}", str);
    // }
    // // steps , 1) tokenize by spaces
    // // parse the separate parts
    // println!("{}", INITIAL_BOARD);
    // let p = Piece {
    //     piece_type: PieceType::Queen,
    //     color: Color::White,
    // };
    // let char: char = 'r';
    // let color = Color::White; //@todo
                              // let piece : Piece = match char {
                              //     'p' => Piece{},
                              //     'n' => Piece{},
                              //     'b' => Piece{},
                              //     'r' => Piece{},
                              //     'q' => Piece{},
                              //     'k' => Piece{},
                              //     _ => panic!("can not read {}", _),
                              // };
}
