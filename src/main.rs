use crate::{state::app_state::AppState, subscription::subscription};

mod events;
mod launcher;
mod state;
mod subscription;
mod ui;

fn main() -> iced::Result {
    iced::application(AppState::new, AppState::update, AppState::view)
        .subscription(|app| subscription(app))
        .run()
}
