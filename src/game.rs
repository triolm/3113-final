
use raylib::prelude::*;
use crate::entity::Entity;
use crate::platformer::Platformer;
use crate::grappler::Grappler;


const FPS: u32 = 60;

#[derive(PartialEq)]
enum AppStatus {
    Running,
    Terminated,
}

pub struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
    app_status: AppStatus,
    previous_ticks: f32,

    player: Grappler,
    blocks: Vec<Platformer>,
}

impl Game {
    pub fn new() -> Game {
        let (mut rl, thread) = 
            raylib::init()
                .size(1200, 675)
                .title("Hello, World")
                .build();

            let texture1 = rl.load_texture(&thread, "assets/blue.png").unwrap();
            let mut player = Grappler::new(texture1,Vector2{x:50.0,y:50.00});
            player.set_position(Vector2 { x: 100.0, y: 0.0 });
            
            
        rl.set_target_fps(FPS);

        Game {
            rl,
            thread,
            app_status: AppStatus::Running,
            previous_ticks: 0.0,
            player,
            blocks: vec![],
        }
    }

    pub fn run(&mut self) {

        for i in 0..10{
            let texture2 =self.rl.load_texture(&self.thread, "assets/blue.png").unwrap();
            let mut block1 = Platformer::new(texture2,Vector2{x:10.0,y:10.0});
            block1.set_position(Vector2 { x: 200.0 + 100.0 * i as f32, y: 10.0 + 50.0 * i as f32 });
            self.blocks.push(block1)
        }

        while self.app_status != AppStatus::Terminated {
            self.process_input();
            self.update();
            self.render();
        }
    }

    fn process_input(&mut self){
        // let key = self.rl.get_key_pressed();
        //  self.player.reset_movement();

         if self.rl.is_key_released(KeyboardKey::KEY_SPACE) { self.player.unset_grapple();}
         else if self.rl.is_key_pressed(KeyboardKey::KEY_SPACE) { self.player.grapple_closest(&self.blocks);}
        //  if self.rl.is_key_down(KeyboardKey::KEY_RIGHT) { self.player.move_right();}
        //  if self.rl.is_key_down(KeyboardKey::KEY_LEFT) { self.player.move_left();}
    }

    fn update(&mut self) {
        if self.rl.window_should_close() {
            self.app_status = AppStatus::Terminated;
            return;
        }

        let ticks: f32 =  self.rl.get_time() as f32;
        let delta_time = ticks - self.previous_ticks;
        self.previous_ticks  = ticks;
      
        
        self.player.update(delta_time);

        self.player.update_position(delta_time);
        // self.player.resolve_collision_x(&self.block2);
        // self.player.resolve_collision_x(&self.block);
        
        
        // self.player.update_position_y(delta_time);
        // self.player.resolve_collision_y(&self.block2);
        // self.player.resolve_collision_y(&self.block);

    }


    fn render(&mut self){
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::WHITE);
        // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        self.player.render(&mut d);
        for block in &self.blocks {
            block.render(&mut d);
        }

    }
}
