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
The AI uses an unoptimized depth first search (minimax) over all possible legal moves to evaluate all board states 3 moves ahead (3 ply). 
Ohh and unfortunately I haven't fully implemented algebraic move notation parsing so you'd have to look at the code to see 
how to write moves that it can correctly parse.
I do have a FEN reader that works that you can probably use btw, that thing is actually pretty handy.

### Running 
Running the project require the rust nightly build because I'm using Bencher at the moment.
https://rust-lang.github.io/rustup/concepts/channels.html


### Short Term Goals
- Finish Chess Notation
  - PGN output
  - Parsing && Outputting moves with Algebraic notation   
  - Game From PGN 

### Plans
- Make/Unmake boards
- Try going back to Piece pointers
- benchmark board cloning 
- Implement draws
  - I think this is why perft is failing at depth 3
- Switch to use Result<> more often 
- Fix bug in bug.log 
- Board Traits
- Try optimizing the move generation
- Test / benchmark
  https://crates.io/crates/chess
- Try bit boards ?
- Threads for searching 

- DONE: Implement Alpha - Beta Searching
- Fully Implement algebraic notation parsing 
- Print pgn files of the engine playing itself for the lolz
- Bit boards
- do more /#[bench] benchmarking 


## changes
Piece Refs
Pins ^
Moves ^
Square returns Piece with remove
Square no longer uses public fields 
Move needs fields for unmake