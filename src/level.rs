
use raylib::prelude::*;
use crate::entity::{Entity, Positioned};
use crate::platformer::Platformer;
use crate::grappler::Grappler;
use crate::goal::Goal;
use crate::scene::{Scene, AppStatus};
use crate::murderer::Murderer;

const SCALE:f32 = 1.2;

pub struct Level {
    app_status: AppStatus,
    previous_ticks: f32,
    player: Grappler,
    bg: Platformer,
    blocks: Vec<Platformer>,
    evils: Vec<Murderer>,
    goals: Vec<Goal>,
    next: i32,
    camera: Camera2D
}

impl Level{
    pub fn add_block(&mut self,  rl:&mut RaylibHandle, thread:&RaylibThread, x:f32, y:f32, texture_path:&str){
        let texture = rl.load_texture(thread, &texture_path).unwrap();
        let mut block = Platformer::new(texture,Vector2{x:10.0,y:10.0});
        block.set_position(Vector2 { x,y });
        self.blocks.push(block);
    }

    pub fn add_goal(&mut self, rl:&mut RaylibHandle, thread:&RaylibThread, x:f32, y:f32, next:u32, texture_path:&str){
        let texture =rl.load_texture(&thread, &texture_path).unwrap();
        let mut goal = Goal::new(texture,Vector2{x:50.0,y:50.0});
        goal.set_position(Vector2 { x,y });
        goal.set_next(next);
        self.goals.push(goal);
    }

    pub fn add_evil(&mut self, rl:&mut RaylibHandle, thread:&RaylibThread, x:f32, y:f32, w:f32, h:f32, texture_path:&str){
        let texture =rl.load_texture(&thread, &texture_path).unwrap();
        let mut evil = Murderer::new(texture,Vector2{x:w, y:h});
        evil.set_position(Vector2 { x,y });
        self.evils.push(evil);
    }

    pub fn new(rl:&mut RaylibHandle, thread:&RaylibThread, bg_path:&str) -> Level {

        let texture1 = rl.load_texture(&thread, "assets/blue.png").unwrap();
        let mut player = Grappler::new(texture1,Vector2{x:20.0,y:20.00});
        player.set_start_position(Vector2 { x: 100.0, y: 100.0 });

        let texture2 = rl.load_texture(&thread, bg_path).unwrap();
        let mut bg = Platformer::new(texture2,Vector2{x:1600.00,y:1600.00});
        bg.set_position(Vector2 { x: 800.0, y: 800.0 });

        let camera = Camera2D{
            offset: Vector2{x:1200.0/2.0, y:675.0/2.0},
            target:*player.get_position(),
            rotation:0.0,
            zoom:SCALE
        };

        Level {
            app_status: AppStatus::Running,
            previous_ticks: 0.0,
            player,
            blocks: vec![],
            goals: vec![],
            evils: vec![],
            next: -1,
            camera,
            bg
        }
    }

    pub fn init(&mut self, rl:&RaylibHandle) {
        self.next = -1;
        self.previous_ticks = rl.get_time() as f32;
        self.player.reset_position();
        self.camera.target = *self.player.get_position()
    }


}
impl Scene for Level {

    fn get_status(&self) -> AppStatus{
        return self.app_status;
    }

    fn get_next(&self) -> i32 {
        self.next
    }

    fn process_input(&mut self, rl:&RaylibHandle){
        // let key = self.rl.get_key_pressed();
        //  self.player.reset_movement();

         if rl.is_key_released(KeyboardKey::KEY_SPACE) { self.player.unset_grapple();}
         if rl.is_key_released(KeyboardKey::KEY_R) { self.init(rl);}
         else if rl.is_key_pressed(KeyboardKey::KEY_SPACE) { self.player.grapple_closest(&self.blocks);}
        //  if self.rl.is_key_down(KeyboardKey::KEY_RIGHT) { self.player.move_right();}
        //  if self.rl.is_key_down(KeyboardKey::KEY_LEFT) { self.player.move_left();}
    }

    fn update(&mut self, rl:&RaylibHandle) {
        if rl.window_should_close() {
            self.app_status = AppStatus::Terminated;
            return;
        }

        let ticks: f32 =  rl.get_time() as f32;
        let delta_time = ticks - self.previous_ticks;
        self.previous_ticks  = ticks;
      
        
        self.player.update(delta_time);

        self.player.update_position(delta_time);

        if self.player.get_position().x > 1600.0 + 400.0 ||
           self.player.get_position().x < 0.0 - 400.0 ||
           //    self.player.get_position().y < 0.0 ||
           self.player.get_position().y > 1600.0 + 400.0 {
            self.init(rl);
            return;
        }

        for goal in &self.goals{
            if self.player.is_colliding(goal) {
                self.next = goal.get_next() as i32;
            }
        }

        let mut is_dead:bool = false;
        for evil in &self.evils{
            if self.player.is_colliding(evil) {
                is_dead = true;
            }
        }
        if is_dead {
            self.init(rl);
        }

        // self.player.resolve_collision_x(&self.block2);
        // self.player.resolve_collision_x(&self.block);
        
        
        // self.player.update_position_y(delta_time);
        // self.player.resolve_collision_y(&self.block2);
        // self.player.resolve_collision_y(&self.block);

    }


    fn render(&mut self, rl:&mut RaylibHandle, thread:&RaylibThread){
        let mut d = rl.begin_drawing(thread);

        d.clear_background(Color::WHITE);
        self.camera.target = Vector2 { 
            x: 
            (self.player.get_position().x*0.04 + self.camera.target.x*0.96), 
            y: 
            (self.player.get_position().y*0.04+ self.camera.target.y*0.96)
        };

        if self.camera.target.x > 1600.0 - (1200.0/2.0)/SCALE { self.camera.target.x = 1600.0 - (1200.0/2.0)/SCALE }
        if self.camera.target.x < (1200.0/2.0)/SCALE { self.camera.target.x = (1200.0/2.0)/SCALE}

        if self.camera.target.y > 1600.0 - (675.0/2.0)/SCALE { self.camera.target.y = 1600.0 - (675.0/2.0)/SCALE }
        if self.camera.target.y < (675.0/2.0)/SCALE { self.camera.target.y = (675.0/2.0)/SCALE }

        // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        {
            let mut d_cam = d.begin_mode2D(self.camera);

            self.bg.render(&mut d_cam);

            self.player.render(&mut d_cam);

            for block in &self.blocks {
                block.render(&mut d_cam);
            }

            for goal in &self.goals {
                goal.render(&mut d_cam);
            }

            for evil in &self.evils {
                evil.render(&mut d_cam);
            }
        }
    }
}
