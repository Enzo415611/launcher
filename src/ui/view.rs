use iced::{Element, Widget};

use crate::{AppState, events::update::AppEvent, state::ui::Page};

impl AppState {
    pub fn view(&self) -> Element<'_, AppEvent> {
        match self.ui_state.current_page {
            Page::Auth => self.auth_page().boxed(),
            Page::Launcher => self.launcher_page().boxed(),
            Page::Config => self.config_page().boxed(),
        }
    }
}
