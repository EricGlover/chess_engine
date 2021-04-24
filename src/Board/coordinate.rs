use std::fmt;
use std::fmt::{Error, Formatter};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Coordinate {
    x: u8, // a - h (traditional coordinates)
    y: u8, // 1 - 8 (traditional coordinates)
}

pub const LOW_X: u8 = 1;
pub const HIGH_X: u8 = 8;
pub const LOW_Y: u8 = 1;
pub const HIGH_Y: u8 = 8;

impl Coordinate {
    pub fn diff(&self, other: &Coordinate) -> (i32, i32) {
        (
            self.x as i32 - other.x as i32,
            self.y as i32 - other.y as i32,
        )
    }

    pub fn x(&self) -> u8 {
        self.x
    }
    pub fn y(&self) -> u8 {
        self.y
    }
    pub fn is_valid_coordinate(&self) -> bool {
        if self.x < LOW_X || self.x > HIGH_X {
            return false;
        }
        if self.y < LOW_Y || self.y > HIGH_Y {
            return false;
        }
        return true;
    }
    pub fn new(x: u8, y: u8) -> Coordinate {
        Coordinate { x, y }
    }
    // const LOW_X
    pub fn add(&self, x: i8, y: i8) -> Coordinate {
        Coordinate::new(((self.x as i8) + x) as u8, ((self.y as i8) + y) as u8)
    }

    pub fn x_to(&self) -> &str {
        match self.x {
            1 => "a",
            2 => "b",
            3 => "c",
            4 => "d",
            5 => "e",
            6 => "f",
            7 => "g",
            8 => "h",
            _ => panic!("not valid coordinate"),
        }
    }

    pub fn y_to(&self) -> String {
        self.y.to_string()
    }

    pub fn to(c: Coordinate) -> String {
        let mut str = String::from(c.x_to());
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
        Coordinate::new(x, y)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_coordinate_test() {
        assert_eq!(Coordinate::from("a1"), Coordinate::new(1, 1));
        assert_eq!(Coordinate::from("h3"), Coordinate::new(8, 3));
        assert_eq!(Coordinate::from("b7"), Coordinate::new(2, 7));
        assert_eq!(Coordinate::from("d5"), Coordinate::new(4, 5));
        assert_eq!(Coordinate::from("a8"), Coordinate::new(1, 8));
        assert_eq!(Coordinate::from("e4"), Coordinate::new(5, 4));
        assert_eq!(Coordinate::from("e5"), Coordinate::new(5, 5));
    }

    #[test]
    fn to_coordinate_test() {
        assert_eq!(Coordinate::to(Coordinate::new(1, 1)), "a1");
        assert_eq!(Coordinate::to(Coordinate::new(8, 3)), "h3");
        assert_eq!(Coordinate::to(Coordinate::new(2, 7)), "b7");
        assert_eq!(Coordinate::to(Coordinate::new(4, 5)), "d5");
        assert_eq!(Coordinate::to(Coordinate::new(1, 8)), "a8");
        assert_eq!(Coordinate::to(Coordinate::new(5, 4)), "e4");
        assert_eq!(Coordinate::to(Coordinate::new(5, 5)), "e5");
    }
}
