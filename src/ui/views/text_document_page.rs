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
    let sidebar_width = Signal::new(Pixels(300.0));

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
        .class("document-scroll")
        .width(Stretch(1.0))
        .height(Stretch(1.0));

        Resizable::new(
            cx,
            sidebar_width,
            ResizeStackDirection::Left,
            move |_cx, new_size| sidebar_width.set(Pixels(new_size)),
            move |cx| {
                build_text_info_sidebar(cx, path_display.clone(), editor_text);
            },
        );
    })
    .size(Stretch(1.0))
    .gap(Pixels(10.0));
}
