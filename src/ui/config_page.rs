use iced::{Widget, widget::container};

use crate::{events::update::AppEvent, state::app_state::AppState};

impl AppState {
    pub fn config_page(&self) -> impl Widget<AppEvent> {
        container("")
    }
}
