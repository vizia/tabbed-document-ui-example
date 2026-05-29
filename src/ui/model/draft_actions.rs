use super::UiModel;
use crate::ui::commands::{documents, drafts};
use crate::ui::events::OpenMode;
use crate::ui::model::{DocumentKind, TabId, TabKind};
use vizia::prelude::*;

impl UiModel {
    pub fn update_new_name(&self, tab_id: TabId, name: String) {
        let mut drafts = self.new_tab_drafts.get_untracked();
        let draft = drafts.entry(tab_id).or_default();
        draft.name = name;
        self.new_tab_drafts.set(drafts);
    }

    pub fn set_new_type(&self, tab_id: TabId, selected: Option<DocumentKind>) {
        let mut drafts = self.new_tab_drafts.get_untracked();
        let draft = drafts.entry(tab_id).or_default();
        draft.selected_type = selected;
        self.new_tab_drafts.set(drafts);
    }

    pub fn pick_new_directory(&self, _cx: &mut EventContext, tab_id: TabId) {
        drafts::pick_draft_directory(*self, tab_id);
    }

    pub fn submit_new_tab(&self, cx: &mut EventContext, tab_id: TabId) {
        let tabs = self.tabs.get_untracked();
        let is_live_new_tab = tabs
            .iter()
            .any(|tab| tab.id == tab_id && matches!(tab.kind, TabKind::New(_)));
        if !is_live_new_tab {
            return;
        }

        let draft = self
            .new_tab_drafts
            .get_untracked()
            .get(&tab_id)
            .cloned()
            .unwrap_or_default();
        let typed_name = draft.name;
        let selected_type = draft.selected_type;
        let selected_directory = draft.directory;

        if typed_name.trim().is_empty() {
            self.update_status("Enter a file name before pressing OK.");
            return;
        }

        let Some(doc_type) = selected_type else {
            self.update_status("Select a file type.");
            return;
        };

        let Some(directory) = selected_directory else {
            self.update_status("Select a directory.");
            return;
        };

        let extension = match doc_type {
            DocumentKind::Text => "txt",
            DocumentKind::Image => "bmp",
        };

        let file_path = directory.join(format!("{}.{}", typed_name.trim(), extension));

        self.update_status(format!("Creating {}...", file_path.display()));
        documents::create_document(cx, file_path, doc_type, OpenMode::ReplaceActiveNewTab);
    }
}
