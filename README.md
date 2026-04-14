# chess_engine
Chess engine in Rust

## Run
Play against the AI
```bash
cargo run -r
```
Watch the AI play itself.
```bash
cargo run -r -- --ai
```
Watch the Opera Game, while the AI suggests moves
```bash
cargo run -r -- --sim
```

Just enter your moves in algebraic notation. https://en.wikipedia.org/wiki/Algebraic_notation_(chess)

Running the project require the rust nightly build because I'm using Bencher at the moment.
https://rust-lang.github.io/rustup/concepts/channels.html

### Purpose 
This is just a learning exercise for me to get a little more familiar with Rust. Feel free to browse around if you're curious.
If you are looking for a chess engine in rust https://docs.rs/chess/3.2.0/chess/ has a board and move generator. 
I haven't looked into chess engines in Rust much but you should use this instead of anything in my project. 
You could probably write your own using the board package and then writing the search algorithms yourself.
The chess programming wiki https://www.chessprogramming.org/Main_Page goes pretty in depth about Searching.

### Project Current State
The project is currently setup to play a chess game on the command line, you'll play against my AI which is fairly bad.
The AI uses a bounded depth first search (alpha-beta searching) over the possible legal moves to evaluate all board states 6 moves ahead (6 ply).
I do have a FEN & PGN reader that works that you can probably use btw, that thing is actually pretty handy.


### Chess Engine Technical Overview
#### Features
* Bit boards
* Alpha-Beta Searching
* Partially Precomputed Attack Arrays
* Legal Move & Pseudo-legal move generation
* PGN reader/printer
* FEN reader/printer

#### Metrics
* Legal Move Generation seems to be around 1 Million nodes per second.

#### Board Representation 
I switched from a mail-box board representation (board.rs) to a bit board one (bit_board.rs). GameState carries all the needed info that's not known from the piece locations themselves. It also has a lookup table of piece locations.
```Rust
#[derive(Debug, Eq, PartialEq)]
pub struct GameState {
    player_to_move: Color,
    white_castling_rights: CastlingRights,
    black_castling_rights: CastlingRights,
    en_passant_target: Option<Coordinate>,
    half_move_clock: u16,
    full_move_number: u16,
    board: BitBoard,
    dirty_squares: bool,
    dirty_pieces: bool,
    squares: Vec<Square>,
    pieces: HashMap<u8, Piece>,
    is_drawn: bool,
}
```
```Rust 
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct BitBoard {
    pieces: u64,
    white_pieces: u64,
    black_pieces: u64,
    pawns: u64,
    knights: u64,
    bishops: u64,
    rooks: u64,
    queens: u64,
    kings: u64,
}
```

#### Searching 
Alpha Beta Search . The algorithm is really simple and something people normally do when playing chess. 
When searching for moves the engine will assume that each player chooses an optimal move, it considers the possible
moves and the possible opponent moves and it's possible responses and etc.. to a given depth. Out of all the paths in 
this move tree it chooses the best one for it. Alpha Beta Searching is a way to make your search over the move tree a bit 
easy by ignoring obviously bad moves. Example , as white,  you fully evaluate one possible move and score it .4 , then when you look at 
the next move you find that black has a killer response scored at -1.5; here you could continue to evaluate the rest of this move tree, 
like in minimax, but instead since you know the previous move had a better score (.4) you skip evaluating anything else with this move
and move on to the next one.

#### Partially Precomputed Attack Maps
I opted not to try magic numbers yet. So the sliding pieces moves are all generated with just bit manipulation. 

#### Legal Move & Pseudo-legal move generation
If you're looking to try and write a chess engine, I would definitely take a look here. There's a lot of optimization
that could be done here but it's worth noting this : take the time to find pinned pieces and checks when you first start writing your legal move generator.
```Rust
pub fn gen_legal_moves(game_state: &GameState, color: Color) -> Vec<Move> {
    // look for pins
    let pinned_pieces = find_pinned_pieces(game_state, color);
    // look for checks
    let checks = generate_checks(game_state, color);
    let mut moves = gen_pseudo_legal_moves(game_state, color);

    if checks.len() > 0 {
        let resolve_checks_moves =
            find_moves_to_resolve_check(game_state, &checks, &moves, Some(&pinned_pieces), color);
        return resolve_checks_moves;
    }
    let king = game_state.get_king(color).unwrap();
    let king_at = king.at().unwrap();
    return moves
        .into_iter()
        .filter(|m| {
            /* stuff */
```

### Running 
Running the project require the rust nightly build because I'm using Bencher at the moment.
https://rust-lang.github.io/rustup/concepts/channels.html

### Plans


### Short Term Goals
* Search Improvements :
    * Pre-ordering moves for searching
* Move generation improvements
    * Legal move generation refactor
    * Precompute slider attacks with magic numbers
