use iced::Task;
use lighty_launcher::event::EventBus;
use toasty::Db;

use crate::{
    events::{db::DbEvent::InitDb, update::AppEvent},
    state::{auth::AuthState, instance::InstanceState, ui::UiState},
};

#[derive(Clone)]
pub struct AppState {
    pub db: Option<Db>,
    pub instance: InstanceState,
    pub ui_state: UiState,
    pub auth_state: AuthState,
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
                instance: InstanceState::default(),
                ui_state: UiState::default(),
                auth_state: AuthState::default(),
            },
            Task::batch(vec![Task::done(AppEvent::Db(InitDb))]),
        )
    }
}
