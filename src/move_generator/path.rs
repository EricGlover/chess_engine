use crate::board::Coordinate;
use crate::bit_board::{A_FILE, BitBoard, H_FILE, ROW_1, ROW_8};
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn get_x_component(&self) -> Option<Direction> {
        match self.x() {
            -1 => Some(Direction::Left),
            0 => None,
            1 => Some(Direction::Right),
            _ => None,
        }
    }
    pub fn get_y_component(&self) -> Option<Direction> {
        match self.x() {
            -1 => Some(Direction::Down),
            0 => None,
            1 => Some(Direction::Up),
            _ => None,
        }
    }

    pub fn add(x: Option<Direction>, y: Option<Direction>) -> Option<Direction> {
        if x.is_none() && y.is_none() {
            return None;
        } else if x.is_none() {
            return y;
        } else if y.is_none() {
            return x;
        } else {
            return match x.unwrap() {
                Direction::Left => match y.unwrap() {
                    Direction::Up => Some(Direction::UpLeft),
                    Direction::Down => Some(Direction::DownLeft),
                    _ => None,
                },
                Direction::Right => match y.unwrap() {
                    Direction::Up => Some(Self::UpRight),
                    Direction::Down => Some(Self::DownRight),
                    _ => None,
                },
                _ => None,
            };
        }
    }
    pub fn make(x_diff: i8, y_diff: i8) -> Option<Direction> {
        let mut x_component = None;
        if x_diff < 0 {
            x_component = Some(Direction::Left);
        } else if x_diff > 0 {
            x_component = Some(Direction::Right);
        }

        let mut y_component = None;
        if y_diff < 0 {
            y_component = Some(Direction::Down);
        } else if y_diff > 0 {
            y_component = Some(Direction::Up);
        }

        return Direction::add(x_component, y_component);
    }
    pub fn x(&self) -> i8 {
        match self {
            Direction::Up => 0,
            Direction::Down => 0,
            Direction::Right => 1,
            Direction::Left => -1,
            Direction::UpLeft => -1,
            Direction::UpRight => 1,
            Direction::DownLeft => -1,
            Direction::DownRight => 1,
        }
    }
    pub fn y(&self) -> i8 {
        match self {
            Direction::Up => 1,
            Direction::Down => -1,
            Direction::Right => 0,
            Direction::Left => 0,
            Direction::UpLeft => 1,
            Direction::UpRight => 1,
            Direction::DownLeft => -1,
            Direction::DownRight => -1,
        }
    }
}

pub fn make_path_bit_board(
    from: &Coordinate,
    to: &Coordinate,
    include_from: bool,
    include_to: bool,
) -> u64 {
    if !is_straight(&from, &to) {
        return 0;
    }
    let (x_diff, y_diff) = to.diff(&from);
    let direction_opt = Direction::make(x_diff as i8, y_diff as i8);
    if let Some(direction) = direction_opt {
        let x_component = direction.get_x_component();
        let y_component = direction.get_y_component();
        let x_boundary = match x_component {
            Some(Direction::Right) => Some(H_FILE),
            Some(Direction::Left) => Some(A_FILE),
            None => None,
            _ => None,
        };
        let y_boundary = match y_component {
            Some(Direction::Up) => Some(ROW_8),
            Some(Direction::Down) => Some(ROW_1),
            None => None,
            _ => None,
        };
        let start_bit = BitBoard::coordinate_to_bit(*from);
        let end_bit = BitBoard::coordinate_to_bit(*to);
        let mut path:u64 = 0;
        let mut current_bit = start_bit;
        while (current_bit > 0) {
            // shift bit 
            current_bit = BitBoard::shift_bit_board(&direction, &current_bit);
            // add bit to path
            path = current_bit | path;
            // check if at end
            if current_bit == end_bit {
                break;
            }
            // check bounds 
            if x_boundary.is_some() && BitBoard::bit_on_bit_board(current_bit, x_boundary.unwrap()) {
                break;
            } else if y_boundary.is_some() && BitBoard::bit_on_bit_board(current_bit, y_boundary.unwrap()) {
                break;
            }
        }
        if (include_from) {
            path |= start_bit;
        }
        if (!include_to) {
            path &= !end_bit;
        }
        return path;
    } else {
        return 0;
    }
    
    // if going down, down right or down left
    0
}

// excludes from and to in the path
pub fn get_path_between(from: &Coordinate, to: &Coordinate) -> Option<Vec<Coordinate>> {
    make_path(from, to, false, false)
}

// gets a straight path
// includes from and to in the path
pub fn get_path_to(from: &Coordinate, to: &Coordinate) -> Option<Vec<Coordinate>> {
    make_path(from, to, true, true)
}

// includes from in the path
pub fn get_path_from(from: &Coordinate, direction: Direction) -> Vec<Coordinate> {
    let delta_x = direction.x();
    let delta_y = direction.y();
    get_path(from, delta_x, delta_y)
}

// includes from in the path
fn get_path(from: &Coordinate, delta_x: i8, delta_y: i8) -> Vec<Coordinate> {
    let mut path: Vec<Coordinate> = vec![];
    let mut current = from.clone();

    while current.is_valid_coordinate() {
        path.push(current.clone());
        current = current.add(delta_x, delta_y);
    }
    path
}

