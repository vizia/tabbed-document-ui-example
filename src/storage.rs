use crate::ui::model::PersistedState;
use std::fs;
use std::io;
use std::path::PathBuf;

const APP_DIR: &str = "tabbed-document-ui-example";
const STATE_FILE: &str = "state.json";

fn storage_dir() -> io::Result<PathBuf> {
    let base = dirs::config_dir().ok_or_else(|| io::Error::other("No config directory"))?;
    Ok(base.join(APP_DIR))
}

fn state_path() -> io::Result<PathBuf> {
    Ok(storage_dir()?.join(STATE_FILE))
}

pub fn load_persisted_state() -> io::Result<Option<PersistedState>> {
    let path = state_path()?;
    load_persisted_state_from_path(&path)
}

pub fn save_persisted_state(state: &PersistedState) -> io::Result<()> {
    let path = state_path()?;
    save_persisted_state_to_path(&path, state)
}

fn load_persisted_state_from_path(path: &PathBuf) -> io::Result<Option<PersistedState>> {
    if !path.exists() {
        return Ok(None);
    }

    let data = fs::read_to_string(path)?;
    let parsed = serde_json::from_str::<PersistedState>(&data).map_err(io::Error::other)?;
    Ok(Some(parsed))
}

fn save_persisted_state_to_path(path: &PathBuf, state: &PersistedState) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let data = serde_json::to_string_pretty(state).map_err(io::Error::other)?;
    fs::write(path, data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::model::{AppLanguage, AppSettings, PersistedState};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn storage_round_trip_persists_state() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time went backwards")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("tabbed-ui-state-test-{unique}.json"));

        let state = PersistedState {
            settings: AppSettings {
                open_home_on_startup: false,
                language: AppLanguage::Spanish,
            },
            open_files: vec![PathBuf::from("/tmp/a.txt"), PathBuf::from("/tmp/b.bmp")],
        };

        save_persisted_state_to_path(&path, &state).expect("save should succeed");
        let loaded = load_persisted_state_from_path(&path)
            .expect("load should succeed")
            .expect("state should exist");

        assert_eq!(loaded.settings.open_home_on_startup, false);
        assert!(matches!(loaded.settings.language, AppLanguage::Spanish));
        assert_eq!(loaded.open_files.len(), 2);

        let _ = fs::remove_file(path);
    }
}
