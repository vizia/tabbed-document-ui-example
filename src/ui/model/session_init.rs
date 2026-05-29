use super::tab_title::{refresh_document_tab_titles_for, title_for_path};
use crate::storage;
use crate::ui::model::{
    AppLanguage, AppSettings, Document, DocumentState, TabId, TabKind, TabState,
};
use crate::worker;

pub(crate) struct RestoredSession {
    pub tabs: Vec<TabState>,
    pub active_tab_id: Option<TabId>,
    pub documents: Vec<Document>,
    pub settings: AppSettings,
    pub selected_language: Option<usize>,
    pub next_tab_id: u64,
    pub next_document_id: u64,
}

pub(crate) fn restore_session_state() -> RestoredSession {
    let persisted = storage::load_persisted_state()
        .ok()
        .flatten()
        .unwrap_or_default();

    let mut tabs = Vec::new();
    let mut documents = Vec::new();
    let mut next_tab_id = 1_u64;
    let mut next_document_id = 1_u64;

    if persisted.settings.open_home_on_startup {
        tabs.push(TabState {
            id: next_tab_id,
            title: "Home".to_string(),
            kind: TabKind::Home,
            closable: true,
        });
        next_tab_id += 1;
    }

    for file in &persisted.open_files {
        if let Ok(doc) = worker::load_document_from_path_blocking(file, next_document_id) {
            documents.push(doc.clone());
            tabs.push(TabState {
                id: next_tab_id,
                title: title_for_path(&doc.path),
                kind: TabKind::Document(DocumentState {
                    id: doc.id,
                    title: title_for_path(&doc.path),
                }),
                closable: true,
            });
            next_tab_id += 1;
            next_document_id += 1;
        }
    }

    refresh_document_tab_titles_for(&mut tabs, &documents);

    let active_tab_id = tabs.first().map(|t| t.id);
    let selected_language = match persisted.settings.language {
        AppLanguage::English => Some(0),
        AppLanguage::Spanish => Some(1),
    };

    RestoredSession {
        tabs,
        active_tab_id,
        documents,
        settings: persisted.settings,
        selected_language,
        next_tab_id,
        next_document_id,
    }
}
