use crate::board::Coordinate;

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
    pub fn x(&self) -> i8 {
        match self {
            Direction::Up => 0,
            Direction::Down =>  0,
            Direction::Right => 1,
            Direction::Left =>  -1,
            Direction::UpLeft =>  -1,
            Direction::UpRight => 1,
            Direction::DownLeft =>  -1,
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

pub fn get_path_from(from: &Coordinate, direction: Direction) -> Vec<Coordinate> {
    let delta_x = direction.x();
    let delta_y  = direction.y();
    get_path(from, delta_x, delta_y)
}

// gets a straight path
pub fn get_path_to(from: &Coordinate, to: &Coordinate) -> Option<Vec<Coordinate>> {
    let (x_diff, y_diff) = to.diff(&from);

    // if they're on the same rank or file then there's a valid straight path
    // or if |from.x - to.x| == |from.y - to.y|
    fn is_straight(from: &Coordinate, to: &Coordinate) -> bool {
        let (x_diff, y_diff) = to.diff(&from);
        // horizontal || vertical || a straight diagonal
        (from.x() == to.x() || from.y() == to.y()) || (x_diff.abs() == y_diff.abs())
    }
    if !is_straight(&from, &to) {
        return None;
    }
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
    while current.is_valid_coordinate() && &current != to{
        path.push(current.clone());
        current = current.add(delta_x, delta_y);
    }
    path.push(current);
    Some(path)
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