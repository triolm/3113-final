use raylib::prelude::*;
use crate::entity::{Entity, Pointable, Sprite};

pub struct Grappler{
    position : Vector2,
    velocity : Vector2,
    acceleration : Vector2,
    collider_dimensions : Vector2,
    colliding_top : bool,
    colliding_bottom : bool,
    colliding_left : bool,
    colliding_right : bool,
    sprite: Sprite,
    gravity: f32,
    grappled_to: Option<*const dyn Pointable>,
}

impl Grappler{

    pub fn new(texture: Texture2D, scale: Vector2) -> Grappler{
        let s = Sprite::new(texture,scale);

        Grappler{
            position : Vector2{x:200.0,y:200.0},
            velocity : Vector2{x:0.0,y:0.0},
            acceleration : Vector2{x:0.0,y:0.0},
            collider_dimensions : scale,
            colliding_top : false,
            colliding_bottom : false,
            colliding_left : false,
            colliding_right : false,
            sprite: s,
            gravity: 2000.0,
            grappled_to: None
        }   
    }

    pub fn is_grappling(&self) -> bool {
        self.grappled_to.is_some()
    }

    // ngl i don't know what static does
    // but the compiler suggested i do this and it worked
    // so W compiler ig
    pub fn set_grapple(&mut self, target: &(impl Pointable + 'static)) {
        self.grappled_to = Some(target as *const dyn Pointable);
    }

    pub fn unset_grapple(&mut self) {
        self.grappled_to = None;
    }

}

impl Pointable for Grappler{
    fn get_position(&self) -> &Vector2 { &self.position }
}

impl Entity for Grappler{

    fn update(&mut self, delta_time: f32){
        if(self.is_grappling()){
            // i'm not fully sure why i have to do this
            if let Some(ptr) = self.grappled_to {
                let target_pos = unsafe { (*ptr).get_position() };

                let x_diff = self.get_position().x - target_pos.x;
                let y_diff = self.get_position().y - target_pos.y;
                
                // ngl i don't know what was wrong with normal tan. 
                let theta = y_diff.atan2(x_diff);

                self.set_acceleration(Vector2 { 
                                        x: self.gravity * theta.cos() * -theta.sin(), 
                                        y: self.gravity * theta.cos() * theta.cos(), 
                                     });
            
            
                let u = Vector2{ x:-y_diff, y:x_diff}.normalized();
                let dot_prod = u.x * self.get_velocity().x + u.y * self.get_velocity().y;
                self.set_velocity(Vector2 { x: u.x * dot_prod, y: u.y * dot_prod });
            
            }        
        } else {
            self.acceleration = Vector2 { x: 0.0, y: self.gravity };
        }

        self.reset_collider_flags();
        self.update_velocity(delta_time);
    }

     fn update_position_x(&mut self, delta_time: f32) {

        // if self.is_grappling() {
        //     return;
        // }

        self.set_position(Vector2 { 
            x: self.get_position().x 
               + self.get_velocity().x * delta_time ,
            y: self.get_position().y 
        });
    }

    fn update_position_y(&mut self, delta_time: f32) {

        // if self.is_grappling() {
        //     return;
        // }

        self.set_position(Vector2 {
            x: self.get_position().x, 
            y: self.get_position().y 
               + self.get_velocity().y * delta_time
        });
    }


    fn get_velocity(&self) -> &Vector2 { &self.velocity }
    fn get_acceleration(&self) -> &Vector2 { &self.acceleration }
    fn get_collider_dimensions(&self) -> &Vector2 { &self.collider_dimensions }

    fn get_sprite(&self) -> & Sprite { &self.sprite }

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