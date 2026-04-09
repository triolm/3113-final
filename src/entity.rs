use raylib::prelude::*;

pub trait Entity{
    fn get_position(&self) -> Vector2;
    fn get_velocity(&self) -> Vector2;
    fn get_acceleration(&self) -> Vector2;
    fn get_collider_dimensions(&self) -> Vector2;
    fn is_active(&self) -> bool { true }

    fn set_position(&mut self, pos: Vector2);
    fn set_velocity(&mut self, vel: Vector2);
    fn set_acceleration(&mut self, acc: Vector2);
    fn set_collider_dimensions(&mut self, acc: Vector2);

    fn update_velocity(&mut self){
        self.set_velocity(self.get_velocity() + self.get_acceleration());
    }
    
    fn update_position(&mut self){
        self.set_position(self.get_position() + self.get_velocity());
    }

    fn is_colliding(&self, other: &impl Entity) -> bool{
        if !other.is_active() { return false }

        let x_dist: f32 = (self.get_position().x - other.get_position().x).abs() - 
            ((self.get_collider_dimensions().x + other.get_collider_dimensions().x) / 2.0);

        let y_dist: f32 = (self.get_position().y - other.get_position().y).abs() - 
            ((self.get_collider_dimensions().y + other.get_collider_dimensions().y) / 2.0);

        x_dist < 0.0 && y_dist < 0.0
    }
}