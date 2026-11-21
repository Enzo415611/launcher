use lighty_launcher::{Launch, Loader, UserProfile, VersionBuilder, event::EventBus};

#[derive(Debug)]
pub struct Instance {
    name: String,
    loader: InstanceLoader,
    loader_version: String,
    mc_version: String,
}

#[derive(Debug)]
pub enum InstanceLoader {
    Vanilla,
    Fabric,
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

impl From<InstanceLoader> for Loader {
    fn from(value: InstanceLoader) -> Self {
        match value {
            InstanceLoader::Fabric => Self::Fabric,
            InstanceLoader::Vanilla => Self::Vanilla,
            _ => Self::Vanilla,
        }
    }
}

pub async fn load_instance(profile: UserProfile, event_bus: &EventBus) -> anyhow::Result<()> {
    // criar uma instancia passando o nome, loader, loader versions(vanilla não precisa) e game version
    let mut instance = VersionBuilder::new("instance name", Loader::Vanilla, "", "1.21.1");
    instance
        .launch(&profile, lighty_launcher::JavaDistribution::Temurin)
        .with_event_bus(&event_bus)
        .run()
        .await?;
    Ok(())
}
