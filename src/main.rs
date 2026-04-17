use raylib::prelude::*;
mod level;
mod entity;
mod platformer;
mod grappler;
mod scene;
mod goal;
mod murderer;
use level::Level;
use crate::scene::{Scene, AppStatus};

const FPS: u32 = 60;

fn main() {

    let (mut rl, thread) = 
        raylib::init()
            .size(1200, 675)
            .title("Game!!")
            .build();
        
    rl.set_target_fps(FPS);

    
    let mut levels: Vec<Level> = vec![];
    let mut current_level:usize = 0;

    levels.push(level1(&mut rl, &thread));
    levels.push(level2(&mut rl, &thread));

    
    levels[current_level].init(&rl);
    while levels[current_level].get_status() != AppStatus::Terminated {

        if levels[current_level].get_next() != -1 {
            current_level = levels[current_level].get_next() as usize;
            levels[current_level].init(&rl);
        } 
        
        levels[current_level].process_input(&mut rl);
        levels[current_level].render(&mut rl, &thread);
        levels[current_level].update(&mut rl);
    }
    
}


fn level1(rl:&mut RaylibHandle, thread:&RaylibThread) -> Level{
    let mut level:Level = Level::new(rl, thread);

    let add: f32 = 20.0;

    level.add_block(rl, thread, 240.0, 260.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 377.0, 299.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 686.0, 381.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 1027.0, 531.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 1006.0, 779.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 772.0, 860.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 474.0, 968.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 256.0, 490.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 263.0, 675.0 + add, "./assets/grapple.png");
    

    level.add_evil(rl, thread, 86.0, 531.0, 80.0, 30.0, "./assets/grapple.png");
    level.add_evil(rl, thread, 516.0, 569.0, 100.0, 30.0, "./assets/grapple.png");

    // level.add_goal(rl, thread, 800.0,500.0, 0, "./assets/horse.jpg");

    level
}

fn level2(rl:&mut RaylibHandle, thread:&RaylibThread) -> Level{
    let mut level:Level = Level::new(rl, thread);
    for i in 0..10{
        level.add_block(rl, thread, 120.0 + 100.0 * i as f32,600.0 - 50.0 * i as f32, "./assets/blue.png");
    }
    level.add_goal(rl, thread, 800.0,500.0, 1, "./assets/horse.jpg");
    level
}
