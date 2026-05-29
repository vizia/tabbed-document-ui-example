use crate::ui::dialogs;
use crate::ui::effects;
use crate::ui::events::{DocumentWorkflowEvent, OpenMode};
use crate::ui::model::DocumentKind;
use std::path::PathBuf;
use vizia::prelude::*;

pub fn request_open_file(cx: &mut EventContext) {
    if let Some(path) = effects::pick_file() {
        effects::emit_document_workflow_event(
            cx,
            DocumentWorkflowEvent::OpenPathRequested {
                path,
                open_mode: OpenMode::OpenInNewTab,
            },
        );
    }
}

pub fn create_document(
    cx: &EventContext<'_>,
    path: PathBuf,
    doc_type: DocumentKind,
    open_mode: OpenMode,
) {
    effects::spawn_create_document(cx, path, doc_type, open_mode);
}

pub fn open_document(cx: &EventContext<'_>, path: PathBuf, doc_id: u64, open_mode: OpenMode) {
    effects::spawn_open_document(cx, path, doc_id, open_mode);
}

pub fn save_text_document(cx: &EventContext<'_>, path: PathBuf, content: String) {
    effects::spawn_save_text_document(cx, path, content);
}

pub fn register_opened_image(
    cx: &mut EventContext,
    path: PathBuf,
    image_bytes: Vec<u8>,
    open_mode: OpenMode,
) {
    effects::register_image_resource(cx, path.as_path(), &image_bytes, open_mode);
}

pub fn show_open_error(err: String) {
    dialogs::show_open_error(&err);
}
