use crate::{
    AppState,
    events::{auth::AuthEvent, db::DbEvent},
};
use iced::{Task, widget::pane_grid};

#[derive(Debug, Clone)]
pub enum AppEvent {
    Db(DbEvent),
    Err(String),
    EventBus(lighty_launcher::event::Event),
    Auth(AuthEvent),
    PaneResize(pane_grid::ResizeEvent),
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
            AppEvent::PaneResize(r) => {
                self.ui_state.launcher_grid.resize(r.split, r.ratio);
                Task::none()
            }
        }
    }
}
