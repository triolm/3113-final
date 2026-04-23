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
mod goomba;
mod shark;
mod swimmer;
use level::Level;
use entity::Entity;
use platformer::Platformer;
use mariolevel::MarioLevel;
use swimlevel::SwimLevel;
use crate::{entity::Positioned, scene::{AppStatus, Scene}};

const FPS: u32 = 60;

#[derive(PartialEq, Copy, Clone)]
enum EffectStatus {
    RUN,
    PEAK,
    NONE
}

fn main() {

    let (mut rl, thread) = 
        raylib::init()
            .size(1200, 675)
            .title("Game!!")
            .build();
        
    rl.set_target_fps(FPS);


    let audio = RaylibAudio::init_audio_device().expect("oh no");
    
    audio.set_master_volume(0.0);

    let mut levels: Vec<Box<dyn Scene>> = vec![];
    let mut current_level:usize = 0;
    
    let mut sounds: Vec<Sound> = vec![];
    sounds.push(audio.new_sound("assets/die.mp3").expect("oh no")); //0
    sounds.push(audio.new_sound("assets/mariojump.mp3").expect("oh no"));//1
    sounds.push(audio.new_sound("assets/smb_mariodie.wav").expect("oh no"));
    sounds.push(audio.new_sound("assets/smb_stomp.wav").expect("oh no"));
    sounds.push(audio.new_sound("assets/smb_pipe.wav").expect("oh no"));
    sounds.push(audio.new_sound("assets/waterdie.mp3").expect("oh no"));
    
    let mut musics: Vec<Music> = vec![];
    musics.push(audio.new_music("assets/lesbaricades.mp3").expect("oh no"));
    musics.push(audio.new_music("assets/mario.mp3").expect("oh no"));
    musics.push(audio.new_music("assets/water.mp3").expect("oh no"));
    musics[1].set_volume(0.7);

    //dummy
    levels.push(Box::new(level_mario())); // 1
    // levels.push(Box::new(level_mario(&mut rl, &thread))); // 1
    
    levels.push(Box::new(level_game())); // 1
    levels.push(Box::new(level_multiplayer())); // 2
    levels.push(Box::new(level_video_game())); // 3
    levels.push(Box::new(level_nintendo())); // 4
    levels.push(Box::new(level_mario())); // 5
    levels.push(Box::new(level_river(&mut rl, &thread))); // 6
    levels.push(Box::new(level_fish(&mut rl, &thread))); // 7
    levels.push(Box::new(level_tuna(&mut rl, &thread))); // 8
    levels.push(Box::new(level_esports())); // 9
    
    levels[current_level].load(&mut rl, &thread);
    levels[current_level].init(&rl);
    let mut music_index = levels[current_level].get_music();
    if music_index != -1 {
        musics[music_index as usize].play_stream();
    }

    let mut effect_y = 10000.0;
    let mut press_space = Platformer::new("assets/blue.png".to_string(), Vector2{x:1200.0, y:700.0});
    press_space.set_start_position(Vector2 { x: 1200.0/2.0, y: effect_y});
    press_space.set_position(Vector2 { x: 1200.0/2.0, y: effect_y });
    press_space.load(&mut rl, &thread);

    
    let mut effect:EffectStatus = EffectStatus::NONE;
    let mut next:i32 = -1;
    let mut init = false;

    while levels[current_level].get_status() != AppStatus::Terminated {
       
        
        if  effect == EffectStatus::PEAK {
            current_level = next as usize;
            levels[current_level].load(&mut rl, &thread);
            
            let prev_music = music_index;
            music_index = levels[current_level].get_music();
            if music_index != -1 && music_index != prev_music { 
                musics[music_index as usize].play_stream(); 
                musics[music_index as usize].seek_stream(0.0); 
            }


            // for aesthetic purposes....
            levels[current_level].init(&rl); 
            effect = EffectStatus::RUN;
        } 
        
        if effect == EffectStatus::NONE {
            if !init { 
                levels[current_level].init(&rl); 
                init = true;
            } 
            levels[current_level].process_input(&mut rl);
            levels[current_level].update(&mut rl);
            let sound_index = levels[current_level].get_sound();
            if sound_index != -1 {
                sounds[sound_index as usize].play();
            }
        }

         if levels[current_level].get_next() != -1 && effect == EffectStatus::NONE {
            effect = EffectStatus::RUN;
            next = levels[current_level].get_next();
            effect_y = -700.0;
        }

        {
            let mut d = rl.begin_drawing(&thread);
            levels[current_level].render(&mut d);
            if music_index != -1 { musics[music_index as usize].update_stream(); }

             if effect == EffectStatus::RUN{
                // println!("{}", effect_y);
                effect_y += 40.0;
                if effect_y < (675.0/2.0) + 20.0 && effect_y > (675.0/2.0) - 20.0 {effect= EffectStatus::PEAK};
                press_space.set_position(Vector2 { x: press_space.get_position().x, y: effect_y });
                press_space.render(&mut d);
                if effect_y > 700.0 * 2.0 { 
                    effect = EffectStatus::NONE; 
                    init = false;
                }
            }
        }
    }
    
}


