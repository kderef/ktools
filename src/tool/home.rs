use super::*;

#[derive(Default)]
pub struct Homescreen {}

impl Tool for Homescreen {
    fn name(&self) -> &'static str {
        "Home"
    }
    fn icon(&self) -> Text<'_> {
        icon_font::home()
    }

    fn update(&mut self, _message: crate::Message) -> Task<crate::Message> {
        Task::none()
    }
    fn view(&self) -> Element<'_, crate::Message> {
        "a".into()
    }
}
