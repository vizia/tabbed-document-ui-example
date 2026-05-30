use super::text_info_sidebar::build_text_info_sidebar;
use crate::ui::events::DocumentEvent;
use crate::ui::model::{DocumentId, UiModel};
use vizia::prelude::*;

pub fn build_text_document_page(
    cx: &mut Context,
    app: UiModel,
    doc_id: DocumentId,
    path_display: String,
) {
    let sidebar_width = app
        .document_ui_states
        .map(move |_| Pixels(app.document_ui_state_for(doc_id).sidebar_width));
    let scroll_x = app
        .document_ui_states
        .map(move |_| app.document_ui_state_for(doc_id).scroll_x);
    let scroll_y = app
        .document_ui_states
        .map(move |_| app.document_ui_state_for(doc_id).scroll_y);

    let editor_text = Signal::new(
        app.document_by_id(doc_id)
            .and_then(|doc| doc.text_content)
            .unwrap_or_default(),
    );

    Binding::new(cx, app.documents, move |_cx| {
        let current_text = app
            .document_by_id(doc_id)
            .and_then(|doc| doc.text_content)
            .unwrap_or_default();

        if editor_text.get_untracked() != current_text {
            editor_text.set(current_text);
        }
    });

    HStack::new(cx, move |cx| {
        ScrollView::new(cx, move |cx| {
            Textbox::new_multiline(cx, editor_text, true)
                .size(Stretch(1.0))
                .min_size(Auto)
                .alignment(Alignment::TopLeft)
                .class("doc-text")
                .on_edit(move |cx, text| {
                    editor_text.set(text.clone());
                    cx.emit(DocumentEvent::UpdateTextBuffer(doc_id, text));
                });
        })
        .scroll_x(scroll_x)
        .scroll_y(scroll_y)
        .on_scroll(move |cx, x, y| {
            cx.emit(DocumentEvent::UpdateScrollPosition(doc_id, x, y));
        })
        .class("document-scroll")
        .width(Stretch(1.0))
        .height(Stretch(1.0));

        Resizable::new(
            cx,
            sidebar_width,
            ResizeStackDirection::Left,
            move |cx, new_size| {
                cx.emit(DocumentEvent::UpdateSidebarWidth(doc_id, new_size));
            },
            move |cx| {
                build_text_info_sidebar(cx, path_display.clone(), editor_text);
            },
        );
    })
    .size(Stretch(1.0))
    .gap(Pixels(10.0));
}
