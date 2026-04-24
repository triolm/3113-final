
use raylib::prelude::*;
use crate::{entity::Entity, platformer::Platformer, scene::{AppStatus, Scene}};

const SCALE:f32 = 1.2;

pub struct Title {
    next_scene_index:i32,
    title: Platformer,
    app_status:AppStatus,

}

impl Title{
    pub fn new(texture:&str) -> Title{
        Title { 
            next_scene_index: -1,
            title: Platformer::new(texture.to_string(), Vector2{x:1200.0, y:675.0}, false),
            app_status: AppStatus::Running
        }
    }
    
}

impl Scene for Title{
    fn process_input(&mut self, rl:&RaylibHandle){
         if rl.is_key_released(KeyboardKey::KEY_SPACE) {self.next_scene_index = 18; }
         if rl.window_should_close() {
            self.app_status = AppStatus::Terminated;
            return;
        }
    }
    fn get_music(&self) ->i32 {
        return 0;
    }

    fn update(&mut self, _rl:&RaylibHandle) {}

    fn init(&mut self, _rl:&RaylibHandle) {
        self.next_scene_index = -1;
        self.title.set_position(Vector2 { x: 1200.0 /2.0, y: 675.0  /2.0});
        self.title.set_start_position(Vector2 { x: 1200.0 /2.0, y: 675.0  /2.0});
     }

    fn get_next(&self) -> i32 {
        self.next_scene_index
    }

    fn load(&mut self, rl:&mut RaylibHandle, thread:&RaylibThread) {
        self.title.load(rl, thread);
    }

    fn render(&mut self, d:&mut RaylibDrawHandle) { 
        d.clear_background(Color::WHITE);
        self.title.render(d);
    }

    fn get_status(&self) -> AppStatus {
        self.app_status
    }
}