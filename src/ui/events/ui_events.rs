use crate::ui::model::{AppLanguage, DocumentId, DocumentKind, TabId};

pub enum TabEvent {
    RequestHomeTab,
    RequestOpenFile,
    CreateTab,
    CloseAllTabs,
    ActivateTab(TabId),
    CloseTab(TabId),
}

pub enum TabContextEvent {
    Duplicate(TabId),
    CloseOthers(TabId),
    DisplayInWindow(TabId),
}

pub enum SettingsEvent {
    ChangeLanguage(AppLanguage),
    ToggleHomeOnStartup,
}

pub enum DraftEvent {
    RenameDraft(TabId, String),
    ChooseDraftType(TabId, Option<DocumentKind>),
    ChooseDraftDirectory(TabId),
    SubmitDraft(TabId),
}

pub enum DocumentEvent {
    UpdateTextBuffer(DocumentId, String),
    RecordImageClick(DocumentId, f32, f32),
}
