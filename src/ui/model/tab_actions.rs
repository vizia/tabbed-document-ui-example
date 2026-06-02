use super::UiModel;
use super::tab_title::{refresh_document_tab_titles_for, tabs_reference_document};
use crate::ui::model::{DocumentId, NewTabState, TabId, TabKind, TabState};
use std::collections::HashMap;
use vizia::prelude::*;

impl UiModel {
    fn has_detached_window_for_document(&self, document_id: DocumentId) -> bool {
        self.detached_windows
            .get_untracked()
            .into_iter()
            .any(|window| window.document_id == document_id)
    }

    pub fn add_new_tab(&self) {
        let tab_id = self.next_tab_id.get_untracked();
        self.next_tab_id.set(tab_id + 1);

        let mut tabs = self.tabs.get_untracked();
        tabs.push(TabState {
            id: tab_id,
            title: "New".to_string(),
            kind: TabKind::New(NewTabState::empty()),
            closable: true,
        });
        self.tabs.set(tabs);

        let mut drafts = self.new_tab_drafts.get_untracked();
        drafts.insert(tab_id, Default::default());
        self.new_tab_drafts.set(drafts);

        self.active_tab_id.set(Some(tab_id));
        self.record_tab_visit(tab_id);
    }

    pub fn ensure_home_tab(&self) {
        let tabs = self.tabs.get_untracked();
        if let Some(tab) = tabs.iter().find(|tab| matches!(tab.kind, TabKind::Home)) {
            self.active_tab_id.set(Some(tab.id));
            self.record_tab_visit(tab.id);
            return;
        }

        let tab_id = self.next_tab_id.get_untracked();
        self.next_tab_id.set(tab_id + 1);

        let mut tabs = tabs;
        tabs.push(
            TabState {
                id: tab_id,
                title: "Home".to_string(),
                kind: TabKind::Home,
                closable: true,
            },
        );
        self.tabs.set(tabs);
        self.active_tab_id.set(Some(tab_id));
        self.record_tab_visit(tab_id);
    }

    pub fn activate_tab(&self, tab_id: TabId) {
        self.active_tab_id.set(Some(tab_id));
        self.record_tab_visit(tab_id);
    }

    pub fn close_tab(&self, tab_id: TabId) {
        let mut tabs = self.tabs.get_untracked();
        let removed = tabs
            .iter()
            .position(|tab| tab.id == tab_id && tab.closable)
            .map(|idx| tabs.remove(idx));

        if let Some(tab) = removed {
            if let TabKind::Document(document) = tab.kind {
                let mut docs = self.documents.get_untracked();
                let doc_still_open = tabs_reference_document(&tabs, document.id)
                    || self.has_detached_window_for_document(document.id);
                if !doc_still_open {
                    docs.retain(|doc| doc.id != document.id);
                    let mut ui_states = self.document_ui_states.get_untracked();
                    ui_states.remove(&document.id);
                    self.document_ui_states.set(ui_states);
                }
                self.documents.set(docs);
            }

            let docs = self.documents.get_untracked();
            refresh_document_tab_titles_for(&mut tabs, &docs);
            self.tabs.set(tabs);

            let mut mru = self.mru_tab_ids.get_untracked();
            mru.retain(|id| *id != tab_id);
            self.mru_tab_ids.set(mru);

            let mut drafts = self.new_tab_drafts.get_untracked();
            drafts.remove(&tab_id);
            self.new_tab_drafts.set(drafts);

            self.active_tab_id.set(self.next_mru_tab());
            self.persist_state();
        }
    }

    pub fn close_all_tabs(&self) {
        self.tabs.set(Vec::new());
        let detached_windows = self.detached_windows.get_untracked();
        let mut docs = self.documents.get_untracked();
        docs.retain(|doc| {
            detached_windows
                .iter()
                .any(|window| window.document_id == doc.id)
        });
        let retained_ids = docs.iter().map(|doc| doc.id).collect::<Vec<_>>();
        let mut ui_states = self.document_ui_states.get_untracked();
        ui_states.retain(|id, _| retained_ids.contains(id));
        self.document_ui_states.set(ui_states);
        self.documents.set(docs);
        self.new_tab_drafts.set(HashMap::new());
        self.active_tab_id.set(None);
        self.mru_tab_ids.set(Vec::new());
        self.persist_state();
    }

