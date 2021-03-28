use crate::board::*;
use crate::fen_reader::make_initial_board;

//@todo: make sure move doesn't put you in check
/**
is_in_check() -> bool ?

gen_moves_from_check() -> Vec<Move>
    - get own king
    - generate enemy moves
    - find moves that threaten king
    - how to deal with multiple checks ?
    - for single checks
        - find moves that interpose if piece_type rook, queen, bishop
        -

-- need to make sure that moves don't put you in check
- get list of square enemy can capture
- make sure king doesn't try to move there
- for castling make sure king doesn't move through there
**/

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Move {
    pub piece: Piece,
    pub from: Coordinate,
    pub to: Coordinate,
    pub promoted_to: Option<PieceType>, // pawn promotion
    pub is_castling: bool,
    pub rook: Option<Piece>,
    pub rook_from: Option<Coordinate>,
    pub rook_to: Option<Coordinate>,
}

impl Move {
    pub fn new(from: Coordinate, to: Coordinate, piece: Piece) -> Move {
        Move {
            piece,
            from,
            to,
            promoted_to: None,
            is_castling: false,
            rook: None,
            rook_from: None,
            rook_to: None,
        }
    }

    pub fn pawn_promotion(
        from: Coordinate,
        to: Coordinate,
        piece: Piece,
        promoted_type: PieceType,
    ) -> Move {
        Move {
            piece,
            from,
            to,
            promoted_to: Some(promoted_type),
            is_castling: false,
            rook: None,
            rook_from: None,
            rook_to: None,
        }
    }

    pub fn castle_king_side(color: Color) -> Move {
        let y: u8 = if color == Color::White { 1 } else { 8 };
        let from = Coordinate { x: 5, y };
        let rook_from = Coordinate { x: 8, y };
        Move {
            piece: Piece::new(color, PieceType::King, Some(from.clone())),
            from,
            to: Coordinate { x: 7, y },
            promoted_to: None,
            is_castling: true,
            rook: Some(Piece::new(color, PieceType::Rook, Some(rook_from.clone()))),
            rook_from: Some(rook_from),
            rook_to: Some(Coordinate { x: 6, y }),
        }
    }
    pub fn castle_queen_side(color: Color) -> Move {
        let y: u8 = if color == Color::White { 1 } else { 8 };
        let from = Coordinate { x: 5, y };
        let rook_from = Coordinate { x: 1, y };
        Move {
            piece: Piece::new(color, PieceType::King, Some(from.clone())),
            from,
            to: Coordinate { x: 3, y },
            promoted_to: None,
            is_castling: true,
            rook: Some(Piece::new(color, PieceType::Rook, Some(rook_from.clone()))),
            rook_from: Some(rook_from),
            rook_to: Some(Coordinate { x: 4, y }),
        }
    }
}

fn print_move_list(moves: Vec<Move>) {
    for m in moves.iter() {
        println!(
            "{:?} moving from ({}, {}) to ({},{}) ",
            m.piece.piece_type, m.from.x, m.from.y, m.to.x, m.to.y
        );
    }
}
// @todo : test

// @todo: castling
// @todo: pawn promotion

// legal moves
// any moves
// legal moves on the board

fn move_list_eq(m: &Vec<Move>, m2: &Vec<Move>) -> bool {
    if m.len() != m2.len() {
        return false;
    }
    for mov in m.iter() {
        let mut found = false;
        for mov_2 in m2.iter() {
            if mov == mov_2 {
                found = true;
                break;
            }
        }
        if !found {
            return false;
        }
    }
    return true;
}

#[test]
fn move_list_is_same() {
    let pawn = Piece::new(
        Color::White,
        PieceType::Pawn,
        Some(Coordinate { x: 1, y: 1 }),
    );
    let pawn_2 = Piece::new(
        Color::White,
        PieceType::Pawn,
        Some(Coordinate { x: 1, y: 1 }),
    );

    let m1 = Move::new(Coordinate { x: 1, y: 1 }, Coordinate { x: 1, y: 1 }, pawn);
    let m2 = Move::new(Coordinate { x: 2, y: 1 }, Coordinate { x: 1, y: 1 }, pawn);
    let m3 = Move::new(Coordinate { x: 1, y: 1 }, Coordinate { x: 1, y: 1 }, pawn_2);

    let ml: Vec<Move> = vec![m1.clone(), m2.clone()];
    let ml2: Vec<Move> = vec![m1.clone(), m2.clone()];
    let ml3: Vec<Move> = vec![m1.clone(), m3.clone()];
    assert!(move_list_eq(&ml, &ml2));
    assert!(!move_list_eq(&ml, &ml3));
}

