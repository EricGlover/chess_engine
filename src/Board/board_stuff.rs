use std::fmt;
use std::fmt::Formatter;


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
            1 => "a",
            2 => "b",
            3 => "c",
            4 => "d",
            5 => "e",
            6 => "f",
            7 => "g",
            8 => "h",
            _ => panic!("not valid coordinate"),
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

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
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

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let string = match self {
            PieceType::King => "King",
            PieceType::Queen => "Queen",
            PieceType::Bishop => "Bishop",
            PieceType::Knight => "Knight",
            PieceType::Rook => "Rook",
            PieceType::Pawn => "Pawn",
        };
        write!(f, "{}", string)
    }
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
        let s = if self == &Color::White {
            "white"
        } else {
            "black"
        };
        write!(f, "{}", s)
    }
}

