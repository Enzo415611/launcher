use iced::{
    Length, Widget,
    widget::{Button, button, text},
};

pub fn r_button<'a, Message>(content: impl Into<String>) -> Button<'a, Message> {
    button(text(content.into()).width(Length::Shrink).center().boxed()).width(Length::Shrink)
}
