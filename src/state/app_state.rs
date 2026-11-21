use std::sync::Arc;

use iced::Task;
use lighty_launcher::{UserProfile, auth::MicrosoftAuth, event::EventBus};
use tokio::sync::Mutex;

use crate::events::update::AppEvent;

#[derive(Clone)]
pub struct AppState {
    pub profile: Option<UserProfile>,
    pub event_bus: EventBus,
    pub auth: Arc<Mutex<Option<MicrosoftAuth>>>,
}

impl std::fmt::Display for AppState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("")
    }
}

impl AppState {
    pub fn new() -> (Self, Task<AppEvent>) {
        (
            Self {
                profile: None,
                event_bus: EventBus::new(100),
                auth: Arc::new(Mutex::new(None)),
            },
            Task::batch(vec![Task::done(AppEvent::LoadProfile)]),
        )
    }
}