fn level_game() -> Level{
    let mut level:Level = Level::new("./assets/Page2.png");

    let add: f32 = 10.0;

    level.add_block(232.0, 270.0 + add, "./assets/grapple.png");
    level.add_block(715.0, 270.0 + add, "./assets/grapple.png");
    level.add_block(825.0, 500.0 + add, "./assets/grapple.png");
    level.add_block(440.0, 681.0 + add, "./assets/grapple.png");
    

    level.add_evil(463.0, 581.0, 800.0, 30.0, "./assets/grapple.png");
    
    //esports
    level.add_goal(845.0,685.0, 9, "./assets/horse.jpg");
    // multiplayer
    level.add_goal(113.0,832.0, 2, "./assets/horse.jpg");

    level
}

fn level_video_game() -> Level{
    let mut level:Level = Level::new("./assets/Page4.png");

    let add: f32 = 10.0;

    level.add_block(272.0, 270.0 + add, "./assets/grapple.png");
    level.add_block(689.0, 428.0 + add, "./assets/grapple.png");
    level.add_block(676.0, 698.0 + add, "./assets/grapple.png");
    level.add_block(1133.0, 899.0 + add, "./assets/grapple.png");
    level.add_block(599.0, 978.0 + add, "./assets/grapple.png");
    level.add_block(78.0, 975.0 + add, "./assets/grapple.png");
    

    level.add_evil(594.0, 500.0, 90.0, 30.0, "./assets/grapple.png");
    level.add_evil(437.0, 543.0, 210.0, 30.0, "./assets/grapple.png");
    level.add_evil(202.0, 583.0, 300.0, 30.0, "./assets/grapple.png");
    level.add_evil(825.0, 690.0, 250.0, 30.0, "./assets/grapple.png");
    level.add_evil(1211.0, 650.0, 450.0, 30.0, "./assets/grapple.png");
    level.add_evil(543.0, 927.0, 900.0, 30.0, "./assets/grapple.png");

    // nintendo
    level.add_goal(97.0,1136.0, 4, "./assets/horse.jpg");

    level
}

fn level_multiplayer() -> Level{
    let mut level:Level = Level::new("./assets/Page5.png");

    let add: f32 = 20.0;

    level.add_block(304.0, 260.0 + add, "./assets/grapple.png");
    level.add_block(654.0, 260.0 + add, "./assets/grapple.png");
    level.add_block(967.0, 381.0 + add, "./assets/grapple.png");
    level.add_block(662.0, 766.0 + add, "./assets/grapple.png");
    level.add_block(335.0, 728.0 + add, "./assets/grapple.png");
    
    level.add_evil(453.0, 647.0, 800.0, 30.0, "./assets/grapple.png");

    // video game
    level.add_goal(353.0,992.0, 3, "./assets/horse.jpg");

    level
}

fn level_nintendo() -> Level{
    let mut level:Level = Level::new("./assets/Page6.png");

    let add: f32 = 20.0;

    level.add_block(321.0, 298.0 + add, "./assets/grapple.png");
    level.add_block(699.0, 475.0 + add, "./assets/grapple.png");
    level.add_block(541.0, 819.0 + add, "./assets/grapple.png");
    
    level.add_evil(276.0, 637.0, 450.0, 30.0, "./assets/grapple.png");
    level.add_evil(303.0, 675.0, 88.0, 30.0, "./assets/grapple.png");
    level.add_evil(437.0, 738.0, 280.0, 30.0, "./assets/grapple.png");

    //  mario
    level.add_goal(149.0,677.0, 5, "./assets/horse.jpg");

    level
}

