use std::sync::Arc;

use iced::Task;
use lighty_launcher::{UserProfile, auth::MicrosoftAuth, event::EventBus};
use toasty::Db;
use tokio::sync::Mutex;

use crate::{
    events::{db::DbEvent::InitDb, update::AppEvent},
    state::{instance::InstanceState, ui::UiState},
};

#[derive(Clone)]
pub struct AppState {
    pub db: Option<Db>,
    pub profile: Option<UserProfile>,
    pub event_bus: EventBus,
    pub auth: Arc<Mutex<Option<MicrosoftAuth>>>,
    pub instance: InstanceState,
    pub ui_state: UiState,
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
                db: None,
                profile: None,
                event_bus: EventBus::new(100),
                auth: Arc::new(Mutex::new(None)),
                instance: InstanceState::default(),
                ui_state: UiState::default(),
            },
            Task::batch(vec![Task::done(AppEvent::Db(InitDb))]),
        )
    }
}
