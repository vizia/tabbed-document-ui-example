use crate::ui::commands::documents;
use crate::ui::events::DocumentWorkflowEvent;
use crate::ui::model::UiModel;
use vizia::prelude::*;

impl UiModel {
    pub(crate) fn handle_workflow_events(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(
            |document_event: &DocumentWorkflowEvent, _| match document_event {
                DocumentWorkflowEvent::OpenPathRequested { path, open_mode } => {
                    self.open_path_async(cx, path.clone(), *open_mode)
                }
                DocumentWorkflowEvent::OpenPathSucceeded {
                    path,
                    document,
                    image_bytes,
                    open_mode,
                } => {
                    if let Some(image_bytes) = image_bytes {
                        documents::register_opened_image(
                            cx,
                            path.clone(),
                            image_bytes.clone(),
                            *open_mode,
                        );
                    }
                    self.apply_opened_document(path.clone(), document.clone(), *open_mode)
                }
                DocumentWorkflowEvent::OpenPathFailed(err) => {
                    documents::show_open_error(err.clone());
                    self.update_status(err.clone());
                }
                DocumentWorkflowEvent::SaveTextFailed(err) => {
                    self.update_status(format!("Failed to save document: {err}"));
                }
            },
        );
    }
}
