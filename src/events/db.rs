use iced::Task;
use indexmap::IndexSet;
use toasty::Db;

use crate::{
    db::{
        db::init_db,
        instance_model::{InstanceModel, LoaderModel},
    },
    events::{db::DbEvent::InitDbFuture, update::AppEvent},
    launcher::instance::Instance,
    state::app_state::AppState,
};

#[derive(Debug, Clone)]
pub enum DbEvent {
    InitDb,
    InitDbFuture(Result<Db, String>),
    GetAllInstances,
    GetAllInstancesFuture(IndexSet<Instance>),
    CreateInstance,
    CreateInstanceFuture(Result<Instance, String>),
    RemoveInstance(u32),
    RemoveInstanceFuture(Result<(), String>),
    EditInstance(u32),
    EditInstanceFuture(Result<(), String>),
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
                Task::batch(vec![Task::done(AppEvent::Db(DbEvent::GetAllInstances))])
            }
            DbEvent::GetAllInstances => {
                if let Some(mut db) = self.db.clone() {
                    return Task::perform(
                        async move {
                            let models = InstanceModel::all().exec(&mut db).await;
                            return models;
                        },
                        |r| match r {
                            Ok(models) => {
                                let models = models
                                    .into_iter()
                                    .map(|i| Instance::from(i))
                                    .collect::<IndexSet<Instance>>();
                                AppEvent::Db(DbEvent::GetAllInstancesFuture(models))
                            }
                            Err(err) => AppEvent::Err(err.to_string()),
                        },
                    );
                }
                Task::none()
            }
            DbEvent::GetAllInstancesFuture(models) => {
                self.instance.instances = models;
                Task::none()
            }
            DbEvent::CreateInstance => {
                if let Some(mut db) = self.db.clone() {
                    let new_instance = self.instance.create_instance.clone().unwrap_or_default();

                    return Task::perform(
                        async move {
                            let model = InstanceModel::create()
                                .id(&new_instance.id)
                                .name(&new_instance.name)
                                .loader(LoaderModel::from(&new_instance.loader))
                                .loader_version(&new_instance.loader_version)
                                .mc_version(&new_instance.mc_version)
                                .exec(&mut db)
                                .await;

                            let model = model
                                .map(|model| Instance::from(model))
                                .map_err(|err| err.to_string());
                            return model;
                        },
                        |r| AppEvent::Db(DbEvent::CreateInstanceFuture(r)),
                    );
                }
                Task::none()
            }
            DbEvent::CreateInstanceFuture(r) => match r {
                Ok(new_instance) => {
                    self.instance.instances.insert(new_instance);
                    Task::none()
                }
                Err(err) => Task::done(AppEvent::Err(err)),
            },
            DbEvent::RemoveInstance(id) => {
                if let Some(mut db) = self.db.clone() {
                    return Task::perform(
                        async move {
                            if let Err(err) = InstanceModel::delete_by_id(&mut db, id).await {
                                return Err(err.to_string());
                            }
                            Ok(())
                        },
                        |r| match r {
                            Ok(_) => AppEvent::Db(DbEvent::RemoveInstanceFuture(Ok(()))),
                            Err(err) => AppEvent::Err(err.to_string()),
                        },
                    );
                }
                Task::none()
            }
            DbEvent::RemoveInstanceFuture(_) => Task::none(),
            DbEvent::EditInstance(id) => {
                if let Some(mut db) = self.db.clone() {
                    if let Some(i) = self.instance.create_instance.clone() {
                        let edit_model = InstanceModel::update_by_id(id)
                            .name(i.name)
                            .loader(LoaderModel::from(&i.loader))
                            .loader_version(i.loader_version)
                            .mc_version(i.mc_version);
                        return Task::perform(async move { edit_model.exec(&mut db).await }, |r| {
                            match r {
                                Ok(_) => AppEvent::Db(DbEvent::EditInstanceFuture(Ok(()))),
                                Err(err) => AppEvent::Err(err.to_string()),
                            }
                        });
                    }
                }
                Task::none()
            }
            DbEvent::EditInstanceFuture(r) => Task::none(),
        }
    }
}
