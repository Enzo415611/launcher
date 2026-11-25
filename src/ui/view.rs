use iced::{Widget, widget::container};

use crate::{AppState, events::update::AppEvent, state::ui::Page};

impl AppState {
    pub fn view(&self) -> impl Widget<AppEvent> {
        container(match self.ui_state.current_page {
            Page::Auth => self.auth_page().boxed(),
            Page::Launcher => self.launcher_page().boxed(),
            Page::Config => self.config_page().boxed(),
            Page::CreateInstance => self.create_instance_page().boxed(),
        })
        .padding(5)
        .boxed()
    }
}
