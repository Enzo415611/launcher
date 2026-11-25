use lighty_launcher::{Launch, Loader, UserProfile, VersionBuilder, event::EventBus};

use crate::db::instance_model::{InstanceModel, LoaderModel};

#[derive(Debug, Clone, Default, Hash, PartialEq, Eq)]
pub struct Instance {
    pub id: u32,
    pub name: String,
    pub loader: Option<InstanceLoader>,
    pub loader_version: String,
    pub mc_version: String,
}

impl From<InstanceModel> for Instance {
    fn from(i: InstanceModel) -> Self {
        Self {
            id: i.id,
            name: i.name,
            loader: Some(InstanceLoader::from(i.loader)),
            loader_version: i.loader_version,
            mc_version: i.mc_version,
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum InstanceLoader {
    Vanilla,
    Fabric,
}

impl ToString for InstanceLoader {
    fn to_string(&self) -> String {
        match self {
            InstanceLoader::Vanilla => String::from("Vanilla"),
            InstanceLoader::Fabric => String::from("Fabric"),
        }
    }
}

impl InstanceLoader {
    pub const ALL: [InstanceLoader; 2] = [InstanceLoader::Vanilla, InstanceLoader::Fabric];
}

impl Default for InstanceLoader {
    fn default() -> Self {
        Self::Vanilla
    }
}

impl From<LoaderModel> for InstanceLoader {
    fn from(v: LoaderModel) -> Self {
        match v {
            LoaderModel::Vanilla => Self::Vanilla,
            LoaderModel::Fabric => Self::Fabric,
        }
    }
}

impl From<Loader> for InstanceLoader {
    fn from(value: Loader) -> Self {
        match value {
            Loader::Fabric => Self::Fabric,
            Loader::Vanilla => Self::Vanilla,
            _ => Self::Vanilla,
        }
    }
}

impl From<&InstanceLoader> for Loader {
    fn from(value: &InstanceLoader) -> Self {
        match value {
            InstanceLoader::Fabric => Self::Fabric,
            InstanceLoader::Vanilla => Self::Vanilla,
            _ => Self::Vanilla,
        }
    }
}

pub async fn load_instance(
    profile: &UserProfile,
    event_bus: &EventBus,
    instance: &Instance,
) -> anyhow::Result<()> {
    // criar uma instancia passando o nome, loader, loader versions(vanilla não precisa) e game version
    let mut instance = VersionBuilder::new(
        &instance.name,
        Loader::from(instance.loader.as_ref().unwrap_or(&InstanceLoader::Vanilla)),
        &instance.loader_version,
        &instance.mc_version,
    );

    instance
        .launch(profile, lighty_launcher::JavaDistribution::Temurin)
        .with_event_bus(&event_bus)
        .run()
        .await?;
    Ok(())
}
