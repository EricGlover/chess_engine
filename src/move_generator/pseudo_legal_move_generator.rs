use crate::board::*;
use crate::fen_reader::make_board;
#[cfg(test)]
use crate::fen_reader::make_initial_board;
use crate::move_generator::Move;

// @todo: split out gen_moves and gen_moves_illegal or something
// difference between legal and pseudo-legal moves ?
pub fn gen_moves_for(board: &Board, piece: &Piece) -> Vec<Move> {
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
    let make_move = |x, y| {
        let to = from.add(x, y);
        Move::new(from, to, piece.clone(), board.has_piece(&to))
    };
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
    if piece.color == Color::White && board.can_castle_queen_side(piece.color) {
        // 2,3,4
        let pass_through_spots = [
            Coordinate::new(2, 1),
            Coordinate::new(3, 1),
            Coordinate::new(4, 1),
        ];
        if pass_through_spots.iter().all(|c| !board.has_piece(&c)) {
            moves.push(Move::castle_queen_side(Color::White));
        }
    }
    if piece.color == Color::White && board.can_castle_king_side(piece.color) {
        // 7, 6
        let pass_through_spots = [Coordinate::new(6, 1), Coordinate::new(7, 1)];
        if pass_through_spots.iter().all(|c| !board.has_piece(&c)) {
            moves.push(Move::castle_king_side(Color::White));
        }
    }
    if piece.color == Color::Black && board.can_castle_queen_side(piece.color) {
        //2,3,4
        let pass_through_spots = [
            Coordinate::new(2, 8),
            Coordinate::new(3, 8),
            Coordinate::new(4, 8),
        ];
        if pass_through_spots.iter().all(|c| !board.has_piece(&c)) {
            moves.push(Move::castle_queen_side(Color::Black));
        }
    }
    if piece.color == Color::Black && board.can_castle_king_side(piece.color) {
        // 7, 6
        let pass_through_spots = [Coordinate::new(6, 8), Coordinate::new(7, 8)];
        if pass_through_spots.iter().all(|c| !board.has_piece(&c)) {
            moves.push(Move::castle_king_side(Color::Black));
        }
    }
    moves
    // @todo: fix the infinite loop
    // let filtered_moves: Vec<Move> = moves.into_iter().filter(|m| {
    //     let new_board = board.make_move(&m);
    //     new_board.is_in_check(m.piece.color)
    // }).collect();
    // filtered_moves
}

fn gen_queen_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let mut moves = gen_rook_moves(board, piece);
    moves.extend(gen_bishop_moves(board, piece).into_iter());
    moves
}
fn gen_bishop_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let from = piece.at().unwrap();
    let make_move =
        |to: Coordinate| Move::new(from, to.clone(), piece.clone(), board.has_piece(&to));
    // up right
    let mut to = from.clone();
    loop {
        to = to.add(1, 1);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to));
        if !square_is_empty(board, &to) {
            break;
        }
    }
    // up left
    let mut to = from.clone();
    loop {
        to = to.add(-1, 1);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to));
        if !square_is_empty(board, &to) {
            break;
        }
    }
    // down left
    let mut to = from.clone();
    loop {
        to = to.add(-1, -1);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to));
        if !square_is_empty(board, &to) {
            break;
        }
    }
    // down right
    let mut to = from.clone();
    loop {
        to = to.add(1, -1);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to));
        if !square_is_empty(board, &to) {
            break;
        }
    }
    moves
}
fn gen_knight_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let from = piece.at().unwrap();
    let make_move = |x, y| {
        let to = from.add(x, y);
        Move::new(from, to, piece.clone(), board.has_piece(&to))
    };
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
    let make_move =
        |to: Coordinate| Move::new(from, to.clone(), piece.clone(), board.has_piece(&to));
    // left
    let mut to = from.clone();
    loop {
        to = to.add(-1, 0);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to));
        if !square_is_empty(board, &to) {
            break;
        }
    }
    // right
    let mut to = from.clone();
    loop {
        to = to.add(1, 0);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to));
        if !square_is_empty(board, &to) {
            break;
        }
    }
    // up
    let mut to = from.clone();
    loop {
        to = to.add(0, 1);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to));
        if !square_is_empty(board, &to) {
            break;
        }
    }
    // down
    let mut to = from.clone();
    loop {
        to = to.add(0, -1);
        if !is_on_board(&to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to));
        if !square_is_empty(board, &to) {
            break;
        }
    }
    moves
}

