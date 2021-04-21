use crate::board::*;
use crate::chess_notation::pgn::make_move_log;
use crate::fen_reader::make_board;
#[cfg(test)]
use crate::fen_reader::make_initial_board;
use crate::move_generator::path::{get_path_from, Direction};
use crate::move_generator::Move;

#[cfg(test)]
mod tests {
    use super::move_generation::*;
    use crate::board::Coordinate;
    use crate::board::*;
    use crate::fen_reader;
    use crate::move_generator::pseudo_legal_move_generator::*;
    use crate::move_generator::Move;

    #[test]
    fn test_king_moves() {
        // King moving from (5, 1) to (7, 1)
        // thread 'main' panicked at 'trying to remove a piece that isn't there.', src\board.rs:255:13
        // stack backtrace:
        // note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
        let board = fen_reader::make_board(
            "rnb1kbnr/pppp1p1p/4pp2/8/8/3BP3/PPPP1PPP/RNB1K1NR b KQkq - 3 4",
        );
        let king = board.get_pieces(Color::White, PieceType::King).remove(0);
        assert_eq!(king.piece_type, PieceType::King);
        let king_moves = gen_king_moves(&board, &king);
        let has_castling_moves = king_moves.iter().any(|m| m.is_castling);
        king_moves.iter().for_each(|m| {
            println!("{:?}", m.to);
        });
        assert!(
            !has_castling_moves,
            "there is no valid castling move for white"
        );
        println!("{:?}", king_moves);
        assert_eq!(king_moves.len(), 3, "only three legal moves");
    }

    #[test]
    fn test_gen_queen_moves() {
        let board =
            fen_reader::make_board("rnb3nr/pp1kpp1p/6pb/1Qpp4/qPPP4/N7/P3PPPP/R1B1KBNR b KQ - 2 7");
        let white_queen = board.get_piece_at(&Coordinate::new(2, 5)).unwrap();
        println!("{:?}", white_queen);
        let test_move = Move::new(
            Coordinate::new(2, 5),
            Coordinate::new(4, 7),
            white_queen.piece_type,
            Some(PieceType::King),
        );
        let moves = gen_queen_moves(&board, &white_queen);
        moves.iter().for_each(|m| println!("{}", m));
        let found = moves.iter().any(|m| m == &test_move);
        assert!(found, "queen can take king ");
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
                pawn.piece_type,
                None,
            ),
            Move::new(
                Coordinate::new(1, 2),
                Coordinate::new(1, 4),
                pawn.piece_type,
                None,
            ),
        ];
        assert!(move_list_eq(&moves, &correct_moves));
    }
    #[cfg(test)]
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
}

pub fn gen_moves_for(board: &dyn BoardTrait, piece: &Piece) -> Vec<Move> {
    let moves = match piece.piece_type {
        PieceType::King => move_generation::gen_king_moves(board, piece),
        PieceType::Queen => move_generation::gen_queen_moves(board, piece),
        PieceType::Bishop => move_generation::gen_bishop_moves(board, piece),
        PieceType::Knight => move_generation::gen_knight_moves(board, piece),
        PieceType::Rook => move_generation::gen_rook_moves(board, piece),
        PieceType::Pawn => move_generation::gen_pawn_moves(board, piece),
    };
    return moves;
}

pub fn gen_vectors_for(board: &dyn BoardTrait, piece: &Piece) -> Vec<Move> {
    let moves = match piece.piece_type {
        PieceType::King => move_generation::gen_king_moves(board, piece),
        PieceType::Queen => vector_generation::gen_queen_vector(board, piece),
        PieceType::Bishop => vector_generation::gen_bishop_vector(board, piece),
        PieceType::Knight => move_generation::gen_knight_moves(board, piece),
        PieceType::Rook => vector_generation::gen_rook_vector(board, piece),
        PieceType::Pawn => move_generation::gen_pawn_moves(board, piece),
    };
    return moves;
}

