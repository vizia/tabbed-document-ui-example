use crate::ui::model::AppLanguage;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppSettings {
    pub open_home_on_startup: bool,
    pub language: AppLanguage,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            open_home_on_startup: true,
            language: AppLanguage::English,
        }
    }
}
