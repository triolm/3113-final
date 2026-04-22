
use raylib::prelude::*;
use crate::entity::{Entity, Positioned};
use crate::platformer::Platformer;
use crate::swimmer::Swimmer;
use crate::goal::Goal;
use crate::shark::Shark;
use crate::scene::{Scene, AppStatus};
use crate::murderer::Murderer;

const SCALE:f32 = 1.2;

pub struct SwimLevel {
    app_status: AppStatus,
    previous_ticks: f32,
    player: Swimmer,
    bg: Platformer,
    blocks: Vec<Platformer>,
    sharks: Vec<Shark>,
    evils: Vec<Platformer>,
    goals: Vec<Goal>,
    next: i32,
    camera: Camera2D,
    screen_shake: f32,
    shader: Shader,
    light_pos_loc:i32,
    time_loc:i32,
    screen_shake_v:Vector2,

}

impl SwimLevel{
    pub fn add_block(&mut self, x:f32, y:f32, w:f32, h:f32, texture_path:&str){
        let mut block = Platformer::new(texture_path.to_string(),Vector2{x:w,y:h});
        block.set_position(Vector2 { x,y });
        self.blocks.push(block);
    }

    pub fn add_goal(&mut self, x:f32, y:f32, next:u32, texture_path:&str){
        let mut goal = Goal::new(texture_path.to_string(),Vector2{x:50.0,y:50.0});
        goal.set_position(Vector2 { x,y });
        goal.set_next(next);
        self.goals.push(goal);
    }

    pub fn add_shark(&mut self, x:f32, y:f32, start:f32, end:f32, texture_path:&str){
        let mut shark = Shark::new(texture_path.to_string(),Vector2{x:20.0, y:20.0},start, end);
        shark.set_start_position(Vector2 { x,y });
        shark.set_position(Vector2 { x,y });
        self.sharks.push(shark);
    }
    pub fn add_evil(&mut self, x:f32, y:f32, start:f32, end:f32, texture_path:&str){
        let mut evil = Platformer::new(texture_path.to_string(),Vector2{x:start, y:end});
        evil.set_start_position(Vector2 { x,y });
        evil.set_position(Vector2 { x,y });
        self.evils.push(evil);
    }

    pub fn new(rl:&mut RaylibHandle, thread:&RaylibThread, bg_path:&str) -> SwimLevel {

        let mut player = Swimmer::new("assets/blue.png".to_string(),Vector2{x:20.0,y:20.00});
        player.set_start_position(Vector2 { x: 100.0, y: 100.0 });
        player.set_position(Vector2 { x: 100.0, y: 100.0 });


        let mut bg = Platformer::new(bg_path.to_string(),Vector2{x:1600.00,y:1600.00});
        bg.set_position(Vector2 { x: 800.0, y: 800.0 });

        let camera = Camera2D{
            offset: Vector2{x:1200.0/2.0, y:675.0/2.0},
            target:*player.get_position(),
            rotation:0.0,
            zoom:SCALE
        };

        let shader = rl.load_shader(&thread, Some("shaders/vertex.glsl"), Some("shaders/fragment.glsl"));
        let light_pos_loc = shader.get_shader_location("lightPosition");
        let time_loc = shader.get_shader_location("time");

        SwimLevel {
            app_status: AppStatus::Running,
            previous_ticks: 0.0,
            player,
            blocks: vec![],
            goals: vec![],
            evils: vec![],
            sharks: vec![],
            next: -1,
            camera,
            bg,
            screen_shake: 0.0,
            shader,
            light_pos_loc,
            time_loc,
            screen_shake_v: Vector2 { x: 0.0, y: 0.0 }, 

        }
    }

    
    
}
impl Scene for SwimLevel {
    fn init(&mut self, rl:&RaylibHandle) {
        self.next = -1;
        self.previous_ticks = rl.get_time() as f32;
        self.player.reset_position();
        self.player.set_acceleration(Vector2 { x: 0.0, y: 80.0 });
        self.camera.target = *self.player.get_position();
        for shark in &mut self.sharks {
            shark.move_left();
        }
    }



