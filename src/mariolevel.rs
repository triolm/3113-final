
use raylib::prelude::*;
use crate::entity::{Entity, Positioned};
use crate::platformer::Platformer;
use crate::goomba::{self, Goomba};
use crate::goal::Goal;
use crate::scene::{Scene, AppStatus};
use crate::murderer::Murderer;

const SCALE:f32 = 1.2;

pub struct MarioLevel {
    app_status: AppStatus,
    previous_ticks: f32,
    player: Platformer,
    bg: Platformer,
    goombas: Vec<Goomba>,
    blocks: Vec<Platformer>,
    goals: Vec<Goal>,
    next: i32,
    camera: Camera2D,
    screen_shake: f32,
}

impl MarioLevel{
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

    pub fn add_goomba(&mut self, x:f32, y:f32){
        let mut evil = Goomba::new("assets/goomba.png".to_string(),Vector2{x:75.0, y:75.0});
        evil.set_start_position(Vector2 { x,y });
        evil.get_sprite_mut().set_sprite_sheet_cols(2);
        self.goombas.push(evil);
    }

    pub fn new(bg_path:&str) -> MarioLevel {

        let mut player = Platformer::new("assets/blue.png".to_string(),Vector2{x:20.0,y:20.00});
        player.set_start_position(Vector2 { x: 100.0, y: 100.0 });
        // player.set_acceleration(Vector2 { x: 0.0, y: 400.0 });


        let mut bg = Platformer::new(bg_path.to_string(),Vector2{x:1600.00,y:1600.00});
        bg.set_position(Vector2 { x: 800.0, y: 800.0 });

        let camera = Camera2D{
            offset: Vector2{x:1200.0/2.0, y:675.0/2.0},
            target:*player.get_position(),
            rotation:0.0,
            zoom:SCALE
        };

     

        MarioLevel {
            app_status: AppStatus::Running,
            previous_ticks: 0.0,
            player,
            blocks: vec![],
            goals: vec![],
            goombas: vec![],
            next: -1,
            camera,
            bg,
            screen_shake: 0.0,
        }
    }

    
    
}
impl Scene for MarioLevel {
    fn init(&mut self, rl:&RaylibHandle) {
        self.next = -1;
        self.previous_ticks = rl.get_time() as f32;
        self.player.reset_position();
        self.player.set_acceleration(Vector2 { x: 0.0, y: 2000.0 });

        for goomba in &mut self.goombas {
            goomba.reset_position();
            goomba.set_acceleration(Vector2 { x: 0.0, y: 2000.0 });
            goomba.move_right();
        }

        self.camera.target = *self.player.get_position()
    }

    fn load(&mut self, rl:&mut RaylibHandle, thread:&RaylibThread){
        self.player.load(rl, thread);
        self.bg.load(rl, thread);

        for block in &mut self.blocks {
            block.load(rl, thread);
        }

        for goal in &mut self.goals {
            goal.load(rl, thread);
        }

        for goomba in &mut self.goombas {
            goomba.load(rl, thread);
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

         if rl.is_key_down(KeyboardKey::KEY_LEFT) { self.player.move_left();}
         if rl.is_key_down(KeyboardKey::KEY_RIGHT) { self.player.move_right();}
         if rl.is_key_down(KeyboardKey::KEY_UP) { self.player.jump();}
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
        
        
        // println!("{} {}", self.goomba.get_position().x, self.goomba.get_position().y);
        for goomba in &mut self.goombas {
            goomba.update(delta_time);
            goomba.update_position_x(delta_time);

            for block in &self.blocks{
                goomba.resolve_collision_x(block);
                
            }

            goomba.update_position_y(delta_time);
            for block in &self.blocks{
                goomba.resolve_collision_y(block);
            }
        }
        
        
        self.player.reset_movement();
        

        if self.player.get_position().x > 1600.0 + 400.0 ||
           self.player.get_position().x < 0.0 - 400.0 ||
           //    self.player.get_position().y < 0.0 ||
           self.player.get_position().y > 1600.0 + 400.0 {
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
        for goomba in &self.goombas{
            if self.player.is_colliding(goomba) {
                is_dead = true;
            }
        }
        if is_dead {
            self.init(rl);
            self.screen_shake = 0.4;
        }

        if self.screen_shake > 0.0 {
            self.screen_shake -= delta_time
        }
    }


    fn render(&mut self, rl:&mut RaylibHandle, thread:&RaylibThread){

        let mut x_add =  0.0;
        let mut y_add =  0.0; 
        if self.screen_shake > 0.0 {
            x_add = rl.get_random_value::<i32>(-100..100) as f32 / 20.0 * (self.screen_shake / 0.4);
            y_add = rl.get_random_value::<i32>(-100..100) as f32 / 20.0 * (self.screen_shake / 0.4);
        }


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

        self.camera.target.x += x_add;
        self.camera.target.y += y_add;


        {
            let mut d_cam = d.begin_mode2D(self.camera);

            self.bg.render(&mut d_cam);

            self.player.render(&mut d_cam);

            // self.goomba.render(&mut d_cam);

            // for block in &self.blocks {
            //     block.render(&mut d_cam);
            // }

            // for goal in &self.goals {
            //     goal.render(&mut d_cam);
            // }

            for goomba in &self.goombas {
                goomba.render(&mut d_cam);
            }
        }
    }
}
