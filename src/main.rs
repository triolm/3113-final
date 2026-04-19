use raylib::prelude::*;
mod level;
mod entity;
mod platformer;
mod grappler;
mod scene;
mod goal;
mod murderer;
mod mariolevel;
mod swimlevel;
mod swimmer;
use level::Level;
use mariolevel::MarioLevel;
use swimlevel::SwimLevel;
use crate::scene::{Scene, AppStatus};

const FPS: u32 = 60;

fn main() {

    let (mut rl, thread) = 
        raylib::init()
            .size(1200, 675)
            .title("Game!!")
            .build();
        
    rl.set_target_fps(FPS);

    
    let mut levels: Vec<Box<dyn Scene>> = vec![];
    let mut current_level:usize = 0;

    //dummy
    levels.push(Box::new(level_mario(&mut rl, &thread))); // 1
    
    
    // levels.push(Box::new(level_game(&mut rl, &thread))); // 1
    // levels.push(Box::new(level_multiplayer(&mut rl, &thread))); // 2
    // levels.push(Box::new(level_video_game(&mut rl, &thread))); // 3
    // levels.push(Box::new(level_nintendo(&mut rl, &thread))); // 4

   
    
    levels[current_level].init(&rl);
    while levels[current_level].get_status() != AppStatus::Terminated {

        if levels[current_level].get_next() != -1 {
            current_level = levels[current_level].get_next() as usize;
            levels[current_level].init(&rl);
        } 
        
        levels[current_level].process_input(&mut rl);
        levels[current_level].update(&mut rl);

        levels[current_level].render(&mut rl, &thread);
    }
    
}


fn level_game(rl:&mut RaylibHandle, thread:&RaylibThread) -> Level{
    let mut level:Level = Level::new(rl, thread, "./assets/Page2.png");

    let add: f32 = 10.0;

    level.add_block(rl, thread, 232.0, 270.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 715.0, 270.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 825.0, 500.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 440.0, 681.0 + add, "./assets/grapple.png");
    

    level.add_evil(rl, thread, 463.0, 581.0, 800.0, 30.0, "./assets/grapple.png");
    
    //esports
    level.add_goal(rl, thread, 845.0,685.0, 1, "./assets/horse.jpg");
    // multiplayer
    level.add_goal(rl, thread, 113.0,832.0, 2, "./assets/horse.jpg");

    level
}

fn level_video_game(rl:&mut RaylibHandle, thread:&RaylibThread) -> Level{
    let mut level:Level = Level::new(rl, thread, "./assets/Page4.png");

    let add: f32 = 10.0;

    level.add_block(rl, thread, 272.0, 270.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 689.0, 428.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 676.0, 698.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 1133.0, 899.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 599.0, 978.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 78.0, 975.0 + add, "./assets/grapple.png");
    

    level.add_evil(rl, thread, 594.0, 500.0, 90.0, 30.0, "./assets/grapple.png");
    level.add_evil(rl, thread, 437.0, 543.0, 210.0, 30.0, "./assets/grapple.png");
    level.add_evil(rl, thread, 202.0, 583.0, 300.0, 30.0, "./assets/grapple.png");
    level.add_evil(rl, thread, 825.0, 690.0, 250.0, 30.0, "./assets/grapple.png");
    level.add_evil(rl, thread, 1211.0, 650.0, 450.0, 30.0, "./assets/grapple.png");
    level.add_evil(rl, thread, 543.0, 927.0, 900.0, 30.0, "./assets/grapple.png");

    // nintendo
    level.add_goal(rl, thread, 97.0,1136.0, 4, "./assets/horse.jpg");

    level
}

fn level_multiplayer(rl:&mut RaylibHandle, thread:&RaylibThread) -> Level{
    let mut level:Level = Level::new(rl, thread, "./assets/Page5.png");

    let add: f32 = 20.0;

    level.add_block(rl, thread, 304.0, 260.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 654.0, 260.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 967.0, 381.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 662.0, 766.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 335.0, 728.0 + add, "./assets/grapple.png");
    
    level.add_evil(rl, thread, 453.0, 647.0, 800.0, 30.0, "./assets/grapple.png");

    // video game
    level.add_goal(rl, thread,353.0,992.0, 3, "./assets/horse.jpg");

    level
}

fn level_nintendo(rl:&mut RaylibHandle, thread:&RaylibThread) -> Level{
    let mut level:Level = Level::new(rl, thread, "./assets/Page6.png");

    let add: f32 = 20.0;

    level.add_block(rl, thread, 321.0, 298.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 699.0, 475.0 + add, "./assets/grapple.png");
    level.add_block(rl, thread, 541.0, 819.0 + add, "./assets/grapple.png");
    
    level.add_evil(rl, thread, 276.0, 637.0, 450.0, 30.0, "./assets/grapple.png");
    level.add_evil(rl, thread, 303.0, 675.0, 88.0, 30.0, "./assets/grapple.png");
    level.add_evil(rl, thread, 437.0, 738.0, 450.0, 30.0, "./assets/grapple.png");

    //  game
    level.add_goal(rl, thread,149.0,677.0, 1, "./assets/horse.jpg");

    level
}
fn level_mario(rl:&mut RaylibHandle, thread:&RaylibThread) -> MarioLevel{
    let mut level:MarioLevel = MarioLevel::new(rl, thread, "./assets/Page8.png");

    let add: f32 = 00.0;

    level.add_block(rl, thread, 562.0, 460.0 + add, 1023.0, 25.0,  "./assets/blue.png");
    level.add_block(rl, thread, 447.0, 261.0 + add, 300.0, 75.0,  "./assets/blue.png");
    level.add_block(rl, thread, 893.0, 691.0 + add, 743.0, 75.0,  "./assets/blue.png");
    level.add_block(rl, thread, 414.0, 1040.0 + add, 300.0, 75.0,  "./assets/blue.png");
    level.add_block(rl, thread, 972.0, 1115.0 + add, 371.0, 75.0,  "./assets/blue.png");
    level.add_block(rl, thread, 1020.0, 1020.0 + add, 160.0, 150.0,  "./assets/blue.png");
    
    level.add_goal(rl, thread, 1026.0, 939.0, 1, "./assets/grapple.png");
    // level.add_evil(rl, thread, 303.0, 675.0, 88.0, 30.0, "./assets/grapple.png");
    // level.add_evil(rl, thread, 437.0, 738.0, 450.0, 30.0, "./assets/grapple.png");

    // //  game
    // level.add_goal(rl, thread,149.0,677.0, 1, "./assets/horse.jpg");

    level
}
