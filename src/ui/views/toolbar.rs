use crate::ui::events::{SettingsEvent, TabEvent};
use crate::ui::model::AppLanguage;
use crate::ui::model::UiModel;
use vizia::prelude::*;

pub fn toolbar(cx: &mut Context, app: UiModel) {
    HStack::new(cx, move |cx| {
        Button::new(cx, |cx| Label::new(cx, Localized::new("home")))
            .on_press(|cx| cx.emit(TabEvent::RequestHomeTab));

        Button::new(cx, |cx| Label::new(cx, Localized::new("open")))
            .on_press(|cx| cx.emit(TabEvent::RequestOpenFile));

        Button::new(cx, |cx| Label::new(cx, Localized::new("new")))
            .on_press(|cx| cx.emit(TabEvent::CreateTab));

        Button::new(cx, |cx| Label::new(cx, Localized::new("close_all")))
            .on_press(|cx| cx.emit(TabEvent::CloseAllTabs));

        Spacer::new(cx);

        HStack::new(cx, move |cx| {
            Label::new(cx, Localized::new("language"));
            Select::new(cx, app.language_options, app.selected_language, true)
                .min_selected(1)
                .on_select(|cx, index| {
                    let language = if index == 1 {
                        AppLanguage::Spanish
                    } else {
                        AppLanguage::English
                    };
                    cx.emit(SettingsEvent::ChangeLanguage(language));
                })
                .width(Pixels(170.0));
        })
        .size(Auto)
        .gap(Pixels(8.0))
        .alignment(Alignment::Left);
    })
    .class("toolbar")
    .width(Stretch(1.0))
    .height(Auto)
    .gap(Pixels(8.0))
    .padding(Pixels(8.0));
}
