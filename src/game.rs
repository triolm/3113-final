
use raylib::prelude::*;
use crate::entity::{Entity, Player};


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
    player: Player
}

impl Game {
    pub fn new() -> Game {
        let (mut rl, thread) = 
            raylib::init()
                .size(640, 480)
                .title("Hello, World")
                .build();

        let texture = rl.load_texture(&thread, "assets/horse.jpg").unwrap();

        let mut p = Player::new(texture,Vector2{x:300.0,y:100.0});
        p.set_acceleration(Vector2 { x: 10.0, y:0.0 });

        rl.set_target_fps(FPS);

        Game {
            rl,
            thread,
            app_status: AppStatus::Running,
            player:p
        }
    }

    pub fn run(&mut self) {
        while self.app_status != AppStatus::Terminated {
            self.update();
        }
    }

    fn update(&mut self) {
        if self.rl.window_should_close() {
            self.app_status = AppStatus::Terminated;
            return;
        }

        
        let mut d = self.rl.begin_drawing(&self.thread);

        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

        self.player.update_velocity();
        self.player.update_position();
        self.player.render(&mut d);
    }
}
