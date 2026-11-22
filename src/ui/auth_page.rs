use iced::{Element, widget::container};

use crate::{events::update::AppEvent, state::app_state::AppState};

impl AppState {
    fn auth_page(&self) -> Element<'_, AppEvent> {
        container("").into()
    }
}
