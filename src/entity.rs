use raylib::prelude::*;

pub trait Entity {
    fn get_position(&self) -> Vector2;
    fn get_velocity(&self) -> Vector2;
    fn get_acceleration(&self) -> Vector2;
    fn get_collider_dimensions(&self) -> Vector2;
    fn is_active(&self) -> bool {
        true
    }

    fn is_colliding_top() -> bool;
    fn is_colliding_bottom() -> bool;
    fn is_colliding_left() -> bool;
    fn is_colliding_right() -> bool;

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
        self.set_velocity(self.get_velocity() + self.get_acceleration());
    }

    fn update_position(&mut self) {
        self.set_position(self.get_position() + self.get_velocity());
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
            self.set_position(self.get_position() - y_overlap);
            self.set_colliding_bottom(true);
        } else if self.get_velocity().y < 0.0 {
            self.set_position(self.get_position() + y_overlap);
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
            self.set_position(self.get_position() - x_overlap);
            self.set_colliding_right(true);
        } else if self.get_velocity().x < 0.0 {
            self.set_position(self.get_position() + x_overlap);
            self.set_colliding_left(true);
        }

        self.set_velocity(Vector2 { x: 0.0, y: 0.0 });
    }
}
