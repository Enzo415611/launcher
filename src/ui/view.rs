use iced::widget::container;

use crate::{AppState, events::update::AppEvent};

impl AppState {
    pub fn view(&self) -> iced::Element<'_, AppEvent> {
        container("").into()
    }
}
