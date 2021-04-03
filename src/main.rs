use chess_engine::game;

/**
alpha - beta searching
**/
fn main() {
    let game = game::Game::new();
    // game.run_ai_versus_ai();
    game.run_human_versus_ai();
}
