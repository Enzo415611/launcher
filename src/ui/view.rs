use iced::widget::container;

use crate::{AppState, events::update::AppEvent, state::ui::Page};

impl AppState {
    pub fn view(&self) -> iced::Element<'_, AppEvent> {
        match self.ui_state.current_page {
            Page::Auth => self.auth_page().into(),
            Page::Launcher => container("").into(),
            Page::Config => container("").into(),
        }
    }
}