fn make_move_to(from: &Coordinate, to: &Coordinate, piece: &Piece, board: &dyn BoardTrait) -> Move {
    Move::new(
        from.clone(),
        to.clone(),
        piece.piece_type,
        board.get_piece_at(&to).map(|p| p.piece_type.clone()),
    )
}

fn make_move(x: i8, y: i8, from: &Coordinate, piece: &Piece, board: &dyn BoardTrait) -> Move {
    let to = from.add(x, y);
    Move::new(
        from.clone(),
        to,
        piece.piece_type,
        board.get_piece_at(&to).map(|p| p.piece_type.clone()),
    )
}

mod move_generation {
    use super::*;
    // @todo : test
    // make linear moves along some path
    pub fn make_moves(path: Vec<Coordinate>, board: &dyn BoardTrait, piece: &Piece) -> Vec<Move> {
        let from = piece.at().unwrap();
        let mut moves: Vec<Move> = Vec::new();

        // for each path , ignore the first square, walk, if blocked by friendly piece stop
        for (i, coordinate) in path.into_iter().enumerate() {
            if 0usize == i {
                continue;
            }
            // if square is off board || square has friendly piece then stop
            if !square_occupiable_by(board, &coordinate, piece.color) {
                break;
            }
            moves.push(make_move_to(from, &coordinate, piece, board));
            // if it wasn't empty then it had an enemy piece so stop
            if !square_is_empty(board, &coordinate) {
                break;
            }
        }
        moves
    }
    // one square any direction
    // castling
    pub fn gen_king_moves(board: &dyn BoardTrait, piece: &Piece) -> Vec<Move> {
        let from = piece.at().unwrap();
        let mut moves: Vec<Move> = vec![
            make_move(-1, -1, from, piece, board),
            make_move(0, -1, from, piece, board),
            make_move(1, -1, from, piece, board),
            make_move(-1, 0, from, piece, board),
            make_move(1, 0, from, piece, board),
            make_move(-1, 1, from, piece, board),
            make_move(0, 1, from, piece, board),
            make_move(1, 1, from, piece, board),
        ]
        .into_iter()
        .filter(|m| square_occupiable_by(board, &m.to, piece.color))
        .collect();

        // castling
        // @todo : clean up
        if piece.color == Color::White
            && board.can_castle_queen_side(piece.color)
            && piece.at().unwrap() == &Coordinate::new(5, 1)
        {
            let rook = board.get_piece_at(&Coordinate::new(1, 1));
            if rook.map_or(false, |p| p.piece_type == PieceType::Rook) {
                let rook = rook.unwrap();
                // 2,3,4
                let pass_through_spots = [
                    Coordinate::new(2, 1),
                    Coordinate::new(3, 1),
                    Coordinate::new(4, 1),
                ];
                if pass_through_spots.iter().all(|c| !board.has_piece(&c)) {
                    moves.push(Move::castle_queen_side(piece, rook));
                }
            }
        }
        if piece.color == Color::White
            && board.can_castle_king_side(piece.color)
            && piece.at().unwrap() == &Coordinate::new(5, 1)
        {
            let rook = board.get_piece_at(&Coordinate::new(8, 1));
            if rook.map_or(false, |p| p.piece_type == PieceType::Rook) {
                let rook = rook.unwrap();
                // 7, 6
                let pass_through_spots = [Coordinate::new(6, 1), Coordinate::new(7, 1)];
                if pass_through_spots.iter().all(|c| !board.has_piece(&c)) {
                    moves.push(Move::castle_king_side(piece, rook));
                }
            }
        }
        if piece.color == Color::Black
            && board.can_castle_queen_side(piece.color)
            && piece.at().unwrap() == &Coordinate::new(5, 8)
        {
            let rook = board.get_piece_at(&Coordinate::new(1, 8));
            if rook.map_or(false, |p| p.piece_type == PieceType::Rook) {
                let rook = rook.unwrap();
                //2,3,4
                let pass_through_spots = [
                    Coordinate::new(2, 8),
                    Coordinate::new(3, 8),
                    Coordinate::new(4, 8),
                ];
                if pass_through_spots.iter().all(|c| !board.has_piece(&c)) {
                    moves.push(Move::castle_queen_side(piece, rook));
                }
            }
        }
        if piece.color == Color::Black
            && board.can_castle_king_side(piece.color)
            && piece.at().unwrap() == &Coordinate::new(5, 8)
        {
            let rook = board.get_piece_at(&Coordinate::new(8, 8));
            if rook.is_some() && rook.map_or(false, |p| p.piece_type == PieceType::Rook) {
                let rook = rook.unwrap();
                // 7, 6
                let pass_through_spots = [Coordinate::new(6, 8), Coordinate::new(7, 8)];
                if pass_through_spots.iter().all(|c| !board.has_piece(&c)) {
                    moves.push(Move::castle_king_side(piece, rook));
                }
            }
        }
        moves
    }

