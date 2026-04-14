
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
    block: Platformer,
    // block2: Platformer
}

impl Game {
    pub fn new() -> Game {
        let (mut rl, thread) = 
            raylib::init()
                .size(640, 480)
                .title("Hello, World")
                .build();

        let texture1 = rl.load_texture(&thread, "assets/blue.png").unwrap();
        let mut block = Platformer::new(texture1,Vector2{x:10.0,y:10.0});
        block.set_position(Vector2 { x: 300.0, y: 10.0 });
        
        // let texture3 = rl.load_texture(&thread, "assets/blue.png").unwrap();
        // let mut block2 = Platformer::new(texture3,Vector2{x:10.0,y:10.0});
        // block2.set_position(Vector2 { x: 100.0, y: 10.0 });
        
        let texture2 = rl.load_texture(&thread, "assets/blue.png").unwrap();
        let mut player = Grappler::new(texture2,Vector2{x:50.0,y:50.00});
        player.set_acceleration(Vector2 { x: 0.0, y:2000.0 });

        rl.set_target_fps(FPS);

        Game {
            rl,
            thread,
            app_status: AppStatus::Running,
            previous_ticks: 0.0,
            player,
            block, 
            // block2, 
        }
    }

    pub fn run(&mut self) {
        while self.app_status != AppStatus::Terminated {
            self.process_input();
            self.update();
            self.render();
        }
    }

    fn process_input(&mut self){
        // let key = self.rl.get_key_pressed();
        //  self.player.reset_movement();

         if self.rl.is_key_pressed(KeyboardKey::KEY_UP) && self.player.is_grappling() { self.player.unset_grapple();}
         else if self.rl.is_key_pressed(KeyboardKey::KEY_UP) { self.player.set_grapple(&self.block);}
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

        self.player.update_position_x(delta_time);
        // self.player.resolve_collision_x(&self.block2);
        self.player.resolve_collision_x(&self.block);
        
        
        self.player.update_position_y(delta_time);
        // self.player.resolve_collision_y(&self.block2);
        self.player.resolve_collision_y(&self.block);

    }


    fn render(&mut self){
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        self.player.render(&mut d);
        self.block.render(&mut d);
        // self.block2.render(&mut d);

    }
}
