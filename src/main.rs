extern crate piston;
extern crate piston_window;
extern crate rand;

mod game;

use game::Game;

fn main() {
    let mut game = Game::new(500, 500);

    game.main_loop();
}
