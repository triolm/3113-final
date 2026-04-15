#[derive(PartialEq, Copy, Clone)]
pub enum AppStatus {
    Running,
    Terminated,
}

pub trait Scene{
    // fn new() -> impl Scene;
    fn process_input(&mut self);
    fn update(&mut self);
    fn render(&mut self);
    fn get_status(&self) -> AppStatus;

    fn run(&mut self) {
        while self.get_status() != AppStatus::Terminated {
            self.process_input();
            self.render();
            self.update();
        }
    }
}