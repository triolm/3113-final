use raylib::prelude::*;
use crate::entity::{Entity, Positioned, Sprite};

pub struct Murderer{
    position : Vector2,
    velocity : Vector2,
    acceleration : Vector2,
    collider_dimensions : Vector2,
    colliding_top : bool,
    colliding_bottom : bool,
    colliding_left : bool,
    colliding_right : bool,
    sprite: Sprite,
}

impl Murderer{

    pub fn new(texture: Texture2D, scale: Vector2) -> Murderer{
        let s = Sprite::new(texture,scale);

        Murderer{
            position : Vector2{x:200.0,y:200.0},
            velocity : Vector2{x:0.0,y:0.0},
            acceleration : Vector2{x:0.0,y:0.0},
            collider_dimensions : scale,
            colliding_top : false,
            colliding_bottom : false,
            colliding_left : false,
            colliding_right : false,
            sprite: s,
        }   
    }


    // pub fn jump(&mut self){
    //     if self.is_colliding_bottom() {
    //         let mut vel : Vector2 = *self.get_velocity();
    //         vel.y = -500.0;
    //         self.set_velocity(vel);
    //     }
    // }
}

impl Positioned for Murderer{
    fn get_position(&self) -> &Vector2 { &self.position }
}

impl Entity for Murderer{
    fn set_start_position(&mut self, _pos: Vector2) {}
    fn reset_position(&mut self) {}

    fn update(&mut self, delta_time: f32){
        // self.reset_movement();
        self.reset_collider_flags();
        self.update_velocity(delta_time);
    }

      fn update_position_y(&mut self, delta_time: f32) {
        self.set_position(Vector2{x: self.get_position().x, 
                                  y: self.get_position().y 
                                     + self.get_velocity().y * delta_time
                                }
        );
    }
    fn update_position_x(&mut self, delta_time: f32) {
        self.set_position(Vector2{ x: self.get_position().x 
                                      + self.get_velocity().x * delta_time,
                                   y: self.get_position().y }
        );
    }

    // fn get_position(&self) -> &Vector2 { &self.position }
    fn get_velocity(&self) -> &Vector2 { &self.velocity }
    fn get_acceleration(&self) -> &Vector2 { &self.acceleration }
    fn get_collider_dimensions(&self) -> &Vector2 { &self.collider_dimensions }

    fn get_sprite(&self) -> &Sprite { &self.sprite }

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