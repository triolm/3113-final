mod level;
mod entity;
mod platformer;
mod grappler;
mod scene;
use level::Level;
use scene::Scene;

fn main() {
    let mut level = level1();
    level.run(); 
    
}


fn level1() -> Level{
    let mut level:Level = Level::new();
    for i in 0..10{
        level.add_block(100.0 + 100.0 * i as f32,100.0 + 50.0 * i as f32, "./assets/blue.png");
    }

    level
}