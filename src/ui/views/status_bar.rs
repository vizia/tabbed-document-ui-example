use crate::ui::model::UiModel;
use crate::ui::model::{DocumentKind, TabKind};
use vizia::prelude::*;

pub fn status_bar(cx: &mut Context, app: UiModel) {
    Binding::new(cx, app.documents, move |cx| {
        let extra_info = if let Some(tab) = app.active_tab() {
            match tab.kind {
                TabKind::Document(document) => {
                    if let Some(doc) = app.document_by_id(document.id) {
                        match doc.kind {
                            DocumentKind::Text => {
                                let len = doc.text_content.as_ref().map_or(0, |text| text.len());
                                format!("Text: {len} chars")
                            }
                            DocumentKind::Image => {
                                let click = doc.image_state.and_then(|state| state.last_click);
                                if let Some((x, y)) = click {
                                    format!("Image click: {x:.1}, {y:.1} px")
                                } else {
                                    "Image click: -".to_string()
                                }
                            }
                        }
                    } else {
                        String::new()
                    }
                }
                _ => String::new(),
            }
        } else {
            String::new()
        };

        HStack::new(cx, move |cx| {
            Label::new(cx, app.status).class("status-text");
            Spacer::new(cx);
            Label::new(cx, extra_info.clone()).class("status-text");
        })
        .class("status-bar")
        .width(Stretch(1.0))
        .height(Auto)
        .padding(Pixels(8.0));
    });
}
