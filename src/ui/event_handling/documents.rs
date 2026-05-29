use crate::ui::events::DocumentEvent;
use crate::ui::model::UiModel;
use vizia::prelude::*;

impl UiModel {
    pub(crate) fn handle_document_events(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|document_event: &DocumentEvent, _| match document_event {
            DocumentEvent::UpdateTextBuffer(doc_id, content) => {
                self.update_text_content(cx, *doc_id, content.clone())
            }
            DocumentEvent::RecordImageClick(doc_id, x, y) => {
                self.set_image_last_click(cx, *doc_id, *x, *y)
            }
        });
    }
}
