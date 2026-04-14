use raylib::prelude::*;

pub struct Sprite{
    texture: Texture2D,
    scale: Vector2,
    animation_index: i32,
    sprite_sheet_cols: i32,
    sprite_sheet_rows: i32
}

impl Sprite{
    pub fn new(texture: Texture2D, scale: Vector2) -> Sprite{

        Sprite{
            texture: texture,
            scale: scale,
            animation_index: 0,
            sprite_sheet_cols: 1,
            sprite_sheet_rows: 1
        }
    }

    fn get_texture_area(&self) -> Rectangle{
        let u_coord:f32  = (((self.animation_index % self.sprite_sheet_cols) 
                            / self.sprite_sheet_cols) * self.texture.width) as f32;

        let v_coord:f32  = (((self.animation_index / self.sprite_sheet_cols) 
                            / self.sprite_sheet_rows) * self.texture.height) as f32;

        let slice_width: f32  = (self.texture.width / self.sprite_sheet_cols) as f32;
        let slice_height: f32 = (self.texture.height / self.sprite_sheet_rows) as f32;

        return Rectangle::new( 
            u_coord,     // top-left x-coord
            v_coord,     // top-left y-coord
            slice_width, // width of slice
            slice_height // height of slice
        );
    }

    fn get_scale(&self) -> &Vector2{ &self.scale }
    fn get_texture(&self) -> &Texture2D { &self.texture }
}


pub trait Pointable {
    fn get_position(&self) -> &Vector2;
}

pub trait Entity: Pointable {
    // fn get_position(&self) -> &Vector2;
    fn get_velocity(&self) -> &Vector2;
    fn get_acceleration(&self) -> &Vector2;
    fn get_collider_dimensions(&self) -> &Vector2;
    fn is_active(&self) -> bool {
        true
    }

    fn get_sprite(&self) -> & Sprite;

    fn is_colliding_top(&self) -> bool;
    fn is_colliding_bottom(&self) -> bool;
    fn is_colliding_left(&self) -> bool;
    fn is_colliding_right(&self) -> bool;

    fn set_colliding_top(&mut self, val: bool);
    fn set_colliding_bottom(&mut self, val: bool);
    fn set_colliding_left(&mut self, val: bool);
    fn set_colliding_right(&mut self, val: bool);

    fn reset_collider_flags(&mut self) {
        self.set_colliding_top(false);
        self.set_colliding_bottom(false);
        self.set_colliding_left(false);
        self.set_colliding_right(false);
    }

    fn update(&mut self, delta_time: f32);

    fn set_position(&mut self, pos: Vector2);
    fn set_velocity(&mut self, vel: Vector2);
    fn set_acceleration(&mut self, acc: Vector2);
    fn set_collider_dimensions(&mut self, acc: Vector2);

    fn update_velocity(&mut self, delta_time: f32) {
        self.set_velocity(*self.get_velocity() + *self.get_acceleration() * delta_time);
    }

    fn update_position_y(&mut self, delta_time: f32) {
        self.set_position(Vector2{x: self.get_position().x, 
                                  y: self.get_position().y + self.get_velocity().y * delta_time}
        );
    }
    fn update_position_x(&mut self, delta_time: f32) {
        self.set_position(Vector2{ x: self.get_position().x + self.get_velocity().x * delta_time,
                                   y: self.get_position().y }
        );
    }

    fn is_colliding(&self, other: &impl Entity) -> bool {
        if !other.is_active() {
            return false;
        }

        let x_dist: f32 = (self.get_position().x - other.get_position().x).abs()
            - ((self.get_collider_dimensions().x + other.get_collider_dimensions().x) / 2.0);

        let y_dist: f32 = (self.get_position().y - other.get_position().y).abs()
            - ((self.get_collider_dimensions().y + other.get_collider_dimensions().y) / 2.0);

        // println!("{}", x_dist);

        x_dist < 0.0 && y_dist < 0.0
    }

    fn resolve_collision_y(&mut self, other: &impl Entity) {
        if !self.is_colliding(other) {
            return;
        }

        let y_dist: f32 = (self.get_position().y - other.get_position().y).abs();
        let y_overlap: f32 = (y_dist
            - (self.get_collider_dimensions().y / 2.0)
            - (other.get_collider_dimensions().y / 2.0))
            .abs();

        if self.get_velocity().y > 0.0 {
            let mut new_pos: Vector2 = *self.get_position();
            new_pos.y -= y_overlap;
            self.set_position(new_pos);
            self.set_colliding_bottom(true);
        } else {
            let mut new_pos: Vector2 = *self.get_position();
            new_pos.y += y_overlap;
            self.set_position(new_pos);
            self.set_colliding_top(true);
        }
        
        self.set_velocity(Vector2 { x: self.get_velocity().x, y: 0.0 });
    }

    fn resolve_collision_x(&mut self, other: &impl Entity) {
        if !self.is_colliding(other) {
            return;
        }

        let x_dist: f32 = (self.get_position().x - other.get_position().x).abs();
        let x_overlap: f32 = (x_dist
            - (self.get_collider_dimensions().x / 2.0)
            - (other.get_collider_dimensions().x / 2.0))
            .abs();

        if self.get_velocity().x > 0.0 {
            let mut new_pos = *self.get_position();
            new_pos.x -= x_overlap;
            self.set_position(new_pos);
            self.set_colliding_right(true);
        } else {
            let mut new_pos = *self.get_position();
            new_pos.x += x_overlap;
            self.set_position(new_pos);
            self.set_colliding_left(true);
        }

        self.set_velocity(Vector2 { x: 0.0, y: self.get_velocity().y });
    }

    // fn resolve_collision(&mut self, other: &impl Entity){
    //     self.resolve_collision_y(other);
    //     self.resolve_collision_x(other);
    // }

    fn render(&self, draw: &mut RaylibDrawHandle){
         if !self.is_active() { return; }
        
        //will be centered on position
        let destination_area = Rectangle::new(
            self.get_position().x,
            self.get_position().y,
            self.get_sprite().get_scale().x,
            self.get_sprite().get_scale().y
        );

        // Origin inside the source texture (centre of the texture)
        let origin_offset = Vector2{
            x:self.get_sprite().get_scale().x / 2.0,
            y:self.get_sprite().get_scale().y / 2.0
        };

        // Render the texture on screen
        draw.draw_texture_pro(
            self.get_sprite().get_texture(), 
            self.get_sprite().get_texture_area(), destination_area, origin_offset,
            0.0, Color::WHITE
        );

    // displayCollider();
    }
}