/**
one square move, two square move, capturing diagonally forward, pawn promotion, en passant
**/
fn gen_pawn_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let from = piece.at().unwrap();
    let direction: i8 = if piece.color == Color::White { 1 } else { -1 };
    let make_move = |x, y| {
        let to = from.add(x, y);
        Move::new(from, to, piece.clone(), board.has_piece(&to))
    };
    let mut moves: Vec<Move> = vec![];

    // pawn promotion, promotion is always a move by one square
    // white pawn must be at top, black pawn must be at bottom && the square needs to open or available to capture
    let to = from.add(0, 1 * direction);
    if (piece.color == Color::White && to.y() == HIGH_Y)
        || (piece.color == Color::Black && to.y() == LOW_Y)
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
                let m = Move::pawn_promotion(
                    from,
                    to,
                    piece.clone(),
                    promotion_type.clone(),
                    board.has_piece(&to),
                );
                moves.push(m);
            }
        }
    } else if is_on_board(&to) && square_occupiable_by(board, &to, piece.color) {
        // normal pawn move
        moves.push(Move::new(from, to, piece.clone(), board.has_piece(&to)));
    }

    // pawn move two squares (both squares must be empty)
    if (piece.color == Color::White && from.y() == 2)
        || (piece.color == Color::Black && from.y() == 7)
    {
        let m1 = make_move(0, 1 * direction);
        let m2 = make_move(0, 2 * direction);
        if is_on_board(&m2.to)
            && square_is_empty(board, &m2.to)
            && is_on_board(&m1.to)
            && square_is_empty(board, &m1.to)
        {
            moves.push(m2);
        }
    }

    // pawn captures , including en passant
    let front_left = from.add(-1, 1 * direction);
    if has_enemy_piece(board, &front_left, piece.color)
        || board.en_passant_target.is_some() && board.en_passant_target.unwrap() == front_left
    {
        moves.push(Move::new(from, front_left, piece.clone(), true));
    }

    let front_right = from.add(1, 1 * direction);
    if has_enemy_piece(board, &front_right, piece.color)
        || board.en_passant_target.is_some() && board.en_passant_target.unwrap() == front_right
    {
        moves.push(Move::new(from, front_right, piece.clone(), true));
    }
    moves
}

fn square_is_empty(board: &Board, at: &Coordinate) -> bool {
    board.get_piece_at(at).is_none()
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

fn has_enemy_piece(board: &Board, at: &Coordinate, own_color: Color) -> bool {
    let enemyColor = match own_color {
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
    c.x() >= LOW_X && c.x() <= HIGH_X && c.y() >= LOW_Y && c.y() <= HIGH_Y
}

fn is_legal(move_: Move) -> bool {
    true
}

#[test]
fn test_pawn_moves() {
    let board = make_initial_board();
    let pawn = board.get_piece_at(&Coordinate::new(1, 2)).unwrap();
    let moves = gen_pawn_moves(&board, &pawn);
    let correct_moves: Vec<Move> = vec![
        Move::new(
            Coordinate::new(1, 2),
            Coordinate::new(1, 3),
            pawn.clone(),
            false,
        ),
        Move::new(
            Coordinate::new(1, 2),
            Coordinate::new(1, 4),
            pawn.clone(),
            false,
        ),
    ];
    assert!(move_list_eq(&moves, &correct_moves));
}
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