fn level_esports() -> Level{
    let mut level:Level = Level::new("./assets/Page7.png");

    let add: f32 = 20.0;

    level.add_block(252.0, 403.0 + add, "./assets/grapple.png");
    level.add_block(637.0, 441.0 + add, "./assets/grapple.png");
    level.add_block(745.0, 752.0 + add, "./assets/grapple.png");
    level.add_block(640.0, 1083.0 + add, "./assets/grapple.png");
    level.add_block(359.0, 897.0 + add, "./assets/grapple.png");
    
    level.add_evil(282.0, 670.0, 461.0, 30.0, "./assets/grapple.png");
    level.add_evil(292.0, 751.0, 484.0, 25.0, "./assets/grapple.png");
    level.add_evil(443.0, 789.0, 46.0, 30.0, "./assets/grapple.png");
    level.add_evil(442.0, 859.0, 74.0, 30.0, "./assets/grapple.png");
    level.add_evil(475.0, 895.0, 84.0, 30.0, "./assets/grapple.png");

    //  fish
    level.add_goal(228.0,781.0, 7, "./assets/horse.jpg");

    level
}
fn level_mario() -> MarioLevel{
    let mut level:MarioLevel = MarioLevel::new("./assets/Page8.png");

    let add: f32 = 00.0;

    level.add_block(562.0, 460.0 + add, 1023.0, 25.0,  "./assets/blue.png");
    level.add_block(447.0, 261.0 + add, 300.0, 75.0,  "./assets/blue.png");
    level.add_block(893.0, 691.0 + add, 743.0, 75.0,  "./assets/blue.png");
    level.add_block(414.0, 1040.0 + add, 300.0, 75.0,  "./assets/blue.png");
    level.add_block(972.0, 1115.0 + add, 371.0, 75.0,  "./assets/blue.png");
    level.add_block(1020.0, 1020.0 + add, 100.0, 150.0,  "./assets/blue.png");
    level.add_block(265.0, 816.0 + add, 150.0, 75.0,  "./assets/blue.png");
    
    level.add_goal(1026.0, 939.0, 1, "./assets/grapple.png");
    level.add_goomba(558.0, 616.0);
    level.add_goomba(302.0, 965.0);

    // //  game
    // level.add_goal(49.0,677.0, 1, "./assets/horse.jpg");

    level
}
fn level_river(rl:&mut RaylibHandle, thread:&RaylibThread) -> SwimLevel{
    let mut level:SwimLevel = SwimLevel::new(rl, thread, "./assets/Page9.png");

    let add: f32 = 00.0;

    level.add_block(290.0 - 50.0, 297.0 + add, 480.0 + 100.0, 25.0,  "./assets/blue.png");
    level.add_block(686.0 + 50.0, 459.0 + add, 683.0 + 100.0, 25.0,  "./assets/blue.png");
    // level.add_block(558.0+ 100.0, 889.0 + add, 781.0 + 200.0, 25.0,  "./assets/blue.png");
    level.add_block(517.0, 1077.0 + add, 2100.0, 25.0,  "./assets/blue.png");
    level.add_block(1057.0, 800.0 + add, 25.0, 2100.0,  "./assets/blue.png");
    // level.add_block(1020.0, 1020.0 + add, 160.0, 150.0,  "./assets/blue.png");
    
    //fish
    level.add_goal(795.0, 1034.0, 7, "./assets/grapple.png");
    // level.add_evil(100.0, 300.0, 100.0, 500.0, "./assets/grapple.png");
    level.add_evil(756.0, 889.0, 386.0, 30.0, "./assets/grapple.png");
    level.add_evil(412.0, 606.0, 722.0, 30.0, "./assets/grapple.png");


    level
}
fn level_fish(rl:&mut RaylibHandle, thread:&RaylibThread) -> SwimLevel{
    let mut level:SwimLevel = SwimLevel::new(rl, thread, "./assets/Page10.png");


    level.add_block(59.0, 662.0, 25.0, 830.0,  "./assets/blue.png");
    level.add_block(471.0, 567.0, 25.0, 634.0,  "./assets/blue.png");
    level.add_block(752.0 + 50.0, 567.0, 544.0 + 100.0, 25.0,  "./assets/blue.png");
    level.add_block(573.0, 1107.0, 1043.0, 25.0,  "./assets/blue.png");
    level.add_block(1057.0, 800.0, 25.0, 2000.0,  "./assets/blue.png");
    
    // tuna
    level.add_goal(770.0, 597.0, 8, "./assets/grapple.png");

    level.add_evil(167.0, 407.0 + 10.0, 216.0, 20.0, "./assets/grapple.png");
    level.add_evil(368.0, 597.0 + 10.0, 187.0, 20.0, "./assets/grapple.png");

    level.add_shark(110.0, 837.0, 110.0, 422.0, "./assets/grapple.png");
    level.add_shark(500.0, 837.0, 500.0, 1000.0, "./assets/grapple.png");
    level.add_shark(500.0, 763.0, 500.0, 1000.0, "./assets/grapple.png");

    level
}
fn level_tuna(rl:&mut RaylibHandle, thread:&RaylibThread) -> SwimLevel{
    let mut level:SwimLevel = SwimLevel::new(rl, thread, "./assets/Page12.png");


    level.add_block(1057.0, 800.0, 25.0, 2000.0,  "./assets/blue.png");
    level.add_block(382.0 - 50.0, 418.0, 665.0 + 110.0, 25.0,  "./assets/blue.png");
    level.add_block(642.0, 605.0, 904.0, 25.0,  "./assets/blue.png");
    level.add_block(204.0, 719.0 - 20.0, 25.0, 160.0 + 40.0,  "./assets/blue.png");
    
    // game
    level.add_goal(367.0, 752.0, 1, "./assets/grapple.png");

    level.add_evil(488.0, 524.0, 302.0, 20.0, "./assets/grapple.png");

    level.add_shark(265.0, 811.0, 265.0, 500.0, "./assets/grapple.png");
    level.add_shark(500.0, 855.0, 265.0, 500.0, "./assets/grapple.png");
    level.add_shark(265.0, 900.0, 265.0, 500.0, "./assets/grapple.png");

    level.add_shark(500.0, 811.0, 500.0, 1000.0, "./assets/grapple.png");
    level.add_shark(1000.0, 855.0, 500.0, 1000.0, "./assets/grapple.png");
    level.add_shark(500.0, 900.0, 500.0, 1000.0, "./assets/grapple.png");

    level
}
