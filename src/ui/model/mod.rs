pub mod document;
pub mod document_ui_state;
mod document_workflow;
mod draft_actions;
pub mod ids;
pub mod language;
pub mod model_data;
pub mod persisted_state;
mod persistence;
mod session_init;
pub mod settings;
mod settings_actions;
pub mod tab;
mod tab_actions;
mod tab_title;
#[cfg(test)]
mod tests;

pub use document::{Document, DocumentKind, ImageState};
pub use document_ui_state::{DocumentUiState, MIN_DOCUMENT_SIDEBAR_WIDTH};
pub use ids::{DocumentId, TabId};
pub use language::AppLanguage;
pub use model_data::{DetachedWindowState, UiModel};
pub use persisted_state::PersistedState;
pub use settings::AppSettings;
pub use tab::{DocumentState, NewTabDraft, NewTabState, TabKind, TabState};
