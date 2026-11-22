use iced::{Task, futures::channel};
use lighty_launcher::UserProfile;
use tokio::sync::Mutex;

use crate::{
    AppState, events::update::AppEvent, launcher::launcher::load_profile, state::ui::Page,
};

#[derive(Debug)]
pub enum AuthEvent {
    StartAuth,
    DeviceCode(String, String),
    LoadProfile,
    LoadProfileFuture(UserProfile),
}

impl AppState {
    pub fn auth_events(&mut self, e: AuthEvent) -> Task<AppEvent> {
        match e {
            AuthEvent::StartAuth => {
                let (tx, rx) = channel::oneshot::channel::<AppEvent>();
                let tx = Mutex::new(Some(tx));
                if let Ok(mut guard) = self.auth.try_lock() {
                    if let Some(auth) = guard.as_mut() {
                        auth.set_device_code_callback(move |code, url| {
                            if let Some(tx) = tx.try_lock().unwrap().take() {
                                _ = tx.send(AppEvent::Auth(AuthEvent::DeviceCode(
                                    code.into(),
                                    url.into(),
                                )));
                            }
                        });
                    }
                }
                Task::future(async move {
                    rx.await
                        .unwrap_or(AppEvent::Err("Login Failed".to_string()))
                })
            }
            AuthEvent::DeviceCode(code, url) => {
                println!("code: {}, url: {}", code, url);
                Task::none()
            }
            AuthEvent::LoadProfile => {
                let event_bus = self.event_bus.clone();
                let auth = self.auth.clone();

                return Task::perform(load_profile(auth, event_bus), |r| match r {
                    Ok(profile) => AppEvent::Auth(AuthEvent::LoadProfileFuture(profile)),
                    Err(err) => AppEvent::Err(err.to_string()),
                });
            }
            AuthEvent::LoadProfileFuture(profile) => {
                self.profile = Some(profile);
                self.ui_state.current_page = Page::Launcher;
                Task::none()
            }
        }
    }
}
