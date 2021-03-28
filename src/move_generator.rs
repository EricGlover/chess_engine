use crate::board::*;
use crate::fen_reader::make_initial_board;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Move {
    pub piece: Piece,
    pub from: Coordinate,
    pub to: Coordinate,
}

fn print_move_list(moves : Vec<Move>) {
    for m in moves.iter() {
        println!("{:?} moving from ({}, {}) to ({},{}) ", m.piece.piece_type, m.from.x, m.from.y, m.to.x, m.to.y);
    }
}




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

    let m1 = Move {
        from: Coordinate { x: 1, y: 1 },
        to: Coordinate { x: 1, y: 1 },
        piece: pawn,
    };
    let m2 = Move {
        from: Coordinate { x: 2, y: 1 },
        to: Coordinate { x: 1, y: 1 },
        piece: pawn,
    };
    let m3 = Move {
        from: Coordinate { x: 1, y: 1 },
        to: Coordinate { x: 1, y: 1 },
        piece: pawn_2,
    };

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
        Move {
            from: Coordinate { x: 1, y: 2 },
            to: Coordinate { x: 1, y: 3 },
            piece: pawn.clone(),
        },
        Move {
            from: Coordinate { x: 1, y: 2 },
            to: Coordinate { x: 1, y: 4 },
            piece: pawn.clone(),
        },
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

fn gen_king_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let from = piece.at().unwrap();
    let make_move = |x, y| Move {
        piece: piece.clone(),
        from,
        to: from.add(x, y),
    };
    let moves = vec![
        make_move(-1, -1),
        make_move(0, -1),
        make_move(1, -1),
        make_move(-1, 0),
        make_move(1, 0),
        make_move(-1, 1),
        make_move(0, 1),
        make_move(1, 1),
    ];
    moves.into_iter().filter(|m| is_on_board(m.to) && square_occupiable_by(board, &m.to, piece.color)).collect()
}
fn gen_queen_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let mut moves = gen_rook_moves(board, piece);
    moves.extend(gen_bishop_moves(board, piece).into_iter());
    moves
}
fn gen_bishop_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let from = piece.at().unwrap();
    let make_move = |to: Coordinate| Move {
        piece: piece.clone(),
        from,
        to: to.clone(),
    };
    // up right
    let mut to = from.clone();
    loop {
        to = to.add(1, 1);
        if !is_on_board(to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    // up left
    let mut to = from.clone();
    loop {
        to = to.add(-1, 1);
        if !is_on_board(to) || !square_occupiable_by(board, &to, piece.color){
            break;
        }
        moves.push(make_move(to))
    }
    // down left
    let mut to = from.clone();
    loop {
        to = to.add(-1, -1);
        if !is_on_board(to) || !square_occupiable_by(board, &to, piece.color){
            break;
        }
        moves.push(make_move(to))
    }
    // down right
    let mut to = from.clone();
    loop {
        to = to.add(1, -1);
        if !is_on_board(to) || !square_occupiable_by(board, &to, piece.color){
            break;
        }
        moves.push(make_move(to))
    }
    moves
}
fn gen_knight_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let from = piece.at().unwrap();
    let make_move = |x, y| Move {
        piece: piece.clone(),
        from,
        to: from.add(x, y),
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
    moves.into_iter().filter(|m| is_on_board(m.to) && square_occupiable_by(board, &m.to, piece.color)).collect()
}
fn gen_rook_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::new();
    let from = piece.at().unwrap();
    let make_move = |to: Coordinate| Move {
        piece: piece.clone(),
        from,
        to: to.clone(),
    };
    // left
    let mut to = from.clone();
    loop {
        to = to.add(-1, 0);
        if !is_on_board(to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    // right
    let mut to = from.clone();
    loop {
        to = to.add( 1, 0);
        if !is_on_board(to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    // up
    let mut to = from.clone();
    loop {
        to = to.add( 0, 1);
        if !is_on_board(to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    // down
    let mut to = from.clone();
    loop {
        to = to.add( 0, -1);
        if !is_on_board(to) || !square_occupiable_by(board, &to, piece.color) {
            break;
        }
        moves.push(make_move(to))
    }
    moves
}

fn gen_pawn_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    let from = piece.at().unwrap();
    let to: i8 = if piece.color == Color::White { 1 } else { -1 };
    let make_move = |x, y| Move {
        piece: piece.clone(),
        from,
        to: from.add(x, y),
    };
    let mut moves: Vec<Move> = vec![];
    let m = make_move(0, 1 * to);
    if is_on_board(m.to) && square_occupiable_by(board, &m.to, piece.color) {
        moves.push(m);
    }
    let m2 = make_move(0, 2 * to);
    if is_on_board(m2.to) && square_occupiable_by(board, &m2.to, piece.color) {
        moves.push(m);
    }
    moves
}

fn square_occupiable_by(board: &Board, at: &Coordinate, color: Color) -> bool {
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

fn is_on_board(c: Coordinate) -> bool {
    c.x >= LOW_X && c.x <= HIGH_X && c.y >= LOW_Y && c.y <= HIGH_Y
}

fn is_legal(move_: Move) -> bool {
    true
}
