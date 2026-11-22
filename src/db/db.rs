use anyhow::Error;
use std::{fs, path::PathBuf};
use toasty::{Db, models};

use crate::db::instance_model::InstanceModel;

pub async fn init_db() -> anyhow::Result<Db> {
    let path = db_path().join("instances.sqlite");
    if let Err(err) = fs::create_dir_all(&path) {
        return Err(Error::msg(err.to_string()));
    }
    let models = models!(InstanceModel);
    let db = toasty::Db::builder()
        .models(models)
        .connect(&format!("sqlite:.{}", &path.to_string_lossy()))
        .await?;

    db.push_schema().await?;

    Ok(db)
}

fn db_path() -> PathBuf {
    if let Some(path) = dirs::data_dir() {
        let path = path.join("Launcher");
        return path;
    }
    PathBuf::new()
}
