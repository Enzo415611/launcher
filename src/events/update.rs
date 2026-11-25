use crate::{
    AppState,
    events::{auth::AuthEvent, db::DbEvent, instance::InstanceEvents, ui::UiEvents},
};
use iced::Task;

#[derive(Debug, Clone)]
pub enum AppEvent {
    Db(DbEvent),
    Err(String),
    EventBus(lighty_launcher::event::Event),
    Auth(AuthEvent),
    Ui(UiEvents),
    Instance(InstanceEvents),
}

impl AppState {
    pub fn update(&mut self, event: AppEvent) -> Task<AppEvent> {
        match event {
            AppEvent::Db(e) => self.db_events(e),
            AppEvent::Err(err) => {
                eprintln!("{}", err);
                Task::none()
            }
            AppEvent::Ui(e) => self.ui_events(e),
            AppEvent::EventBus(event) => match event {
                lighty_launcher::event::Event::Auth(e) => {
                    println!("{:?}", e);
                    Task::none()
                }
                _ => {
                    println!("{:?}", event);
                    Task::none()
                }
            },
            AppEvent::Auth(e) => self.auth_events(e),
            AppEvent::Instance(e) => self.instance_events(e),
        }
    }
}
