mod ui_events;
mod workflow_events;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OpenMode {
    OpenInNewTab,
    ReplaceActiveNewTab,
}

pub use ui_events::{DocumentEvent, DraftEvent, SettingsEvent, TabContextEvent, TabEvent};
pub use workflow_events::{DocumentWorkflowEvent, ResourceEvent};
