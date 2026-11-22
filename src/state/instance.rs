use indexmap::IndexSet;

use crate::launcher::instance::Instance;

#[derive(Clone)]
pub struct InstanceState {
    pub create_instance: Option<Instance>,
    pub instances: IndexSet<Instance>,
}

impl Default for InstanceState {
    fn default() -> Self {
        Self {
            create_instance: None,
            instances: IndexSet::new(),
        }
    }
}
