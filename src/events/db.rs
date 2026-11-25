use iced::Task;
use toasty::Db;

use crate::{
    db::db::init_db,
    events::{db::DbEvent::InitDbFuture, instance::InstanceEvents, update::AppEvent},
    state::app_state::AppState,
};

#[derive(Debug, Clone)]
pub enum DbEvent {
    InitDb,
    InitDbFuture(Result<Db, String>),
}

impl AppState {
    pub fn db_events(&mut self, e: DbEvent) -> Task<AppEvent> {
        match e {
            DbEvent::InitDb => Task::perform(init_db(), |db| match db {
                Ok(db) => AppEvent::Db(InitDbFuture(Ok(db))),
                Err(err) => AppEvent::Err(err.to_string()),
            }),
            DbEvent::InitDbFuture(db) => {
                match db {
                    Ok(db) => self.db = Some(db),
                    Err(_) => {}
                }
                Task::batch(vec![Task::done(AppEvent::Instance(
                    InstanceEvents::GetAllInstances,
                ))])
            }
        }
    }
}
