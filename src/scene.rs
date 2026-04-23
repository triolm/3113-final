use raylib::prelude::*;


#[derive(PartialEq, Copy, Clone)]
pub enum AppStatus {
    Running,
    Terminated,
}

pub trait Scene{
    // fn new() -> impl Scene;
    fn process_input(&mut self, rl:&RaylibHandle);
    fn update(&mut self, rl:&RaylibHandle);
    fn render(&mut self, d:&mut RaylibDrawHandle);
    fn get_status(&self) -> AppStatus;

    fn get_next(&self) -> i32;
    fn init(&mut self, rl:&RaylibHandle);
    fn load(&mut self, rl:&mut RaylibHandle, thread:&RaylibThread);

    fn get_sound(&mut self) ->i32{
        return -1;
    }
    fn get_music(&self) ->i32{
        return -1;
    }

}