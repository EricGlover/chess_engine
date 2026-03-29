#![warn(unused_extern_crates)]
use chess_engine::board::Coordinate;
use chess_engine::chess_notation;
use chess_engine::{chess_notation::pgn, game};
use chess_engine::board::PieceType;
use getopts::Options;
use std::io::Read;
use std::{env, path};
use std::fs::{self, File, Metadata};
use std::path::Path;
use regex::*;

use pgn::Game as notated_game;


fn print_help_menu() {
    println!("For ai vs ai game \ncargo run -- --ai\n");
    println!("For human vs ai game \ncargo run\n");
}

fn main() {

    let s = r#"[Event "Paris"]
[Site "Paris FRA"]
[Date "1858.??.??"]
[Round "?"]
[White "Paul Morphy"]
[Black "Duke Karl / Count Isouard"]
[Result "1-0"]
[ECO "C41"]
[PlyCount "33"]
[EventDate "1858.??.??"]

1. e4 e5 2. Nf3 d6 3. d4 Bg4 {This is a weak move already.--Fischer // Another
important example that showes the viability of "Knights before bishops" rule}
4. dxe5 Bxf3 5. Qxf3 dxe5 6. Bc4 Nf6 7. Qb3 {[pgndiagram] Everything with the
tempo} Qe7 8. Nc3 c6 9. Bg5 {Black is in what's like a zugzwang position here.
He can't develop the [Queen's] knight because the pawn is hanging, the bishop
is blocked because of the Queen.--Fischer} b5 {[pgndiagram]} 10. Nxb5 $1 {It
begins. One flashy sacrifice after another} cxb5 11. Bxb5+ Nbd7 12. O-O-O Rd8 {
[pgndiagram]} 13. Rxd7 $1 Rxd7 14. Rd1 Qe6 15. Bxd7+ Nxd7 {[pgndiagram]} 16.
Qb8+ $3 Nxb8 17. Rd8# {What a game and what a crush!} 1-0"#;

    //@todo:  use get opts for choosing the game modes and stuff
    let args: Vec<String> = env::args().collect();
    let mut opts = Options::new();
    opts.optflag("a", "ai", "run ai versus ai game");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("s", "sim", "run sim game to test engine");
    opts.optflag("p", "pvp", "run player vs player");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };
    if matches.opt_present("h") {
        print_help_menu();
        return;
    }
    if matches.opt_present("p") {
        // @todo:
    }
    if matches.opt_present("s") {
        println!("hello");
        // @todo:
        // pgn::
        let game = game::Game::new();
        // read pgn file 
        //check for Games directory 
        let res = fs::metadata("./Games");
        let is_dir: bool = match res {
            Ok(f) => f.is_dir(),
            Err(err) => false,
        };
        if is_dir {
            let mut path_str = format!("./Games/");
            path_str.push_str("1.pgn");
            let path = Path::new(path_str.as_str());
            let display = path.display();
            let contents = match std::fs::read_to_string(&path) {
                Err(err) => panic!("couldn't read {}: {}", display, err),
                Ok(file) => file,
            };
            // let mut contents = String::new();
            // let mut file = match fs::File::open(&path) {
            //     Err(err) => panic!("couldn't read {}: {}", display, err),
            //     Ok(file) => file,
            // };
            // file.read_to_string(&mut contents);
            println!("{}", contents);
            let moves = pgn::moves_from_pgn(contents);
            // let g = pgn::Game::new_from_pgn(contents);
            game.run_sim_game(moves);
        }

        let str = String::from("");
        // let g = pgn::Game::new_from_pgn(str);
    }
    let game = game::Game::new();
    if matches.opt_present("ai") {
        game.run_ai_versus_ai();
    } else {
        game.run_human_versus_ai();
    }
}
