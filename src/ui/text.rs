use iced::{
    Border, Color, Theme,
    widget::{TextInput, text_input},
};

use crate::Message;

/// text which is selectable by the mouse cursor (iced's `Text` is not selectable)
/// NOTE: this has a placeholder value of "", use `selectable_maybe` if you want custom placeholder.
#[allow(unused)]
pub fn selectable(text: &str) -> TextInput<'_, Message> {
    text_input("", text)
        .on_input(|_| Message::Ignore)
        .padding(0)
        .style(selectable::style)
}

/// `selectable()` but with a custom `placeholder` value
pub fn selectable_maybe<'a>(text: &'a str, placeholder: &'a str) -> TextInput<'a, Message> {
    text_input(placeholder, text)
        .on_input(|_| Message::Ignore)
        .padding(0)
        .style(selectable::style)
}

pub mod selectable {
    use super::*;
    use crate::base::BACKGROUND_TRANSPARENT;

    pub fn style(theme: &Theme, _status: text_input::Status) -> text_input::Style {
        text_input::Style {
            background: BACKGROUND_TRANSPARENT,
            border: Border::default()
                .width(0)
                .rounded(0)
                .color(Color::TRANSPARENT),
            icon: theme.extended_palette().background.base.text,
            placeholder: theme.extended_palette().background.weak.text,
            value: theme.extended_palette().background.base.text,
            selection: theme.extended_palette().primary.weak.color,
        }
    }

    pub fn danger(theme: &Theme, _status: text_input::Status) -> text_input::Style {
        text_input::Style {
            value: theme.extended_palette().danger.base.color,
            ..style(theme, _status)
        }
    }
}
