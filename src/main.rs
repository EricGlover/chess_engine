use chess_engine::game;

/**
alpha - beta searching
**/
fn main() {
    let game = game::Game::new();
    game.run();
}
