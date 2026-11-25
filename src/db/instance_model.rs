use crate::launcher::instance::{Instance, InstanceLoader};

#[derive(Debug, toasty::Model)]
pub struct InstanceModel {
    #[key]
    #[auto]
    pub id: u32,
    pub name: String,
    pub loader: LoaderModel,
    pub loader_version: String,
    pub mc_version: String,
}

impl From<&Instance> for InstanceModel {
    fn from(i: &Instance) -> Self {
        Self {
            id: i.id,
            name: i.name.to_string(),
            loader: LoaderModel::from(i.loader.as_ref().unwrap_or(&InstanceLoader::Vanilla)),
            loader_version: i.loader_version.to_string(),
            mc_version: i.mc_version.to_string(),
        }
    }
}

#[derive(Debug, toasty::Embed)]
pub enum LoaderModel {
    Vanilla,
    Fabric,
}

impl From<&InstanceLoader> for LoaderModel {
    fn from(l: &InstanceLoader) -> Self {
        match l {
            InstanceLoader::Vanilla => Self::Vanilla,
            InstanceLoader::Fabric => Self::Fabric,
        }
    }
}
