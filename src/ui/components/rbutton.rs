use iced::{
    Length, Widget,
    border::Radius,
    widget::{Button, button, text},
};

pub fn r_button<'a, Message>(content: impl Into<String>) -> Button<'a, Message> {
    button(text(content.into()).width(Length::Shrink).center().boxed())
        .width(Length::Shrink)
        .style(|t, s| {
            let mut theme = button::primary(t, s);
            theme.border.radius = Radius::new(10);
            theme
        })
}
