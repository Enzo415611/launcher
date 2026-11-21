use iced::Task;
use lighty_launcher::UserProfile;

use crate::{AppState, launcher::launcher::load_profile};

#[derive(Debug)]
pub enum AppEvent {
    Err(String),
    LoadProfile,
    LoadProfileFuture(UserProfile),
    EventBus(lighty_launcher::event::Event),
}

impl AppState {
    pub fn update(&mut self, event: AppEvent) -> Task<AppEvent> {
        match event {
            AppEvent::Err(err) => {
                eprintln!("{}", err);
                Task::none()
            }
            AppEvent::LoadProfile => {
                let event_bus = self.event_bus.clone();
                Task::perform(load_profile(event_bus), |r| match r {
                    Ok(profile) => AppEvent::LoadProfileFuture(profile),
                    Err(err) => AppEvent::Err(err.to_string()),
                })
            }
            AppEvent::LoadProfileFuture(profile) => {
                self.profile = Some(profile);
                Task::none()
            }
            AppEvent::EventBus(event) => {
                match event {
                    _ => {
                        println!("{:?}", event)
                    }
                }
                Task::none()
            }
        }
    }
}
