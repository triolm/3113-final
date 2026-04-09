mod game;
mod entity;
use game::Game;

fn main() {
    let mut game = Game::new();
    game.run(); 
  
}

