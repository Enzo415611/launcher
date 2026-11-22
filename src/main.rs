use crate::{state::app_state::AppState, subscription::subscription};

mod db;
mod events;
mod launcher;
mod state;
mod subscription;
mod ui;

fn main() -> iced::Result {
    lighty_launcher::core::AppState::init("Launcher").expect("");
    iced::application(AppState::new, AppState::update, AppState::view)
        .subscription(|app| subscription(app))
        .run()
}
