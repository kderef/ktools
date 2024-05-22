use raylib::prelude::*;

#[derive(Default)]
pub struct GameContext {
    pub dt: f32,
    pub mouse_left_down: bool,
    pub mouse_right_down: bool,
    pub screen_width: i32,
    pub screen_height: i32
}

impl GameContext {
    pub fn update(&mut self, rl: &RaylibHandle) {
        self.dt = rl.get_frame_time();
        self.mouse_left_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT);
        self.mouse_right_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT);
        self.screen_width = rl.get_screen_width();
        self.screen_height = rl.get_screen_height();
    }
}