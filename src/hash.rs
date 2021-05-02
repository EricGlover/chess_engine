use crate::board::{BoardTrait, Color, Coordinate, PieceType};
use rand::prelude::ThreadRng;
use rand::Rng;

// initialization
// one number per piece_type/color per square
fn init() {
    // https://www.chessprogramming.org/Zobrist_Hashing
    // 64 bit hash
    let mut rng = rand::thread_rng();
    let max = (u64::pow(2, 64) - 1);
    let num = rng.gen_range((0..max));
}

#[derive(Debug)]
struct Zobrist {
    white_pawn: [u64; 64],
    white_knight: [u64; 64],
    white_bishop: [u64; 64],
    white_rook: [u64; 64],
    white_queen: [u64; 64],
    white_king: [u64; 64],
    black_pawn: [u64; 64],
    black_knight: [u64; 64],
    black_bishop: [u64; 64],
    black_rook: [u64; 64],
    black_queen: [u64; 64],
    black_king: [u64; 64],
    black_to_move: u64,
    white_king_side_castle: u64,
    white_queen_side_castle: u64,
    black_king_side_castle: u64,
    black_queen_side_castle: u64,
    files: [u64; 8], // for en passant
}

impl Zobrist {
    pub fn new() -> Zobrist {
        // https://www.chessprogramming.org/Zobrist_Hashing
        // 64 bit hash
        let mut rng = rand::thread_rng();
        let mut gen = || rng.gen_range((0..u64::MAX));
        let mut gen_pieces = || {
            [
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
                gen(),
            ]
        };

        Zobrist {
            white_pawn: gen_pieces(),
            white_knight: gen_pieces(),
            white_bishop: gen_pieces(),
            white_rook: gen_pieces(),
            white_queen: gen_pieces(),
            white_king: gen_pieces(),
            black_pawn: gen_pieces(),
            black_knight: gen_pieces(),
            black_bishop: gen_pieces(),
            black_rook: gen_pieces(),
            black_queen: gen_pieces(),
            black_king: gen_pieces(),
            black_to_move: gen(),
            white_king_side_castle: gen(),
            white_queen_side_castle: gen(),
            black_king_side_castle: gen(),
            black_queen_side_castle: gen(),
            files: [gen(), gen(), gen(), gen(), gen(), gen(), gen(), gen()],
        }
    }

    pub fn hash_board(&self, board: &dyn BoardTrait) -> u64 {
        // hash pieces
        let mut white_pieces = board.get_all_pieces(Color::White);
        let mut pieces = board.get_all_pieces(Color::Black);
        pieces.append(&mut white_pieces);
        let mut hash = pieces.into_iter().fold(0u64, |hash, p| {
            hash ^ self.hash_piece(&p.piece_type, &p.color, p.at().unwrap())
        });

        // side to move
        if board.player_to_move() == Color::Black {
            hash ^= self.black_to_move;
        }

        // castling rights
        if board.can_castle_king_side(Color::White) {
            hash ^= self.white_king_side_castle;
        }
        if board.can_castle_king_side(Color::Black) {
            hash ^= self.black_king_side_castle;
        }
        if board.can_castle_queen_side(Color::White) {
            hash ^= self.white_queen_side_castle;
        }
        if board.can_castle_queen_side(Color::Black) {
            hash ^= self.black_king_side_castle;
        }
        // en passant file
        if board.en_passant_target().is_some() {
            let file = board.en_passant_target().unwrap().x();
            hash ^= self.files.get((file - 1) as usize).unwrap();
        }
        hash
    }

    fn coordinate_to_index(at: &Coordinate) -> usize {
        // prev rows = (at.y - 1) * 8
        // ((at.y - 1) * 8 ) +
        // (at.x - 1)
        (((at.y() -1) * 8) + (at.x() -1)) as usize
    }

