use super::UiModel;
use crate::storage;
use crate::ui::model::PersistedState;
use std::io;
use vizia::prelude::SignalGet;

pub(crate) fn persist_model_state(model: &UiModel) -> io::Result<()> {
    let payload = build_persisted_state(model);
    storage::save_persisted_state(&payload)
}

fn build_persisted_state(model: &UiModel) -> PersistedState {
    let settings = model.settings.get_untracked();
    let tabs = model.tabs.get_untracked();
    let docs = model.documents.get_untracked();

    let open_files = tabs
        .iter()
        .filter_map(|tab| match tab.kind {
            super::TabKind::Document(ref document) => docs
                .iter()
                .find(|doc| doc.id == document.id)
                .map(|doc| doc.path.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();

    PersistedState {
        settings,
        open_files,
    }
}
