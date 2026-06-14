//! The UI of the home page.

use iced::{Background, Border, border::Radius, widget::*};

use crate::{Message, base::icon_font};

#[allow(unused)]
pub fn search_bar<'a>(state: &'a str) -> TextInput<'a, Message> {
    use text_input::Status;

    // let icon = icon_font::search();

    let (icon_str, icon_font, _icon_shaping) = icon_font::advanced_text::search();

    let icon = text_input::Icon {
        font: icon_font,
        code_point: icon_str.chars().next().unwrap(),
        size: Some(15.into()),
        spacing: 10.0,
        side: text_input::Side::Left,
    };

    text_input("search for tools...", state)
        .on_input(Message::Search)
        .icon(icon)
        .style(|theme: &Theme, status| {
            let ex = theme.extended_palette();
            let pal = theme.palette();
            text_input::Style {
                background: Background::Color(ex.background.weak.color),
                border: Border {
                    color: match status {
                        Status::Focused { is_hovered: _ } => pal.text,
                        Status::Active | _ => ex.secondary.base.color,
                    },
                    width: 1.0,
                    radius: Radius::new(5.0),
                },
                icon: pal.text,
                placeholder: ex.secondary.base.color,
                value: pal.text,
                selection: ex.primary.base.color,
            }
        })
}
