use raylib::prelude::*;
use crate::entity::{Entity, Sprite, Positioned};

pub struct Goal{
    position : Vector2,
    velocity : Vector2,
    acceleration : Vector2,
    collider_dimensions : Vector2,
    colliding_top : bool,
    colliding_bottom : bool,
    colliding_left : bool,
    colliding_right : bool,
    sprite: Sprite,
    next_scene_index: u32,
}

impl Goal{

    pub fn new(texture: String, scale: Vector2) -> Goal{
        let s = Sprite::new(texture,scale);

        Goal{
            position : Vector2{x:200.0,y:200.0},
            velocity : Vector2{x:0.0,y:0.0},
            acceleration : Vector2{x:0.0,y:0.0},
            collider_dimensions : scale,
            colliding_top : false,
            colliding_bottom : false,
            colliding_left : false,
            colliding_right : false,
            sprite: s,
            next_scene_index: 0
        }   
    }
    // ngl i don't know what static does
    // but the compiler suggested i do this and it worked
    // so W compiler ig
    pub fn set_next(&mut self, scene_index:u32) {
        self.next_scene_index = scene_index
    }

    pub fn get_next(&self) -> u32 {
        self.next_scene_index
    }


    pub fn update_position(&mut self, _delta_time: f32) { }

}

impl Positioned for Goal{
    fn get_position(&self) -> &Vector2 { &self.position }
}

impl Entity for Goal{
    fn set_start_position(&mut self, _pos: Vector2) {}
    fn reset_position(&mut self) {}

    fn update(&mut self, _delta_time: f32){ }

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