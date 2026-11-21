use iced::{
    Subscription, Task,
    futures::{SinkExt, Stream},
    stream,
};
use lighty_launcher::{
    UserProfile,
    auth::MicrosoftAuth,
    event::{EVENT_BUS, EventBus},
};

use crate::events::update::AppEvent;

mod events;
mod launcher;
mod ui;

#[derive(Clone)]
pub struct AppState {
    profile: Option<UserProfile>,
    event_bus: EventBus,
}

impl AppState {
    fn new() -> (Self, Task<AppEvent>) {
        (
            Self {
                profile: None,
                event_bus: EventBus::new(100),
            },
            Task::batch(vec![Task::done(AppEvent::LoadProfile)]),
        )
    }
}

fn subscription(app: &AppState) -> Subscription<AppEvent> {
    Subscription::batch(vec![Subscription::run(subscription_event_bus)])
}

fn subscription_event_bus() -> impl Stream<Item = AppEvent> {
    stream::channel(100, async |mut out| {
        let mut receiver = EVENT_BUS.subscribe();

        loop {
            match receiver.next().await {
                Ok(event) => {
                    if out.send(AppEvent::EventBus(event)).await.is_err() {
                        break;
                    }
                }
                Err(_) => continue,
            }
        }
    })
}

fn main() -> iced::Result {
    iced::application(AppState::new, AppState::update, AppState::view)
        .subscription(|app| subscription(app))
        .run()
}
