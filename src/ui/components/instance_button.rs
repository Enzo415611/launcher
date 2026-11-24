use iced::{
    Widget,
    widget::{button, row},
};

use crate::{events::update::AppEvent, launcher::instance::Instance};

pub fn instance_button(i: &Instance) -> impl Widget<AppEvent> {
    row![button(i.name.as_str())]
}
