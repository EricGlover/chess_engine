use crate::move_generator::{gen_moves, Move, print_move_list, print_move};
use crate::chess_notation::read_move;
use crate::fen_reader;
use crate::board_console_printer::print_board;
use std::fmt;
use std::fmt::Formatter;

pub struct Player {
    time_used: u16,      // milliseconds
    time_remaining: u16, // milliseconds
    name: String,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Color {
        if self == &Color::White {
            Color::Black
        } else {
            Color::White
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = if self == &Color::White { "white" } else { "black" };
        write!(f, "{}", s)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
    at: Option<Coordinate>,
}

impl Piece {
    pub fn new(color: Color, piece_type: PieceType, at: Option<Coordinate>) -> Piece {
        Piece {
            piece_type,
            color,
            at,
        }
    }
    pub fn at(&self) -> Option<Coordinate> {
        self.at
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

impl PieceType {
    pub fn from(char: &str) -> Option<PieceType> {
        match char {
            "p" => Some(PieceType::Pawn),
            "n" => Some(PieceType::Knight),
            "b" => Some(PieceType::Bishop),
            "r" => Some(PieceType::Rook),
            "q" => Some(PieceType::Queen),
            "k" => Some(PieceType::King),
            _ => None,
        }
    }
}

#[test]
fn from_coordinate_test() {
    assert_eq!(Coordinate::from("a1"), Coordinate { x: 1, y: 1 });
    assert_eq!(Coordinate::from("h3"), Coordinate { x: 8, y: 3 });
    assert_eq!(Coordinate::from("b7"), Coordinate { x: 2, y: 7 });
    assert_eq!(Coordinate::from("d5"), Coordinate { x: 4, y: 5 });
    assert_eq!(Coordinate::from("a8"), Coordinate { x: 1, y: 8 });
    assert_eq!(Coordinate::from("e4"), Coordinate { x: 5, y: 4 });
    assert_eq!(Coordinate::from("e5"), Coordinate { x: 5, y: 5 });
}

#[test]
fn to_coordinate_test() {
    assert_eq!(Coordinate::to(Coordinate { x: 1, y: 1 }), "a1");
    assert_eq!(Coordinate::to(Coordinate { x: 8, y: 3 }), "h3");
    assert_eq!(Coordinate::to(Coordinate { x: 2, y: 7 }), "b7");
    assert_eq!(Coordinate::to(Coordinate { x: 4, y: 5 }), "d5");
    assert_eq!(Coordinate::to(Coordinate { x: 1, y: 8 }), "a8");
    assert_eq!(Coordinate::to(Coordinate { x: 5, y: 4 }), "e4");
    assert_eq!(Coordinate::to(Coordinate { x: 5, y: 5 }), "e5");
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Coordinate {
    pub x: u8, // a - h (traditional coordinates)
    pub y: u8, // 1 - 8 (traditional coordinates)
}

pub const LOW_X: u8 = 1;
pub const HIGH_X: u8 = 8;
pub const LOW_Y: u8 = 1;
pub const HIGH_Y: u8 = 8;

impl Coordinate {
    // const LOW_X
    pub fn add(&self, x: i8, y: i8) -> Coordinate {
        Coordinate {
            x: ((self.x as i8) + x) as u8,
            y: ((self.y as i8) + y) as u8,
        }
    }

    pub fn to(c: Coordinate) -> String {
        let mut x = match c.x {
            1 => "a" ,
            2 => "b" ,
            3 => "c" ,
            4 => "d" ,
            5 => "e" ,
            6 => "f" ,
            7 => "g" ,
            8 => "h" ,
            _ => panic!("{} not valid coordinate"),
        };
        let mut str = x.to_string();
        str.push_str(c.y.to_string().as_str());
        str
    }

    pub fn from(str: &str) -> Coordinate {
        if str.len() < 2 {
            panic!("{} is not a valid coordinate", str);
        }
        let c: Vec<char> = str.chars().collect();
        let x = match c.get(0).unwrap() {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            _ => panic!("{} not valid coordinate", str),
        };
        let y = c.get(1).unwrap().to_string().parse::<u8>().unwrap();
        Coordinate { x, y }
    }
}

#[derive(Debug)]
pub struct Square {
    pub coordinate: Coordinate,
    pub piece: Option<Piece>,
    pub color: Color,
}

#[test]
fn test_in_check() {
    let board = fen_reader::read(fen_reader::BLACK_IN_CHECK);
    assert!(board.is_in_check(Color::Black));
    assert!(!board.is_in_check(Color::White));
    let board = fen_reader::read(fen_reader::WHITE_IN_CHECK);
    assert!(!board.is_in_check(Color::Black));
    assert!(board.is_in_check(Color::White));
}


#[derive(Debug)]
pub struct Board {
    pub white_to_move: bool,
    pub white_can_castle_king_side: bool,
    pub white_can_castle_queen_side: bool,
    pub black_can_castle_king_side: bool,
    pub black_can_castle_queen_side: bool,
    pub en_passant_target: Option<Coordinate>,
    pub half_move_clock: u8,
    pub full_move_number: u8,
    squares: Vec<Vec<Square>>,
}

impl Board {
    fn clone(&self) -> Board {
        let mut squares : Vec<Vec<Square>> = vec![];
        for row in self.squares.iter() {
            let mut new_row : Vec<Square> = vec![];
            for square in row.iter() {
                new_row.push(Square {
                    coordinate: square.coordinate.clone(),
                    piece: square.piece.clone(),
                    color: square.color.clone(),
                });
            }
            squares.push(new_row);
        }
        Board {
            white_to_move: self.white_to_move,
            white_can_castle_king_side: self.white_can_castle_king_side,
            white_can_castle_queen_side: self.white_can_castle_queen_side,
            black_can_castle_king_side: self.black_can_castle_king_side,
            black_can_castle_queen_side: self.black_can_castle_queen_side,
            en_passant_target: self.en_passant_target.clone(),
            half_move_clock: self.half_move_clock,
            full_move_number: self.full_move_number,
            squares
        }
    }
    // change return to piece list or something ?
    pub fn is_in_check(&self, color: Color) -> bool {
        println!("is {} in check ? ", color);
        println!("{}", color.opposite());
        let moves = gen_moves(self, color.opposite());
        let king = self.get_king(color).unwrap();
        println!("{:?}", king);
        let at = king.at().unwrap();
        print_board(self);
        for m in moves {
            if m.to == at {
                print_move(&m);
                return true;
            }
        }
        false
    }

    fn get_king(&self, color: Color) -> Option<Piece> {
        for row in self.squares.iter() {
            for square in row.iter() {
                if square.piece.is_none() {
                    continue;
                }
                let piece = square.piece.unwrap();
                if piece.piece_type == PieceType::King && piece.color == color {
                    return Some(piece.clone());
                }
            }
        }
        None
    }

    // doesn't check legality of moves
    pub fn make_move_mut(&mut self, m: &Move) {
        let mut p = self.get_piece_at(&m.from).unwrap();

        //@todo : pawn promotion
        if m.promoted_to.is_some() && m.piece.piece_type == PieceType::Pawn {
            p.piece_type = m.promoted_to.unwrap();
        }
        p.at = Some(m.to);
        let s = self.get_square_mut(&m.to);
        if s.piece.is_some() {
            let mut other = s.piece.unwrap();
            if other.color != p.color {
                other.at = None;
            } else {
            }
        }
        s.piece = Some(p);
        let s = self.get_square_mut(&m.from);
        s.piece = None;

        // castling
        if m.is_castling && m.rook_from.is_some() && m.rook_to.is_some() {
            self.make_move_mut(&Move::new(
                m.rook_from.unwrap(),
                m.rook_to.unwrap(),
                m.rook.unwrap(),
            ))
        }

        // update 50 move rule draw counter
        if m.piece.piece_type != PieceType::Pawn && self.get_piece_at(&m.to).is_none() {
            self.half_move_clock = self.half_move_clock + 1;
        }

        // update move counter
        if m.piece.color == Color::Black {
            self.full_move_number = self.full_move_number + 1;
        }
    }

    pub fn make_move(&self, m: &Move) -> Board {
        let mut board = self.clone();
        board.make_move_mut(m);
        board
    }

    pub fn place_piece(&mut self, mut piece: Piece, at: Coordinate) {
        piece.at = Some(at);
        self.get_square_mut(&at).piece = Some(piece);
    }

    pub fn has_piece(&self, at: &Coordinate) -> bool {
        self.squares[(at.y - 1) as usize][(at.x - 1) as usize]
            .piece
            .is_some()
    }

    fn is_on_board(c: Coordinate) -> bool {
        c.x >= LOW_X && c.x <= HIGH_X && c.y >= LOW_Y && c.y <= HIGH_Y
    }

    pub fn get_piece_at(&self, at: &Coordinate) -> Option<Piece> {
        let square = self.get_square(at);
        if square.piece.is_some() {
            return Some(square.piece.unwrap().clone());
        } else {
            return None;
        }
    }

    pub fn get_pieces(&self, color: Color) -> Vec<Piece> {
        let mut pieces = Vec::<Piece>::new();
        for row in self.squares.iter() {
            for square in row.iter() {
                if square.piece.is_none() {
                    continue;
                }
                let piece = square.piece.unwrap();
                if piece.color == color {
                    pieces.push(piece.clone());
                }
            }
        }
        return pieces;
    }

    fn get_square(&self, at: &Coordinate) -> &Square {
        self.squares
            .get((at.y - 1) as usize)
            .unwrap()
            .get((at.x - 1) as usize)
            .unwrap()
    }

    fn get_square_mut(&mut self, at: &Coordinate) -> &mut Square {
        self.squares
            .get_mut((at.y - 1) as usize)
            .unwrap()
            .get_mut((at.x - 1) as usize)
            .unwrap()
    }

    fn make_squares() -> Vec<Vec<Square>> {
        let mut vec: Vec<Vec<Square>> = vec![];

        for (_, y) in (1..9).enumerate() {
            let mut row: Vec<Square> = Vec::new();
            for (_, x) in (1..9).enumerate() {
                // odd numbered rows have black squares on even x's
                let color: Color;
                if y % 2 == 0 {
                    // even row , white is even, black is odd
                    color = if x % 2 == 0 {
                        Color::White
                    } else {
                        Color::Black
                    }
                } else {
                    // odd row , white is odd , black is even
                    color = if x % 2 == 0 {
                        Color::Black
                    } else {
                        Color::White
                    }
                }
                row.push(Square {
                    coordinate: Coordinate { y, x },
                    piece: None,
                    color,
                });
            }
            vec.push(row);
        }
        return vec;
    }

    //@todo : clean up the architecture here, should it pass in a format and matrix display ?
    pub fn get_squares(&self) -> Vec<&Square> {
        return self
            .squares
            .iter()
            .map(|vec| {
                return vec.iter().rev();
            })
            .flatten()
            .collect();
    }

    pub fn new() -> Board {
        Board {
            white_to_move: true,
            white_can_castle_king_side: true,
            white_can_castle_queen_side: true,
            black_can_castle_king_side: true,
            black_can_castle_queen_side: true,
            en_passant_target: None,
            half_move_clock: 0,
            full_move_number: 0,
            squares: Board::make_squares(),
        }
    }
}
