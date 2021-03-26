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
pub const TEST_BOARD_1: &str = "r1bqkb1r/ppp2p1p/2n2np1/1B1pp3/3PPB2/5N2/PPP2PPP/RN1Q1RK1 b kq - 1 6";
pub const TEST_BOARD_2: &str = "2kr1b1r/1bp4p/ppn3p1/1B1pNp2/P2PnBQq/N1P5/1P3PPP/4RRK1 w - - 1 13";

#[test]
fn testing() {
    assert_eq!(1 + 3, 4);
}

fn read_piece(char: &str) -> Piece {
    let color = if char.to_lowercase() == char {
        Color::White
    } else {
        Color::Black
    };
    let piece: Piece = match char {
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

fn read_pieces(piece_string: &str, mut board : Board) -> Board {
    // tokenize by row
    let piece_chars = "PNBRQKpnbrqk";
    let numbers = "123456789";
    let rows = piece_string.split("/");
    // println!("reading pieces ");
    for (i, row) in rows.enumerate() {
        let y = 8 - (i as u8);
        let mut x: u8 = 1;
        // read each character of the string
        for (j, char) in row.chars().enumerate() {
            println!("x = {:?}", x);
            let coordinate = Coordinate { y, x };
            // println!("{:?}", coordinate);
            if numbers.contains(char) {
                // @todo : fix check out the third board
                println!("empty squares");
                println!("{:?}", char.to_string().parse::<u8>().unwrap());
                x += char.to_string().parse::<u8>().unwrap();
                println!("x = {:?}", x);
            } else if piece_chars.contains(char) {
                println!("making piece");
                let color = if char.to_string() == char.to_string().to_lowercase() { Color::Black } else { Color::White };
                let piece : Piece = match char.to_string().to_lowercase().as_str() {
                    "p" => Piece{color, piece_type: PieceType::Pawn},
                    "n" => Piece{color, piece_type: PieceType::Knight},
                    "b" => Piece{color, piece_type: PieceType::Bishop},
                    "r" => Piece{color, piece_type: PieceType::Rook},
                    "q" => Piece{color, piece_type: PieceType::Queen},
                    "k" => Piece{color, piece_type: PieceType::King},
                    _ => panic!("can not read {}", char),
                };
                board = board.place_piece(piece, &coordinate);
                x += 1;
            } else {
                panic!("{} char not recognized", char);
            }
        }
    }
    return board;
}

pub fn read(fen_string : &str) -> Board {
    // println!("number of string {}, chars = {}", numbers.split("").count(), numbers.chars().as_str());
    // println!("{:?}", numbers.split("").map(|x| x ).collect()); /// @todo collect ? , how many chars are in this
    //

    let mut board = Board::new();
    // board.place_piece(Piece::make_white_king(), Coordinate {x: 1, y: 8});
    // prin
    let parts = fen_string.split(" ").collect::<Vec<&str>>();
    board = read_pieces(parts[0], board);
    return board;
    board_console_printer::print_board(board.get_squares());

    // let parts = string.split(" ").collect()
    println!("{:?}", fen_string.split(" ").collect::<Vec<&str>>());
    for str in fen_string.split(" ") {
        println!("doing stuff {}", str);
    }
    // steps , 1) tokenize by spaces
    // parse the separate parts
    println!("{}", INITIAL_BOARD);
    let p = Piece {
        piece_type: PieceType::Queen,
        color: Color::White,
    };
    let char: char = 'r';
    let color = Color::White; //@todo
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