    fn hash_piece(&self, piece_type: &PieceType, color: &Color, at: &Coordinate) -> u64 {
        let index = Zobrist::coordinate_to_index(at);
        let arr = match piece_type {
            &PieceType::King => {
                match color {
                    &Color::White => {
                        &self.white_king
                    },
                    &Color::Black => {
                        &self.black_king
                    }
                }
            }
            &PieceType::Queen => {
                match color {
                    &Color::White => {
                        &self.white_queen
                    },
                    &Color::Black => {
                        &self.black_queen
                    }
                }
            }
            &PieceType::Bishop => {
                match color {
                    &Color::White => {
                        &self.white_bishop
                    },
                    &Color::Black => {
                        &self.black_bishop
                    }
                }
            }
            &PieceType::Knight => {
                match color {
                    &Color::White => {
                        &self.white_knight
                    },
                    &Color::Black => {
                        &self.black_knight
                    }
                }
            }
            &PieceType::Rook => {
                match color {
                    &Color::White => {
                        &self.white_rook
                    },
                    &Color::Black => {
                        &self.black_rook
                    }
                }
            }
            &PieceType::Pawn => {
                match color {
                    &Color::White => {
                        &self.white_pawn
                    },
                    &Color::Black => {
                        &self.black_pawn
                    }
                }
            }
        };
        arr.get(index).unwrap().clone()
    }

    pub fn add_piece(&self, hash: u64, piece_type: &PieceType, color: &Color, at: &Coordinate) -> u64 {
        hash ^ self.hash_piece(piece_type, color, at)
    }

    pub fn remove_piece(&self, hash: u64, piece_type: &PieceType, color: &Color, at: &Coordinate) -> u64 {
        hash ^ self.hash_piece(piece_type, color, at)
    }
}

mod test {
    use super::*;
    use crate::chess_notation::fen_reader;

    #[test]
    fn zobrist_new() {
        let z = Zobrist::new();
        println!("{:?}", z);
    }

    #[test]
    fn test_hash_board() {
        let hasher = Zobrist::new();
        let board = fen_reader::make_initial_board();
        let hash = hasher.hash_board(&board);
        let hash2 = hasher.hash_board(&board);
        assert_eq!(hash, hash2, "same board, same hash");
        let board2 = fen_reader::make_board(fen_reader::TEST_BOARD_1);
        let hash2 = hasher.hash_board(&board2);
        assert_ne!(hash, hash2, "different board, different hash");
        let board2 = fen_reader::make_board(fen_reader::BLACK_IN_CHECK);
        let hash2 = hasher.hash_board(&board2);
        assert_ne!(hash, hash2, "different board, different hash");
        let board2 = fen_reader::make_board(fen_reader::WHITE_IN_CHECK);
        let hash2 = hasher.hash_board(&board2);
        assert_ne!(hash, hash2, "different board, different hash");
        let board2 = fen_reader::make_board(fen_reader::TEST_BOARD_2);
        let hash2 = hasher.hash_board(&board2);
        assert_ne!(hash, hash2, "different board, different hash");
    }

    #[test]
    fn test_coordinate_to_index() {
        // starting index
        let expected = 0usize;
        let index = Zobrist::coordinate_to_index(&Coordinate::new(1,1));
        assert_eq!(expected, index, "a1 is starting index");

        // ending index
        let expected = 63usize;
        let index = Zobrist::coordinate_to_index(&Coordinate::new(8,8));
        assert_eq!(expected, index, "h8 is ending index");

        let expected = 8usize;
        let index = Zobrist::coordinate_to_index(&Coordinate::new(1, 2));
        assert_eq!(expected, index);

        let expected = 9usize;
        let index = Zobrist::coordinate_to_index(&Coordinate::new(2, 2));
        assert_eq!(expected, index);
    }

    #[test]
    fn test_hash_piece() {
        let hasher = Zobrist::new();
        let h = hasher.hash_piece(&PieceType::Rook, &Color::White, &Coordinate::new(1, 1));
        let h2 = hasher.hash_piece(&PieceType::Rook, &Color::White, &Coordinate::new(1, 1));
        assert_eq!(h, h2);
        let h2 = hasher.hash_piece(&PieceType::Rook, &Color::Black, &Coordinate::new(1, 1));
        assert_ne!(h, h2);
        let h2 = hasher.hash_piece(&PieceType::Rook, &Color::White, &Coordinate::new(2, 1));
        assert_ne!(h, h2);
        let h2 = hasher.hash_piece(&PieceType::Rook, &Color::White, &Coordinate::new(1, 2));
        assert_ne!(h, h2);
    }

    #[test]
    fn add_piece() {
        //@todo::
    }
    #[test]
    fn remove_piece() {
        //@todo::
    }
}

// hash board

// make move

// unmake move
