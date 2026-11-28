use iced::{
    Alignment, Length, Widget,
    widget::{column, container},
};

use crate::{
    events::{auth::AuthEvent, update::AppEvent},
    state::app_state::AppState,
    ui::components::rbutton::r_button,
};

impl AppState {
    pub fn auth_page(&self) -> impl Widget<AppEvent> {
        let code_and_url_button = if self.auth_state.is_auth_mode {
            Some(
                column![
                    r_button(format!("Copy Code - {}", self.auth_state.code))
                        .on_press(AppEvent::Auth(AuthEvent::CopyCode)),
                    r_button(format!("Open Url - {}", self.auth_state.url))
                        .on_press(AppEvent::Auth(AuthEvent::OpenUrl)),
                ]
                .align_x(Alignment::Center)
                .spacing(3),
            )
        } else {
            None
        };

        let login_button = if !self.auth_state.is_auth_mode {
            Some(r_button("Login with Microsoft").on_press(AppEvent::Auth(AuthEvent::StartAuth)))
        } else {
            None
        };

        let content = column![login_button, code_and_url_button,]
            .align_x(Alignment::Center)
            .spacing(3);
        container(content).center(Length::Fill)
    }
}
