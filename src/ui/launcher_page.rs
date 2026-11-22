use iced::{
    Element,
    widget::{Scrollable, column, container, row},
};

use crate::{events::update::AppEvent, state::app_state::AppState};

impl AppState {
    fn launcher_page(&self) -> Element<'_, AppEvent> {
        container(row![self.instance_list()]).into()
    }

    fn instance_list(&self) -> Element<'_, AppEvent> {
        Scrollable::new(column![]).into()
    }
}
