use crate::ui::model::{DocumentId, DocumentKind, TabId};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct DocumentState {
    pub id: DocumentId,
    pub title: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct NewTabState;

impl NewTabState {
    pub fn empty() -> Self {
        Self
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq)]
pub struct NewTabDraft {
    pub name: String,
    pub selected_type: Option<DocumentKind>,
    pub directory: Option<PathBuf>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum TabKind {
    Home,
    New(NewTabState),
    Document(DocumentState),
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct TabState {
    pub id: TabId,
    pub title: String,
    pub kind: TabKind,
    pub closable: bool,
}
