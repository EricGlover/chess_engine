# chess_engine
Chess engine in Rust

### Purpose 
This is just a learning exercise for me to get a little more familiar with Rust. Feel free to browse around if you're curious.
If you are looking for a chess engine in rust https://docs.rs/chess/3.2.0/chess/ has a board and move generator. 
I haven't looked into chess engines in Rust much but you should use this instead of anything in my project. 
You could probably write your own using the board package and then writing the search algorithms yourself.
The chess programming wiki https://www.chessprogramming.org/Main_Page goes pretty in depth about Searching.

### Project Current State
The project is currently setup to play a chess game on the command line, you'll play against my AI which is quite bad.
The AI uses a bounded depth first search (alpha-beta searching) over all possible legal moves to evaluate all board states 3 moves ahead (3 ply). 
Ohh and unfortunately I haven't fully implemented algebraic move notation parsing so you'd have to look at the code to see 
how to write moves that it can correctly parse.
I do have a FEN reader that works that you can probably use btw, that thing is actually pretty handy.


### Chess Engine Technical Overview
#### Board Representation 
```Rust 
#[derive(Debug, Eq, PartialEq)]
pub struct Board {
    player_to_move: Color,
    white_castling_rights: CastlingRights,
    black_castling_rights: CastlingRights,
    en_passant_target: Option<Coordinate>,
    half_move_clock: u32,
    full_move_number: u32,
    squares: Vec<Vec<Square>>,
}
```

The board is a very simple mailbox representation https://www.chessprogramming.org/Mailbox, where
the pieces reside in a 2d vector.


#### Board Evaluation algorithm
```
f(p) = 200(K-K')
   + 9(Q-Q')
   + 5(R-R')
   + 3(B-B' + N-N')
   + 1(P-P')
   - 0.5(D-D' + S-S' + I-I')
   + 0.1(M-M')
```
KQRBNP = number of kings, queens, rooks, bishops, knights and pawns
D,S,I = doubled, blocked and isolated pawns
M = Mobility (the number of legal moves)

#### Searching 
Alpha Beta Search . The algorithm is really simple and something people normally do when playing chess. 
When searching for moves the engine will assume that each player chooses an optimal move, it considers the possible
moves and the possible opponent moves and it's possible responses and etc.. to a given depth. Out of all the paths in 
this move tree it chooses the best one for it. Alpha Beta Searching is a way to make your search over the move tree a bit 
easy by ignoring obviously bad moves. Example , as white,  you fully evaluate one possible move and score it .4 , then when you look at 
the next move you find that black has a killer response scored at -1.5; here you could continue to evaluate the rest of this move tree, 
like in minimax, but instead since you know the previous move had a better score (.4) you skip evaluating anything else with this move
and move on to the next one.


### Running 
Running the project require the rust nightly build because I'm using Bencher at the moment.
https://rust-lang.github.io/rustup/concepts/channels.html

### Plans
- Try bit boards ?
- Threads for searching
- Fully Implement algebraic notation parsing
- Bit boards
- DONE: Implement Alpha - Beta Searching

### Short Term Goals
- Finish Chess Notation
  - PGN output
  - Parsing && Outputting moves with Algebraic notation
  - Game From PGN

- benchmark board cloning
- Implement draws
  - I think this is why perft is failing at depth 3
- Switch to use Result<> more often
- Try optimizing the move generation
- Test / benchmark
  https://crates.io/crates/chess