    pub fn duplicate_tab(&self, tab_id: TabId) {
        let Some(source_tab) = self
            .tabs
            .get_untracked()
            .into_iter()
            .find(|tab| tab.id == tab_id)
        else {
            return;
        };

        let new_tab_id = self.next_tab_id.get_untracked();
        self.next_tab_id.set(new_tab_id + 1);

        let mut tabs = self.tabs.get_untracked();
        let duplicated_tab = match source_tab.kind.clone() {
            TabKind::Home => TabState {
                id: new_tab_id,
                title: source_tab.title,
                kind: TabKind::Home,
                closable: source_tab.closable,
            },
            TabKind::New(_) => TabState {
                id: new_tab_id,
                title: source_tab.title,
                kind: TabKind::New(NewTabState::empty()),
                closable: source_tab.closable,
            },
            TabKind::Document(document) => TabState {
                id: new_tab_id,
                title: source_tab.title,
                kind: TabKind::Document(document),
                closable: source_tab.closable,
            },
        };
        tabs.push(duplicated_tab);
        self.tabs.set(tabs);

        if matches!(source_tab.kind, TabKind::New(_)) {
            let mut drafts = self.new_tab_drafts.get_untracked();
            if let Some(draft) = drafts.get(&tab_id).cloned() {
                drafts.insert(new_tab_id, draft);
                self.new_tab_drafts.set(drafts);
            }
        }

        self.active_tab_id.set(Some(new_tab_id));
        self.record_tab_visit(new_tab_id);
        self.persist_state();
    }

    pub fn close_other_tabs(&self, keep_tab_id: TabId) {
        let tabs = self.tabs.get_untracked();
        if !tabs.iter().any(|tab| tab.id == keep_tab_id) {
            return;
        }

        let mut remaining_tabs = tabs
            .into_iter()
            .filter(|tab| tab.id == keep_tab_id)
            .collect::<Vec<_>>();

        let mut docs = self.documents.get_untracked();
        docs.retain(|doc| {
            tabs_reference_document(&remaining_tabs, doc.id)
                || self.has_detached_window_for_document(doc.id)
        });
        let retained_ids = docs.iter().map(|doc| doc.id).collect::<Vec<_>>();
        let mut ui_states = self.document_ui_states.get_untracked();
        ui_states.retain(|id, _| retained_ids.contains(id));
        self.document_ui_states.set(ui_states);
        self.documents.set(docs);

        let current_docs = self.documents.get_untracked();
        refresh_document_tab_titles_for(&mut remaining_tabs, &current_docs);
        self.tabs.set(remaining_tabs);

        let mut drafts = self.new_tab_drafts.get_untracked();
        drafts.retain(|tab_id, _| *tab_id == keep_tab_id);
        self.new_tab_drafts.set(drafts);

        self.active_tab_id.set(Some(keep_tab_id));
        self.mru_tab_ids.set(vec![keep_tab_id]);
        self.persist_state();
    }

    pub fn display_document_in_window(&self, tab_id: TabId) {
        let Some(document_id) = self.tabs.get_untracked().into_iter().find_map(|tab| {
            if tab.id == tab_id {
                if let TabKind::Document(document) = tab.kind {
                    Some(document.id)
                } else {
                    None
                }
            } else {
                None
            }
        }) else {
            return;
        };

        let window_id = self.next_detached_window_id.get_untracked();
        self.next_detached_window_id.set(window_id + 1);

        let mut detached_windows = self.detached_windows.get_untracked();
        detached_windows.push(crate::ui::model::DetachedWindowState {
            id: window_id,
            document_id,
        });
        self.detached_windows.set(detached_windows);
    }

    pub fn clear_detached_document_window(&self, window_id: u64) {
        let mut detached_windows = self.detached_windows.get_untracked();
        let Some(document_id) = detached_windows
            .iter()
            .find(|window| window.id == window_id)
            .map(|window| window.document_id)
        else {
            return;
        };

        detached_windows.retain(|window| window.id != window_id);
        self.detached_windows.set(detached_windows);

        if tabs_reference_document(&self.tabs.get_untracked(), document_id)
            || self.has_detached_window_for_document(document_id)
        {
            return;
        }

        let mut docs = self.documents.get_untracked();
        docs.retain(|doc| doc.id != document_id);
        let mut ui_states = self.document_ui_states.get_untracked();
        ui_states.remove(&document_id);
        self.document_ui_states.set(ui_states);
        self.documents.set(docs);
    }

    fn next_mru_tab(&self) -> Option<TabId> {
        let tab_ids = self
            .tabs
            .get_untracked()
            .into_iter()
            .map(|tab| tab.id)
            .collect::<Vec<_>>();
        self.mru_tab_ids
            .get_untracked()
            .into_iter()
            .find(|id| tab_ids.contains(id))
    }

    pub(crate) fn record_tab_visit(&self, tab_id: TabId) {
        let mut mru = self.mru_tab_ids.get_untracked();
        mru.retain(|id| *id != tab_id);
        mru.insert(0, tab_id);
        self.mru_tab_ids.set(mru);
    }
}
