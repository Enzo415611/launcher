use std::sync::Mutex;

use iced::{Task, futures::channel};
use lighty_launcher::UserProfile;

use crate::{AppState, launcher::launcher::load_profile};

#[derive(Debug)]
pub enum AppEvent {
    Err(String),
    LoadProfile,
    LoadProfileFuture(UserProfile),
    EventBus(lighty_launcher::event::Event),
    DeviceCode(String, String),
    StartLogin,
}

impl AppState {
    pub fn update(&mut self, event: AppEvent) -> Task<AppEvent> {
        match event {
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
            AppEvent::StartLogin => {
                let (tx, rx) = channel::oneshot::channel::<AppEvent>();
                let tx = Mutex::new(Some(tx));
                if let Ok(mut guard) = self.auth.try_lock() {
                    if let Some(auth) = guard.as_mut() {
                        auth.set_device_code_callback(move |code, url| {
                            if let Some(tx) = tx.lock().unwrap().take() {
                                _ = tx
                                    .send(AppEvent::DeviceCode(code.to_string(), url.to_string()));
                            }
                        });
                    }
                }
                Task::future(async move {
                    rx.await
                        .unwrap_or(AppEvent::Err("Login Failed".to_string()))
                })
            }
            AppEvent::DeviceCode(code, url) => {
                println!("code: {}, url: {}", code, url);
                Task::none()
            }
            AppEvent::LoadProfile => {
                let event_bus = self.event_bus.clone();
                let auth = self.auth.clone();

                return Task::perform(load_profile(auth, event_bus), |r| match r {
                    Ok(profile) => AppEvent::LoadProfileFuture(profile),
                    Err(err) => AppEvent::Err(err.to_string()),
                });
            }
            AppEvent::LoadProfileFuture(profile) => {
                self.profile = Some(profile);
                Task::none()
            }
        }
    }
}
