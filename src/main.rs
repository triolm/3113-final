mod game;
mod entity;
mod platformer;
mod grappler;
use game::Game;

fn main() {
    let mut game = Game::new();
    game.run(); 
  
}

