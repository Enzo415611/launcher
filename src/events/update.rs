use crate::{
    AppState,
    events::{auth::AuthEvent, db::DbEvent},
};
use iced::Task;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Db(DbEvent),
    Err(String),
    EventBus(lighty_launcher::event::Event),
    Auth(AuthEvent),
}

impl AppState {
    pub fn update(&mut self, event: AppEvent) -> Task<AppEvent> {
        match event {
            AppEvent::Db(e) => self.db_events(e),
            AppEvent::Err(err) => {
                eprintln!("{}", err);
                Task::none()
            }
            AppEvent::EventBus(event) => match event {
                _ => {
                    println!("{:?}", event);
                    Task::none()
                }
            },
            AppEvent::Auth(e) => self.auth_events(e),
        }
    }
}
