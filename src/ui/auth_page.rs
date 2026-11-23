use iced::{
    Length, Widget,
    widget::{button, column, container, text},
};

use crate::{
    events::{auth::AuthEvent, update::AppEvent},
    state::app_state::AppState,
};

impl AppState {
    pub fn auth_page(&self) -> impl Widget<AppEvent> {
        let code_and_url_button =
            if !self.auth_state.url.is_empty() && !self.auth_state.code.is_empty() {
                Some(column![
                    button(text(format!("Copy Code: {}", self.auth_state.code)))
                        .on_press(AppEvent::Auth(AuthEvent::CopyCode)),
                    button(text(format!("Open Url: {}", self.auth_state.url)))
                        .on_press(AppEvent::Auth(AuthEvent::OpenUrl)),
                ])
            } else {
                None
            };
        let content = column![
            button(text(format!("Login with Microsoft")))
                .on_press(AppEvent::Auth(AuthEvent::StartAuth)),
            code_and_url_button,
        ];
        container(content).center(Length::Fill)
    }
}