    pub fn gen_queen_moves(board: &dyn BoardTrait, piece: &Piece) -> Vec<Move> {
        let mut moves = gen_rook_moves(board, piece);
        moves.extend(gen_bishop_moves(board, piece).into_iter());
        moves
    }
    pub fn gen_bishop_moves(board: &dyn BoardTrait, piece: &Piece) -> Vec<Move> {
        let from = piece.at().unwrap();
        let up_left = get_path_from(&from, Direction::UpLeft);
        let up_right = get_path_from(&from, Direction::UpRight);
        let down_left = get_path_from(&from, Direction::DownLeft);
        let down_right = get_path_from(&from, Direction::DownRight);
        vec![
            make_moves(up_left, board, piece),
            make_moves(up_right, board, piece),
            make_moves(down_left, board, piece),
            make_moves(down_right, board, piece),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
    pub fn gen_knight_moves(board: &dyn BoardTrait, piece: &Piece) -> Vec<Move> {
        let from = piece.at().unwrap();
        let moves = vec![
            make_move(-2, 1, from, piece, board),
            make_move(-1, 2, from, piece, board),
            make_move(1, 2, from, piece, board),
            make_move(2, 1, from, piece, board),
            make_move(2, -1, from, piece, board),
            make_move(1, -2, from, piece, board),
            make_move(-1, -2, from, piece, board),
            make_move(-2, -1, from, piece, board),
        ];
        moves
            .into_iter()
            .filter(|m| is_on_board(&m.to) && square_occupiable_by(board, &m.to, piece.color))
            .collect()
    }
    pub fn gen_rook_moves(board: &dyn BoardTrait, piece: &Piece) -> Vec<Move> {
        let from = piece.at().unwrap();
        let left = get_path_from(from, Direction::Left);
        let right = get_path_from(from, Direction::Right);
        let up = get_path_from(from, Direction::Up);
        let down = get_path_from(from, Direction::Down);
        vec![
            make_moves(left, board, piece),
            make_moves(right, board, piece),
            make_moves(up, board, piece),
            make_moves(down, board, piece),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
    /**
    one square move, two square move, capturing diagonally forward, pawn promotion, en passant
    **/
    pub fn gen_pawn_moves(board: &dyn BoardTrait, piece: &Piece) -> Vec<Move> {
        let from = piece.at().unwrap();
        let direction: i8 = if piece.color == Color::White { 1 } else { -1 };
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
                        from.clone(),
                        to,
                        piece.piece_type,
                        promotion_type.clone(),
                        board.get_piece_at(&to).map(|p| p.piece_type.clone()),
                    );
                    moves.push(m);
                }
            }
        } else if is_on_board(&to) && square_is_empty(board, &to) {
            // normal pawn move
            moves.push(Move::new(
                from.clone(),
                to,
                piece.piece_type,
                board.get_piece_at(&to).map(|p| p.piece_type.clone()),
            ));
        }

        // pawn move two squares (both squares must be empty)
        if (piece.color == Color::White && from.y() == 2)
            || (piece.color == Color::Black && from.y() == 7)
        {
            let m1 = make_move(0, 1 * direction, from, piece, board);
            let m2 = make_move(0, 2 * direction, from, piece, board);
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
        let front_right = from.add(1, 1 * direction);
        fn make_pawn_capture(
            board: &dyn BoardTrait,
            from: &Coordinate,
            at: &Coordinate,
            pawn_color: Color,
            piece: &Piece,
        ) -> Option<Move> {
            if has_enemy_piece(board, at, pawn_color)
                || board.en_passant_target().is_some() && board.en_passant_target().unwrap() == *at
            {
                return Some(Move::new(
                    from.clone(),
                    at.clone(),
                    piece.piece_type,
                    board.get_piece_at(at).map(|p| p.piece_type.clone()),
                ));
            }
            None
        }
        let capture = make_pawn_capture(board, &from, &front_left, piece.color, piece);
        if capture.is_some() {
            moves.push(capture.unwrap());
        }
        let capture = make_pawn_capture(board, &from, &front_right, piece.color, piece);
        if capture.is_some() {
            moves.push(capture.unwrap());
        }
        moves
    }
}

mod vector_generation {
    use super::*;
    pub fn make_vector_moves(
        path: Vec<Coordinate>,
        board: &dyn BoardTrait,
        piece: &Piece,
    ) -> Vec<Move> {
        let from = piece.at().unwrap();
        let mut moves: Vec<Move> = Vec::new();

        // for each path , ignore the first square, walk, if blocked by friendly piece stop
        for (i, coordinate) in path.into_iter().enumerate() {
            if 0usize == i {
                continue;
            }
            if !square_occupiable_by(board, &coordinate, piece.color) {
                break;
            }
            moves.push(make_move_to(from, &coordinate, piece, board))
        }
        moves
    }
    pub fn gen_queen_vector(board: &dyn BoardTrait, piece: &Piece) -> Vec<Move> {
        vec![
            gen_bishop_vector(board, piece),
            gen_rook_vector(board, piece),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
    pub fn gen_bishop_vector(board: &dyn BoardTrait, piece: &Piece) -> Vec<Move> {
        let from = piece.at().unwrap();
        let up_left = get_path_from(&from, Direction::UpLeft);
        let up_right = get_path_from(&from, Direction::UpRight);
        let down_left = get_path_from(&from, Direction::DownLeft);
        let down_right = get_path_from(&from, Direction::DownRight);
        vec![
            make_vector_moves(up_left, board, piece),
            make_vector_moves(up_right, board, piece),
            make_vector_moves(down_left, board, piece),
            make_vector_moves(down_right, board, piece),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
    pub fn gen_rook_vector(board: &dyn BoardTrait, piece: &Piece) -> Vec<Move> {
        let from = piece.at().unwrap();
        let left = get_path_from(from, Direction::Left);
        let right = get_path_from(from, Direction::Right);
        let up = get_path_from(from, Direction::Up);
        let down = get_path_from(from, Direction::Down);
        vec![
            make_vector_moves(left, board, piece),
            make_vector_moves(right, board, piece),
            make_vector_moves(up, board, piece),
            make_vector_moves(down, board, piece),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

fn square_is_empty(board: &dyn BoardTrait, at: &Coordinate) -> bool {
    board.get_piece_at(at).is_none()
}

// if square is off board || square has friendly price => false
fn square_occupiable_by(board: &dyn BoardTrait, at: &Coordinate, color: Color) -> bool {
    if !is_on_board(at) {
        return false;
    }
    board.get_piece_at(at).map_or(true, |&p| p.color != color)
}

fn has_enemy_piece(board: &dyn BoardTrait, at: &Coordinate, own_color: Color) -> bool {
    if !is_on_board(at) {
        return false;
    }
    board
        .get_piece_at(at)
        .map_or(false, |piece| piece.color == own_color.opposite())
}

fn is_on_board(c: &Coordinate) -> bool {
    c.x() >= LOW_X && c.x() <= HIGH_X && c.y() >= LOW_Y && c.y() <= HIGH_Y
}
