use arboard::Clipboard;
use iced::Task;
use lighty_launcher::UserProfile;

use crate::{
    AppState, events::update::AppEvent, launcher::launcher::load_profile, state::ui::Page,
};

#[derive(Debug, Clone)]
pub enum AuthEvent {
    CopyCode,
    OpenUrl,
    StartAuth,
    DeviceCode(String, String),
    LoadProfileFuture(UserProfile),
}

impl AppState {
    pub fn auth_events(&mut self, e: AuthEvent) -> Task<AppEvent> {
        match e {
            AuthEvent::CopyCode => {
                let mut clipboard = Clipboard::new().unwrap();
                if let Err(err) = clipboard.set_text(&self.auth_state.code) {
                    return Task::done(AppEvent::Err(err.to_string()));
                }
                Task::none()
            }
            AuthEvent::OpenUrl => {
                if !self.auth_state.url.is_empty() {
                    if let Err(err) = open::that(&self.auth_state.url) {
                        return Task::done(AppEvent::Err(err.to_string()));
                    }
                }

                Task::none()
            }
            AuthEvent::StartAuth => {
                let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                let auth = self.auth_state.auth.clone();
                let event_bus = self.event_bus.clone();

                let auth_task = Task::perform(load_profile(auth, event_bus, tx), |r| match r {
                    Ok(profile) => AppEvent::Auth(AuthEvent::LoadProfileFuture(profile)),
                    Err(err) => AppEvent::Err(err.to_string()),
                });
                let callback_task = Task::run(
                    async_stream::stream! {
                        while let Some(event) = rx.recv().await {
                           yield event;
                        }
                    },
                    |event| event,
                );
                Task::batch(vec![auth_task, callback_task])
            }
            AuthEvent::DeviceCode(code, url) => {
                println!("{} {}", code, url);
                self.auth_state.code = code;
                self.auth_state.url = url;
                Task::none()
            }
            AuthEvent::LoadProfileFuture(profile) => {
                self.auth_state.profile = Some(profile);
                self.ui_state.current_page = Page::Launcher;
                Task::none()
            }
        }
    }
}
