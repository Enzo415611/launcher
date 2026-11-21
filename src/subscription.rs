use iced::{
    Subscription,
    futures::{SinkExt, Stream},
    stream,
};
use lighty_launcher::event::EVENT_BUS;

use crate::{events::update::AppEvent, state::app_state::AppState};

pub fn subscription(_: &AppState) -> Subscription<AppEvent> {
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
