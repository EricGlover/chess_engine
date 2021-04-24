use std::fmt::{Error, Formatter};
use std::fmt;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn from_char(c: char) -> Result<Color, Error> {
        match c.to_lowercase().to_string().as_str() {
            "w" => Ok(Color::White),
            "b" => Ok(Color::Black),
            _ => Err(Error),
        }
    }
    pub fn to_char(&self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }
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
