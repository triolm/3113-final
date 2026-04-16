
use raylib::prelude::*;
use crate::entity::Entity;
use crate::platformer::Platformer;
use crate::grappler::Grappler;
use crate::goal::Goal;
use crate::scene::{Scene, AppStatus};

pub struct Level {
    app_status: AppStatus,
    previous_ticks: f32,
    player: Grappler,
    blocks: Vec<Platformer>,
    goals: Vec<Goal>,
    next: i32
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

    pub fn new(rl:&mut RaylibHandle, thread:&RaylibThread) -> Level {

        let texture1 = rl.load_texture(&thread, "assets/blue.png").unwrap();
        let mut player = Grappler::new(texture1,Vector2{x:50.0,y:50.00});
        player.set_position(Vector2 { x: 100.0, y: 0.0 });

        Level {
            app_status: AppStatus::Running,
            previous_ticks: 0.0,
            player,
            blocks: vec![],
            goals: vec![],
            next: -1
        }
    }

    pub fn init(&mut self, rl:&RaylibHandle) {
        self.next = -1;
        self.previous_ticks = rl.get_time() as f32;
        self.player.set_position(Vector2 { x: 100.0, y: 0.0 });
        self.player.set_velocity(Vector2 { x: 0.0, y: 0.0 });
        self.player.set_acceleration(Vector2 { x: 0.0, y: 0.0 });
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

        for goal in &self.goals{
            if self.player.is_colliding(goal) {
                self.next = goal.get_next() as i32;
            }
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
        // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        self.player.render(&mut d);
        for block in &self.blocks {
            block.render(&mut d);
        }

        for goal in &self.goals {
            goal.render(&mut d);
        }

    }
}