    fn load(&mut self, rl:&mut RaylibHandle, thread:&RaylibThread){
        self.player.load(rl, thread);
        self.bg.load(rl, thread);


        for block in &mut self.blocks {
            block.load(rl, thread);
        }

        for shark in &mut self.sharks {
            shark.load(rl, thread);
        }

        for goal in &mut self.goals {
            goal.load(rl, thread);
        }

        for evil in &mut self.evils {
            evil.load(rl, thread);
        }
    }
    
    fn get_status(&self) -> AppStatus{
        return self.app_status;
    }

    fn get_next(&self) -> i32 {
        self.next
    }

    fn process_input(&mut self, rl:&RaylibHandle){
        // let key = self.rl.get_key_pressed();
        //  self.player.reset_movement();

         if rl.is_key_down(KeyboardKey::KEY_A) { self.player.move_left();}
         if rl.is_key_down(KeyboardKey::KEY_D) { self.player.move_right();}
         if rl.is_key_down(KeyboardKey::KEY_W) { self.player.move_up();}
         if rl.is_key_down(KeyboardKey::KEY_S) { self.player.move_down();}
        //  if rl.is_key_down(KeyboardKey::KEY_DOWN) { self.player.move_down();}
         if rl.is_key_down(KeyboardKey::KEY_R) { self.init(rl);}
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

        self.player.update_position_x(delta_time);
        for block in &self.blocks{
            self.player.resolve_collision_x(block);
        }

        self.player.update_position_y(delta_time);
        for block in &self.blocks{
            self.player.resolve_collision_y(block);
        }
        

        self.player.reset_movement();
        

        if self.player.get_position().x > 1600.0 + 000.0 ||
           self.player.get_position().x < 0.0 - 000.0 ||
           //    self.player.get_position().y < 0.0 ||
           self.player.get_position().y > 1600.0 + 000.0 {
            self.init(rl);
            self.screen_shake = 0.4;
            return;
        }

        for goal in &self.goals{
            if self.player.is_colliding(goal) {
                self.next = goal.get_next() as i32;
            }
        }

        let mut is_dead:bool = false;
        for shark in &mut self.sharks{
            shark.update(delta_time);
            shark.update_position_x(delta_time);
            shark.update_position_y(delta_time);
            if self.player.is_colliding(shark) {
                is_dead = true;
            }
        }

        for evil in &mut self.evils{
            if self.player.is_colliding(evil) {
                is_dead = true;
            }
        }
        if is_dead {
            self.init(rl);
            self.screen_shake = 0.4;
        }

        if self.screen_shake > 0.0 {
            self.screen_shake -= delta_time;
            self.screen_shake_v.x = rl.get_random_value::<i32>(-100..100) as f32 / 20.0 * (self.screen_shake / 0.4);
            self.screen_shake_v.y = rl.get_random_value::<i32>(-100..100) as f32 / 20.0 * (self.screen_shake / 0.4);
        }

        self.shader.set_shader_value(self.light_pos_loc, *self.player.get_position());
        self.shader.set_shader_value(self.time_loc, ticks);


    }


    fn render(&mut self,d:&mut RaylibDrawHandle){

        // let mut d = rl.begin_drawing(thread);


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

        self.camera.target.x += self.screen_shake_v.x;
        self.camera.target.y += self.screen_shake_v.y;


        {
            let mut d_cam = d.begin_mode2D(self.camera);
            let mut sd = d_cam.begin_shader_mode(&mut self.shader);

            self.bg.render(&mut sd);

            self.player.render(&mut sd);

            // for block in &self.blocks {
            //     block.render(&mut sd);
            // }

            // for goal in &self.goals {
            //     goal.render(&mut sd);
            // }

            // for evil in &self.evils {
            //     evil.render(&mut sd);
            // }

            for shark in &self.sharks {
                shark.render(&mut sd);
            }
        }
    }
}
