use crate::ui::model::AppSettings;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PersistedState {
    pub settings: AppSettings,
    pub open_files: Vec<PathBuf>,
}