#[test]
fn test_pawn_moves() {
    // how to determine if two vecs of moves are equal ?
    let board = make_initial_board();
    let pawn = board.get_piece_at(&Coordinate { x: 1, y: 2 }).unwrap();
    let moves = gen_pawn_moves(&board, &pawn);
    let correct_moves: Vec<Move> = vec![
        Move::new(
            Coordinate { x: 1, y: 2 },
            Coordinate { x: 1, y: 3 },
            pawn.clone(),
        ),
        Move::new(
            Coordinate { x: 1, y: 2 },
            Coordinate { x: 1, y: 4 },
            pawn.clone(),
        ),
    ];
    assert!(move_list_eq(&moves, &correct_moves));
}

// generates all moves to squares on the board
// could be illegal
pub fn gen_moves(board: &Board, color: Color) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let pieces = board.get_pieces(color);
    for piece in pieces.iter() {
        let m = gen_moves_for(board, piece);
        moves.extend(m.into_iter());
    }
    return moves;
}

fn gen_moves_for(board: &Board, piece: &Piece) -> Vec<Move> {
    let moves = match piece.piece_type {
        PieceType::King => gen_king_moves(board, piece),
        PieceType::Queen => gen_queen_moves(board, piece),
        PieceType::Bishop => gen_bishop_moves(board, piece),
        PieceType::Knight => gen_knight_moves(board, piece),
        PieceType::Rook => gen_rook_moves(board, piece),
        PieceType::Pawn => gen_pawn_moves(board, piece),
    };
    return moves;
}

