use iced::{Program, Theme};

use crate::{state::app_state::AppState, subscription::subscription};

mod db;
mod events;
mod launcher;
mod state;
mod subscription;
mod ui;

fn main() -> iced::Result {
    lighty_launcher::core::AppState::init("RLauncher").expect("");

    let env_content = include_str!("../.env");
    dotenvy::from_read(env_content.as_bytes()).expect("error reading the env file");

    iced::application(AppState::new, AppState::update, AppState::view)
        .subscription(|app| subscription(app))
        .theme(theme)
        .run()
}

fn theme(app: &AppState) -> Theme {
    app.ui_state.current_theme.to_owned()
}
