use crate::board::*;
use crate::fen_reader::make_initial_board;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Move {
    pub piece: Piece,
    pub from: Coordinate,
    pub to: Coordinate,
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
    vec![]
}
fn gen_queen_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    vec![]
}
fn gen_bishop_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    vec![]
}
fn gen_knight_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    vec![]
}
fn gen_rook_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    vec![]
}

fn gen_pawn_moves(board: &Board, piece: &Piece) -> Vec<Move> {
    // @todo : check if move is on the board
    let from = piece.at().unwrap();
    let to = if piece.color == Color::White { 1 } else { -1 };
    let m = Move {
        piece: piece.clone(),
        from,
        to: Coordinate {
            x: from.x,
            y: from.y + (1 * to),
        },
    };
    let m2 = Move {
        piece: piece.clone(),
        from,
        to: Coordinate {
            x: from.x,
            y: from.y + (2 * to),
        },
    };
    return vec![m, m2];
}

fn is_legal(move_: Move) -> bool {
    true
}