// one square any direction
// castling
fn gen_king_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let from = piece.at().unwrap();
    let make_move = |x, y| Move::new(from, from.add(x, y), piece.clone());
    let mut moves: Vec<Move> = vec![
        make_move(-1, -1),
        make_move(0, -1),
        make_move(1, -1),
        make_move(-1, 0),
        make_move(1, 0),
        make_move(-1, 1),
        make_move(0, 1),
        make_move(1, 1),
    ]
    .into_iter()
    .filter(|m| is_on_board(&m.to) && square_occupiable_by(board, &m.to, piece.color))
    .collect();

    // castling
    // @todo : clean up
    if piece.color == Color::White && board.white_can_castle_queen_side {
        // 2,3,4
        let pass_through_spots = [
            Coordinate { x: 2, y: 1 },
            Coordinate { x: 3, y: 1 },
            Coordinate { x: 4, y: 1 },
        ];
        if pass_through_spots.iter().all(|c| !board.has_piece(&c)) {
            moves.push(Move::castle_queen_side(Color::White));
        }
    }
    if piece.color == Color::White && board.white_can_castle_king_side {
        // 7, 6
        let pass_through_spots = [Coordinate { x: 6, y: 1 }, Coordinate { x: 7, y: 1 }];
        if pass_through_spots.iter().all(|c| !board.has_piece(&c)) {
            moves.push(Move::castle_king_side(Color::White));
        }
    }
    if piece.color == Color::Black && board.black_can_castle_queen_side {
        //2,3,4
        let pass_through_spots = [
            Coordinate { x: 2, y: 8 },
            Coordinate { x: 3, y: 8 },
            Coordinate { x: 4, y: 8 },
        ];
        if pass_through_spots.iter().all(|c| !board.has_piece(&c)) {
            moves.push(Move::castle_queen_side(Color::Black));
        }
    }
    if piece.color == Color::Black && board.black_can_castle_king_side {
        // 7, 6
        let pass_through_spots = [Coordinate { x: 6, y: 8 }, Coordinate { x: 7, y: 8 }];
        if pass_through_spots.iter().all(|c| !board.has_piece(&c)) {
            moves.push(Move::castle_king_side(Color::Black));
        }
    }
    moves
}
fn gen_queen_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let mut moves = gen_rook_moves(board, piece);
    moves.extend(gen_bishop_moves(board, piece).into_iter());
    moves
}
fn gen_bishop_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let from = piece.at().unwrap();
    let make_move = |to: Coordinate| Move::new(from, to.clone(), piece.clone());
    // up right
    let mut to = from.clone();
    loop {
        to = to.add(1, 1);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    // up left
    let mut to = from.clone();
    loop {
        to = to.add(-1, 1);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    // down left
    let mut to = from.clone();
    loop {
        to = to.add(-1, -1);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    // down right
    let mut to = from.clone();
    loop {
        to = to.add(1, -1);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    moves
}
fn gen_knight_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let from = piece.at().unwrap();
    let make_move = |x, y| Move::new(from, from.add(x, y), piece.clone());
    let moves = vec![
        make_move(-2, 1),
        make_move(-1, 2),
        make_move(1, 2),
        make_move(2, 1),
        make_move(2, -1),
        make_move(1, -2),
        make_move(-1, -2),
        make_move(-2, -1),
    ];
    moves
        .into_iter()
        .filter(|m| is_on_board(&m.to) && square_occupiable_by(board, &m.to, piece.color))
        .collect()
}
fn gen_rook_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let from = piece.at().unwrap();
    let make_move = |to: Coordinate| Move::new(from, to.clone(), piece.clone());
    // left
    let mut to = from.clone();
    loop {
        to = to.add(-1, 0);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    // right
    let mut to = from.clone();
    loop {
        to = to.add(1, 0);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    // up
    let mut to = from.clone();
    loop {
        to = to.add(0, 1);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    // down
    let mut to = from.clone();
    loop {
        to = to.add(0, -1);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    moves
}

/**
one square move, two square move, capturing diagonally forward, pawn promotion, en passant
**/
fn gen_pawn_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let from = piece.at().unwrap();
    let direction: i8 = if piece.color == Color::White { 1 } else { -1 };
    let make_move = |x, y| Move::new(from, from.add(x, y), piece.clone());
    let mut moves: Vec<Move> = vec![];

    // pawn promotion, promotion is always a move by one square
    // white pawn must be at top, black pawn must be at bottom && the square needs to open or available to capture
    let to = from.add(0, 1 * direction);
    if (piece.color == Color::White && to.y == HIGH_Y)
        || (piece.color == Color::Black && to.y == LOW_Y)
    {
        if square_occupiable_by(board, &to, piece.color) {
            // why you can't promote to a king , idk but it's lame
            let promotion_pieces = [
                PieceType::Rook,
                PieceType::Queen,
                PieceType::Bishop,
                PieceType::Knight,
            ];
            for promotion_type in promotion_pieces.iter() {
                let m = Move::pawn_promotion(from, to, piece.clone(), promotion_type.clone());
                moves.push(m);
            }
        }
    } else if is_on_board(&to) && square_occupiable_by(board, &to, piece.color) {
        // normal pawn move
        moves.push(Move::new(from, to, piece.clone()));
    }

    // pawn move two squares
    if (piece.color == Color::White && from.y == 2) || (piece.color == Color::White && from.y == 7)
    {
        let m2 = make_move(0, 2 * direction);
        if is_on_board(&m2.to) && square_occupiable_by(board, &m2.to, piece.color) {
            moves.push(m2);
        }
    }

    // pawn captures , including en passant
    let front_left = from.add(1, 1 * direction);
    if has_enemy_piece(board, &front_left, piece.color)
        || board.en_passant_target.is_some() && board.en_passant_target.unwrap() == front_left
    {
        moves.push(Move::new(from, front_left, piece.clone()));
    }

    let front_right = from.add(1, 1 * direction);
    if has_enemy_piece(board, &front_right, piece.color)
        || board.en_passant_target.is_some() && board.en_passant_target.unwrap() == front_right
    {
        moves.push(Move::new(from, front_right, piece.clone()));
    }
    moves
}

fn square_occupiable_by(board: &Board, at: &Coordinate, color: Color) -> bool {
    if !is_on_board(at) {
        return false;
    }
    let piece = board.get_piece_at(at);
    if piece.is_none() {
        return true;
    } else {
        let piece = piece.unwrap();
        if piece.color == color {
            return false;
        } else {
            return true;
        }
    }
}

fn has_enemy_piece(board: &Board, at: &Coordinate, ownColor: Color) -> bool {
    let enemyColor = match ownColor {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    if !is_on_board(at) {
        return false;
    }
    let piece = board.get_piece_at(at);
    if piece.is_some() {
        let piece = piece.unwrap();
        if piece.color == enemyColor {
            return true;
        }
    }
    false
}

fn is_on_board(c: &Coordinate) -> bool {
    c.x >= LOW_X && c.x <= HIGH_X && c.y >= LOW_Y && c.y <= HIGH_Y
}

fn is_legal(move_: Move) -> bool {
    true
}