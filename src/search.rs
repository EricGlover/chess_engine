use std::collections::HashMap;
use std::time::Instant;

use crate::board::BoardTrait;
use crate::chess_notation::{ print_move};
use crate::game_state::{ GameState};
use crate::move_generator::gen_legal_moves;
/*
a2a3: 8928
b2b3: 9788
c2c3: 10945
d2d3: 10598
e2e3: 8934
f2f3: 9381
g2g3: 11956
h2h3: 8487
a2a4: 8914
b2b4: 10770
c2c4: 11468
d2d4: 10542
e2e4: 9841
f2f4: 9334
g2g4: 12419
h2h4: 8478
a1b3: 9784
d1c3: 9760
d1e3: 9783
f1g1: 8487

Nodes searched: 198597

position fen rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1
go perft 3
a7a6: 361
b7b6: 399
c7c6: 399
d7d6: 512...
e7e6: 569
f7f6: 361
g7g6: 399
h7h6: 361
a7a5: 399
b7b5: 400
c7c5: 419...
d7d5: 532
e7e5: 570
f7f5: 381
g7g5: 400
h7h5: 399
b8a6: 380
b8c6: 418
g8f6: 418
g8h6: 380

Nodes searched: 8457

*/
const FEN_1:&str = "rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 1";


pub fn perft(game_state: &mut GameState, depth: u8) -> u64 {
    let mut moves = gen_legal_moves(game_state, game_state.get_player_to_move());
    if depth == 1 {
        return moves.len() as u64;
    }
    let mut nodes_searched = 0u64;
    for mut current_move in moves.iter_mut() {
        game_state.make_move_mut(current_move);
        nodes_searched += perft(game_state, depth - 1);
        game_state.unmake_move_mut(current_move);
    }
    return nodes_searched;
}

pub fn perft_divided(game_state: &mut GameState, depth: u8) -> (u64, Option<HashMap<String, u64>>) {
    let mut moves = gen_legal_moves(game_state, game_state.get_player_to_move());
    if depth == 1 {
        return (moves.len() as u64, None);
    }
    let mut move_map: HashMap<String, u64> = HashMap::new();
    let mut nodes_searched = 0u64;
    for mut current_move in moves.iter_mut() {
        game_state.make_move_mut(&mut current_move);

        // println!("\n");
        // print_bit_board(game_state.get_board_ref());
        // println!("\n");

        let (current_searched, map) = perft_divided(game_state, depth - 1);

        game_state.unmake_move_mut(&mut current_move);
        // println!("UNMAKE\n");
        // print_bit_board(game_state.get_board_ref());
        // println!("UNMAKE\n");

        move_map.insert(print_move(&current_move, &game_state), current_searched);
        nodes_searched += current_searched;
    }
    return (nodes_searched, Some(move_map));
}

pub fn test() {
    let mut game_state = GameState::starting_game();
    // let (total, map_opt) = perft_divided(&mut game_state, 4);
    // // let mut game_state = fen_reader::make_game_state(FEN_1);
    // // let (total, map_opt) = perft_divided(&mut game_state, 3);
    // if let Some(map) = map_opt {
    //     for s in map.keys() {
    //         println!("{} : {} ", s, map.get(s).unwrap());
    //     }
    //     println!("{} nodes", total);
    //     let keys:Vec<&String> = map.keys().collect();
    //     println!("{} 1st order moves", keys.len())
    // }
    let start = Instant::now();
    let nodes = perft(&mut game_state, 3);
    let seconds = start.elapsed().as_millis();
    println!("{} nodes, {} ms, {} nodes/ms", nodes, seconds, nodes as u128 /seconds);
    
}

mod test {
    use super::*;

    #[test]
    fn test_init_perft() {
        let mut game_state = GameState::starting_game();
        let nodes = perft(&mut game_state, 1);
        assert_eq!(nodes, 20, "perft 1");
        let nodes = perft(&mut game_state, 2);
        assert_eq!(nodes, 400, "perft 2");
        let nodes = perft(&mut game_state, 3);
        assert_eq!(nodes, 8902, "perft 3");
        let nodes = perft(&mut game_state, 4);
        assert_eq!(nodes, 197281, "perft 4");
        let nodes = perft(&mut game_state, 5);
        assert_eq!(nodes, 4865609, "perft 5");
        // let nodes = perft(&mut game_state, 6);
        // assert_eq!(nodes, 119060324, "perft 6");
        // let nodes = perft(&mut game_state, 7);
        // assert_eq!(nodes, 3195901860, "perft 7");
        // let nodes = perft(&mut game_state, 8);
        // assert_eq!(nodes, 84998978956, "perft 8");
    }
}

mod bench {
    use ::test::Bencher;
    use super::*;

    #[bench]
    fn perft_1(b: &mut Bencher) {
        let mut game_state = GameState::starting_game();
        b.iter(|| perft(&mut game_state, 1));
    }
    #[bench]
    fn perft_2(b: &mut Bencher) {
        let mut game_state = GameState::starting_game();
        b.iter(|| perft(&mut game_state, 2));
    }
    #[bench]
    fn perft_3(b: &mut Bencher) {
        let mut game_state = GameState::starting_game();
        b.iter(|| perft(&mut game_state, 3));
    }
    #[bench]
    fn perft_4(b: &mut Bencher) {
        let mut game_state = GameState::starting_game();
        b.iter(|| perft(&mut game_state, 4));
    }
    // #[bench]
    // fn perft_5(b: &mut Bencher) {
    //     let mut game_state = GameState::starting_game();
    //     b.iter(|| perft(&mut game_state, 5));
    // }
    // #[bench]
    // fn perft_6(b: &mut Bencher) {
    //     let mut game_state = GameState::starting_game();
    //     b.iter(|| perft(&mut game_state, 6));
    // }
    // #[bench]
    // fn perft_7(b: &mut Bencher) {
    //     let mut game_state = GameState::starting_game();
    //     b.iter(|| perft(&mut game_state, 7));
    // }
    // #[bench]
    // fn perft_8(b: &mut Bencher) {
    //     let mut game_state = GameState::starting_game();
    //     b.iter(|| perft(&mut game_state, 8));
    // }
}
