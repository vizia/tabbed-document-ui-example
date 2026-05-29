use crate::ui::events::SettingsEvent;
use crate::ui::model::UiModel;
use vizia::prelude::*;

pub fn build_home(cx: &mut Context, app: UiModel) {
    VStack::new(cx, move |cx| {
        Label::new(cx, Localized::new("welcome")).class("home-title");
        HStack::new(cx, move |cx| {
            Label::new(cx, Localized::new("open_on_startup")).describing("open-startup-switch");
            Switch::new(cx, app.settings.map(|s| s.open_home_on_startup))
                .id("open-startup-switch")
                .on_toggle(|cx| cx.emit(SettingsEvent::ToggleHomeOnStartup));
        })
        .height(Auto)
        .width(Auto)
        .alignment(Alignment::Left)
        .gap(Pixels(8.0));
    })
    .alignment(Alignment::Center)
    .gap(Pixels(12.0))
    .width(Stretch(1.0));
}