// if they're on the same rank or file then there's a valid straight path
// or if |from.x - to.x| == |from.y - to.y|
fn is_straight(from: &Coordinate, to: &Coordinate) -> bool {
    let (x_diff, y_diff) = to.diff(&from);
    // horizontal || vertical || a straight diagonal
    (from.x() == to.x() || from.y() == to.y()) || (x_diff.abs() == y_diff.abs())
}

fn make_path(
    from: &Coordinate,
    to: &Coordinate,
    include_from: bool,
    include_to: bool,
) -> Option<Vec<Coordinate>> {
    if !is_straight(&from, &to) {
        return None;
    }
    let (x_diff, y_diff) = to.diff(&from);
    let delta_x = if x_diff > 0 {
        1
    } else if x_diff < 0 {
        -1
    } else {
        0
    };
    let delta_y = if y_diff > 0 {
        1
    } else if y_diff < 0 {
        -1
    } else {
        0
    };
    let mut path: Vec<Coordinate> = vec![];
    let mut current = from.clone();
    let mut is_first = true;
    while current.is_valid_coordinate() && &current != to {
        if is_first && include_from {
            path.push(current.clone());
        } else if !is_first {
            path.push(current.clone());
        }
        is_first = false;
        current = current.add(delta_x, delta_y);
    }
    if include_to {
        path.push(current);
    }
    Some(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_path_between() {
        let start = Coordinate::new(5, 1);
        let end = Coordinate::new(8, 1);
        let expected = Coordinate::new_vec(vec![(6, 1), (7, 1)]);
        let path = get_path_between(&start, &end);
        assert_eq!(
            path.unwrap(),
            expected,
            "finds path between start and end squares, excluding start and end"
        );
    }

    #[test]
    fn test_get_path_to() {
        let start = Coordinate::new(5, 1);
        let end = Coordinate::new(8, 1);
        let expected = Coordinate::new_vec(vec![(5, 1), (6, 1), (7, 1), (8, 1)]);
        let path = get_path_to(&start, &end);
        assert_eq!(
            path.unwrap(),
            expected,
            "finds inclusive path between start and end squares"
        );
    }

    #[test]
    fn test_get_path() {
        // diagonal
        let a = Coordinate::new(1, 1);
        let b = Coordinate::new(8, 8);
        let path = get_path_to(&a, &b);
        assert!(path.is_some(), "There is a path");
        let path = path.unwrap();
        assert_eq!(path.len(), 8);
        println!("{:?}", path);
        let expected: Vec<Coordinate> = vec![
            Coordinate::new(1, 1),
            Coordinate::new(2, 2),
            Coordinate::new(3, 3),
            Coordinate::new(4, 4),
            Coordinate::new(5, 5),
            Coordinate::new(6, 6),
            Coordinate::new(7, 7),
            Coordinate::new(8, 8),
        ];
        assert_eq!(path, expected);

        // right
        let a = Coordinate::new(1, 1);
        let b = Coordinate::new(2, 1);
        let path = get_path_to(&a, &b);
        assert!(path.is_some(), "There is a path");
        let path = path.unwrap();
        println!("{:?}", path);
        assert_eq!(path.len(), 2);
        let expected: Vec<Coordinate> = vec![a.clone(), b.clone()];
        assert_eq!(path, expected);

        // up
        let a = Coordinate::new(1, 1);
        let b = Coordinate::new(1, 2);
        let path = get_path_to(&a, &b);
        assert!(path.is_some(), "There is a path");
        let path = path.unwrap();
        assert_eq!(path.len(), 2);
        let expected: Vec<Coordinate> = vec![a.clone(), b.clone()];
        assert_eq!(path, expected);

        // test no path
        let a = Coordinate::new(1, 1);
        let b = Coordinate::new(2, 3);
        let path = get_path_to(&a, &b);
        assert!(path.is_none(), "There is no path");
    }

    #[test]
    fn test_make_path_bit_board() {
        let a = Coordinate::new(1, 1);
        let b = Coordinate::new(8, 8);
        let path = make_path_bit_board(&a, &b, true, true);
        BitBoard::print_bitboard(path);
        assert_eq!(u64::count_ones(path), 8);
        let path = make_path_bit_board(&a, &b, false, false);
        BitBoard::print_bitboard(path);
        assert_eq!(u64::count_ones(path), 6);

        // right
        let a = Coordinate::new(1, 1);
        let b = Coordinate::new(2, 1);
        let path = make_path_bit_board(&a, &b, true, true);
        BitBoard::print_bitboard(path);
        assert_eq!(u64::count_ones(path), 2);

        // up
        let a = Coordinate::new(1, 1);
        let b = Coordinate::new(1, 2);
        let path = make_path_bit_board(&a, &b, true, true);
        BitBoard::print_bitboard(path);
        assert_eq!(u64::count_ones(path), 2);

        // test no path
        let a = Coordinate::new(1, 1);
        let b = Coordinate::new(2, 3);
        let path = make_path_bit_board(&a, &b, true, true);
        BitBoard::print_bitboard(path);
        assert_eq!(u64::count_ones(path), 0);
    }
}
