use raylib::prelude::*;

pub struct Sprite{
    texture: Option<Texture2D>,
    texture_path: String,
    scale: Vector2,
    animation_index: i32,
    sprite_sheet_cols: i32,
    sprite_sheet_rows: i32,
    start_index: i32,
    end_index: i32,
}

impl Sprite{
    pub fn new(texture_path:String, scale: Vector2) -> Sprite{

        Sprite{
            texture_path: texture_path,
            texture: None,
            scale: scale,
            animation_index: 0,
            sprite_sheet_cols: 1,
            sprite_sheet_rows: 1,
            start_index: 0,
            end_index: 1,
        }
    }

    pub fn set_sprite_sheet_cols(&mut self, cols:i32){
        self.sprite_sheet_cols = cols;
    }
    pub fn set_sprite_sheet_rows(&mut self, rows:i32){
        self.sprite_sheet_rows = rows;
    }

    pub fn get_sprite_sheet_cols(&self) -> i32{
        self.sprite_sheet_cols
    }
    pub fn get_sprite_sheet_rows(&self) -> i32{
        self.sprite_sheet_rows
    }

    pub fn set_start_index(&mut self, s:i32){
        self.start_index = s;
    }
    pub fn set_end_index(&mut self, e:i32){
        self.end_index = e;
    }

    pub fn increment_frame(&mut self){
        self.animation_index += 1;
        self.animation_index %=  self.end_index - self.start_index;
    }

    fn load(&mut self, rl:&mut RaylibHandle,  thread:&RaylibThread){
        self.texture = Some(rl.load_texture(&thread, &self.texture_path).unwrap());
        // println!("loaded {}", self.texture_path);
    }

    fn get_texture_area(&self) -> Rectangle{
        let texture = self.texture.as_ref().unwrap();

        let frame = self.animation_index + self.start_index;

        let u_coord:f32  = (((frame % self.sprite_sheet_cols) as f32
                            / self.sprite_sheet_cols as f32) * texture.width as f32) as f32;

        let v_coord:f32  = (((frame / self.sprite_sheet_cols) as f32
                            / self.sprite_sheet_rows as f32) * texture.height as f32) as f32;

        let slice_width: f32  = (texture.width / self.sprite_sheet_cols) as f32;
        let slice_height: f32 = (texture.height / self.sprite_sheet_rows) as f32;

        return Rectangle::new( 
            u_coord,     // top-left x-coord
            v_coord,     // top-left y-coord
            slice_width, // width of slice
            slice_height // height of slice
        );
    }

    fn get_scale(&self) -> &Vector2{ &self.scale }
    fn has_texture(&self) -> bool { !self.texture.is_none()}
    fn get_texture(&self) -> &Texture2D { &self.texture.as_ref().unwrap() }
}


pub trait Positioned {
    fn get_position(&self) -> &Vector2;
}

pub trait Entity: Positioned {

    fn set_start_position(&mut self, pos: Vector2);
    fn reset_position(&mut self);

    // fn get_position(&self) -> &Vector2;
    fn get_velocity(&self) -> &Vector2;
    fn get_acceleration(&self) -> &Vector2;
    fn get_collider_dimensions(&self) -> &Vector2;
    fn is_active(&self) -> bool {
        true
    }

    // i had to ask AI. i don't understand why i had to do htis.
    fn load(&mut self,rl:&mut RaylibHandle, thread:&RaylibThread) { self.get_sprite_mut().load(rl,thread) }

    fn total_mvement(&self) -> Vector2 { *self.get_velocity() }


    fn get_sprite_mut(&mut self) -> &mut Sprite;
    fn get_sprite(&self) -> &Sprite;

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

        if self.total_mvement().y > 0.0 {
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

        if self.total_mvement().x > 0.0 {
            let mut new_pos = *self
            .get_position();
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

    fn _render_sprite(&self, draw: &mut RaylibDrawHandle){
        if !self.is_active() { return; }
        if !self.get_sprite().has_texture() {return}
        
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

    fn render(&self, draw: &mut RaylibDrawHandle){
        self._render_sprite(draw);
    }
}