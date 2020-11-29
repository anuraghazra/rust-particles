mod game;

use game::Game;

fn main() {
    let mut game = Game::new(800, 600);

    game.main_loop();
}
