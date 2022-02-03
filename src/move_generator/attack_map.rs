use crate::move_generator::{Move, Pin, gen_pseudo_legal_moves};
use crate::board::{Coordinate, Square, Piece, BoardTrait, Color};
use std::collections::HashMap;

// this should replace the find_attack_pieces and find_pinned_pieces
// so it needs to say what is attacking this square
// && pieces defending attacks to this square
// should be able to find pieces pinned to this square
// as in for square x what pieces are defending it that if moved, square x
// would be under attack

// need something that takes coordinates and gives back
#[derive(Debug)]
pub struct AttackMap<'a> {
    attack_map_info: HashMap<(u8, u8), AttackMapInfo<'a>>
}

impl<'a> AttackMap<'a> {
    fn initialize_map(board: &'a dyn BoardTrait) -> HashMap<(u8, u8), AttackMapInfo<'a>> {
        let mut attack_map_info : HashMap<(u8, u8), AttackMapInfo<'a>> = HashMap::new();
        board.get_squares_iter().for_each(|square| {
            let coordinate = square.coordinate();
            attack_map_info.insert(
                (coordinate.x(), coordinate.y()),
                AttackMapInfo {
                    square,
                    attacking_white_pieces: Vec::new(),
                    attacking_black_pieces: Vec::new(),
                }
            );
        });
        return attack_map_info;
    }

    fn add_moves(&mut self, color: Color, moves: &Vec<Move>, board: &'a dyn BoardTrait) {
        moves.iter().for_each(|m| {
            let at = m.to.at();
            let info = self.attack_map_info.get_mut(&at).unwrap();
            match color {
                Color::White => info.attacking_white_pieces.push(board.get_piece_at(&m.from).unwrap()),
                Color::Black => info.attacking_black_pieces.push(board.get_piece_at(&m.from).unwrap())
            }
        });
    }

    pub fn from_moves(board: &'a dyn BoardTrait, white_moves: &Vec<Move>, black_moves: &Vec<Move>) -> AttackMap<'a> {
        let mut map = AttackMap {
            attack_map_info: AttackMap::initialize_map(board)
        };
        map.add_moves(Color::White, white_moves, board);
        map.add_moves(Color::Black, black_moves, board);
        return map;
    }

    pub fn new(board: &'a dyn BoardTrait) -> AttackMap<'a> {
        // how to do this ?

        // this should find attackers and defenders ..
        // initialize map for all squares
        let mut map = AttackMap {
            attack_map_info: AttackMap::initialize_map(board)
        };
        let moves = gen_pseudo_legal_moves(board, Color::White);
        map.add_moves(Color::White, &moves, board);
        let moves = gen_pseudo_legal_moves(board, Color::Black);
        map.add_moves(Color::Black, &moves, board);
        return map;
    }
    // pub fn update(&mut self, move_ : Move) {
    //     unimplemented!("update not implemented");
    // }
    pub fn get_info(&self, at: &Coordinate) -> &AttackMapInfo {
        self.attack_map_info.get(&at.at()).unwrap()
    }
    // pub fn find_pieces_pinned_to(&self, coordinate: &Coordinate) -> Vec<Pin> {
    //     unimplemented!("find_pinned pieces not done ");
    // }
}

#[derive(Debug)]
pub struct AttackMapInfo<'a> {
    square: &'a Square,
    attacking_white_pieces: Vec<&'a Piece>,
    attacking_black_pieces: Vec<&'a Piece>,
}

#[cfg(test)]
mod tests {
    use crate::chess_notation::fen_reader::*;
    use crate::chess_notation::get_path;
    use crate::move_generator::AttackMap;
    use crate::move_generator::path::get_path_to;
    use crate::board::{Coordinate, BoardTrait};

    #[test]
    fn new() {

        let board = make_board("r1b2rk1/pp5p/2pq1pp1/2bp2N1/2P5/1PQ5/PB1P1PPP/R4K1R b - - 3 18");
        let attack_map = AttackMap::new(&board);
        // white pieces
        // row by row
        // white rook h1
        let attacker = board.get_piece_at(&Coordinate::from("h1")).unwrap();
        let mut s = get_path("g1", "f1").unwrap();
        s.push(Coordinate::from("h2"));
        println!("{:?}", s);
        s.iter().for_each(|c| {
            let info = attack_map.get_info(&c);
            let attacker_found = info.attacking_white_pieces.iter().any(|&piece| {
                piece == attacker
            });
            println!("{:?}", info);
            println!("{:?}", attacker);
            assert!(attacker_found, "attack map has white rook");
        });
        // @todo : check totals

        let expected = AttackMap::initialize_map(&board);
        // expected.att
        // how to test ?
    }
}