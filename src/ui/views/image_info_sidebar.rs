use crate::ui::model::{DocumentId, ImageState, UiModel};
use vizia::prelude::*;

pub fn build_image_info_sidebar(
    cx: &mut Context,
    app: UiModel,
    doc_id: DocumentId,
    path_display: String,
    image_state: Option<ImageState>,
) {
    VStack::new(cx, move |cx| {
        Label::new(cx, Localized::new("info")).class("section-title");

        HStack::new(cx, |cx| {
            Label::new(cx, Localized::new("path"));
            Label::new(cx, path_display.clone())
                .width(Stretch(1.0))
                .text_wrap(true);
        })
        .height(Auto)
        .gap(Pixels(8.0));

        if let Some(image_state) = &image_state {
            HStack::new(cx, move |cx| {
                Label::new(cx, Localized::new("size"));
                Label::new(cx, format!("{}x{}", image_state.width, image_state.height));
            })
            .height(Auto)
            .gap(Pixels(8.0));

            HStack::new(cx, {
                let app = app.clone();
                move |cx| {
                    Label::new(cx, Localized::new("last_click"));

                    let value = app.documents.map(move |_| {
                        app.document_by_id(doc_id)
                            .and_then(|doc| doc.image_state)
                            .and_then(|state| state.last_click)
                            .map(|(x, y)| format!("{x:.1}, {y:.1} px"))
                            .unwrap_or_else(|| "-".to_string())
                    });

                    Label::new(cx, value);
                }
            })
            .height(Auto)
            .gap(Pixels(8.0));
        }
    })
    .class("info-sidebar")
    .width(Stretch(1.0))
    .height(Stretch(1.0))
    .gap(Pixels(8.0));
}
