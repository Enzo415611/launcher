use iced::Task;
use indexmap::IndexSet;

use crate::{
    db::instance_model::{InstanceModel, LoaderModel},
    events::update::AppEvent,
    launcher::instance::{Instance, InstanceLoader},
    state::app_state::AppState,
};

#[derive(Debug, Clone)]
pub enum InstanceEvents {
    CreateInstanceEvents(CreateInstanceEvents),
    InitInstance,
    GetAllInstances,
    GetAllInstancesFuture(IndexSet<Instance>),
    CreateInstance,
    CreateInstanceFuture(Result<Instance, String>),
    RemoveInstance(u32),
    RemoveInstanceFuture(Result<(), String>),
    EditInstance(u32),
    EditInstanceFuture(Result<(), String>),
}

#[derive(Debug, Clone)]
pub enum CreateInstanceEvents {
    Name(String),
    Loader(InstanceLoader),
    LoaderVersion(String),
}

impl AppState {
    pub fn instance_events(&mut self, e: InstanceEvents) -> Task<AppEvent> {
        match e {
            InstanceEvents::CreateInstanceEvents(e) => match e {
                CreateInstanceEvents::Name(n) => {
                    self.instance.create_instance.name = n;
                    Task::none()
                }
                CreateInstanceEvents::Loader(l) => {
                    self.instance.create_instance.loader = Some(l);
                    Task::none()
                }
                CreateInstanceEvents::LoaderVersion(v) => {
                    self.instance.create_instance.loader_version = v;
                    Task::none()
                }
            },
            InstanceEvents::InitInstance => Task::none(),
            InstanceEvents::GetAllInstances => {
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
                                AppEvent::Instance(InstanceEvents::GetAllInstancesFuture(models))
                            }
                            Err(err) => AppEvent::Err(err.to_string()),
                        },
                    );
                }
                Task::none()
            }
            InstanceEvents::GetAllInstancesFuture(models) => {
                self.instance.instances = models;
                Task::none()
            }
            InstanceEvents::CreateInstance => {
                if let Some(mut db) = self.db.clone() {
                    let new_instance = self.instance.create_instance.clone();

                    return Task::perform(
                        async move {
                            let model = InstanceModel::create()
                                .id(&new_instance.id)
                                .name(&new_instance.name)
                                .loader(LoaderModel::from(
                                    &new_instance.loader.unwrap_or(InstanceLoader::Vanilla),
                                ))
                                .loader_version(&new_instance.loader_version)
                                .mc_version(&new_instance.mc_version)
                                .exec(&mut db)
                                .await;

                            let model = model
                                .map(|model| Instance::from(model))
                                .map_err(|err| err.to_string());
                            return model;
                        },
                        |r| AppEvent::Instance(InstanceEvents::CreateInstanceFuture(r)),
                    );
                }
                Task::none()
            }
            InstanceEvents::CreateInstanceFuture(r) => match r {
                Ok(new_instance) => {
                    self.instance.instances.insert(new_instance);
                    Task::none()
                }
                Err(err) => Task::done(AppEvent::Err(err)),
            },
            InstanceEvents::RemoveInstance(id) => {
                if let Some(mut db) = self.db.clone() {
                    return Task::perform(
                        async move {
                            if let Err(err) = InstanceModel::delete_by_id(&mut db, id).await {
                                return Err(err.to_string());
                            }
                            Ok(())
                        },
                        |r| match r {
                            Ok(_) => {
                                AppEvent::Instance(InstanceEvents::RemoveInstanceFuture(Ok(())))
                            }
                            Err(err) => AppEvent::Err(err.to_string()),
                        },
                    );
                }
                Task::none()
            }
            InstanceEvents::RemoveInstanceFuture(_) => Task::none(),
            InstanceEvents::EditInstance(id) => {
                if let Some(mut db) = self.db.clone() {
                    let i = self.instance.create_instance.clone();
                    let edit_model = InstanceModel::update_by_id(id)
                        .name(i.name)
                        .loader(LoaderModel::from(
                            &i.loader.unwrap_or(InstanceLoader::Vanilla),
                        ))
                        .loader_version(i.loader_version)
                        .mc_version(i.mc_version);
                    return Task::perform(
                        async move { edit_model.exec(&mut db).await },
                        |r| match r {
                            Ok(_) => AppEvent::Instance(InstanceEvents::EditInstanceFuture(Ok(()))),
                            Err(err) => AppEvent::Err(err.to_string()),
                        },
                    );
                }
                Task::none()
            }
            InstanceEvents::EditInstanceFuture(r) => Task::none(),
        }
    }
}
