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
    fn render(&mut self, rl:&mut RaylibHandle, thread:&RaylibThread);
    fn get_status(&self) -> AppStatus;

    fn get_next(&self) -> i32;
    fn init(&mut self, rl:&RaylibHandle);

}