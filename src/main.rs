extern crate getopts;
use chess_engine::game;
use dialoguer::Confirm;
use std::io::Result as IoResult;
use dialoguer::MultiSelect;
use std::env;
use getopts::Options;

fn run() -> IoResult<String> {
    // with this you could print out moves to choose from ... interesting ...
    let items = vec!["Option 1", "Option 2"];
    let chosen : Vec<usize> = MultiSelect::new()
        .items(&items)
        .interact()?;
    println!("{:?}", chosen);
    IoResult::Ok(String::from("ok"))

    // if Confirm::new().with_prompt("Do you want to continue?").interact()? {
    //     println!("Looks like you want to continue");
    // } else {
    //     println!("nevermind then :(");
    // }
    // Ok(true)
}

enum GameMode {

}

fn print_help_menu() {
    println!("For ai vs ai game \ncargo run -- --ai\n");
    println!("For human vs ai game \ncargo run\n");
}

/**
alpha - beta searching
**/
fn main() {
    //@todo:  use get opts for choosing the game modes and stuff
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("a", "ai", "run ai versus ai game");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_help_menu();
        return;
    }
    let game = game::Game::new();
    if matches.opt_present("ai") {
        game.run_ai_versus_ai();
    } else {
        game.run_human_versus_ai();
    }
}
