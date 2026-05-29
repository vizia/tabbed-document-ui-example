use super::UiModel;
use super::tab_title::{refresh_document_tab_titles_for, title_for_path};
use crate::ui::commands::documents;
use crate::ui::events::OpenMode;
use crate::ui::model::{Document, DocumentId, DocumentKind, DocumentState, TabKind, TabState};
use std::path::PathBuf;
use vizia::prelude::*;

impl UiModel {
    pub fn open_path_async(&self, cx: &mut EventContext<'_>, path: PathBuf, open_mode: OpenMode) {
        let doc_id = self.next_document_id.get_untracked();
        self.update_status(format!("Loading {}...", path.display()));
        documents::open_document(cx, path, doc_id, open_mode);
    }

    pub fn apply_opened_document(&self, path: PathBuf, document: Document, open_mode: OpenMode) {
        self.next_document_id
            .set(self.next_document_id.get_untracked().max(document.id + 1));

        let active_id = self.active_tab_id.get_untracked();
        let mut tabs = self.tabs.get_untracked();
        let replaced_new = if matches!(open_mode, OpenMode::ReplaceActiveNewTab) {
            if let Some(active_id) = active_id {
                if let Some(active_tab) = tabs.iter_mut().find(|tab| tab.id == active_id) {
                    if matches!(active_tab.kind, TabKind::New(_)) {
                        active_tab.title = title_for_path(&path);
                        active_tab.kind = TabKind::Document(DocumentState {
                            id: document.id,
                            title: title_for_path(&path),
                        });
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if !replaced_new {
            let tab_id = self.next_tab_id.get_untracked();
            self.next_tab_id.set(tab_id + 1);
            tabs.push(TabState {
                id: tab_id,
                title: title_for_path(&document.path),
                kind: TabKind::Document(DocumentState {
                    id: document.id,
                    title: title_for_path(&document.path),
                }),
                closable: true,
            });
            self.active_tab_id.set(Some(tab_id));
            self.record_tab_visit(tab_id);
        } else if let Some(active_id) = active_id {
            let mut drafts = self.new_tab_drafts.get_untracked();
            drafts.remove(&active_id);
            self.new_tab_drafts.set(drafts);
        }

        let mut docs = self.documents.get_untracked();
        docs.push(document);
        self.documents.set(docs);

        let docs = self.documents.get_untracked();
        refresh_document_tab_titles_for(&mut tabs, &docs);
        self.tabs.set(tabs);

        self.update_status("Opened file.");
        self.persist_state();
    }

    pub fn active_tab(&self) -> Option<TabState> {
        let tabs = self.tabs.get_untracked();
        let active = self.active_tab_id.get_untracked()?;
        tabs.iter().find(|tab| tab.id == active).cloned()
    }

    pub fn document_by_id(&self, id: DocumentId) -> Option<Document> {
        self.documents
            .get_untracked()
            .into_iter()
            .find(|doc| doc.id == id)
    }

    pub fn set_image_last_click(&self, cx: &mut EventContext<'_>, id: DocumentId, x: f32, y: f32) {
        let mut docs = self.documents.get_untracked();
        if let Some(doc) = docs.iter_mut().find(|doc| doc.id == id) {
            if let Some(image_state) = &mut doc.image_state {
                let image_x = cx.physical_to_logical(x);
                let image_y = cx.physical_to_logical(y);

                image_state.last_click = Some((image_x.max(0.0), image_y.max(0.0)));
            }
        }
        self.documents.set(docs);
    }

    pub fn update_text_content(&self, cx: &mut EventContext<'_>, id: DocumentId, content: String) {
        let mut docs = self.documents.get_untracked();
        let mut save_path: Option<PathBuf> = None;
        if let Some(doc) = docs.iter_mut().find(|doc| doc.id == id) {
            if matches!(doc.kind, DocumentKind::Text) {
                doc.text_content = Some(content.clone());
                save_path = Some(doc.path.clone());
            }
        }
        self.documents.set(docs);

        if let Some(path) = save_path {
            documents::save_text_document(cx, path, content);
        }
    }
}
