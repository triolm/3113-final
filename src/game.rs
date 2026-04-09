
use raylib::prelude::*;
use crate::entity::Entity;

const FPS: u32 = 60;

#[derive(PartialEq)]
enum AppStatus {
    Running,
    Terminated,
}

pub struct Game {
    rl: RaylibHandle,
    thread: RaylibThread,
    app_status: AppStatus,
}

impl Game {
    pub fn new() -> Game {
        let (rl, thread) = 
            raylib::init()
                .size(640, 480)
                .title("Hello, World")
                .build();

        Game {
            rl,
            thread,
            app_status: AppStatus::Running,
        }
    }

    pub fn run(&mut self) {
        self.init();
        while self.app_status != AppStatus::Terminated {
            self.update();
        }
    }

    fn init(&mut self) {
        self.rl.set_target_fps(FPS);
    }

    fn update(&mut self) {
        // let mut ent = Entity::new();
        

        if self.rl.window_should_close() {
            self.app_status = AppStatus::Terminated;
            return;
        }

        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}
