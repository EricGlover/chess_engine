use crate::board::*;
use crate::move_generator::*;
use rand::prelude::ThreadRng;
use rand::Rng;

pub struct AI {
    rng: ThreadRng,
    color: Color,
}

impl AI {
    pub fn new(color: Color) -> AI {
        AI {
            rng: rand::thread_rng(),
            color,
        }
    }
    pub fn make_move(&mut self, board: &Board) -> Move {
        let mut moves = gen_moves(&board, self.color);
        let moveCount = moves.iter().len();
        let i = self.rng.gen_range((0..moveCount));
        moves.remove(i)
    }
}
