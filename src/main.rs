#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::Element;
use iced::widget::*;

fn main() {
    iced::application(App::default, App::update, App::view)
        .title("KTools")
        .resizable(true)
        .window_size((900, 600))
        .run()
        .unwrap();
}

#[derive(Debug, Clone)]
pub enum Message {
    Test,
}

#[derive(Debug, Clone)]
pub enum App {
    Home,
}

impl App {
    pub fn update(&mut self, message: Message) {
        //
    }
    pub fn view(&self) -> Element<'_, Message> {
        text("hello world!").into()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::Home
    }
}
