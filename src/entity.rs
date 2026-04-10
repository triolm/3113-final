use raylib::prelude::*;

pub trait Sprite{
    fn get_texture_area(&self) -> Rectangle;
    fn get_scale(&self) -> &Vector2;
    fn get_texture(&self) -> &Texture2D;
}

pub trait Entity {
    fn get_position(&self) -> &Vector2;
    fn get_velocity(&self) -> &Vector2;
    fn get_acceleration(&self) -> &Vector2;
    fn get_collider_dimensions(&self) -> &Vector2;
    fn is_active(&self) -> bool {
        true
    }

    fn get_sprite(&self) -> &impl Sprite;

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

    fn set_position(&mut self, pos: Vector2);
    fn set_velocity(&mut self, vel: Vector2);
    fn set_acceleration(&mut self, acc: Vector2);
    fn set_collider_dimensions(&mut self, acc: Vector2);

    fn update_velocity(&mut self) {
        self.set_velocity(*self.get_velocity() + *self.get_acceleration());
    }

    fn update_position(&mut self) {
        self.set_position(*self.get_position() + *self.get_velocity());
    }

    fn is_colliding(&self, other: &impl Entity) -> bool {
        if !other.is_active() {
            return false;
        }

        let x_dist: f32 = (self.get_position().x - other.get_position().x).abs()
            - ((self.get_collider_dimensions().x + other.get_collider_dimensions().x) / 2.0);

        let y_dist: f32 = (self.get_position().y - other.get_position().y).abs()
            - ((self.get_collider_dimensions().y + other.get_collider_dimensions().y) / 2.0);

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
            self.set_position(*self.get_position() - y_overlap);
            self.set_colliding_bottom(true);
        } else if self.get_velocity().y < 0.0 {
            self.set_position(*self.get_position() + y_overlap);
            self.set_colliding_top(true);
        }

        self.set_velocity(Vector2 { x: 0.0, y: 0.0 });
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
        } else if self.get_velocity().x < 0.0 {
            let mut new_pos = *self.get_position();
            new_pos.x += x_overlap;
            self.set_position(new_pos);
            self.set_colliding_left(true);
        }

        self.set_velocity(Vector2 { x: 0.0, y: 0.0 });
    }

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


pub struct PlayerSprite{
    texture: Texture2D,
    scale: Vector2,
    animation_index: i32,
    sprite_sheet_cols: i32,
    sprite_sheet_rows: i32
}
impl PlayerSprite{
    pub fn new(texture: Texture2D, scale: Vector2) -> PlayerSprite{

        PlayerSprite{
            texture: texture,
            scale: scale,
            animation_index: 0,
            sprite_sheet_cols: 1,
            sprite_sheet_rows: 1
        }
    }
}


impl Sprite for PlayerSprite{

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
pub struct Player{
    position : Vector2,
    velocity : Vector2,
    acceleration : Vector2,
    collider_dimensions : Vector2,
    colliding_top : bool,
    colliding_bottom : bool,
    colliding_left : bool,
    colliding_right : bool,
    sprite: PlayerSprite,
}

impl Player{
    pub fn new(texture: Texture2D, scale: Vector2) -> Player{
        let s = PlayerSprite::new(texture,scale);
        Player{
            position : Vector2{x:200.0,y:200.0},
            velocity : Vector2{x:0.0,y:0.0},
            acceleration : Vector2{x:0.0,y:0.0},
            collider_dimensions : Vector2{x:0.0,y:0.0},
            colliding_top : false,
            colliding_bottom : false,
            colliding_left : false,
            colliding_right : false,
            sprite: s,
        }   
    }
}

impl Entity for Player{
    fn get_position(&self) -> &Vector2 { &self.position }
    fn get_velocity(&self) -> &Vector2 { &self.velocity }
    fn get_acceleration(&self) -> &Vector2 { &self.acceleration }
    fn get_collider_dimensions(&self) -> &Vector2 { &self.collider_dimensions }

    fn get_sprite(&self) -> &impl Sprite { &self.sprite }

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