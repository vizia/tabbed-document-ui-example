use crate::ui::model::persistence::persist_model_state;
use crate::ui::model::session_init::restore_session_state;
use crate::ui::model::{
    AppSettings, Document, DocumentId, DocumentUiState, NewTabDraft, TabId, TabState,
};
use std::collections::HashMap;
use vizia::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DetachedWindowState {
    pub id: u64,
    pub document_id: DocumentId,
}

#[derive(Clone, Copy)]
pub struct UiModel {
    pub tabs: Signal<Vec<TabState>>,
    pub active_tab_id: Signal<Option<TabId>>,
    pub documents: Signal<Vec<Document>>,
    pub document_ui_states: Signal<HashMap<DocumentId, DocumentUiState>>,
    pub detached_windows: Signal<Vec<DetachedWindowState>>,
    pub new_tab_drafts: Signal<HashMap<TabId, NewTabDraft>>,
    pub settings: Signal<AppSettings>,
    pub status: Signal<String>,
    pub language_options: Signal<Vec<String>>,
    pub selected_language: Signal<Option<usize>>,
    pub new_type_options: Signal<Vec<String>>,
    pub next_tab_id: Signal<u64>,
    pub next_document_id: Signal<u64>,
    pub next_detached_window_id: Signal<u64>,
    pub mru_tab_ids: Signal<Vec<TabId>>,
}

impl UiModel {
    pub fn new(_cx: &mut Context) -> Self {
        let session = restore_session_state();

        Self {
            tabs: Signal::new(session.tabs),
            active_tab_id: Signal::new(session.active_tab_id),
            documents: Signal::new(session.documents),
            document_ui_states: Signal::new(session.document_ui_states),
            detached_windows: Signal::new(Vec::new()),
            new_tab_drafts: Signal::new(HashMap::new()),
            settings: Signal::new(session.settings),
            status: Signal::new("Ready".to_string()),
            language_options: Signal::new(vec!["English".to_string(), "Español".to_string()]),
            selected_language: Signal::new(session.selected_language),
            new_type_options: Signal::new(vec!["Text".to_string(), "Bitmap".to_string()]),
            next_tab_id: Signal::new(session.next_tab_id),
            next_document_id: Signal::new(session.next_document_id),
            next_detached_window_id: Signal::new(1),
            mru_tab_ids: Signal::new(Vec::new()),
        }
    }

    pub fn update_status(&self, message: impl Into<String>) {
        self.status.set(message.into());
    }

    pub fn persist_state(&self) {
        if let Err(err) = persist_model_state(self) {
            self.status.set(format!("Failed to persist state: {err}"));
        }
    }
}
