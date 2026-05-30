use crate::ui::events::DocumentEvent;
use crate::ui::model::{DocumentId, ImageState, UiModel};
use crate::ui::views::image_info_sidebar::build_image_info_sidebar;
use vizia::prelude::*;

pub fn build_image_document_page(
    cx: &mut Context,
    app: UiModel,
    doc_id: DocumentId,
    path_display: String,
    image_state: Option<ImageState>,
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
    let image_path = path_display.clone();

    HStack::new(cx, move |cx| {
        ScrollView::new(cx, move |cx| {
            Image::new(cx, image_path.clone())
                .width(Auto)
                .height(Auto)
                .background_size(vec![BackgroundSize::Contain])
                .on_mouse_down(move |cx, _| {
                    let bounds = cx.bounds();
                    let x = cx.mouse().cursor_x - bounds.x;
                    let y = cx.mouse().cursor_y - bounds.y;
                    println!("Mouse down at logical coordinates: ({:.1}, {:.1})", x, y);
                    cx.emit(DocumentEvent::RecordImageClick(doc_id, x, y));
                })
                .class("doc-image")
                .alignment(Alignment::TopLeft);
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
                build_image_info_sidebar(
                    cx,
                    app.clone(),
                    doc_id,
                    path_display.clone(),
                    image_state,
                );
            },
        );
    })
    .size(Stretch(1.0))
    .gap(Pixels(10.0));
}
