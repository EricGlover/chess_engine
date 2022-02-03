use crate::board::Piece;
use crate::board::Coordinate;
use super::*;

#[derive(Eq, PartialEq, Debug)]
pub struct Pin<'a> {
    pub pinned_piece: &'a Piece,
    pub pinned_by: &'a Piece,
    pub pinned_to: &'a Piece,
    pub can_move_to: Vec<Coordinate>,
}


pub fn find_pinned_pieces(board: &dyn BoardTrait, defender_color: Color) -> Vec<Pin> {
    let attacker_color = defender_color.opposite();
    //@todo generate legal? moves

    // get defender king
    let mut king_pieces = board.get_pieces(defender_color, PieceType::King);
    if king_pieces.get(0).is_none() {
        return vec![];
    }
    let king = king_pieces.remove(0);

    // get pieces that can attack king (ignoring our own pieces)
    let attacking_pieces = find_attacking_pieces(board, attacker_color, &king.at().unwrap());

    // use piece.at and king.at to generate a range of Coordinates where pieces can interpose at
    let mut pins = vec![];
    for attacking_piece in attacking_pieces.iter() {
        // if piece is knight skip
        // if piece is one square away from the king then skip
        // assume King and Pawn can't attack the enemy king / from more than a square away
        let t = attacking_piece.piece_type;
        if t == PieceType::Queen || t == PieceType::Bishop || t == PieceType::Rook {
            let from = attacking_piece.at().unwrap();
            let to = king.at().unwrap();
            // if piece is Queen, Bishop, or Rook then
            // walk through the squares, from attacking piece to the king
            // if only one defender is in those squares then it's a pin
            let path = get_path_to(&from, &to);
            if path.is_none() {
                panic!("invalid path")
            }
            let mut path = path.unwrap();
            // remove the kings part of the path
            path.pop();
            // @todo: refactor this
            let mut defenders: Vec<&Piece> = vec![];

            for coordinate in path.iter() {
                let piece = board.get_piece_at(coordinate);
                if piece.is_none() {
                    continue;
                } else {
                    let piece = piece.unwrap();
                    if piece.color == attacker_color {
                        continue;
                    } else {
                        defenders.push(piece);
                    }
                }
            }
            if defenders.len() == 1 {
                let mut can_move_to = path.clone();
                // piece can move to where the king is, but can move to the attacker
                can_move_to.pop();
                let pinned_piece = defenders.pop().unwrap();
                let pin = Pin {
                    pinned_piece,
                    pinned_by: attacking_piece,
                    pinned_to: king,
                    can_move_to,
                };
                pins.push(pin);
            }
        }
    }
    pins
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::{ai, AiSearch};
    use crate::chess_notation::fen_reader;
    use crate::chess_notation::fen_reader::*;
    use crate::move_generator::chess_move::MoveType;
    use test::Bencher;
    #[test]
    fn test_find_pinned_pieces() {
        // pinned by black bishop, can capture or move 1
        let white_bishop_pinned =
            "rnbqk1nr/pppp1ppp/4p3/8/1b1P4/5N2/PPPBPPPP/RN1QKB1R b KQkq - 3 3";
        let board = make_board(white_bishop_pinned);
        // diagonal from pinning piece to one space before the king
        // it'd be neat to make diagonal from / to function, and file from / to, and rank from / to
        let mut pins = find_pinned_pieces(&board, Color::White);
        assert_eq!(pins.len(), 1, "There is one pin");
        let bishop = board.get_piece_at(&Coordinate::new(2, 4)).unwrap();
        let white_bishop = board.get_piece_at(&Coordinate::new(4, 2)).unwrap();
        let king = board.get_piece_at(&Coordinate::new(5, 1)).unwrap();
        let can_move_to = vec![Coordinate::new(2, 4), Coordinate::new(3, 3)];

        let found_pin = pins.pop().unwrap();

        let expected_pin = Pin {
            pinned_piece: white_bishop,
            pinned_by: bishop,
            pinned_to: king,
            can_move_to: can_move_to.clone(),
        };

        assert_eq!(found_pin.pinned_piece, white_bishop);
        assert_eq!(found_pin.pinned_by, bishop);
        assert_eq!(found_pin.pinned_to, king);
        assert_eq!(found_pin.can_move_to, can_move_to);

        assert_eq!(
            found_pin, expected_pin,
            "Black bishop pins white bishop to king"
        );

        // am I pinned if you're pinned ?
        let pinned_piece_attacks_kings =
            "rnb1k1nr/ppp2qpp/8/B1b1p2Q/3p4/1K2P2P/PPP2PP1/RN3B1R w kq - 0 17";
    }

}