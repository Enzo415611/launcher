use iced::{
    Widget,
    widget::{Scrollable, column, container, row, text},
};

use crate::{events::update::AppEvent, state::app_state::AppState};

impl AppState {
    pub fn launcher_page(&self) -> impl Widget<AppEvent> {
        container(row![self.instance_list()])
    }

    fn instance_list(&self) -> impl Widget<AppEvent> {
        Scrollable::new(column![text("")])
    }
}
