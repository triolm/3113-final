use raylib::prelude::*;
use crate::entity::{Entity, Positioned, Sprite};

pub struct Goomba{
    position : Vector2,
    velocity : Vector2,
    movement : Vector2,
    acceleration : Vector2,
    collider_dimensions : Vector2,
    colliding_top : bool,
    colliding_bottom : bool,
    colliding_left : bool,
    colliding_right : bool,
    off_edge_left : bool,
    off_edge_right: bool,
    sprite: Sprite,
    start_position: Vector2,
    jump_countdown: f32,
    frame_counter: f32,
}

impl Goomba{

    pub fn new(texture_path:String, scale: Vector2) -> Goomba{
        let s = Sprite::new(texture_path,scale);

        Goomba{
            position : Vector2{x:200.0,y:200.0},
            velocity : Vector2{x:0.0,y:0.0},
            movement : Vector2{x:0.0,y:0.0},
            acceleration : Vector2{x:0.0,y:0.0},
            collider_dimensions : scale,
            colliding_top : false,
            colliding_bottom : false,
            colliding_left : false,
            colliding_right : false,
            sprite: s,
            start_position : Vector2{x:200.0,y:200.0},
            jump_countdown: 0.0,
            off_edge_left : false,
            off_edge_right: false,
            frame_counter: 0.0,
        }   
    }

    // pub fn reset_movement(&mut self){
    //     self.movement = Vector2{x:0.0, y:0.0};
    // }

    pub fn move_left(&mut self){
        self.movement.x = -100.0;
    }
    pub fn move_right(&mut self){
        self.movement.x = 100.0;
    }
    // pub fn move_up(&mut self){
    //     self.movement.y = -100.0;
    // }
    // pub fn move_down(&mut self){
    //     self.movement.y = 100.0;
    // }

  

//     pub fn jump(&mut self){
//         if self.is_colliding_bottom(){
//             self.jump_countdown = 0.3;
//             let mut vel : Vector2 = *self.get_velocity();
//             vel.y = -800.0;
//             self.set_velocity(vel);
//         }
//         // println!("{}", self.jump_countdown);
//     }
}

impl Positioned for Goomba{
    fn get_position(&self) -> &Vector2 { &self.position }
}

impl Entity for Goomba{

    fn resolve_collision_y(&mut self, other: &impl Entity) {
        if !self.is_colliding(other) {
            return;
        }

        let y_dist: f32 = (self.get_position().y - other.get_position().y).abs();
        let y_overlap: f32 = (y_dist
            - (self.get_collider_dimensions().y / 2.0)
            - (other.get_collider_dimensions().y / 2.0))
            .abs();

        
        if self.total_mvement().y > 0.0 {
            let mut new_pos: Vector2 = *self.get_position();
            new_pos.y -= y_overlap;
            self.set_position(new_pos);
            self.set_colliding_bottom(true);

            if self.get_position().x - self.get_collider_dimensions().x / 2.0 < 
               other.get_position().x - other.get_collider_dimensions().x / 2.0 {
                self.off_edge_left = true;
            }
            
            if self.get_position().x + self.get_collider_dimensions().x / 2.0 > 
            other.get_position().x + other.get_collider_dimensions().x / 2.0 {
                self.off_edge_right = true;
            }


        } else {
            let mut new_pos: Vector2 = *self.get_position();
            new_pos.y += y_overlap;
            self.set_position(new_pos);
            self.set_colliding_top(true);
        }
        
        self.set_velocity(Vector2 { x: self.get_velocity().x, y: 0.0 });
    }

    fn total_mvement(&self) -> Vector2 { *self.get_velocity() + self.movement }

    fn set_start_position(&mut self, pos: Vector2) { self.start_position = pos }

    fn reset_position(&mut self){
        self.velocity = Vector2 { x: 0.0, y: 0.0 };
        self.acceleration = Vector2 { x: 0.0, y: 0.0 };
        self.position = self.start_position;
    }

    fn update(&mut self, delta_time: f32){
        // self.reset_movement();
        self.jump_countdown -= delta_time;

        if self.movement.x > 0.0 && self.movement.y > 0.0 {
            self.movement.x /= 1.4142135;
            self.movement.y /= 1.4142135;
        }

        if self.off_edge_left {
            self.move_right();
            self.off_edge_left = false;
        }
        if self.off_edge_right {
            self.move_left();
            self.off_edge_right = false;
        }

        self.reset_collider_flags();
        self.update_velocity(delta_time);

        self.frame_counter += delta_time;
        if self.frame_counter > 0.3 { 
            self.get_sprite_mut().increment_frame(); 
            self.frame_counter = 0.0;
        }
    }

      fn update_position_y(&mut self, delta_time: f32) {
        self.set_position(Vector2{x: self.get_position().x, 
                                  y: self.get_position().y 
                                     + self.get_velocity().y * delta_time
                                     + self.movement.y * delta_time
                                }
        );
    }
    fn update_position_x(&mut self, delta_time: f32) {
        self.set_position(Vector2{ x: self.get_position().x 
                                      + self.get_velocity().x * delta_time
                                      + self.movement.x * delta_time,
                                   y: self.get_position().y }
        );
    }


    // fn get_position(&self) -> &Vector2 { &self.position }
    fn get_velocity(&self) -> &Vector2 { &self.velocity }
    fn get_acceleration(&self) -> &Vector2 { &self.acceleration }
    fn get_collider_dimensions(&self) -> &Vector2 { &self.collider_dimensions }

    fn get_sprite_mut(&mut self) -> & mut Sprite { &mut self.sprite }
    fn get_sprite(&self) -> &  Sprite { & self.sprite }

    fn is_colliding_top(&self) -> bool { self.colliding_top }
    fn is_colliding_bottom(&self) -> bool { self.colliding_bottom }
    fn is_colliding_left(&self) -> bool { self.colliding_left }
    fn is_colliding_right(&self) -> bool { self.colliding_right }

    fn set_colliding_top(&mut self, val: bool) { self. colliding_top = val}
    fn set_colliding_bottom(&mut self, val: bool) { self. colliding_bottom = val}
    fn set_colliding_left(&mut self, val: bool) { self. colliding_left = val}
    fn set_colliding_right(&mut self, val: bool) { self. colliding_right = val}

    fn set_position(&mut self, pos: Vector2) { self.position = pos }
    fn set_velocity(&mut self, vel: Vector2) { self.velocity = vel }
    fn set_acceleration(&mut self, acc: Vector2) { self.acceleration = acc }
    fn set_collider_dimensions(&mut self, acc: Vector2) { self.collider_dimensions = acc }

}