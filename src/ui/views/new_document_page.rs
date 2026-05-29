use crate::ui::events::DraftEvent;
use crate::ui::model::UiModel;
use crate::ui::model::{DocumentKind, TabId};
use vizia::prelude::*;

pub fn build_new_tab(cx: &mut Context, app: UiModel, tab_id: TabId) {
    let name_signal = app.new_tab_drafts.map(move |drafts| {
        drafts
            .get(&tab_id)
            .map(|draft| draft.name.clone())
            .unwrap_or_default()
    });
    let type_index_signal = app.new_tab_drafts.map(move |drafts| {
        match drafts.get(&tab_id).and_then(|draft| draft.selected_type) {
            Some(DocumentKind::Text) => Some(0),
            Some(DocumentKind::Image) => Some(1),
            None => None,
        }
    });
    let directory_signal = app.new_tab_drafts.map(move |drafts| {
        drafts
            .get(&tab_id)
            .and_then(|draft| draft.directory.as_ref())
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "-".to_string())
    });

    VStack::new(cx, move |cx| {
        Label::new(cx, Localized::new("new_document")).class("section-title");

        HStack::new(cx, |cx| {
            Label::new(cx, Localized::new("name")).width(Pixels(120.0));
            Textbox::new(cx, name_signal)
                .width(Stretch(1.0))
                .placeholder(Localized::new("name_placeholder"))
                .on_edit(move |cx, value| cx.emit(DraftEvent::RenameDraft(tab_id, value)));
        })
        .gap(Pixels(10.0))
        .height(Auto)
        .alignment(Alignment::Left)
        .width(Stretch(1.0));

        HStack::new(cx, |cx| {
            Label::new(cx, Localized::new("type")).width(Pixels(120.0));
            Select::new(cx, app.new_type_options, type_index_signal, true)
                .on_select(move |cx, index| {
                    let kind = if index == 1 {
                        DocumentKind::Image
                    } else {
                        DocumentKind::Text
                    };
                    cx.emit(DraftEvent::ChooseDraftType(tab_id, Some(kind)));
                })
                .width(Stretch(1.0));
        })
        .height(Auto)
        .alignment(Alignment::Left)
        .width(Stretch(1.0))
        .gap(Pixels(10.0));

        HStack::new(cx, |cx| {
            Label::new(cx, Localized::new("directory")).width(Pixels(120.0));
            Binding::new(cx, directory_signal, move |cx| {
                let label = directory_signal.get();
                Label::new(cx, label).width(Stretch(1.0));
            });
            Button::new(cx, |cx| Label::new(cx, Localized::new("browse")))
                .on_press(move |cx| cx.emit(DraftEvent::ChooseDraftDirectory(tab_id)));
        })
        .height(Auto)
        .alignment(Alignment::Left)
        .width(Stretch(1.0))
        .gap(Pixels(10.0));

        Button::new(cx, |cx| Label::new(cx, Localized::new("ok")))
            .variant(ButtonVariant::Primary)
            .on_press(move |cx| cx.emit(DraftEvent::SubmitDraft(tab_id)))
            .width(Pixels(120.0));
    })
    .gap(Pixels(12.0))
    .padding(Pixels(12.0))
    .width(Stretch(1.0))
    .max_width(Pixels(800.0));
}
