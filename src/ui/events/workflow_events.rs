use crate::ui::events::OpenMode;
use crate::ui::model::Document;
use std::path::PathBuf;

pub enum DocumentWorkflowEvent {
    OpenPathRequested {
        path: PathBuf,
        open_mode: OpenMode,
    },
    OpenPathSucceeded {
        path: PathBuf,
        document: Document,
        image_bytes: Option<Vec<u8>>,
        open_mode: OpenMode,
    },
    OpenPathFailed(String),
    SaveTextFailed(String),
}

pub enum ResourceEvent {
    CacheImageResource(PathBuf, Vec<u8>),
}
